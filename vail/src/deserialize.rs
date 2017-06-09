use std::io::{Cursor, Read};
use std::io;

use byteorder::{LittleEndian, ReadBytesExt};

use {Int128, Int256};

pub trait Deserialize<D> {
    fn _deserialize(&mut self, tl_id: u32) -> Result<D, io::Error>; // I need a more elegant solution
}

pub trait Deserializer {
    fn deserialize<T>(&mut self, tl_id: u32) -> Result<T, io::Error> where Self: Deserialize<T>;
}

impl Deserializer for Cursor<Vec<u8>> {
    fn deserialize<T>(&mut self, tl_id: u32) -> Result<T, io::Error> where Cursor<Vec<u8>>: Deserialize<T> {
        <Self as Deserialize<T>>::_deserialize(self, tl_id)
    }
}

// impl Deserialize<bool> for Cursor<Vec<u8>> {
//     fn _deserialize(&mut self, tl_id: u32) -> Result<bool, io::Error> {
//         Ok(tl_id == 0x997275b5)
//     }
// }

impl Deserialize<u32> for Cursor<Vec<u8>> {
    fn _deserialize(&mut self, tl_id: u32) -> Result<u32, io::Error> {
        Ok(self.read_u32::<LittleEndian>()?)
    }
}

impl Deserialize<i32> for Cursor<Vec<u8>> {
    fn _deserialize(&mut self, tl_id: u32) -> Result<i32, io::Error> {
        Ok(self.read_i32::<LittleEndian>()?)
    }
}

impl Deserialize<f32> for Cursor<Vec<u8>> {
    fn _deserialize(&mut self, tl_id: u32) -> Result<f32, io::Error> {
        Ok(self.read_f32::<LittleEndian>()?)
    }
}

impl Deserialize<i64> for Cursor<Vec<u8>> {
    fn _deserialize(&mut self, tl_id: u32) -> Result<i64, io::Error> {
        Ok(self.read_i64::<LittleEndian>()?)
    }
}

impl Deserialize<f64> for Cursor<Vec<u8>> {
    fn _deserialize(&mut self, tl_id: u32) -> Result<f64, io::Error> {
        Ok(self.read_f64::<LittleEndian>()?)
    }
}

impl Deserialize<Int128> for Cursor<Vec<u8>> {
    fn _deserialize(&mut self, tl_id: u32) -> Result<Int128, io::Error> {
        let part1 = self.read_u64::<LittleEndian>()?;
        let part2 = self.read_u64::<LittleEndian>()?;

        Ok((part2, part1))
    }
}

impl Deserialize<Int256> for Cursor<Vec<u8>> {
    fn _deserialize(&mut self, tl_id: u32) -> Result<Int256, io::Error> {
        let part1 = self.read_u64::<LittleEndian>()?;
        let part2 = self.read_u64::<LittleEndian>()?;
        let part3 = self.read_u64::<LittleEndian>()?;
        let part4 = self.read_u64::<LittleEndian>()?;

        Ok((part4, part3, part2, part1))
    }
}

impl Deserialize<String> for Cursor<Vec<u8>> {
    fn _deserialize(&mut self, tl_id: u32) -> Result<String, io::Error> {
        let buf = self.deserialize::<Vec<u8>>(0)?;
        
        Ok(String::from_utf8(buf).unwrap()) // the string better be correct
    }
}

impl Deserialize<Vec<u8>> for Cursor<Vec<u8>> {
    fn _deserialize(&mut self, tl_id: u32) -> Result<Vec<u8>, io::Error> {
        let mut len = self.read_u8()? as usize;
        
        if len == 254 {
            let mut buf = [0; 3];
            self.read_exact(&mut buf)?;

            len = buf[0] as usize + ((buf[1] as usize) << 8) + ((buf[2] as usize) << 16);
        }

        let mut buffer = vec![0; len];

        self.read_exact(buffer.as_mut_slice())?;

        if len < 254 { len += 1; }

        let mut zbuf = vec![0; (4 - (len % 4)) % 4];
        self.read_exact(zbuf.as_mut_slice())?;

        Ok(buffer)
    }
}

impl<T> Deserialize<Vec<T>> for Cursor<Vec<u8>> where
        Cursor<Vec<u8>>: Deserialize<T> {
    fn _deserialize(&mut self, tl_id: u32) -> Result<Vec<T>, io::Error> {
        assert!(self.deserialize::<u32>(0)? == 0x1cb5c415); // Vector id
        // oh no i made an assert; U should just raise an Error u fuck

        let len = self.deserialize::<u32>(0)?;

        let mut items: Vec<T> = Vec::with_capacity(len as usize);

        for _ in 0..len {
            items.push(self.deserialize::<T>(0)?);
        }
        
        Ok(items)
    }
}
