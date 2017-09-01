/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::io::Cursor;
use std::io;
use serialize::Serialize;
use deserialize::{Deserialize, Deserializer};

use {Int128, Int256};

include!(concat!(env!("OUT_DIR"), "/constructors.rs"));
