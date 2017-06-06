use std::io::Cursor;
use std::io;
use serialize::Serialize;

use constructors::*;
use Int128;

include!(concat!(env!("OUT_DIR"), "/functions.rs"));
