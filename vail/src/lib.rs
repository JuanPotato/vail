#[macro_use]
extern crate tl_derive;
extern crate byteorder;
extern crate time;
extern crate rand;

use std::io;
use std::io::Write;

use std::net::{TcpStream, ToSocketAddrs};
// use deserialize::Deserializer;

use byteorder::LittleEndian;
use byteorder::WriteBytesExt;


mod constructors;
mod functions;
mod serialize;
mod deserialize;


// PROD_SERVER = "149.154.167.50:443";
// TEST_SERVER = "149.154.167.40:443";

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
}

impl MtProtoConnection {
    fn new(encrypted: bool) -> Result<MtProtoConnection, io::Error> {
        let connection = TcpStream::connect("149.154.167.50:443")?;

        Ok(MtProtoConnection {
            conn: connection,
            connected: false,
            encrypted: encrypted,
            msg_ig_offset: 0,
        })
    }

    fn new_custom<A: ToSocketAddrs>(encrypted: bool, addr: A) -> Result<MtProtoConnection, io::Error> {
        let connection = TcpStream::connect(addr)?;

        Ok(MtProtoConnection {
            conn: connection,
            connected: false,
            encrypted: encrypted,
            msg_ig_offset: 0,
        })
    }

    fn set_encrypt(&mut self, encrypted: bool) {
        self.encrypted = encrypted;
    }

    fn send(&mut self, message_data: &[u8]) -> Result<(), io::Error> {
        if self.encrypted {
            unimplemented!();
        } else {
            let msg_id = self.get_msg_id();

            self.conn.write_i64::<LittleEndian>(0)?; // auth_key_id = 0; int64
            self.conn.write_i64::<LittleEndian>(msg_id)?; // message_id; int64
            self.conn.write_i32::<LittleEndian>(message_data.len() as i32)?; // message_data_length; i32
            self.conn.write_all(message_data)?; // message_data; bytes
        }

        Ok(())
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

    // #[test]
    // fn authenticate() {
    //     use std::io::Read;
    //     use rand::Rng;
        
    //     let connection = MtProtoConnection::new(false).unwrap();
    //     let mut rng = rand::thread_rng();
    //     let mut buf: Cursor<Vec<u8>> = Cursor::new(Vec::new());

    //     buf.serialize(&functions::ReqPq {
    //         nonce: (rng.gen::<u64>(), rng.gen::<u64>())
    //     });

    //     connection.send(buf.get_ref());

    //     let mut buffer = [0; 16];

    //     // read up to 10 bytes
    //     connection.conn.read(&mut buffer).unwrap();
    //     println!("{:?}", buffer);
    // }

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

        buf.serialize(&start);

        let mut s: String = String::new();

        for (i, bytes) in buf.get_ref().chunks(16).enumerate() { // hexdump -C
            write!(s, "\n{:08x}  ", i * 16);

            for (i, b) in bytes.iter().enumerate() {
                if i == 8 {
                    write!(s, " ");
                }
                write!(s, "{:02x} ", b);
            }

            if bytes.len() < 16 {
                for x in 0..(16 - bytes.len()) {
                    let num = 15 - x;
                    if num == 8 {
                        write!(s, "    ");
                    } else {
                        write!(s, "   ");
                    }
                }
            }

            
            write!(s, " |");
            
            for b in bytes {
                if *b > 31  && *b < 127 {
                    write!(s, "{}", char::from(*b));
                } else {
                    write!(s, ".");
                }
            }

            write!(s, "|");
        }

        write!(s, "\n");

        buf.set_position(0);
        let end: constructors::User = buf.deserialize().unwrap();

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

            buf.serialize(&start);

            buf.set_position(0);
            let end: String = buf.deserialize().unwrap();
            
            assert!(start == end, "start = {}, end = {}", start, end);
            // TODO: fuzz?
        }
    }

    #[test]
    fn test_i32() {
        let mut buf: Cursor<Vec<u8>> = Cursor::new(Vec::new());
        let start = -1234;

        buf.serialize(&start);

        buf.set_position(0);
        let end = buf.deserialize::<i32>().unwrap();
            
        assert!(start == end, "start = {}, end = {}", start, end);
    }

    #[test]
    fn test_bool() {
        let mut buf: Cursor<Vec<u8>> = Cursor::new(Vec::new());

        let start1 = true;
        let start2 = false;

        buf.serialize(&start1);
        buf.serialize(&start2);

        buf.set_position(0);
        let end1 = buf.deserialize::<bool>().unwrap();
        let end2 = buf.deserialize::<bool>().unwrap();
            
        assert!(start1 == end1, "start = {}, end = {}", start1, end1);
        assert!(start2 == end2, "start = {}, end = {}", start2, end2);
    }

    #[test]
    fn test_u32() {
        let mut buf: Cursor<Vec<u8>> = Cursor::new(Vec::new());
        let start = 1234;

        buf.serialize(&start);

        buf.set_position(0);
        let end = buf.deserialize::<u32>().unwrap();
            
        assert!(start == end, "start = {}, end = {}", start, end);
    }

    #[test]
    fn test_f32() {
        let mut buf: Cursor<Vec<u8>> = Cursor::new(Vec::new());
        let start = 1.234;

        buf.serialize(&start);

        buf.set_position(0);
        let end = buf.deserialize::<f32>().unwrap();
            
        assert!(start == end, "start = {}, end = {}", start, end);
    }

    #[test]
    fn test_i64() {
        let mut buf: Cursor<Vec<u8>> = Cursor::new(Vec::new());
        let start = -1234;

        buf.serialize(&start);

        buf.set_position(0);
        let end = buf.deserialize::<i64>().unwrap();
            
        assert!(start == end, "start = {}, end = {}", start, end);
    }

    #[test]
    fn test_f64() {
        let mut buf: Cursor<Vec<u8>> = Cursor::new(Vec::new());
        let start = 1.234;

        buf.serialize(&start);

        buf.set_position(0);
        let end = buf.deserialize::<f64>().unwrap();
            
        assert!(start == end, "start = {}, end = {}", start, end);
    }

    #[test]
    fn test_i128() {
        use Int128;

        let mut buf: Cursor<Vec<u8>> = Cursor::new(Vec::new());
        let start = (0x0001020304050607, 0x08090A0B0C0D0E0F);

        buf.serialize(&start);
        println!("{:?}", buf.get_ref());

        buf.set_position(0);
        let end = buf.deserialize::<Int128>().unwrap();
            
        assert!(start == end, "start = {:?}, end = {:?}", start, end);
    }

    #[test]
    fn test_i256() {
        use Int256;

        let mut buf: Cursor<Vec<u8>> = Cursor::new(Vec::new());
        let start = (0x0001020304050607, 0x08090A0B0C0D0E0F, 0x001112131415161718, 0x18191A1B1C1D1E1F);

        buf.serialize(&start);
        println!("{:?}", buf.get_ref());

        buf.set_position(0);
        let end = buf.deserialize::<Int256>().unwrap();
            
        assert!(start == end, "start = {:?}, end = {:?}", start, end);
    }
}