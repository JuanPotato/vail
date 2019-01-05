/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use std::collections::HashMap;

#[macro_use]
mod build_utils;
mod build_process_scheme;
use build_process_scheme::process_tl_scheme;
mod build_types;
use build_types::{write_enum, write_struct, write_method};
mod build_ser;
use build_ser::{ser_enum, ser_struct};
mod build_des;
use build_des::{des_enum, des_struct};

#[derive(Debug)]
pub struct TlCombinator {
    id: u32,
    name: String,
    type_: TlType,
    args: Vec<TlArg>,
}

#[derive(Debug)]
pub struct TlArg {
    name: String,
    type_: TlType,
    bit: Option<usize>,
}

#[derive(Debug)]
pub struct TlType {
    name: String,
    primitive: bool,
    vec: bool,
    boxed: bool,
    vec_boxed: bool,
}


fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let constructors_path = Path::new(&out_dir).join("constructors.rs");
    let cons_ser_path = Path::new(&out_dir).join("constructors_serialize.rs");
    let cons_des_path = Path::new(&out_dir).join("constructors_deserialize.rs");

    let functions_path = Path::new(&out_dir).join("functions.rs");
    let func_ser_path = Path::new(&out_dir).join("functions_serialize.rs");

    let mut constructors_file = File::create(&constructors_path).unwrap();
    let mut cons_ser_file = File::create(&cons_ser_path).unwrap();
    let mut cons_des_file = File::create(&cons_des_path).unwrap();

    let mut functions_file = File::create(&functions_path).unwrap();
    let mut func_ser_file = File::create(&func_ser_path).unwrap();

    let (tl_functions, tl_constructors) = process_tl_scheme();

    let mut type_groups: HashMap<&str, Vec<&str>> = HashMap::new();

    for cons in tl_constructors.values() {
        let mut b = true;

        if let Some(list) = type_groups.get_mut(cons.type_.name.as_str()) {
            list.push(&cons.name);
            b = false;
        }

        if b {
            type_groups.insert(&cons.type_.name, vec![&cons.name]);
        }
    }

    println!("{:?}", type_groups);

    for func in tl_functions.iter().chain(tl_constructors.values()) {
        for arg in &func.args {
            if !type_groups.contains_key(arg.type_.name.as_str()) && !arg.type_.primitive {
                panic!("{} was not a found type. {:?}", &arg.type_.name, &arg);
            }
        }
        if !type_groups.contains_key(func.type_.name.as_str()) && !func.type_.primitive {
            panic!("{} was not a found type. {:?}", &func.type_.name, &func);
        }
    }

    for (type_, group) in type_groups {
        let first = tl_constructors.get(group[0]).unwrap();
        let (obj, ser, des) = if group.len() == 1 && type_ == group[0] {
            process_struct(&first)
        } else {
            process_enum(&group, &tl_constructors, &first.type_.name)
        };

        write!(constructors_file, "{}", obj).unwrap();
        write!(cons_ser_file, "{}", ser).unwrap();
        write!(cons_des_file, "{}", des).unwrap();
    }

    for func in tl_functions {
        let obj = write_struct(&func);
        let ser = ser_struct(&func);
        let impl_meth = write_method(&func);

        write!(functions_file, "{}", obj).unwrap();
        write!(functions_file, "{}", impl_meth).unwrap();
        write!(func_ser_file, "{}", ser).unwrap();
    }
}

fn process_struct(cons: &TlCombinator) -> (String, String, String) {
    (write_struct(cons), ser_struct(cons), des_struct(cons))
}

fn process_enum(
    group: &[&str],
    constructors: &HashMap<String, TlCombinator>,
    type_name: &str,
    ) -> (String, String, String) {
    (
        write_enum(group, constructors, type_name),
        ser_enum(group, constructors, type_name),
        des_enum(group, constructors, type_name),
        )
}

