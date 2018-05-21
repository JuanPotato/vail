#![allow(dead_code)]
extern crate byteorder;

pub mod types;
pub mod serialize;

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use serialize::Serialize;

    #[test]
    fn it_works() {
        let mut buffer = Cursor::new(Vec::new());
        buffer.serialize(&0_u32, false).unwrap();
        buffer.serialize(&0_i64, false).unwrap();
        println!("{:?}", buffer);
    }
}
