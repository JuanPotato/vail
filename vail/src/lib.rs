#[macro_use]
extern crate tl_derive;
extern crate byteorder;
extern crate time;
extern crate rand;
extern crate crc;
extern crate openssl;
extern crate sha1;

use openssl::rsa::Rsa;
use openssl::bn::{BigNum, BigNumContext};

use crc::{crc32, Hasher32};

use std::io;
use std::io::{Write, Read, Cursor};
use std::ops::Deref;

use rand::Rng;

use std::net::{TcpStream, ToSocketAddrs};
use serialize::Serialize;
use deserialize::{Deserializer, Deserialize};

use byteorder::{WriteBytesExt, ReadBytesExt, LittleEndian, ByteOrder, BigEndian};

use std::env;

mod constructors;
mod functions;
mod serialize;
mod deserialize;
mod math;


// PROD_SERVER = "149.154.167.50:443";
// TEST_SERVER = "149.154.167.40:443";

// -----BEGIN RSA PUBLIC KEY-----
// MIIBCgKCAQEAwVACPi9w23mF3tBkdZz+zwrzKOaaQdr01vAbU4E1pvkfj4sqDsm6
// lyDONS789sVoD/xCS9Y0hkkC3gtL1tSfTlgCMOOul9lcixlEKzwKENj1Yz/s7daS
// an9tqw3bfUV/nqgbhGX81v/+7RFAEd+RwFnK7a+XYl9sluzHRyVVaTTveB2GazTw
// Efzk2DWgkBluml8OREmvfraX3bkHZJTKX4EQSjBbbdJ2ZXIsRrYOXfaA+xayEGB+
// 8hdlLmAjbCVfaigxX0CDqWeR1yFL9kwd9P0NsZRPsmoqVwMbMu7mStFai6aIhc3n
// Slv8kg9qv1m6XHVQY3PnEw+QQtqSIXklHwIDAQAB
// -----END RSA PUBLIC KEY-----

// -----BEGIN PUBLIC KEY-----
// MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAwVACPi9w23mF3tBkdZz+
// zwrzKOaaQdr01vAbU4E1pvkfj4sqDsm6lyDONS789sVoD/xCS9Y0hkkC3gtL1tSf
// TlgCMOOul9lcixlEKzwKENj1Yz/s7daSan9tqw3bfUV/nqgbhGX81v/+7RFAEd+R
// wFnK7a+XYl9sluzHRyVVaTTveB2GazTwEfzk2DWgkBluml8OREmvfraX3bkHZJTK
// X4EQSjBbbdJ2ZXIsRrYOXfaA+xayEGB+8hdlLmAjbCVfaigxX0CDqWeR1yFL9kwd
// 9P0NsZRPsmoqVwMbMu7mStFai6aIhc3nSlv8kg9qv1m6XHVQY3PnEw+QQtqSIXkl
// HwIDAQAB
// -----END PUBLIC KEY-----


type Int128 = (u64, u64);
type Int256 = (u64, u64, u64, u64);

#[derive(Debug)]
enum TlErrors {
    Io(io::Error),
}

#[derive(Debug)]
pub struct MtProtoConnection {
    pub conn: TcpStream,
    connected: bool,
    encrypted: bool,
    time_offset: i64,
    packet_seq_no: i32,
    message_seq_no: i32,
    auth_key: Vec<u8>,
    server_salt: u64,
    session_id: i64,
    auth_key_id: i64,
}

