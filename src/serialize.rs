use byteorder::{LittleEndian, WriteBytesExt};

use std::io::{Cursor, Write};
use std::io;

pub trait Serialize<T> {
    fn serialize(&mut self, obj: T, boxed: bool) -> Result<(), io::Error>;
}

pub trait SerializeVector<'a, T> {
    fn serialize_vec(&mut self, obj: &'a [T], boxed: bool, vec_boxed: bool) -> Result<(), io::Error>;
}

impl<'a, T: 'a> SerializeVector<'a, T> for Cursor<Vec<u8>> where Cursor<Vec<u8>>: Serialize<&'a T> {
    fn serialize_vec(&mut self, vector: &'a [T], boxed: bool, vec_boxed: bool) -> Result<(), io::Error> {
        if vec_boxed {
            Serialize::<&u32>::serialize(self, &0xd8292816_u32, false)?;
        }

        for element in vector {
            Serialize::<&T>::serialize(self, element, boxed)?;
        }

        Ok(())
    }
}

impl<'a> Serialize<&'a i32> for Cursor<Vec<u8>> {
    fn serialize(&mut self, obj: &i32, _: bool) -> Result<(), io::Error> {
        self.write_i32::<LittleEndian>(*obj)?;

        Ok(())
    }
}

impl<'a> Serialize<&'a u32> for Cursor<Vec<u8>> {
    fn serialize(&mut self, obj: &u32, _: bool) -> Result<(), io::Error> {
        self.write_u32::<LittleEndian>(*obj)?;

        Ok(())
    }
}

impl<'a> Serialize<&'a i64> for Cursor<Vec<u8>> {
    fn serialize(&mut self, obj: &i64, _: bool) -> Result<(), io::Error> {
        self.write_i64::<LittleEndian>(*obj)?;

        Ok(())
    }
}

impl<'a> Serialize<&'a f64> for Cursor<Vec<u8>> {
    fn serialize(&mut self, obj: &f64, _: bool) -> Result<(), io::Error> {
        self.write_f64::<LittleEndian>(*obj)?;

        Ok(())
    }
}

impl<'a> Serialize<&'a [u8; 16]> for Cursor<Vec<u8>> {
    fn serialize(&mut self, obj: &[u8; 16], _: bool) -> Result<(), io::Error> {
        self.write_all(obj)?;

        Ok(())
    }
}

impl<'a> Serialize<&'a [u8; 32]> for Cursor<Vec<u8>> {
    fn serialize(&mut self, obj: &[u8; 32], _: bool) -> Result<(), io::Error> {
        self.write_all(obj)?;

        Ok(())
    }
}

impl<'a> Serialize<&'a [u8]> for Cursor<Vec<u8>> {
    fn serialize(&mut self, obj: &[u8], _: bool) -> Result<(), io::Error> {
        let mut len = obj.len();

        if len < 254 {
            self.write_all(&[len as u8])?;
            len += 1;
        } else {
            self.write_all(&[254, len as u8, (len >> 8) as u8, (len >> 16) as u8])?; // 3 bytes
        }

        self.write_all(obj)?;

        for _ in 0..(4 - (len % 4)) % 4 {
            self.write_all(&[00u8])?;
        }

        Ok(())
    }
}

