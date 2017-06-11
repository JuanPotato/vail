#[macro_use]
extern crate tl_derive;
extern crate byteorder;
extern crate time;
extern crate rand;
extern crate crc;
extern crate openssl;
extern crate sha1;

// extern crate num_bigint;

// use num_bigint::BigUint;

use openssl::rsa::{NO_PADDING, Rsa};

use crc::{crc32, Hasher32};

use std::io;
use std::io::{Write, Read};
use std::io::Cursor;

use std::net::{TcpStream, ToSocketAddrs};
// use deserialize::Deserializer;

use byteorder::{WriteBytesExt, ReadBytesExt, LittleEndian, ByteOrder, BigEndian};


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
    msg_ig_offset: i64,
    seq_num: i32,
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
            msg_ig_offset: 0,
            seq_num: 0,
        })
    }

    fn new_custom<A: ToSocketAddrs>(encrypted: bool, addr: A) -> Result<MtProtoConnection, io::Error> {
        let connection = TcpStream::connect(addr)?;

        Ok(MtProtoConnection {
            conn: connection,
            connected: false,
            encrypted: encrypted,
            msg_ig_offset: 0,
            seq_num: 0,
        })
    }

    fn set_encrypt(&mut self, encrypted: bool) {
        self.encrypted = encrypted;
    }

    fn send(&mut self, message_data: &[u8]) -> Result<(), io::Error> {
        if self.encrypted {
            unimplemented!();
        } else {
            let mut buffer = Cursor::new(Vec::new());

            buffer.write_i32::<LittleEndian>((4 + 4 + (8 + 8 + 4 + message_data.len()) + 4) as i32)?;
            // length of everything. length, seq_num, auth_key_id, message_id,
            // message_data_length, message_data, crc32
            buffer.write_i32::<LittleEndian>(self.seq_num)?;

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

        self.conn.flush();
        Ok(())
    }

    fn receive(&mut self) -> Result<Vec<u8>, io::Error> {
        if self.encrypted {
            unimplemented!();
        } else {
            let buf_size = self.conn.read_i32::<LittleEndian>()?;
            let seq_num = self.conn.read_i32::<LittleEndian>()?;

            let mut buffer = vec![0; buf_size as usize - 12];
            self.conn.read_exact(buffer.as_mut_slice())?;

            let received_checksum = self.conn.read_u32::<LittleEndian>()?;

            let mut digest = crc32::Digest::new(crc32::IEEE);
            digest.write(&i32_to_bytes(buf_size));
            digest.write(&i32_to_bytes(seq_num));
            digest.write(buffer.as_ref());

            let calculated_checksum = digest.sum32();

            if received_checksum != calculated_checksum {
                panic!("Placeholder error, checksums are not ok");
            }


            let mut buf = Cursor::new(buffer);

            let auth_key_id = buf.read_i64::<LittleEndian>()?;
            let message_id = buf.read_i64::<LittleEndian>()?;
            let message_data_length = buf.read_i32::<LittleEndian>()?;


            let mut message_data = vec![0; message_data_length as usize];
            buf.read_exact(message_data.as_mut_slice())?;

            Ok(message_data)
        }
    }

    fn get_msg_id(&self) -> i64 {
        let current_time = time::get_time();

        return current_time.sec << 32 | (current_time.nsec as i64) << 2;
    }
}


#[cfg(test)]
mod tests {
    use deserialize::Deserializer;
    use serialize::Serialize;
    use std::io::Cursor;
    use super::*;

    #[test]
    fn time() {
        let current_time = time::get_time();

        let msg_id = current_time.sec << 32 | (current_time.nsec as i64) << 2;

        println!("0x{:x}", msg_id);
    }

