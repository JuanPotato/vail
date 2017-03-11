#[macro_use]
extern crate tl_derive;
extern crate byteorder;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Cursor, Read, Write, Error, Seek};
use std::io;

mod tl;

#[derive(Debug)]
enum TlErrors {
    Io(io::Error),
}

// impl From<Error>

trait Serialize<S> {
    fn serialize(&mut self, obj: &S) -> Result<(), io::Error>;
}

trait Deserialize<D> {
    fn deserialize(&mut self) -> Result<D, io::Error>;
}

impl Serialize<bool> for Cursor<Vec<u8>> {
    fn serialize(&mut self, obj: &bool) -> Result<(), io::Error> {
        let id: u32 = if *obj { 0x997275b5 } else { 0xbc799737 };

        <Self as Serialize<u32>>::serialize(self, &id)?;

        Ok(())
    }
}

impl Deserialize<bool> for Cursor<Vec<u8>> {
    fn deserialize(&mut self) -> Result<bool, io::Error> {
        Ok(self.read_u32::<LittleEndian>()? == 0x997275b5)
    }
}

impl Serialize<u32> for Cursor<Vec<u8>> {
    fn serialize(&mut self, obj: &u32) -> Result<(), io::Error> {
        self.write_u32::<LittleEndian>(*obj)?;
        
        Ok(())
    }
}

impl Deserialize<u32> for Cursor<Vec<u8>> {
    fn deserialize(&mut self) -> Result<u32, io::Error> {
        Ok(self.read_u32::<LittleEndian>()?)
    }
}

impl Serialize<i32> for Cursor<Vec<u8>> {
    fn serialize(&mut self, obj: &i32) -> Result<(), io::Error> {
        self.write_i32::<LittleEndian>(*obj)?;
        
        Ok(())
    }
}

impl Deserialize<i32> for Cursor<Vec<u8>> {
    fn deserialize(&mut self) -> Result<i32, io::Error> {
        Ok(self.read_i32::<LittleEndian>()?)
    }
}

impl Serialize<f32> for Cursor<Vec<u8>> {
    fn serialize(&mut self, obj: &f32) -> Result<(), io::Error> {
        self.write_f32::<LittleEndian>(*obj)?;
        
        Ok(())
    }
}

impl Deserialize<f32> for Cursor<Vec<u8>> {
    fn deserialize(&mut self) -> Result<f32, io::Error> {
        Ok(self.read_f32::<LittleEndian>()?)
    }
}

impl Serialize<i64> for Cursor<Vec<u8>> {
    fn serialize(&mut self, obj: &i64) -> Result<(), io::Error> {
        self.write_i64::<LittleEndian>(*obj)?;
        
        Ok(())
    }
}

impl Deserialize<i64> for Cursor<Vec<u8>> {
    fn deserialize(&mut self) -> Result<i64, io::Error> {
        Ok(self.read_i64::<LittleEndian>()?)
    }
}

impl Serialize<f64> for Cursor<Vec<u8>> {
    fn serialize(&mut self, obj: &f64) -> Result<(), io::Error> {
        self.write_f64::<LittleEndian>(*obj)?;
        
        Ok(())
    }
}

impl Deserialize<f64> for Cursor<Vec<u8>> {
    fn deserialize(&mut self) -> Result<f64, io::Error> {
        Ok(self.read_f64::<LittleEndian>()?)
    }
}

impl Serialize<Vec<u8>> for Cursor<Vec<u8>> {
    fn serialize(&mut self, obj: &Vec<u8>) -> Result<(), io::Error>  {
        let mut len = obj.len();
        
        if len < 254 {
            self.write(&[len as u8])?;
            len += 1; 
        } else {
            self.write_all(&[254, len as u8, (len >> 8) as u8, (len >> 16) as u8])?; // 3 bytes
        }

        self.write_all(obj)?;

        for _ in 0 .. (4 - (len % 4)) % 4 {
            self.write(&[00u8])?;
        }
        
        Ok(())
    }
}

impl Serialize<String> for Cursor<Vec<u8>> {
    fn serialize(&mut self, obj: &String) -> Result<(), io::Error> {
        let mut len = obj.len();
        
        if len < 254 {
            self.write(&[len as u8])?;
            len += 1; 
        } else {
            self.write_all(&[254, len as u8, (len >> 8) as u8, (len >> 16) as u8])?; // 3 bytes
        }

        self.write_all(obj.as_bytes())?;

        for _ in 0 .. (4 - (len % 4)) % 4 {
            self.write(&[00u8])?;
        }
        
        Ok(())
    }
}

impl Deserialize<String> for Cursor<Vec<u8>> {
    fn deserialize(&mut self) -> Result<String, io::Error> {
        let mut len = self.read_u8()? as usize;
        
        if len == 254 {
            let mut buf = [0; 3];
            self.read_exact(&mut buf)?;

            len = buf[0] as usize + ((buf[1] as usize) << 8) + ((buf[2] as usize) << 16);
        }

        let mut buffer = vec![0; len];

        self.read_exact(buffer.as_mut_slice())?;

        if len < 254 { len += 1; }

        let mut zbuf = Vec::with_capacity((4 - (len % 4)) % 4);
        self.read_exact(&mut zbuf)?;

        Ok(String::from_utf8(buffer).unwrap())
    }
}

impl<T> Serialize<Vec<T>> for Cursor<Vec<u8>> where
        Cursor<Vec<u8>>: Serialize<T> {
    fn serialize(&mut self, obj: &Vec<T>) -> Result<(), io::Error> {
        <Self as Serialize<u32>>::serialize(self, &0x1cb5c415u32)?; // Vector id
        <Self as Serialize<u32>>::serialize(self, &(obj.len() as u32))?; // Vector id

        for item in obj.iter() {
            self.serialize(item)?;
        }
        
        Ok(())
    }
}

impl<T> Serialize<Box<T>> for Cursor<Vec<u8>> where
        Cursor<Vec<u8>>: Serialize<T> {
    fn serialize(&mut self, obj: &Box<T>) -> Result<(), io::Error> {
        self.serialize(&**obj)?;
        
        Ok(())
    }
}

#[test]
fn test_string() {
    let s = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
    for i in 253..256 {
        let mut buf: Cursor<Vec<u8>> = Cursor::new(Vec::new());
        buf.serialize(&(s[0..(i+1)].to_string()));
        println!("{:?}", buf);

        buf.set_position(0);
        let s: String = buf.deserialize().unwrap();
        println!("{:?}", s);

        // what is assert
    }
}
