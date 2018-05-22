use std::io::Write;
use serialize::{Serializable, Serializer};
use std::io;

include!(concat!(env!("OUT_DIR"), "/constructors.rs"));
include!(concat!(env!("OUT_DIR"), "/constructors_serialize.rs"));

