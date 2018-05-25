/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use byteorder::{LittleEndian, WriteBytesExt};

use std::io::{Write, Result};


pub trait Serializer: Write + Sized {
    fn serialize<T: Serializable>(&mut self, obj: &T, boxed: bool) -> Result<()> {
        obj.serialize_into(self, boxed)
    }

    fn serialize_vec<T: SerializableVector>(&mut self, obj: &T, boxed: bool, vec_boxed: bool) -> Result<()> {
        obj.serialize_vec_into(self, boxed, vec_boxed)
    }
}

impl<S: Write> Serializer for S {}


pub trait Serializable {
    fn serialize_into<B: Write>(&self, buf: &mut B, boxed: bool) -> Result<()>;
}

pub trait SerializableVector {
    fn serialize_vec_into<B: Write>(&self, buf: &mut B, boxed: bool, vec_boxed: bool) -> Result<()>;
}

impl<T> SerializableVector for Vec<T> where T: Serializable {
    fn serialize_vec_into<B: Write>(&self, buf: &mut B, boxed: bool, vec_boxed: bool) -> Result<()> {
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
    fn serialize_into<B: Write>(&self, buf: &mut B, _boxed: bool) -> Result<()> {
        buf.write_u32::<LittleEndian>(*self)?;

        Ok(())
    }
}

impl Serializable for i32 {
    fn serialize_into<B: Write>(&self, buf: &mut B, _boxed: bool) -> Result<()> {
        buf.write_i32::<LittleEndian>(*self)?;

        Ok(())
    }
}

impl Serializable for i64 {
    fn serialize_into<B: Write>(&self, buf: &mut B, _boxed: bool) -> Result<()> {
        buf.write_i64::<LittleEndian>(*self)?;

        Ok(())
    }
}

impl Serializable for f64 {
    fn serialize_into<B: Write>(&self, buf: &mut B, _boxed: bool) -> Result<()> {
        buf.write_f64::<LittleEndian>(*self)?;

        Ok(())
    }
}

impl Serializable for i128 {
    fn serialize_into<B: Write>(&self, buf: &mut B, _boxed: bool) -> Result<()> {
        buf.write_i128::<LittleEndian>(*self)?;

        Ok(())
    }
}

impl Serializable for [u8; 32] {
    fn serialize_into<B: Write>(&self, buf: &mut B, _boxed: bool) -> Result<()> {
        buf.write_all(self)?;

        Ok(())
    }
}

impl Serializable for Vec<u8> {
    fn serialize_into<B: Write>(&self, buf: &mut B, _boxed: bool) -> Result<()> {
        serialize_bytes(buf, self)
    }
}

impl Serializable for String {
    fn serialize_into<B: Write>(&self, buf: &mut B, _boxed: bool) -> Result<()> {
        serialize_bytes(buf, self.as_bytes())
    }
}


fn serialize_bytes<B: Write>(buf: &mut B, slice: &[u8]) -> Result<()> {
    let mut len = slice.len();

    if len <= 253 {
        buf.write_all(&[len as u8])?;
        len += 1;
    } else {
        buf.write_all(&[254, len as u8, (len >> 8) as u8, (len >> 16) as u8])?;
    }

    buf.write_all(slice)?;

    for _ in 0..(4 - (len % 4)) % 4 {
        buf.write_all(&[00u8])?;
    }

    Ok(())
}
