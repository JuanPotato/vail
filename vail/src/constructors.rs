use std::io::Cursor;
use std::io;
use serialize::Serialize;
use deserialize::{Deserialize, Deserializer};

use {Int128, Int256};

include!(concat!(env!("OUT_DIR"), "/constructors.rs"));
