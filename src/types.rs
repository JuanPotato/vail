use std::io::Cursor;
use serialize::Serialize;
use serialize::SerializeVector;
use std::io;

include!(concat!(env!("OUT_DIR"), "/constructors.rs"));
include!(concat!(env!("OUT_DIR"), "/constructors_serialize.rs"));

