use std::io::Cursor;
use std::io;
use serialize::Serialize;
use deserialize::{Deserialize, Deserializer};

use constructors::*;


include!(concat!(env!("OUT_DIR"), "/functions.rs"));
