use std::io::{Write, Result, Read};
use serialize::{Serializable, Serializer};
use deserialize::{Deserializable, Deserializer};

include!(concat!(env!("OUT_DIR"), "/constructors.rs"));
include!(concat!(env!("OUT_DIR"), "/constructors_serialize.rs"));
include!(concat!(env!("OUT_DIR"), "/constructors_deserialize.rs"));

