extern crate vail_types;
extern crate byteorder;
extern crate time;
extern crate rand;
extern crate crc;
extern crate openssl;

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate failure;

use std::net::TcpStream;
use std::io::{Read, Write, Cursor, Result};

use crc::crc32::checksum_ieee as crc32;

use vail_types::{types, methods};
use vail_types::serialize::*;
use vail_types::deserialize::*;

use byteorder::{ByteOrder, ReadBytesExt, LittleEndian as LE, BigEndian as BE};

use openssl::bn::*;
use openssl::rsa::Rsa;

use rand::RngCore;

use failure::Error;

mod math;
mod rsa;




#[derive(Debug, Fail)]
enum DeserializationError {
    //IO(std::io::Error),

    #[fail(display = "Uknown ID: 0x{:08x}", id)]
    UnknownId {
        id: u32,
    },

    #[fail(display = "Wrong ID, expected 0x{:08x}, got 0x{:08x}", expected, got_id)]
    WrongId {
        got_id: u32,
        expected: u32,
    },
}
/*
#[derive(Debug, Fail)]
enum SerializationError {
IO(std::io::Error),
}
*/

#[derive(Debug, Fail)]
enum SecurityError {
    #[fail(display = "Uknown ID: 0x{:08x}", id)]
    WrongHash {
        id: u32,
    },

    #[fail(display = "Wrong ID, expected 0x{:08x}, got 0x{:08x}", expected, got_id)]
    WrongSeqNo {
        got_id: u32,
        expected: u32,
    },
}


pub trait TransportMode where Self: Sized {
    fn new() -> Result<Self>;
    fn send(&mut self, packet: &[u8]) -> Result<()>;
    fn recv(&mut self) -> Result<Vec<u8>>;
}

#[derive(Debug)]
pub struct TcpFull {
    tcp_client: TcpStream,
    seq_no: i32,
}


impl TransportMode for TcpFull {
    fn new() -> Result<Self> {
        Ok(TcpFull {
            tcp_client: TcpStream::connect("149.154.167.40:443")?,
            seq_no: 0,
        })
    }

    fn send(&mut self, packet: &[u8]) -> Result<()> {
        //println!("WRITE TCP BEGIN");
        // 4 bytes for len, seq_no, and crc32
        let len = 4 * 3 + packet.len();
        let mut buffer = vec![0; len];

        LE::write_i32(&mut buffer[0..4], len as i32);
        LE::write_i32(&mut buffer[4..8], self.seq_no);
        (&mut buffer[8..]).write_all(packet)?;
        let checksum = crc32(&buffer[..len - 4]);
        LE::write_u32(&mut buffer[len - 4..], checksum);

        //println!("\nBYTES + MTPROTO + TCP HEADERS:{}", dump_bytes(buffer.as_ref()).unwrap());

        println!("SEQ_NO: {}", self.seq_no);
        self.tcp_client.write_all(&buffer)?;
        self.tcp_client.flush()?;

        self.seq_no += 1;

        //println!("WRITE TCP END");
        Ok(())
    }

    fn recv(&mut self) -> Result<Vec<u8>> {
        //println!("READ TCP");
        let len = self.tcp_client.read_i32::<LE>()?;
        let seq_no = self.tcp_client.read_i32::<LE>()?;

        if seq_no + 1 != self.seq_no {

        }

        let mut buffer = vec![0; len as usize - 12];

        self.tcp_client.read_exact(&mut buffer)?;

        println!("READ TCP:{}", dump_bytes(buffer.as_ref()).unwrap());


        let checksum = self.tcp_client.read_u32::<LE>();

        Ok(buffer)
    }
}

pub struct MtprotoClient<T: TransportMode> {
    transport: T,
    time_offset: i64,
}


impl<T: TransportMode> MtprotoClient<T> {
    pub fn new() -> Self {
        MtprotoClient {
            transport: T::new().unwrap(),
            time_offset: 0,
        }
    }

    fn get_msg_id(&self) -> i64 {
        let current_time = time::get_time();

        return (current_time.sec + self.time_offset) << 32 | (current_time.nsec & 0xffff_fffc) as i64;
    }

    pub fn send_unencrypted(&mut self, bytes: &[u8]) -> Result<()> {
        //println!("WRITE UNENCRYPTED BEGIN");
        let auth_key_id: i64 = 0;
        let message_id: i64 = self.get_msg_id();
        let message_data_length: i32 = bytes.len() as i32;

        let mut buffer = vec![0; 8 + 8 + 4 + bytes.len()];
        LE::write_i64(&mut buffer[0..8], auth_key_id);
        LE::write_i64(&mut buffer[8..16], message_id);
        LE::write_i32(&mut buffer[16..20], message_data_length);
        (&mut buffer[20..]).write_all(&bytes)?;

        //println!("\nBYTES + MTPROTO HEADERS:{}", dump_bytes(buffer.as_ref()).unwrap());
        self.transport.send(buffer.as_ref())
            //println!("WRITE UNENCRYPTED END");
    }

