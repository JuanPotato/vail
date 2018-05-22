#![allow(dead_code)]
extern crate byteorder;

pub mod types;
pub mod serialize;

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use serialize::Serialize;
    use types::User;
    use types;
    use super::dump_bytes;


    #[test]
    fn salt() {
        let mut buffer = Cursor::new(Vec::<u8>::new());

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

        buffer.serialize(&f, true);
        
        println!("{}", dump_bytes(buffer.get_ref()).unwrap());
        println!("{:?}", buffer.get_ref());
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

        buffer.serialize(&u, true);
        
        println!("{}", dump_bytes(buffer.get_ref()).unwrap());
    }
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
