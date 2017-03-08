#[macro_use]
extern crate tl_derive;
extern crate byteorder;

use byteorder::{LittleEndian, WriteBytesExt};
use std::io::{Cursor, Write};

mod tl;

trait Serialize<T> {
    fn serialize(&mut self, obj: &T);
}

impl Serialize<bool> for Cursor<Vec<u8>> {
    fn serialize(&mut self, obj: &bool) {
        if *obj {
            <Self as Serialize<u32>>::serialize(self, &0xbc799737u32); // False id
        } else {
            <Self as Serialize<u32>>::serialize(self, &0x997275b5u32); // True id
        }
    }
}

impl Serialize<u32> for Cursor<Vec<u8>> {
    fn serialize(&mut self, obj: &u32) {
        self.write_u32::<LittleEndian>(*obj).unwrap();
    }
}

impl Serialize<i32> for Cursor<Vec<u8>> {
    fn serialize(&mut self, obj: &i32) {
        self.write_i32::<LittleEndian>(*obj).unwrap();
    }
}

impl Serialize<f32> for Cursor<Vec<u8>> {
    fn serialize(&mut self, obj: &f32) {
        self.write_f32::<LittleEndian>(*obj).unwrap();
    }
}

impl Serialize<i64> for Cursor<Vec<u8>> {
    fn serialize(&mut self, obj: &i64) {
        self.write_i64::<LittleEndian>(*obj).unwrap();
    }
}

impl Serialize<f64> for Cursor<Vec<u8>> {
    fn serialize(&mut self, obj: &f64) {
        self.write_f64::<LittleEndian>(*obj).unwrap();
    }
}

impl Serialize<Vec<u8>> for Cursor<Vec<u8>> {
    fn serialize(&mut self, obj: &Vec<u8>) {
        let mut len = obj.len();
        
        if len < 254 {
            let _ = self.write(&[len as u8]);
            len += 1; 
        } else {
            let _ = self.write_all(&[len as u8, (len >> 8) as u8, (len >> 16) as u8]); // 3 bytes
        }

        let _ = self.write_all(obj);

        for _ in 0 .. (4 - len % 4) {
            let _ = self.write(&[00u8]);
        }
    }
}

impl Serialize<String> for Cursor<Vec<u8>> {
    fn serialize(&mut self, obj: &String) {
        let mut len = obj.len();
        
        if len < 254 {
            let _ = self.write(&[len as u8]);
            len += 1; 
        } else {
            let _ = self.write_all(&[len as u8, (len >> 8) as u8, (len >> 16) as u8]); // 3 bytes
        }

        let _ = self.write_all(obj.as_bytes());

        for _ in 0 .. (4 - len % 4) {
            let _ = self.write(&[00u8]);
        }
    }
}

impl<T> Serialize<Vec<T>> for Cursor<Vec<u8>> where
        Cursor<Vec<u8>>: Serialize<T> {
    fn serialize(&mut self, obj: &Vec<T>) {
        <Self as Serialize<u32>>::serialize(self, &0x1cb5c415u32); // Vector id
        <Self as Serialize<u32>>::serialize(self, &(obj.len() as u32)); // Vector id

        for item in obj.iter() {
            self.serialize(item);
        }
    }
}

impl<T> Serialize<Box<T>> for Cursor<Vec<u8>> where
        Cursor<Vec<u8>>: Serialize<T> {
    fn serialize(&mut self, obj: &Box<T>) {
        self.serialize(&**obj);
    }
}

#[test]
fn test() {
    let mut buf: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    buf.serialize(String::from("Potato"));
    println!("{:?}", buf);
}
