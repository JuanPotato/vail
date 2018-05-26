/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

extern crate byteorder;

pub mod types;
pub mod serialize;
pub mod deserialize;

#[cfg(test)]
mod tests {
    use std::io::{Cursor, Seek};
    use std::fmt::Debug;
    use serialize::{Serializer, Serializable};
    use deserialize::{Deserializer, Deserializable};
    use types::User;
    use types;
    use types::Bare;
    use super::*;

    fn ser_des_equal<T: Serializable + Deserializable + PartialEq + Debug>(orig: &T) {
        let mut buffer = Cursor::new(Vec::<u8>::new());

        buffer.serialize(orig).unwrap();
        println!("{}", dump_bytes(buffer.get_ref()).unwrap());

        let orig_pos = buffer.position();
        buffer.seek(std::io::SeekFrom::Start(0)).unwrap();

        let read: T = buffer.deserialize().unwrap();

        assert_eq!(*orig, read);
        assert_eq!(orig_pos, buffer.position());
    }

    #[test]
    #[allow(overflowing_literals)]
    fn i32() {
        ser_des_equal(&0xDEADC0DE_i32);
    }

    #[test]
    fn u32() {
        ser_des_equal(&0xDEADC0DE_u32);
    }

    #[test]
    #[allow(overflowing_literals)]
    fn i64() {
        ser_des_equal(&0xDEADC0DE_BEEFCAFE_i64);
    }

    #[test]
    fn f64() {
        ser_des_equal(&(5674893.56978 as f64));
    }

    #[test]
    fn strings() {
        let mut orig: String = 
            "0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF\
             0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF\
             0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF\
             0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF01234567".into();

        for _ in 0..12 {
            println!("len: {}", orig.len());
            ser_des_equal(&orig);
            orig.push('!');
        }
    }

    #[test]
    fn salt() {
        let mut salts = Vec::new();
        salts.push(types::FutureSalt {
            valid_since: 0x1EAD_BEEF_i32,
            valid_until: 0x1A2B_3C4D_i32,
            salt: 0x123A_BC46_50FE_3C61_i64,
        });

        let f = types::FutureSalts {
            req_msg_id: 0x1012_3456_789A_BCDE_i64,
            now: 0x1234_5678,
            salts: salts,
        };

        ser_des_equal(&f);
    }

    #[test]
    fn user() {
        let mut buffer = Cursor::new(Vec::<u8>::new());

        let u = User::User {
            id: 0x123,
            flags: 0,
            self_: Some(Box::new(types::True)),
            contact: None,
            mutual_contact: None,
            deleted: None,
            bot: None,
            bot_chat_history: None,
            bot_nochats: None,
            verified: None,
            restricted: None,
            min: None,
            bot_inline_geo: None,
            access_hash: None,
            first_name: None,
            last_name: None,
            username: Some("JuanPotato".into()),
            phone: None,
            photo: None,
            status: None,
            bot_info_version: None,
            restriction_reason: None,
            bot_inline_placeholder: None,
            lang_code: None,
        };

        buffer.serialize(&u).unwrap();
        buffer.seek(std::io::SeekFrom::Start(0)).unwrap();

        println!("{}", dump_bytes(buffer.get_ref()).unwrap());

        let new_u: User = buffer.deserialize().unwrap();

        println!("{:?}", &u);
        println!("{:?}", &new_u);

    }

    fn dump_bytes(buf: &[u8]) -> Result<String, std::fmt::Error> {
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
}

