#![allow(dead_code)]
extern crate byteorder;

mod types;
mod serialize;

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use serialize::Serialize;

    #[test]
    fn it_works() {
        let mut buffer = Cursor::new(Vec::new());
        buffer.serialize(0_u32).unwrap();
        buffer.serialize(0_i64).unwrap();
        println!("{:?}", buffer);
    }
}
