/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::io::{Write, Result, Read};
use serialize::{Serializable, Serializer};
use deserialize::{Deserializable, Deserializer};


#[derive(Debug, Clone, PartialEq)]
pub struct Bare<T>(pub T);

impl<T: Serializable> Serializable for Bare<T> {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        self.0.serialize_bare(buf)
    }
    
    #[inline]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        self.0.serialize_bare(buf)
    }
}

impl<T: Deserializable> Deserializable for Bare<T> {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        T::deserialize_bare(buf, 0).map(|t| Bare(t))
    }

    #[inline]
    fn deserialize_bare<B: Read>(buf: &mut B, _id: u32) -> Result<Self> {
        T::deserialize_bare(buf, 0).map(|t| Bare(t))
    }
}


include!(concat!(env!("OUT_DIR"), "/constructors.rs"));
include!(concat!(env!("OUT_DIR"), "/constructors_serialize.rs"));
include!(concat!(env!("OUT_DIR"), "/constructors_deserialize.rs"));