    pub fn send_obj_unencrypted<O: Serializable>(&mut self, obj: &O) -> Result<()> {
        let mut buf = Vec::new();
        buf.serialize(obj)?;

        //println!("SEND_OBJ:{}", dump_bytes(&buf).unwrap());
        self.send_unencrypted(&buf)
    }

    pub fn invoke_unencrypted<M: methods::Method>(&mut self, obj: &M) -> Result<M::ReturnType> {
        self.send_obj_unencrypted(obj)?;
        self.recv_obj_unencrypted()
    }

    pub fn recv_unencrypted(&mut self) -> Result<Vec<u8>> {
        //println!("READ UNENCRYPTED BEGIN");
        let buffer = self.transport.recv()?;

        let auth_key_id: i64 = LE::read_i64(&buffer[0..8]);
        println!("auth_key_id: {}", auth_key_id);
        let message_id: i64 = LE::read_i64(&buffer[8..16]);
        println!("message_id: {}", message_id);
        let message_data_length: i32 = LE::read_i32(&buffer[16..20]);

        //println!("READ UNENCRYPTED END");
        Ok(buffer[20..20+message_data_length as usize].to_vec())
    }

    pub fn recv_obj_unencrypted<O: Deserializable>(&mut self) -> Result<O> {
        let res = self.recv_unencrypted()?;
        //println!("RECV_OBJ:{}", dump_bytes(&res).unwrap());

        let mut cur = Cursor::new(res);

        cur.deserialize()
    }
}


fn main() {
    let mut m = MtprotoClient::<TcpFull>::new();

    authenticate(&mut m);
}

