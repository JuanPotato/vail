/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use byteorder::{LittleEndian, ReadBytesExt};

use std::io::{Read, Result};


pub trait Deserializer: Read + Sized {
    #[inline]
    fn deserialize<T: Deserializable>(&mut self) -> Result<T> {
        T::deserialize_from(self)
    }

    #[inline]
    fn deserialize_with_id<T: Deserializable>(&mut self, id: u32) -> Result<T> {
        T::deserialize_bare(self, id)
    }
}

impl<D: Read> Deserializer for D {}


pub trait Deserializable where Self: Sized {
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self>;
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<Self>;
}

impl<T> Deserializable for Vec<T> where T: Deserializable {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x1cb5c415_u32);

        Self::deserialize_bare(buf, id)
    }

    #[inline]
    fn deserialize_bare<B: Read>(buf: &mut B, _id: u32) -> Result<Vec<T>> {
        let len = i32::deserialize_from(buf)?;

        let mut new_vec = Vec::new();
        for _ in 0..len {
            new_vec.push(T::deserialize_from(buf)?);
        }

        Ok(new_vec)
    }
}

impl Deserializable for u32 {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        Self::deserialize_bare(buf, 0)
    }

    #[inline]
    fn deserialize_bare<B: Read>(buf: &mut B, _id: u32) -> Result<u32> {
        Ok(buf.read_u32::<LittleEndian>()?)
    }
}

impl Deserializable for i32 {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        Self::deserialize_bare(buf, 0)
    }

    #[inline]
    fn deserialize_bare<B: Read>(buf: &mut B, _id: u32) -> Result<i32> {
        Ok(buf.read_i32::<LittleEndian>()?)
    }
}

impl Deserializable for i64 {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        Self::deserialize_bare(buf, 0)
    }

    #[inline]
    fn deserialize_bare<B: Read>(buf: &mut B, _id: u32) -> Result<i64> {
        Ok(buf.read_i64::<LittleEndian>()?)
    }
}

impl Deserializable for f64 {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        Self::deserialize_bare(buf, 0)
    }

    #[inline]
    fn deserialize_bare<B: Read>(buf: &mut B, _id: u32) -> Result<f64> {
        Ok(buf.read_f64::<LittleEndian>()?)
    }
}

impl Deserializable for i128 {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        Self::deserialize_bare(buf, 0)
    }
 
    #[inline]
    fn deserialize_bare<B: Read>(buf: &mut B, _id: u32) -> Result<i128> {
        Ok(buf.read_i128::<LittleEndian>()?)
    }
}

impl Deserializable for [u8; 32] {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        Self::deserialize_bare(buf, 0)
    }

    #[inline]
    fn deserialize_bare<B: Read>(buf: &mut B, _id: u32) -> Result<[u8; 32]> {
        let mut buffer = [0u8; 32];

        buf.read_exact(&mut buffer[0..32])?;

        Ok(buffer)
    }
}

impl Deserializable for Vec<u8> {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        Self::deserialize_bare(buf, 0)
    }

    #[inline]
    fn deserialize_bare<B: Read>(buf: &mut B, _id: u32) -> Result<Vec<u8>> {
        deserialize_bytes(buf)
    }
}

impl Deserializable for String {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        Self::deserialize_bare(buf, 0)
    }

    #[inline]
    fn deserialize_bare<B: Read>(buf: &mut B, _id: u32) -> Result<String> {
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
