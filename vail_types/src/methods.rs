/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::io::{Write, Result, Read};
use serialize::{Serializable, Serializer};
use deserialize::{Deserializable};
use types;
use types::*;


pub trait Method where Self: Serializable, Self::ReturnType: Deserializable {
    type ReturnType;
}

include!(concat!(env!("OUT_DIR"), "/functions.rs"));
include!(concat!(env!("OUT_DIR"), "/functions_serialize.rs"));

