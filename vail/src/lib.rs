#[macro_use]
extern crate tl_derive;
extern crate byteorder;

use std::io;
use std::io::Cursor;

use std::net::TcpStream;
use deserialize::Deserializer;

mod constructors;
mod functions;
mod serialize;
mod deserialize;

#[derive(Debug)]
enum TlErrors {
    Io(io::Error),
}

#[derive(Debug)]
pub struct MtProtoConnection {
    conn: TcpStream
}

#[test]
fn test() {
    use::serialize::Serialize;
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
    use::serialize::Serialize;

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
    use::serialize::Serialize;

    let mut buf: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    let start = -1234;

    buf.serialize(&start);

    buf.set_position(0);
    let end = buf.deserialize::<i32>().unwrap();
        
    assert!(start == end, "start = {}, end = {}", start, end);
}

#[test]
fn test_bool() {
    use::serialize::Serialize;

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
    use::serialize::Serialize;

    let mut buf: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    let start = 1234;

    buf.serialize(&start);

    buf.set_position(0);
    let end = buf.deserialize::<u32>().unwrap();
        
    assert!(start == end, "start = {}, end = {}", start, end);
}

#[test]
fn test_f32() {
    use::serialize::Serialize;

    let mut buf: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    let start = 1.234;

    buf.serialize(&start);

    buf.set_position(0);
    let end = buf.deserialize::<f32>().unwrap();
        
    assert!(start == end, "start = {}, end = {}", start, end);
}

#[test]
fn test_i64() {
    use::serialize::Serialize;

    let mut buf: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    let start = -1234;

    buf.serialize(&start);

    buf.set_position(0);
    let end = buf.deserialize::<i64>().unwrap();
        
    assert!(start == end, "start = {}, end = {}", start, end);
}

#[test]
fn test_f64() {
    use::serialize::Serialize;

    let mut buf: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    let start = 1.234;

    buf.serialize(&start);

    buf.set_position(0);
    let end = buf.deserialize::<f64>().unwrap();
        
    assert!(start == end, "start = {}, end = {}", start, end);
}
