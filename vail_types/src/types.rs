/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::io::{Write, Result, Read};
use serialize::{Serializable, Serializer};
use deserialize::{Deserializable, Deserializer};

include!(concat!(env!("OUT_DIR"), "/constructors.rs"));
include!(concat!(env!("OUT_DIR"), "/constructors_serialize.rs"));
include!(concat!(env!("OUT_DIR"), "/constructors_deserialize.rs"));