// Generating an auth_key so we can then encrypt our messages
// https://core.telegram.org/mtproto/auth_key
fn authenticate<T: TransportMode>(m: &mut MtprotoClient<T>) {
    // ## DH exchange initiation
    // 1) Client sends query to server
    // req_pq_multi#be7e8ef1 nonce:int128 = ResPQ;

    let nonce: i128 = rand::random();
    let req_pq_multi = methods::ReqPq {
        nonce,
    };


    // 2) Server sends response
    // resPQ#05162463 nonce:int128 server_nonce:int128 pq:string server_public_key_fingerprints:Vector<long> = ResPQ;

    let res_pq = m.invoke_unencrypted(&req_pq_multi).unwrap();

    if res_pq.nonce != nonce {
        panic!("Nonce is not nonce");
    }


    // ## Proof of work
    // 3) Client decomposes pq into prime factors such that p < q.
    // This starts a round of Diffie-Hellman key exchanges.

    let (p, q) = math::factor(BE::read_u64(&res_pq.pq));


    // ## Presenting proof of work; Server authentication
    let mut p_bytes = vec![0u8; 4];
    BE::write_u32(&mut p_bytes, p as u32);

    let mut q_bytes = vec![0u8; 4];
    BE::write_u32(&mut q_bytes, q as u32);


    // new_nonce := another (good) random number generated by the client; after this query, it is
    // known to both client and server;
    let new_nonce: [u8; 32] = rand::random();


    // data := a serialization of
    // p_q_inner_data#83c95aec pq:string p:string q:string nonce:int128 server_nonce:int128 new_nonce:int256 = P_Q_inner_data
    // OR
    // p_q_inner_data_temp#3c6a84d4 pq:string p:string q:string nonce:int128 server_nonce:int128 new_nonce:int256 expires_in:int = P_Q_inner_data;

    let pq_inner_data = types::PQInnerData {
        pq: res_pq.pq,
        p: p_bytes.clone(),
        q: q_bytes.clone(),
        nonce: nonce,
        server_nonce: res_pq.server_nonce,
        new_nonce: new_nonce
    };


    // data_with_hash := SHA1(data) + data + (any random bytes); such that the length equal 255 bytes;
    let mut pq_data_with_hash = sha_and_ser(&pq_inner_data);
    fill_to_random(&mut pq_data_with_hash, 255);


    // From the fingerprints given in res_pq, find all the matching keys and return the first one
    // If we don't find any matching keys, this is an issue
    let fingerprint = res_pq.server_public_key_fingerprints
        .iter()
        .filter(|k| rsa::RSA_PUBLIC_KEYS.contains_key(k))
        .next()
        .expect("No matching fingerprint found, thats bad yo.");

    let rsa_key = rsa::RSA_PUBLIC_KEYS.get(fingerprint).unwrap();

    // a 255-byte long number (big endian) is raised to the requisite power over the requisite
    // modulus, and the result is stored as a 256-byte number.
    let encrypted_data = do_rsa_with_key(&pq_data_with_hash, rsa_key);


    // ## Presenting proof of work; Server authentication
    // 4) Client sends query to server
    // req_DH_params#d712e4be nonce:int128 server_nonce:int128 p:string q:string public_key_fingerprint:long encrypted_data:string = Server_DH_Params

    let req_dh_params = methods::ReqDhParams {
        nonce: nonce,
        server_nonce: res_pq.server_nonce,
        p: p_bytes,
        q: q_bytes,
        public_key_fingerprint: *fingerprint,
        encrypted_data: encrypted_data,
    };


    // 5) Server responds in one of two ways:
    // server_DH_params_fail#79cb045d nonce:int128 server_nonce:int128 new_nonce_hash:int128 = Server_DH_Params;
    // server_DH_params_ok#d0e8075c nonce:int128 server_nonce:int128 encrypted_answer:string = Server_DH_Params;

    let dh_params = m.invoke_unencrypted(&req_dh_params).unwrap();

    match dh_params {
        types::ServerDhParams::Ok { nonce, server_nonce, encrypted_answer } => {
            // generate key and iv for aes from hashes of nonces
            let (tmp_key, mut tmp_iv) = generate_tmp_aes(server_nonce, new_nonce);

            let aes_key_dec = openssl::aes::AesKey::new_decrypt(&tmp_key).unwrap();
            let mut decrypted_answer = vec![0; encrypted_answer.len()];

            // encrypted_answer := AES256_ige_encrypt (answer_with_hash, tmp_aes_key, tmp_aes_iv);
            // here, tmp_aes_key is a 256-bit key, and tmp_aes_iv is a 256-bit initialization
            // vector. The same as in all the other instances that use AES encryption, the
            // encrypted data is padded with random bytes to a length divisible by 16 immediately
            // prior to encryption
            openssl::aes::aes_ige(
                &encrypted_answer,
                &mut decrypted_answer,
                &aes_key_dec,
                &mut tmp_iv,
                openssl::symm::Mode::Decrypt,
                );

            // create a cursor to be able to read
            let mut cursor_answer = Cursor::new(&decrypted_answer);

            // skip the hash, probably should verify it
            cursor_answer.set_position(20);

            // read the Dh inner data
            let dh_inner_data: types::ServerDhInnerData = cursor_answer.deserialize().unwrap();


            // 6) Client computes random 2048-bit number b (using a sufficient amount of entropy)
            // and sends the server a message
            // set_client_DH_params#f5045f1f nonce:int128 server_nonce:int128 encrypted_data:string = Set_client_DH_params_answer;

            // turn the dh_prime from bytes to a bignum
            let dh_prime = BigNum::from_slice(&dh_inner_data.dh_prime).unwrap();

            // genreate a 2048 bit random number
            let mut b = BigNum::new().unwrap();
            b.rand(2048, MsbOption::MAYBE_ZERO, false).unwrap();

            // turn g into a bignum
            let g = BigNum::from_u32(dh_inner_data.g as u32).unwrap();

            // create g_b which to put the result of g^b % dh_prime in  
            let mut g_b = BigNum::new().unwrap();
            let mut ctx = BigNumContext::new().unwrap();

            // g_b := pow(g, b) mod dh_prime;
            g_b.mod_exp(&g, &b, &dh_prime, &mut ctx).unwrap();


            // get data to encrypt
            let client_dh_inner_data = types::ClientDhInnerData {
                nonce: nonce,
                server_nonce: server_nonce,
                retry_id: 0,
                g_b: g_b.to_vec(),
            };

            // serialize and add hash at front
            let mut ser_dh_inner_data = sha_and_ser(&client_dh_inner_data);

            // pad up to a multiple of 16 with random bytes
            let new_len = ser_dh_inner_data.len() + to_mod(16, ser_dh_inner_data.len());
            fill_to_random(&mut ser_dh_inner_data, new_len);

            // use the same iv and key from earlier, I'm regennerating the iv since it changed
            // after it was used in the preivous decryption, will clone
            let (tmp_key, mut tmp_iv) = generate_tmp_aes(server_nonce, new_nonce);

            // encryption key
            let aes_key_enc = openssl::aes::AesKey::new_encrypt(&tmp_key).unwrap();
            let mut encrypted_dh = vec![0; ser_dh_inner_data.len()];

            //println!("SERIALIZE_DH:{}", dump_bytes(&ser_dh_inner_data).unwrap());

            // encrypt the data with hash and put it in encrypted_dh
            openssl::aes::aes_ige(
                &ser_dh_inner_data,
                &mut encrypted_dh,
                &aes_key_enc,
                &mut tmp_iv,
                openssl::symm::Mode::Encrypt,
                );

            //println!("ENCRYPTED_DH:{}", dump_bytes(&encrypted_dh).unwrap());

            // Finally send the encrypted data back to the server so complete the DH exchange
            let set_client_dh_params = methods::SetClientDhParams {
                nonce: nonce,
                server_nonce: server_nonce,
                encrypted_data: encrypted_dh,
            };

            // ## DH key exchange complete
            // 9) Server responds in one of three ways:
            // dh_gen_ok#3bcbf734 nonce:int128 server_nonce:int128 new_nonce_hash1:int128 = Set_client_DH_params_answer;
            // dh_gen_retry#46dc1fb9 nonce:int128 server_nonce:int128 new_nonce_hash2:int128 = Set_client_DH_params_answer;
            // dh_gen_fail#a69dae02 nonce:int128 server_nonce:int128 new_nonce_hash3:int128 = Set_client_DH_params_answer;

            let dh_answer = m.invoke_unencrypted(&set_client_dh_params).unwrap();
            println!("{:?}", &dh_answer);
        },

        _ => {
            println!("OHNO");
        }
    }
}

