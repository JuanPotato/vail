use regex::Regex;

use std::fs::File;
use std::io::Read;

use std::collections::HashMap;
use build_utils::{is_boxed, is_vec, is_primitive, normalize_name, fix_reserved_name};

use TlType;
use TlArg;
use TlCombinator;


pub fn process_tl_scheme() -> (Vec<TlCombinator>, HashMap<String, TlCombinator>) {
    let mut tl_scheme_file = File::open("./scheme.tl").expect("Could not open scheme file");
    let mut tl_scheme_contents = String::new();

    tl_scheme_file
        .read_to_string(&mut tl_scheme_contents)
        .expect("Could not read scheme file");

    let mut tl_functions = Vec::new();
    let mut tl_constructors = HashMap::new();

    let mut is_function = false;

    for line in tl_scheme_contents.lines() {
        match line {
            "---types---" => {
                is_function = false;
            }

            "---functions---" => {
                is_function = true;
            }

            "" => {}

            _ => {
                if line.starts_with("//") {
                    continue;
                }

                let cons = process_tl_line(line);

                if is_function {
                    tl_functions.push(cons);
                } else {
                    tl_constructors.insert(cons.name.clone(), cons);
                }
            }
        }
    }

    (tl_functions, tl_constructors)
}

pub fn process_tl_line(line: &str) -> TlCombinator {
    lazy_static! {
        static ref SPLITTER: Regex =
            Regex::new(r"^([\w\.]+)#([0-9a-fA-F]{6,8}) (.*?) ?= ([\w\.<>]+);$").unwrap();
    }

    let captures = SPLITTER.captures(line);
    if let Some(tokens) = captures {
        let name = tokens.get(1).unwrap();
        let id = u32::from_str_radix(tokens.get(2).unwrap().as_str(), 16).unwrap();

        let arg_str = tokens.get(3).unwrap().as_str();
        let args = parse_args(arg_str);

        let mut type_ = process_type(tokens.get(4).unwrap().as_str());
        type_.name = fix_reserved_name(&type_.name);

        TlCombinator {
            name: normalize_name(name.as_str()),
            id: id,
            args: args,
            type_: type_,
        }
    } else {
        panic!("Error: \"{}\" Did not match the first regex", line);
    }
}

pub fn parse_args(argstr: &str) -> Vec<TlArg> {
    lazy_static! {
        static ref ARG_REGEX: Regex = Regex::new(r"(\w+):(?:flags\.(\d+)\?)?([\w<>#!\.]+)").unwrap();
    }

    let mut arguments = Vec::new();

    for arg_piece in argstr.split(' ') {
        if arg_piece.is_empty() {
            continue;
        }
        if &arg_piece[..1] == "{" {
            continue;
        }
        if let Some(tokens) = ARG_REGEX.captures(arg_piece.trim()) {
            let name = tokens.get(1).unwrap();
            let bit = tokens.get(2).map(|n| n.as_str().parse::<usize>().unwrap());

            let mut type_ = process_type(tokens.get(3).unwrap().as_str());
            type_.name = fix_reserved_name(&type_.name);

            arguments.push(TlArg {
                name: fix_reserved_name(name.as_str()),
                type_: type_,
                bit: bit,
            });
        } else {
            panic!("Error: \"{}\" Did not match the argument regex", arg_piece);
        }
    }

    arguments
}

pub fn process_type(type_str: &str) -> TlType {
    let (vec, type_) = is_vec(type_str);
    let vec_boxed = if vec {
        (type_str.as_bytes()[0] as char).is_uppercase()
    } else {
        false
    };

    let mut primitive = false;
    let fixed_type = if let Some(new_name) = is_primitive(type_) {
        primitive = true;
        new_name.to_string()
    } else {
        normalize_name(type_)
    };

    let boxed = is_boxed(type_str);

    TlType {
        name: fixed_type,
        primitive: primitive,
        vec: vec,
        boxed: boxed,
        vec_boxed: vec_boxed
    }
}