fn dump_bytes(buf: &[u8]) -> Result<String, std::fmt::Error> {
    use std::fmt::Write;

    let mut s: String = String::new();

    for (i, bytes) in buf.chunks(16).enumerate() { // hexdump -C
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
            if *b > 31  && *b < 127 {
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

fn i32_to_bytes(num: i32) -> [u8; 4] {
    [num as u8, (num >> 8) as u8, (num >> 16) as u8, (num >> 24) as u8]
}

// fn u32_to_bytes(num: u32) -> [u8; 4] {
//     [num as u8, (num >> 8) as u8, (num >> 16) as u8, (num >> 24) as u8]
// }

// fn i64_to_bytes(num: i64) -> [u8; 8] {
//     [num as u8, (num >> 8) as u8, (num >> 16) as u8, (num >> 24) as u8, (num >> 32) as u8, (num >> 40) as u8, (num >> 48) as u8, (num >> 56) as u8]
// }

// fn u64_to_bytes(num: u64) -> [u8; 8] {
//     [num as u8, (num >> 8) as u8, (num >> 16) as u8, (num >> 24) as u8, (num >> 32) as u8, (num >> 40) as u8, (num >> 48) as u8, (num >> 56) as u8]
// }

impl MtProtoConnection {
    fn new(encrypted: bool) -> Result<MtProtoConnection, io::Error> {
        let connection = TcpStream::connect("91.108.56.165:443")?;

        Ok(MtProtoConnection {
            conn: connection,
            connected: false,
            encrypted: encrypted,
            time_offset: 0,
            packet_seq_no: 0,
            message_seq_no: 0,
            auth_key: Vec::new(),
            server_salt: 0,
            session_id: 0,
            auth_key_id: 0,
        })
    }

    fn new_custom<A: ToSocketAddrs>(encrypted: bool, addr: A) -> Result<MtProtoConnection, io::Error> {
        let connection = TcpStream::connect(addr)?;

        Ok(MtProtoConnection {
            conn: connection,
            connected: false,
            encrypted: encrypted,
            time_offset: 0,
            packet_seq_no: 0,
            message_seq_no: 0,
            auth_key: Vec::new(),
            server_salt: 0,
            session_id: 0,
            auth_key_id: 0,
        })
    }

    fn set_encrypt(&mut self, encrypted: bool) {
        self.encrypted = encrypted;
    }

    fn send(&mut self, message_data: &[u8]) -> Result<(), io::Error> {
        if self.encrypted {
            let mut buffer = Cursor::new(Vec::new());

            let to_encrypt_length = 8 + 8 + 8 + 4 + 4 + message_data.len();
            let to_mod16 = (16 - to_encrypt_length % 16) % 16;

            buffer.write_i32::<LittleEndian>(
                (4 + 4 +
                    (8 + 16 + to_encrypt_length + to_mod16) +
                4)
            as i32)?;
            // length of everything.
            // length, packet_seq_no
                // auth_key_id, msg_key, server_salt, session_id, message_id, msg_seq_no, message_data_length, message_data, padding
            // crc32

            buffer.write_i32::<LittleEndian>(self.packet_seq_no)?;
            self.packet_seq_no += 1;

            let msg_id = self.get_msg_id();


            let mut to_encrypt_buf = Cursor::new(Vec::with_capacity(to_encrypt_length + to_mod16));

            to_encrypt_buf.write_u64::<LittleEndian>(self.server_salt)?; // server_salt; int64
            to_encrypt_buf.write_i64::<LittleEndian>(self.session_id)?; // session_id; int64
            to_encrypt_buf.write_i64::<LittleEndian>(msg_id)?; // message_id; int64
            to_encrypt_buf.write_i32::<LittleEndian>(self.get_msg_seq_no(true))?; // FIXME
            to_encrypt_buf.write_i32::<LittleEndian>(message_data.len() as i32)?; // message_data_length; i32
            to_encrypt_buf.write_all(message_data)?; // message_data; bytes


            let mut hasher = sha1::Sha1::new();

            hasher.update(to_encrypt_buf.get_ref());
            let to_encrypt_hash = hasher.digest().bytes();
            let mut msg_key = to_encrypt_hash[4..20].to_vec();
            msg_key.reverse();


            let mut rng = rand::thread_rng();

            let rand_bytes = rng.gen_iter::<u8>().take(to_mod16 as usize).collect::<Vec<u8>>();
            to_encrypt_buf.write_all(&rand_bytes)?;


            let (aes_key, mut aes_iv) = self.get_key_iv(&msg_key, true);

            let aes_encrypt_key = openssl::aes::AesKey::new_encrypt(&aes_key).unwrap();
            let mut encrypted_data = vec![0u8; to_encrypt_buf.get_ref().len()];

            openssl::aes::aes_ige(&to_encrypt_buf.get_ref(), &mut encrypted_data, &aes_encrypt_key, &mut aes_iv, openssl::symm::Mode::Encrypt);


            buffer.write_i64::<LittleEndian>(self.auth_key_id)?; // auth_key_id; int64
            buffer.write_all(&msg_key)?;
            buffer.write_all(&encrypted_data)?;

            let mut digest = crc32::Digest::new(crc32::IEEE);
            digest.write(buffer.get_ref());
            
            buffer.write_u32::<LittleEndian>(digest.sum32())?;

            self.conn.write_all(buffer.get_ref())?;
        } else {
            let mut buffer = Cursor::new(Vec::new());

            buffer.write_i32::<LittleEndian>(
                (4 + 4 +
                    (8 + 8 + 4 + message_data.len()) +
                4)
            as i32)?;

            // length of everything.
            // length, packet_seq_no
                // auth_key_id, message_id, message_data_length, message_data
            // crc32

            buffer.write_i32::<LittleEndian>(self.packet_seq_no)?;
            self.packet_seq_no += 1;

            let msg_id = self.get_msg_id();

            buffer.write_i64::<LittleEndian>(0)?; // auth_key_id = 0; int64
            buffer.write_i64::<LittleEndian>(msg_id)?; // message_id; int64
            buffer.write_i32::<LittleEndian>(message_data.len() as i32)?; // message_data_length; i32
            buffer.write_all(message_data)?; // message_data; bytes

            let mut digest = crc32::Digest::new(crc32::IEEE);
            digest.write(buffer.get_ref());
            
            buffer.write_u32::<LittleEndian>(digest.sum32())?;

            self.conn.write_all(buffer.get_ref())?;
        }

        self.conn.flush()?;
        Ok(())
    }

    fn receive(&mut self) -> Result<Vec<u8>, io::Error> {
        let buf_size = self.conn.read_i32::<LittleEndian>()?;
        let seq_no = self.conn.read_i32::<LittleEndian>()?;

        println!("\nSTART RECEIVE DEBUG");

        println!("buf_size: {}", buf_size);
        println!("seq_no: {}", seq_no);

        let mut buffer = vec![0; buf_size as usize - 12];
        self.conn.read_exact(buffer.as_mut_slice())?;
        println!("recv_buf: {}", dump_bytes(&buffer).unwrap());

        let received_checksum = self.conn.read_u32::<LittleEndian>()?;
        println!("received_checksum: 0x{:08x}", received_checksum);

        let mut digest = crc32::Digest::new(crc32::IEEE);
        digest.write(&i32_to_bytes(buf_size));
        digest.write(&i32_to_bytes(seq_no));
        digest.write(buffer.as_ref());

        let calculated_checksum = digest.sum32();

        if received_checksum != calculated_checksum {
            panic!("Placeholder error, checksums are not ok");
        }

        if self.encrypted {
            let mut buf = Cursor::new(buffer);

            let auth_key_id = buf.read_i64::<LittleEndian>()?;
            println!("auth_key_id: 0x{:016x}", auth_key_id);

            let mut msg_key = vec![0; 16];
            buf.read_exact(&mut msg_key)?;

            let (aes_key, mut aes_iv) = self.get_key_iv(&msg_key, false);

            let aes_decrypt_key = openssl::aes::AesKey::new_decrypt(&aes_key).unwrap();
            let mut decrypted_data = vec![0u8; buf.get_ref()[24..].len()];

            openssl::aes::aes_ige(&buf.get_ref()[24..] , &mut decrypted_data, &aes_decrypt_key, &mut aes_iv, openssl::symm::Mode::Decrypt);


            let mut decrypted_buffer = Cursor::new(decrypted_data);

            let server_salt = decrypted_buffer.read_u64::<LittleEndian>()?;
            println!("server_salt: 0x{:016x}", server_salt);

            let session_id = decrypted_buffer.read_i64::<LittleEndian>()?;
            println!("session_id: 0x{:016x}", session_id);

            let message_id = decrypted_buffer.read_i64::<LittleEndian>()?;
            println!("message_id: 0x{:016x}", message_id);

            let seq_no = decrypted_buffer.read_i32::<LittleEndian>()?;
            println!("seq_no: 0x{:08x}", seq_no);

            let message_data_length = decrypted_buffer.read_i32::<LittleEndian>()?;
            println!("message_data_length: {}", message_data_length);

            let pos = decrypted_buffer.position();
            let mut message_data = vec![0u8; message_data_length as usize];
            decrypted_buffer.read_exact(&mut message_data);

            println!("message_data: {}", dump_bytes(&message_data).unwrap());

            println!("END RECEIVE DEBUG\n");

            Ok(message_data)
        } else {
            let mut buf = Cursor::new(buffer);

            let auth_key_id = buf.read_i64::<LittleEndian>()?;
            println!("auth_key_id: 0x{:016x}", auth_key_id);

            let message_id = buf.read_i64::<LittleEndian>()?;
            println!("message_id: 0x{:016x}", message_id);

            let message_data_length = buf.read_i32::<LittleEndian>()?;
            println!("message_data_length: {}", message_data_length);


            let mut message_data = vec![0; message_data_length as usize];
            buf.read_exact(message_data.as_mut_slice())?;

            println!("message_data: {}", dump_bytes(&message_data).unwrap());

            println!("END RECEIVE DEBUG\n");

            Ok(message_data)
        }
    }

    fn send_obj<T>(&mut self, request: &T) -> Result<(), io::Error> where Cursor<Vec<u8>>: Serialize<T> {
        let mut buf = Cursor::new(Vec::new());
        buf.serialize(request)?;

        self.send(buf.get_ref())?;

        Ok(())
    }

    fn recv_obj<T>(&mut self) -> Result<T, io::Error> where Cursor<Vec<u8>>: Deserialize<T> {
        let mut message_data = Cursor::new(self.receive()?);
        
        let object: T = message_data.deserialize(0)?;

        Ok(object)
    }

    fn get_key_iv(&self, msg_key: &[u8], encrypt: bool) -> (Vec<u8>, Vec<u8>) {
        let x = if encrypt { 0 } else { 8 };

        let mut hasher = sha1::Sha1::new();


        hasher.update(msg_key);
        hasher.update(&self.auth_key[x..x+32]);
        let sha1_a = hasher.digest().bytes();

        hasher.reset();
        hasher.update(&self.auth_key[x+32..x+32+16]);
        hasher.update(msg_key);
        hasher.update(&self.auth_key[x+48..x+48+16]);
        let sha1_b = hasher.digest().bytes();

        hasher.reset();
        hasher.update(&self.auth_key[x+64..x+64+32]);
        hasher.update(msg_key);
        let sha1_c = hasher.digest().bytes();

        hasher.reset();
        hasher.update(msg_key);
        hasher.update(&self.auth_key[x+96..x+96+32]);
        let sha1_d = hasher.digest().bytes();


        let mut aes_key = Vec::with_capacity(32);
        aes_key.extend_from_slice(&sha1_a[0..0+8]);
        aes_key.extend_from_slice(&sha1_b[8..8+12]);
        aes_key.extend_from_slice(&sha1_c[4..4+12]);

        let mut aes_iv = Vec::with_capacity(32);
        aes_iv.extend_from_slice(&sha1_a[8..8+12]);
        aes_iv.extend_from_slice(&sha1_b[0..0+8]);
        aes_iv.extend_from_slice(&sha1_c[16..16+4]);
        aes_iv.extend_from_slice(&sha1_d[0..0+8]);

        (aes_key, aes_iv)
    }

    fn get_msg_id(&self) -> i64 {
        let current_time = time::get_time();

        return (current_time.sec + self.time_offset) << 32 | (current_time.nsec as i64) << 2;
    }

    fn get_msg_seq_no(&mut self, content_related: bool) -> i32 {
        if content_related {
            let res = self.message_seq_no * 2 + 1;
            self.message_seq_no += 1;

            res
        } else {
            self.message_seq_no * 2
        }
    }

    fn set_time_offset(&mut self, offset: i64) {
        self.time_offset = offset;
    }

    fn authenticate(&mut self) -> Result<(), io::Error> {

        fn ser_and_hash<T>(buf: &mut Cursor<Vec<u8>>, obj: &T) -> Result<(), io::Error> where Cursor<Vec<u8>>: Serialize<T> {
            let start_pos = buf.position();

            buf.set_position(start_pos + 20);
            buf.serialize(&obj)?;
            
            let end_pos = buf.position();

            let mut hasher = sha1::Sha1::new();
            hasher.update(&buf.get_ref()[(start_pos + 20) as usize .. end_pos as usize]);

            buf.set_position(start_pos);
            let hash = hasher.digest().bytes();
            buf.write_all(&hash)?;

            buf.set_position(end_pos);

            Ok(())
        }

        // ﻿Creating an Authorization Key
        // https://core.telegram.org/mtproto/auth_key

        let mut rng = rand::thread_rng();

        // 1) Client sends query to server
        // req_pq#60469778 nonce:int128 = ResPQ

        let nonce = (rng.gen::<u64>(), rng.gen::<u64>());

        let request = functions::ReqPq {
            nonce: nonce.clone()
        };

        self.send_obj(&request)?;

        // logs

        println!("{:#?}", request);



        // 2) Server sends response of the form
        // resPQ#05162463 nonce:int128 server_nonce:int128 pq:string server_public_key_fingerprints:Vector long = ResPQ
        
        let res_pq = self.recv_obj::<constructors::ResPQ>()?;
        let server_nonce = res_pq.server_nonce;
        
        // logs

        println!("{:#?}", res_pq);



        // Proof of work
        // 3) Client decomposes pq into prime factors such that p < q.

        let (p, q) = math::factor(BigEndian::read_u64(&res_pq.pq));

        // logs

        println!("p:{}\nq:{}", p, q);



        // Presenting proof of work; Server authentication
        // 4) Client sends query to server
        // req_DH_params#d712e4be nonce:int128 server_nonce:int128 p:string q:string public_key_fingerprint:long encrypted_data:string = Server_DH_Params

        let mut p_bytes = vec![0u8; 4];
        BigEndian::write_u32(p_bytes.as_mut_slice(), p as u32);

        let mut q_bytes = vec![0u8; 4];
        BigEndian::write_u32(q_bytes.as_mut_slice(), q as u32);

        let new_nonce = (rng.gen::<u64>(), rng.gen::<u64>(), rng.gen::<u64>(), rng.gen::<u64>());
        let p_q_inner_data = constructors::PQInnerData {
            pq: res_pq.pq,
            p: p_bytes.clone(),
            q: q_bytes.clone(),
            nonce: nonce.clone(),
            server_nonce: server_nonce.clone(),
            new_nonce: new_nonce.clone(),
        };

        let mut ser_p_q_inner_data = Cursor::new(Vec::new());
        ser_and_hash(&mut ser_p_q_inner_data, &p_q_inner_data)?;

        let rand_bytes = rng.gen_iter::<u8>().take(255 - ser_p_q_inner_data.position() as usize).collect::<Vec<u8>>();
        ser_p_q_inner_data.write_all(&rand_bytes)?;

        // logs
        // let p_q_inner_data_dump = dump_bytes(&ser_p_q_inner_data.get_ref()[20..return_position as usize]).unwrap();
        let full_p_q_inner_data_dump = dump_bytes(&ser_p_q_inner_data.get_ref()).unwrap();
        
        // println!("p_q_inner_data_dump: {}", p_q_inner_data_dump);
        println!("full_p_q_inner_data_dump: {}", full_p_q_inner_data_dump);
        println!("pq: {:?}", p_q_inner_data.pq);
        println!("q: {:?}", q_bytes);
        println!("p: {:?}", p_bytes);
        println!("p_q_inner_data: {:#?}", p_q_inner_data);

        let rsa_key_bytes = String::from("-----BEGIN PUBLIC KEY-----\n\
        MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAwVACPi9w23mF3tBkdZz+\n\
        zwrzKOaaQdr01vAbU4E1pvkfj4sqDsm6lyDONS789sVoD/xCS9Y0hkkC3gtL1tSf\n\
        TlgCMOOul9lcixlEKzwKENj1Yz/s7daSan9tqw3bfUV/nqgbhGX81v/+7RFAEd+R\n\
        wFnK7a+XYl9sluzHRyVVaTTveB2GazTwEfzk2DWgkBluml8OREmvfraX3bkHZJTK\n\
        X4EQSjBbbdJ2ZXIsRrYOXfaA+xayEGB+8hdlLmAjbCVfaigxX0CDqWeR1yFL9kwd\n\
        9P0NsZRPsmoqVwMbMu7mStFai6aIhc3nSlv8kg9qv1m6XHVQY3PnEw+QQtqSIXkl\n\
        HwIDAQAB\n\
        -----END PUBLIC KEY-----\n").into_bytes();

        let rsa_key = Rsa::public_key_from_pem(&rsa_key_bytes).unwrap(); // no, return error

        let base = BigNum::from_slice(ser_p_q_inner_data.get_ref()).unwrap();
        let e = rsa_key.e().unwrap();
        let n = rsa_key.n().unwrap();

        let mut res = BigNum::new().unwrap();
        let mut ctx = BigNumContext::new().unwrap();

        res.mod_exp(base.deref(), e, n, &mut ctx).unwrap();


        let encrypted_data = res.to_vec();

        println!("encrypted_data:{}", dump_bytes(encrypted_data.as_ref()).unwrap());

        let req_dh_params = functions::ReqDHParams {
            nonce: nonce.clone(),
            server_nonce: server_nonce.clone(),
            p: p_bytes,
            q: q_bytes,
            public_key_fingerprint: res_pq.server_public_key_fingerprints[0],
            encrypted_data: encrypted_data,
        };

        self.send_obj(&req_dh_params)?;

        // logs

        println!("req_dh_params: {:?}", req_dh_params);



        // 5) Server responds in one of two ways:
        // server_DH_params_fail#79cb045d nonce:int128 server_nonce:int128 new_nonce_hash:int128 = Server_DH_Params;
        // server_DH_params_ok#d0e8075c nonce:int128 server_nonce:int128 encrypted_answer:string = Server_DH_Params;

        let server_dh_params = self.recv_obj::<constructors::ServerDHParams>()?;
        
        
        // logs

        println!("server_dh_params: {:?}", &server_dh_params);


        let encrypted_answer: Vec<u8> = match server_dh_params {
            constructors::ServerDHParams::Ok {
                nonce, server_nonce, encrypted_answer
            } => {
                // if 
                encrypted_answer
            },

            constructors::ServerDHParams::Fail {
                nonce, server_nonce, new_nonce_hash
            } => {
                panic!("ServerDHParams failed, replace with real error")
            }
        };


        let mut new_nonce_bytes = vec![0u8; 32];
        LittleEndian::write_u64(&mut new_nonce_bytes[0..8], new_nonce.3);
        LittleEndian::write_u64(&mut new_nonce_bytes[8..16], new_nonce.2);
        LittleEndian::write_u64(&mut new_nonce_bytes[16..24], new_nonce.1);
        LittleEndian::write_u64(&mut new_nonce_bytes[24..32], new_nonce.0);

        let mut server_nonce_bytes = vec![0u8; 16];
        LittleEndian::write_u64(&mut server_nonce_bytes[0..8], server_nonce.1);
        LittleEndian::write_u64(&mut server_nonce_bytes[8..16], server_nonce.0);

        
        let mut hasher = sha1::Sha1::new();
        hasher.update(&new_nonce_bytes);
        hasher.update(&server_nonce_bytes);

        let new_nonce_server_nonce_hash = hasher.digest().bytes();
        println!("new_nonce_server_nonce_hash: {}", dump_bytes(&new_nonce_server_nonce_hash).unwrap());
        
        
        hasher.reset();
        hasher.update(&server_nonce_bytes);
        hasher.update(&new_nonce_bytes);

        let server_nonce_new_nonce_hash = hasher.digest().bytes();
        println!("server_nonce_new_nonce_hash: {}", dump_bytes(&server_nonce_new_nonce_hash).unwrap());
        
        
        hasher.reset();
        hasher.update(&new_nonce_bytes);
        hasher.update(&new_nonce_bytes);

        let new_nonce_new_nonce_hash = hasher.digest().bytes();
        println!("new_nonce_new_nonce_hash: {}", dump_bytes(&new_nonce_new_nonce_hash).unwrap());


        let mut tmp_aes_key = Vec::with_capacity(32);
        tmp_aes_key.extend_from_slice(&new_nonce_server_nonce_hash);
        tmp_aes_key.extend_from_slice(&server_nonce_new_nonce_hash[0..12]);

        let mut tmp_aes_iv = Vec::with_capacity(32);
        tmp_aes_iv.extend_from_slice(&server_nonce_new_nonce_hash[12..20]);
        tmp_aes_iv.extend_from_slice(&new_nonce_new_nonce_hash);
        tmp_aes_iv.extend_from_slice(&new_nonce_bytes[0..4]);


        // logs
        println!("");
        println!("tmp_aes_key: {}", dump_bytes(&tmp_aes_key).unwrap());
        println!("tmp_aes_iv: {}", dump_bytes(&tmp_aes_iv).unwrap());

        let aes_decrypt_key = openssl::aes::AesKey::new_decrypt(&tmp_aes_key).unwrap();
        let mut decrypted_answer = vec![0u8; encrypted_answer.len()];

        let mut iv = tmp_aes_iv.clone();
        openssl::aes::aes_ige(&encrypted_answer, &mut decrypted_answer, &aes_decrypt_key, &mut iv, openssl::symm::Mode::Decrypt);

        let mut answer = Cursor::new(decrypted_answer);

        answer.set_position(20); // Don't skip the sha1, verify it
        let server_dh_inner_data: constructors::ServerDHInnerData = answer.deserialize(0)?;

        // logs

        let answer_dump = dump_bytes(answer.get_ref()).unwrap();
        
        println!("answer dump: {}", answer_dump);
        println!("server_dh_inner_data: {:?}", &server_dh_inner_data);

        let now = time::get_time();
        println!("server_time: {}", &server_dh_inner_data.server_time);
        println!("now: {}.{}", now.sec, now.nsec);
        println!("now - server: {}.{}", now.sec - server_dh_inner_data.server_time as i64, now.nsec);
        self.set_time_offset(now.sec - server_dh_inner_data.server_time as i64);


        // 6) Client computes random 2048-bit number b (using a sufficient amount of entropy) and sends the server a message
        // set_client_DH_params#f5045f1f nonce:int128 server_nonce:int128 encrypted_data:string = Set_client_DH_params_answer;

        let dh_prime = BigNum::from_slice(&server_dh_inner_data.dh_prime).unwrap();
        let g = BigNum::from_u32(server_dh_inner_data.g as u32).unwrap();
        let g_a = BigNum::from_slice(&server_dh_inner_data.g_a).unwrap();

        let mut b = BigNum::new().unwrap();
        b.rand(2048, openssl::bn::MSB_ONE, true).unwrap();

        let mut g_b = BigNum::new().unwrap();
        let mut ctx = BigNumContext::new().unwrap();

        g_b.mod_exp(g.deref(), b.deref(), dh_prime.deref(), &mut ctx).unwrap();

        let client_dh_inner_data = constructors::ClientDHInnerData {
            nonce: nonce.clone(),
            server_nonce: server_nonce.clone(),
            retry_id: 0, // TODO: do something when fail, ye
            g_b: g_b.to_vec(),
        };

        let mut ser_client_dh_inner_data = Cursor::new(Vec::new());
        ser_and_hash(&mut ser_client_dh_inner_data, &client_dh_inner_data)?;

        let to_mod16 = (16 - ser_client_dh_inner_data.get_ref().len() % 16) % 16;

        let rand_bytes = rng.gen_iter::<u8>().take(to_mod16 as usize).collect::<Vec<u8>>();
        ser_client_dh_inner_data.write_all(&rand_bytes)?;


        let aes_encrypt_key = openssl::aes::AesKey::new_encrypt(&tmp_aes_key).unwrap();
        let mut encrypted_data = vec![0u8; ser_client_dh_inner_data.get_ref().len()];

        openssl::aes::aes_ige(ser_client_dh_inner_data.get_ref(), &mut encrypted_data, &aes_encrypt_key, &mut tmp_aes_iv, openssl::symm::Mode::Encrypt);

        let set_client_dh_params = functions::SetClientDHParams {
            nonce: nonce.clone(),
            server_nonce: server_nonce.clone(),
            encrypted_data: encrypted_data,
        };

        self.send_obj(&set_client_dh_params)?;

        //logs

        println!("client_dh_inner_data: {:?}", &client_dh_inner_data);

        println!("set_client_dh_params: {:?}", &set_client_dh_params);


        // 7) Thereafter, auth_key equals pow(g, {ab}) mod dh_prime; on the server,
        // it is computed as pow(g_b, a) mod dh_prime, and on the client as (g_a)^b mod dh_prime.

        let mut auth_key = BigNum::new().unwrap();
        let mut ctx = BigNumContext::new().unwrap();
        auth_key.mod_exp(g_a.deref(), b.deref(), dh_prime.deref(), &mut ctx).unwrap();



        // 8) auth_key_hash is computed := 64 lower-order bits of SHA1 (auth_key).
        // The server checks whether there already is another key with the same auth_key_hash
        // and responds in one of the following ways.


        // DH key exchange complete
        // 9) Server responds in one of three ways:

        // dh_gen_ok#3bcbf734 nonce:int128 server_nonce:int128 new_nonce_hash1:int128 = Set_client_DH_params_answer;
        // dh_gen_retry#46dc1fb9 nonce:int128 server_nonce:int128 new_nonce_hash2:int128 = Set_client_DH_params_answer;
        // dh_gen_fail#a69dae02 nonce:int128 server_nonce:int128 new_nonce_hash3:int128 = Set_client_DH_params_answer;

        let set_client_dh_params_answer = self.recv_obj::<constructors::SetClientDHParamsAnswer>()?;
        
        println!("{:?}", set_client_dh_params_answer);

        match set_client_dh_params_answer {
            constructors::SetClientDHParamsAnswer::DhGenOk {
                nonce, server_nonce, new_nonce_hash1
            } => {
                self.session_id = rng.gen::<i64>();
                self.server_salt = new_nonce.0 ^ server_nonce.0;
                self.auth_key = auth_key.to_vec();

                hasher.reset();
                hasher.update(&self.auth_key);
                let auth_key_hash = hasher.digest().bytes();

                self.auth_key_id = BigEndian::read_i64(&auth_key_hash[12..20]);
                self.encrypted = true;

                // We good, save some variables
            }, 
            
            constructors::SetClientDHParamsAnswer::DhGenRetry {
                nonce, server_nonce, new_nonce_hash2
            } => {
                // devise some way to retry correctly
            },

            constructors::SetClientDHParamsAnswer::DhGenFail {
                nonce, server_nonce, new_nonce_hash3
            } => {
                // lol u messed up
            },
        }

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    extern crate rand;

    use rand::Rng;
    use deserialize::Deserializer;
    use serialize::Serialize;
    use std::io::Cursor;
    use super::*;


    #[test]
    fn authenticate() {
        let mut connection = MtProtoConnection::new(false).unwrap();
        connection.authenticate().unwrap();

        let get_config = functions::HelpGetConfig;

        let init_connection = functions::InitConnection {
            api_id: 12332,
            device_model: "Desktop".into(),
            system_version: "4.11.5-1".into(),
            app_version: "0.0.0".into(),
            lang_code: "en_US".into(),
            query: Box::new(get_config.into()),
        };

        let invoke_with_layer = functions::InvokeWithLayer {
            layer: 66,
            query: Box::new(init_connection.into()),
        };

        connection.send_obj(&invoke_with_layer).unwrap();
        let res = connection.recv_obj::<constructors::TlType>();

        println!("{:?}", res);
    }

    #[test]
    fn test_ser_des() {
        use std::fmt::Write;

        let mut buf: Cursor<Vec<u8>> = Cursor::new(Vec::new());
        let start = constructors::User::User {
            flags: 0, // flags is generated on serialize
            sself: true,
            contact: false,
            mutual_contact: false,
            deleted: false,
            bot: false,
            bot_chat_history: false,
            bot_nochats: false,
            verified: false,
            restricted: false,
            min: false,
            bot_inline_geo: false,
            id: 987654321,
            access_hash: Some(123456789),
            first_name: Some("Juan".to_string()),
            last_name: Some("Potato".to_string()),
            username: None,
            phone: None,
            photo: Some(constructors::UserProfilePhoto::Empty.into()),
            status: None,
            bot_info_version: None,
            restriction_reason: None,
            bot_inline_placeholder: None,
            lang_code: None,
        };
        // let start = constructors::InputPeerNotifySettings {
        //     flags: 0,
        //     show_previews: true,
        //     silent: false,
        //     mute_until: 420,
        //     sound: "woof".to_string(),
        // };

        buf.serialize(&start).unwrap();

        let s = dump_bytes(buf.get_ref()).unwrap();

        buf.set_position(0);
        let end: constructors::User = buf.deserialize(0).unwrap();
        // let end: constructors::InputPeerNotifySettings = buf.deserialize(0).unwrap();

        println!("{:#?}", start);

        println!("{}", s);
        
        println!("{:#?}", end);

        // assert!(start == end, "start = {}, end = {}", start, end);
    }

    #[test]
    fn test_string() {
        let master_string = "0123456789abcdéf0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
        for (i, _) in master_string.char_indices() {
            let mut buf: Cursor<Vec<u8>> = Cursor::new(Vec::new());
            let start = master_string[0..i].to_string();

            buf.serialize(&start).unwrap();

            buf.set_position(0);
            let end: String = buf.deserialize(0).unwrap();
            
            assert!(start == end, "start = {}, end = {}", start, end);
            // TODO: fuzz?
        }
    }

    #[test]
    fn test_i32() {
        let mut buf: Cursor<Vec<u8>> = Cursor::new(Vec::new());
        let start = -1234;

        buf.serialize(&start).unwrap();

        buf.set_position(0);
        let end = buf.deserialize::<i32>(0).unwrap();
            
        assert!(start == end, "start = {}, end = {}", start, end);
    }

    // #[test]
    // fn test_bool() {
    //     let mut buf: Cursor<Vec<u8>> = Cursor::new(Vec::new());

    //     let start1 = true;
    //     let start2 = false;

    //     buf.serialize(&start1).unwrap();
    //     buf.serialize(&start2).unwrap();

    //     buf.set_position(0);
    //     let end1 = buf.deserialize::<bool>(0).unwrap();
    //     let end2 = buf.deserialize::<bool>(0).unwrap();
            
    //     assert!(start1 == end1, "start = {}, end = {}", start1, end1);
    //     assert!(start2 == end2, "start = {}, end = {}", start2, end2);
    // }

    #[test]
    fn test_u32() {
        let mut buf: Cursor<Vec<u8>> = Cursor::new(Vec::new());
        let start = 1234;

        buf.serialize(&start).unwrap();

        buf.set_position(0);
        let end = buf.deserialize::<u32>(0).unwrap();
            
        assert!(start == end, "start = {}, end = {}", start, end);
    }

    #[test]
    fn test_f32() {
        let mut buf: Cursor<Vec<u8>> = Cursor::new(Vec::new());
        let start = 1.234;

        buf.serialize(&start).unwrap();

        buf.set_position(0);
        let end = buf.deserialize::<f32>(0).unwrap();
            
        assert!(start == end, "start = {}, end = {}", start, end);
    }

    #[test]
    fn test_i64() {
        let mut buf: Cursor<Vec<u8>> = Cursor::new(Vec::new());
        let start = -1234;

        buf.serialize(&start).unwrap();

        buf.set_position(0);
        let end = buf.deserialize::<i64>(0).unwrap();
            
        assert!(start == end, "start = {}, end = {}", start, end);
    }

    #[test]
    fn test_f64() {
        let mut buf: Cursor<Vec<u8>> = Cursor::new(Vec::new());
        let start = 1.234;

        buf.serialize(&start).unwrap();

        buf.set_position(0);
        let end = buf.deserialize::<f64>(0).unwrap();
            
        assert!(start == end, "start = {}, end = {}", start, end);
    }

    #[test]
    fn test_i128() {
        use Int128;

        let mut buf: Cursor<Vec<u8>> = Cursor::new(Vec::new());
        let start = (0x0001020304050607, 0x08090A0B0C0D0E0F);


        let mut start_bytes = vec![0u8; 16];
        BigEndian::write_u64(&mut start_bytes[0..8], start.0);
        BigEndian::write_u64(&mut start_bytes[8..16], start.1);

        println!("{}", dump_bytes(&start_bytes).unwrap());



        buf.serialize(&start).unwrap();
        println!("{:?}", buf.get_ref());

        buf.set_position(0);
        let end = buf.deserialize::<Int128>(0).unwrap();
            
        assert!(start == end, "start = {:?}, end = {:?}", start, end);
    }

    #[test]
    fn test_i256() {
        use Int256;

        let mut buf: Cursor<Vec<u8>> = Cursor::new(Vec::new());
        let start = (0x0001020304050607, 0x08090A0B0C0D0E0F, 0x001112131415161718, 0x18191A1B1C1D1E1F);

        buf.serialize(&start).unwrap();
        println!("{:?}", buf.get_ref());

        buf.set_position(0);
        let end = buf.deserialize::<Int256>(0).unwrap();
            
        assert!(start == end, "start = {:?}, end = {:?}", start, end);
    }
}