fn do_rsa_with_key(data: &[u8], rsa_key: &Rsa<openssl::pkey::Public>) -> Vec<u8> {
    let base = BigNum::from_slice(data).unwrap();

    let mut res = BigNum::new().unwrap();
    let mut ctx = BigNumContext::new().unwrap();

    res.mod_exp(&base, rsa_key.e(), rsa_key.n(), &mut ctx).unwrap();

    res.to_vec()
}



#[inline]
fn to_mod(len_mod: usize, len: usize) -> usize {
    (len_mod - len % len_mod) % len_mod
}

fn fill_to_random(buffer: &mut Vec<u8>, size: usize) {
    assert!(buffer.len() <= size);

    let end = buffer.len();
    buffer.resize(size, 0);

    rand::thread_rng().fill_bytes(&mut buffer[end..]);
}

fn sha_and_ser<T: Serializable>(obj: &T) -> Vec<u8> {
    let mut data = Cursor::new(vec![0u8; 255]);
    data.set_position(20); // leave room for sha1
    data.serialize(obj).unwrap();

    let end = data.position() as usize;

    let data_hash = openssl::sha::sha1(&mut data.get_mut()[20..end]);

    data.set_position(0);
    data.write_all(&data_hash).unwrap();

    data.into_inner()
}

fn add_and_hash(bytes_1: &[u8], bytes_2: &[u8]) -> [u8; 20] {
    let mut hasher = openssl::sha::Sha1::new();
    hasher.update(&bytes_1);
    hasher.update(&bytes_2);

    hasher.finish()
}

fn generate_tmp_aes(server_nonce: i128, new_nonce: [u8; 32]) -> (Vec<u8>, Vec<u8>) {
    let mut server_nonce_bytes = vec![0u8; 16];
    LE::write_i128(&mut server_nonce_bytes, server_nonce);

    let hash1 = add_and_hash(&new_nonce, &server_nonce_bytes);
    let hash2 = add_and_hash(&server_nonce_bytes, &new_nonce);
    let hash3 = add_and_hash(&new_nonce, &new_nonce);

    //tmp_aes_key := SHA1(new_nonce + server_nonce) + substr (SHA1(server_nonce + new_nonce), 0, 12);
    let mut tmp_aes_key = Vec::with_capacity(32);
    tmp_aes_key.extend(&hash1);
    tmp_aes_key.extend(&hash2[0..12]);

    //tmp_aes_iv := substr (SHA1(server_nonce + new_nonce), 12, 8) + SHA1(new_nonce + new_nonce) + substr (new_nonce, 0, 4);
    let mut tmp_aes_iv = Vec::with_capacity(32);
    tmp_aes_iv.extend(&hash2[12..20]);
    tmp_aes_iv.extend(&hash3);
    tmp_aes_iv.extend(&new_nonce[0..4]);

    (tmp_aes_key, tmp_aes_iv)
}



fn dump_bytes(buf: &[u8]) -> std::result::Result<String, std::fmt::Error> {
    use std::fmt::Write;

    let mut s: String = String::new();

    for (i, bytes) in buf.chunks(16).enumerate() {
        // hexdump -C
        write!(s, "\n{:08x}  ", i * 16)?;

        for (i, b) in bytes.iter().enumerate() {
            if i == 8 {
                write!(s, " ")?;
            }
            write!(s, "{:02x} ", b)?;
        }

        if bytes.len() < 16 {
            for x in 0..(16 - bytes.len()) {
                let num = 15 - x;
                if num == 8 {
                    write!(s, "    ")?;
                } else {
                    write!(s, "   ")?;
                }
            }
        }


        write!(s, " |")?;

        for b in bytes {
            if *b > 31 && *b < 127 {
                write!(s, "{}", char::from(*b))?;
            } else {
                write!(s, ".")?;
            }
        }

        write!(s, "|")?;
    }

    write!(s, "\n")?;

    Ok(s)
}

