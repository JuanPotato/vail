#[macro_use]
extern crate tl_derive;

extern crate byteorder;
use byteorder::{LittleEndian, ByteOrder};

mod tl;

trait Serialize {
    fn serialize(&self, buf: &mut [u8]); // serializes the object to the bytes, if there isnt room, panics
    fn size(&self) -> usize; // gives the size in bytes that the object will be needing for serialize
}

impl Serialize for i32 {
    fn serialize(&self, buf: &mut [u8]) {
        LittleEndian::write_i32(buf, *self);
    }

    fn size(&self) -> usize {
        4
    }
}