use byteorder::{LittleEndian, ReadBytesExt};

use std::io::{Read, Result};


pub trait Deserializer: Read where Self: Sized {
    fn deserialize<T: Deserializable + ?Sized>(&mut self, boxed: bool) -> Result<T> {
        T::deserialize_from(self, boxed)
    }

    fn deserialize_with_id<T: Deserializable + ?Sized>(&mut self, boxed: bool, id: u32) -> Result<T> {
        T::deserialize_from_id(self, boxed, id)
    }

    fn deserialize_vec<T: DeserializableVector>(&mut self, boxed: bool, vec_boxed: bool) -> Result<T> {
        T::deserialize_vec_from(self, boxed, vec_boxed)
    }

    fn deserialize_vec_with_id<T: DeserializableVector>(&mut self, boxed: bool, vec_boxed: bool, id: u32) -> Result<T> {
        T::deserialize_vec_from_id(self, boxed, vec_boxed, id)
    }
}


impl<D: Read> Deserializer for D {}


pub trait Deserializable where Self: Sized {
    fn deserialize_from_id<B: Read>(buf: &mut B, boxed: bool, id: u32) -> Result<Self>;
    fn deserialize_from<B: Read>(buf: &mut B, boxed: bool) -> Result<Self> {
        let id = if boxed {
            u32::deserialize_from(buf, false)?
        } else {
            0
        };

        Self::deserialize_from_id(buf, boxed, id)
    }
}
pub trait DeserializableVector where Self: Sized {
    fn deserialize_vec_from_id<B: Read>(buf: &mut B, boxed: bool, vec_boxed: bool, id: u32) -> Result<Self>;
    fn deserialize_vec_from<B: Read>(buf: &mut B, boxed: bool, vec_boxed: bool) -> Result<Self> {
        let id = if boxed {
            u32::deserialize_from(buf, false)?
        } else {
            0
        };

        Self::deserialize_vec_from_id(buf, boxed, vec_boxed, id)
    }
}

impl<T> DeserializableVector for Vec<T> where T: Deserializable {
    fn deserialize_vec_from_id<B: Read>(buf: &mut B, boxed: bool, vec_boxed: bool, vec_id: u32) -> Result<Vec<T>> {
        if vec_boxed {
            assert_eq!(vec_id, 0xd8292816_u32);
        }

        let len = i32::deserialize_from(buf, false)?;

        let mut new_vec = Vec::new();
        for _ in 0..len {
            new_vec.push(T::deserialize_from(buf, boxed)?);
        }

        Ok(new_vec)
    }
}

impl Deserializable for u32 {
    fn deserialize_from_id<B: Read>(buf: &mut B, _boxed: bool, _id: u32) -> Result<u32> {
        Ok(buf.read_u32::<LittleEndian>()?)
    }
}

impl Deserializable for i32 {
    fn deserialize_from_id<B: Read>(buf: &mut B, _boxed: bool, _id: u32) -> Result<i32> {
        Ok(buf.read_i32::<LittleEndian>()?)
    }
}

impl Deserializable for i64 {
    fn deserialize_from_id<B: Read>(buf: &mut B, _boxed: bool, _id: u32) -> Result<i64> {
        Ok(buf.read_i64::<LittleEndian>()?)
    }
}

impl Deserializable for f64 {
    fn deserialize_from_id<B: Read>(buf: &mut B, _boxed: bool, _id: u32) -> Result<f64> {
        Ok(buf.read_f64::<LittleEndian>()?)
    }
}

impl Deserializable for i128 {
    fn deserialize_from_id<B: Read>(buf: &mut B, _boxed: bool, _id: u32) -> Result<i128> {
        Ok(buf.read_i128::<LittleEndian>()?)
    }
}

impl Deserializable for [u8; 32] {
    fn deserialize_from_id<B: Read>(buf: &mut B, _boxed: bool, _id: u32) -> Result<[u8; 32]> {
        let mut buffer = [0u8; 32];

        buf.read_exact(&mut buffer[0..32])?;

        Ok(buffer)
    }
}

impl Deserializable for Vec<u8> {
    fn deserialize_from_id<B: Read>(buf: &mut B, _boxed: bool, _id: u32) -> Result<Vec<u8>> {
        deserialize_bytes(buf)
    }
}

impl Deserializable for String {
    fn deserialize_from_id<B: Read>(buf: &mut B, _boxed: bool, _id: u32) -> Result<String> {
        let bytes = deserialize_bytes(buf)?;
        Ok(String::from_utf8(bytes).unwrap())
    }
}

fn deserialize_bytes<B: Read>(buf: &mut B) -> Result<Vec<u8>> {
    let mut len = buf.read_u8()? as usize;
    let mut bytes_read = len;

    if len <= 253 {
        bytes_read += 1;
    } else {
        len = buf.read_uint::<LittleEndian>(3)? as usize;
        bytes_read = len + 4;
    }

    let mut buffer: Vec<u8> = vec![0; len];
    buf.read_exact(&mut buffer[0..len])?;

    // padding 0's
    for _ in 0..(4 - (bytes_read % 4)) % 4 {
        buf.read_u8()?;
    }

    Ok(buffer)
}
