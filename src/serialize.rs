use byteorder::{LittleEndian, WriteBytesExt};

use std::io::Write;
use std::io;


pub trait Serializer: Write where Self: Sized {
    fn serialize<T: Serializable + ?Sized>(&mut self, obj: &T, boxed: bool) -> Result<(), io::Error> {
        obj.serialize_into(self, boxed)
    }

    fn serialize_vec<T: SerializableVector>(&mut self, obj: &T, boxed: bool, vec_boxed: bool) -> Result<(), io::Error> {
        obj.serialize_vec_into(self, boxed, vec_boxed)
    }
}

impl<S: Write> Serializer for S {}


pub trait Serializable {
    fn serialize_into<B: Write>(&self, buf: &mut B, boxed: bool) -> Result<(), io::Error>;
}

pub trait SerializableVector {
    fn serialize_vec_into<B: Write>(&self, buf: &mut B, boxed: bool, vec_boxed: bool) -> Result<(), io::Error>;
}

impl<T> SerializableVector for Vec<T> where T: Serializable {
    fn serialize_vec_into<B: Write>(&self, buf: &mut B, boxed: bool, vec_boxed: bool) -> Result<(), io::Error> {
        if vec_boxed {
            0xd8292816_u32.serialize_into(buf, false)?;
        }

        (self.len() as i32).serialize_into(buf, false)?;

        for element in self {
            element.serialize_into(buf, boxed)?;
        }

        Ok(())
    }
}

impl Serializable for u32 {
    fn serialize_into<B: Write>(&self, buf: &mut B, _boxed: bool) -> Result<(), io::Error> {
        buf.write_u32::<LittleEndian>(*self)?;

        Ok(())
    }
}

impl Serializable for i32 {
    fn serialize_into<B: Write>(&self, buf: &mut B, _boxed: bool) -> Result<(), io::Error> {
        buf.write_i32::<LittleEndian>(*self)?;

        Ok(())
    }
}

impl Serializable for i64 {
    fn serialize_into<B: Write>(&self, buf: &mut B, _boxed: bool) -> Result<(), io::Error> {
        buf.write_i64::<LittleEndian>(*self)?;

        Ok(())
    }
}

impl Serializable for f64 {
    fn serialize_into<B: Write>(&self, buf: &mut B, _boxed: bool) -> Result<(), io::Error> {
        buf.write_f64::<LittleEndian>(*self)?;

        Ok(())
    }
}

impl Serializable for [u8; 16] {
    fn serialize_into<B: Write>(&self, buf: &mut B, _boxed: bool) -> Result<(), io::Error> {
        buf.write_all(self)?;

        Ok(())
    }
}

impl Serializable for [u8; 32] {
    fn serialize_into<B: Write>(&self, buf: &mut B, _boxed: bool) -> Result<(), io::Error> {
        buf.write_all(self)?;

        Ok(())
    }
}

impl Serializable for [u8] {
    fn serialize_into<B: Write>(&self, buf: &mut B, _boxed: bool) -> Result<(), io::Error> {
        let mut len = self.len();

        if len < 254 {
            buf.write_all(&[len as u8])?;
            len += 1;
        } else {
            buf.write_all(&[254, len as u8, (len >> 8) as u8, (len >> 16) as u8])?;
        }

        buf.write_all(self)?;

        for _ in 0..(4 - (len % 4)) % 4 {
            buf.write_all(&[00u8])?;
        }

        Ok(())
    }
}

