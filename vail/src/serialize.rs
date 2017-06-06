use std::io::{Cursor, Write};
use std::io;

use byteorder::{LittleEndian, WriteBytesExt};

use {Int128, Int256};

pub trait Serialize<S> {
    fn serialize(&mut self, obj: &S) -> Result<(), io::Error>;
}

impl Serialize<bool> for Cursor<Vec<u8>> {
    fn serialize(&mut self, obj: &bool) -> Result<(), io::Error> {
        let id: u32 = if *obj { 0x997275b5 } else { 0xbc799737 };

        <Self as Serialize<u32>>::serialize(self, &id)?;

        Ok(())
    }
}

impl Serialize<u32> for Cursor<Vec<u8>> {
    fn serialize(&mut self, obj: &u32) -> Result<(), io::Error> {
        self.write_u32::<LittleEndian>(*obj)?;
        
        Ok(())
    }
}

impl Serialize<i32> for Cursor<Vec<u8>> {
    fn serialize(&mut self, obj: &i32) -> Result<(), io::Error> {
        self.write_i32::<LittleEndian>(*obj)?;
        
        Ok(())
    }
}

impl Serialize<f32> for Cursor<Vec<u8>> {
    fn serialize(&mut self, obj: &f32) -> Result<(), io::Error> {
        self.write_f32::<LittleEndian>(*obj)?;
        
        Ok(())
    }
}

impl Serialize<i64> for Cursor<Vec<u8>> {
    fn serialize(&mut self, obj: &i64) -> Result<(), io::Error> {
        self.write_i64::<LittleEndian>(*obj)?;
        
        Ok(())
    }
}

impl Serialize<f64> for Cursor<Vec<u8>> {
    fn serialize(&mut self, obj: &f64) -> Result<(), io::Error> {
        self.write_f64::<LittleEndian>(*obj)?;
        
        Ok(())
    }
}

impl Serialize<Int128> for Cursor<Vec<u8>> {
    fn serialize(&mut self, obj: &Int128) -> Result<(), io::Error> {
        self.write_u64::<LittleEndian>(obj.1)?;
        self.write_u64::<LittleEndian>(obj.0)?;
        
        Ok(())
    }
}

impl Serialize<Int256> for Cursor<Vec<u8>> {
    fn serialize(&mut self, obj: &Int256) -> Result<(), io::Error> {
        self.write_u64::<LittleEndian>(obj.3)?;
        self.write_u64::<LittleEndian>(obj.2)?;
        self.write_u64::<LittleEndian>(obj.1)?;
        self.write_u64::<LittleEndian>(obj.0)?;
        
        Ok(())
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

impl<T> Serialize<Vec<T>> for Cursor<Vec<u8>> where
        Cursor<Vec<u8>>: Serialize<T> {
    fn serialize(&mut self, obj: &Vec<T>) -> Result<(), io::Error> {
        <Self as Serialize<u32>>::serialize(self, &0x1cb5c415u32)?; // Vector id
        <Self as Serialize<u32>>::serialize(self, &(obj.len() as u32))?;

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