    #[test]
    fn authenticate() {
        use std::io::Read;
        use rand::Rng;

        // ﻿Creating an Authorization Key
        // https://core.telegram.org/mtproto/auth_key

        let mut connection = MtProtoConnection::new(false).unwrap();
        let mut rng = rand::thread_rng();
        let mut buf: Cursor<Vec<u8>> = Cursor::new(Vec::new());


        // 1) Client sends query to server
        // req_pq#60469778 nonce:int128 = ResPQ

        let request = functions::ReqPq {
            nonce: (rng.gen::<u64>(), rng.gen::<u64>())
        };

        buf.serialize(&request).unwrap();

        connection.send(buf.get_ref());

        // logs
        
        let req_pq_dump = dump_bytes(buf.get_ref()).unwrap();
        
        println!("{}", req_pq_dump);
        println!("{:#?}", request);



        // 2) Server sends response of the form
        // resPQ#05162463 nonce:int128 server_nonce:int128 pq:string server_public_key_fingerprints:Vector long = ResPQ
        
        let mut message_data = Cursor::new(connection.receive().unwrap());
        
        let res_pq: constructors::ResPQ = message_data.deserialize(0).unwrap();
        
        // logs

        let res_pq_dump = dump_bytes(message_data.get_ref()).unwrap();
        
        println!("{}", res_pq_dump);
        println!("{:#?}", res_pq);



        // Proof of work
        // 3) Client decomposes pq into prime factors such that p < q.

        let (p, q) = math::factor(BigEndian::read_u64(&res_pq.pq));

        // logs

        println!("p:{}\nq:{}", p, q);



        // Presenting proof of work; Server authentication
        // 4) Client sends query to server
        // req_DH_params#d712e4be nonce:int128 server_nonce:int128 p:string q:string public_key_fingerprint:long encrypted_data:string = Server_DH_Params

        let mut p_bytes = vec![0u8; 8];
        BigEndian::write_u64(p_bytes.as_mut_slice(), p);

        let mut q_bytes = vec![0u8; 8];
        BigEndian::write_u64(q_bytes.as_mut_slice(), q);

        let p_q_inner_data = constructors::PQInnerData {
            pq: res_pq.pq,
            p: p_bytes.clone(),
            q: q_bytes.clone(),
            nonce: res_pq.nonce,
            server_nonce: res_pq.server_nonce,
            new_nonce: (rng.gen::<u64>(), rng.gen::<u64>(), rng.gen::<u64>(), rng.gen::<u64>()),
        };

        let mut ser_p_q_inner_data = Cursor::new(Vec::new());

        ser_p_q_inner_data.set_position(20);
        ser_p_q_inner_data.serialize(&p_q_inner_data).unwrap();
        
        let return_position = ser_p_q_inner_data.position();

        let mut hasher = sha1::Sha1::new();
        hasher.update(&ser_p_q_inner_data.get_ref()[20..]);

        ser_p_q_inner_data.set_position(0);
        let hash = hasher.digest().bytes();
        ser_p_q_inner_data.write_all(&hash);

        ser_p_q_inner_data.set_position(return_position);
        ser_p_q_inner_data.write_all(&vec![0u8; 256 - return_position as usize]);


        // logs
        let p_q_inner_data_dump = dump_bytes(ser_p_q_inner_data.get_ref()).unwrap();
        
        println!("p_q_inner_data_dump: {}", p_q_inner_data_dump);
        println!("pq: {:#?}", p_bytes);
        println!("q: {:#?}", q_bytes);
        println!("p: {:#?}", p_bytes);
        // println!("p_q_inner_data: {:#?}", p_q_inner_data);


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


        let mut encrypted_data = vec![0; 256];
        rsa_key.public_encrypt(ser_p_q_inner_data.get_ref(), encrypted_data.as_mut_slice(), NO_PADDING).unwrap();

        println!("encrypted_data:{}", dump_bytes(encrypted_data.as_ref()).unwrap());


        let req_dh_params = functions::ReqDHParams {
            nonce: res_pq.nonce,
            server_nonce: res_pq.server_nonce,
            p: p_bytes,
            q: q_bytes,
            public_key_fingerprint: res_pq.server_public_key_fingerprints[0],
            encrypted_data: encrypted_data,
        };


        let mut ser_req_dh_params: Cursor<Vec<u8>> = Cursor::new(Vec::new());

        ser_req_dh_params.serialize(&req_dh_params).unwrap();

        connection.send(ser_req_dh_params.get_ref());

        // logs

        let req_dh_params_dump = dump_bytes(ser_req_dh_params.get_ref()).unwrap();
        
        println!("req_dh_params_dump: {}", req_dh_params_dump);
        println!("req_dh_params: {:#?}", req_dh_params);



        // 5) Server responds in one of two ways:
        // server_DH_params_fail#79cb045d nonce:int128 server_nonce:int128 new_nonce_hash:int128 = Server_DH_Params;
        // server_DH_params_ok#d0e8075c nonce:int128 server_nonce:int128 encrypted_answer:string = Server_DH_Params;

        let mut message_data = Cursor::new(connection.receive().unwrap());
        
        let server_dh_params: constructors::ServerDHParams = message_data.deserialize(0).unwrap();
        
        // logs

        let server_dh_params_dump = dump_bytes(message_data.get_ref()).unwrap();
        
        println!("server_dh_params_dump: {}", server_dh_params_dump);
        println!("server_dh_params: {:#?}", server_dh_params);

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