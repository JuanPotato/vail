#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;

use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::fmt::Write as FmtWrite;
use std::path::Path;

use std::collections::HashMap;

#[derive(Debug)]
struct TlCombinator {
    id: u32,
    name: String,
    type_: TlType,
    args: Vec<TlArg>,
}

#[derive(Debug)]
struct TlArg {
    name: String,
    type_: TlType,
    bit: Option<usize>,
}

#[derive(Debug)]
struct TlType {
    name: String,
    primitive: bool,
    vec: bool,
    boxed: bool,
    vec_boxed: bool,
}

macro_rules! optional {
    ($var:ident, $default:expr) => (
        if $var { $default } else { "" }
    );


    ($var:ident, $default:expr, $else:expr) => (
        if $var { $default } else { $else }
    )
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let constructors_path = Path::new(&out_dir).join("constructors.rs");
    let cons_ser_path = Path::new(&out_dir).join("constructors_serialize.rs");
    let functions_path = Path::new(&out_dir).join("functions.rs");

    let mut constructors_file = File::create(&constructors_path).unwrap();
    let mut cons_sed_file = File::create(&cons_ser_path).unwrap();
    let mut functions_file = File::create(&functions_path).unwrap();

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
        let (obj, ser) = if group.len() == 1 && type_ == group[0] {
            process_struct(&first)
        } else {
            process_enum(&group, &tl_constructors, &first.type_.name)
        };

        write!(constructors_file, "{}", obj).unwrap();
        write!(cons_sed_file, "{}", ser).unwrap();
    }
}

fn process_struct(cons: &TlCombinator) -> (String, String) {
    (write_struct(cons), ser_struct(cons))
}

fn ser_struct(cons: &TlCombinator) -> String {
    format!(
        "impl<'a> Serialize<&'a {name}> for Cursor<Vec<u8>> {{\n    \
             fn serialize(&mut self, obj: &{name}, boxed: bool) -> Result<(), io::Error> {{\n        \
                if boxed {{\n            \
                    self.serialize(&0x{id:08x}_u32, false)?;\n        \
                }}\n\n\
                {args}\n        \
                Ok(())\n    \
            }}\n\
         }}\n\n",
        name = &cons.name,
        args = ser_args(&cons.args, 8, true),
        id = cons.id
    )
}

fn write_struct(cons: &TlCombinator) -> String {
    if cons.args.len() != 0 {
        format!(
"#[derive(Debug)]\n\
pub struct {name} {{\n\
    {args}\
}}\n\n",
            name = &cons.name,
            args = write_args(&cons.args, 4, true)
        )
    } else {
        format!(
"#[derive(Debug)]\n\
    pub struct {name};\n\n",
            name = &cons.name
        )
    }
}

fn process_enum(
    group: &[&str],
    constructors: &HashMap<String, TlCombinator>,
    type_name: &str,
) -> (String, String) {
    (
        write_enum(group, constructors, type_name),
        ser_enum(group, constructors, type_name),
    )
}

fn ser_enum(
    group: &[&str],
    constructors: &HashMap<String, TlCombinator>,
    type_name: &str,
) -> String {
    let mut out = format!(
"impl<'a> Serialize<&'a {name}> for Cursor<Vec<u8>> {{\n    \
    fn serialize(&mut self, obj: &{name}, boxed: bool) -> Result<(), io::Error> {{\n        \
        if boxed {{\n            \
            match obj {{",

        name = type_name
    );

    for variant in group {
        let cons = constructors.get(*variant).unwrap();
        let variant_name = filter_variant(&cons.name, &cons.type_.name);

        write!(
            out,
            "\n                \
                {type_name}::{name} {{ .. }} => self.serialize(&0x{id:08x}_u32, false)?,",
            type_name = type_name,
            name = &variant_name,
            id = cons.id,
        ).unwrap();
    }

    write!(out, "\n            \
            }}\n        \
        }}\n\n        \
        Ok(())\n        \
        " // \
        //match obj {{"
        ).unwrap();
    // end if boxed


    /*
    for variant in group {
        let cons = constructors.get(*variant).unwrap();
        let variant_name = filter_variant(&cons.name, &cons.type_.name);

        write!(
            out,
            "\n            \
                {type_name}::{name} => {{\n                ",
            type_name = type_name,
            name = &variant_name,
            id = cons.id,
        );


        if cons.args.len() != 0 {
            writeln!(
                out,
                "\n    {name} {{\n\
                 {args}    \
                 }},",
                name = &variant_name,
                args = write_args(&cons.args, 8, false)
            );
        } else {
            writeln!(out, "\n    {name},", name = &variant_name).unwrap();
        }
    }
    */


    writeln!(out, "}}\n").unwrap();
    writeln!(out, "}}\n").unwrap();
    out
}

fn write_enum(
    group: &[&str],
    constructors: &HashMap<String, TlCombinator>,
    type_name: &str,
) -> String {
    let mut out = format!(
        "#[derive(Debug)]\n\
         pub enum {name} {{",
        name = type_name
    );

    for variant in group {
        let cons = constructors.get(*variant).unwrap();
        let variant_name = filter_variant(&cons.name, &cons.type_.name);

        if cons.args.len() != 0 {
            writeln!(
                out,
                "\n    {name} {{\n\
                 {args}    \
                 }},",
                name = &variant_name,
                args = write_args(&cons.args, 8, false)
            );
        } else {
            writeln!(out, "\n    {name},", name = &variant_name).unwrap();
        }
    }

    writeln!(out, "}}\n").unwrap();
    out
}

fn write_args(args: &[TlArg], indent: usize, do_pub: bool) -> String {
    let mut out = String::new();

    for arg in args {
        let mut adjusted_type = arg.type_.name.clone();

        if arg.type_.vec {
            adjusted_type = format!("Vec<{}>", adjusted_type);
        }

        if !arg.type_.primitive && !arg.type_.vec {
            adjusted_type = format!("Box<{}>", adjusted_type);
        }

        if arg.bit.is_some() {
            adjusted_type = format!("Option<{}>", adjusted_type);
        }

        writeln!(
            out,
            "{:indent$}{pu}{name}: {typ},",
            "",
            pu = if do_pub { "pub " } else { "" },
            name = &arg.name,
            typ = adjusted_type,
            indent = indent
        );
    }

    out
}

fn ser_args(args: &[TlArg], indent: usize, do_obj: bool) -> String {
    let mut out = String::new();

    for arg in args {
        let as_ref = !arg.type_.primitive
            || arg.type_.vec
            || arg.type_.name == "String"
            || arg.type_.name == "string"
            || arg.type_.name == "Vec<u8>";
        
        let mut ser_arg = arg.name.clone();

        if do_obj {
            ser_arg = format!("obj.{}", ser_arg);
        }

        let as_ref_func = match arg.type_.name.as_ref() /* heh */ {
            "String" | "string" => ".as_bytes()",
            "Vec<u8>" => ".as_slice()",
            _ => ".as_ref()"
        };

        if arg.bit.is_some() {
            writeln!(
                out,
                "\n{empty:indent$}if let Some(ref value) = {obj}{name} {{\n    \
                    {empty:indent$}self.serialize{vec_func}(value{as_ref}, {boxed}{vec})?;\n\
                {empty:indent$}}}\n",
                empty = "",
                indent = indent,
                vec_func = if arg.type_.vec { "_vec" } else { "" },
                obj = if do_obj { "obj." } else { "" },
                name = &arg.name,
                as_ref = if as_ref { as_ref_func } else { "" },
                boxed = arg.type_.boxed,
                vec = if arg.type_.vec { format!(", {}", arg.type_.vec_boxed) } else { String::new() },
            );
        } else {
            writeln!(
                out,
                "{empty:indent$}self.serialize{vec_func}({reff}{obj}{name}{as_ref}, {boxed}{vec})?;",
                empty = "",
                indent = indent,
                vec_func = if arg.type_.vec { "_vec" } else { "" },
                reff = if !as_ref { "&" } else { "" },
                obj = if do_obj { "obj." } else { "" },
                name = &arg.name,
                as_ref = if as_ref { as_ref_func } else { "" },
                boxed = arg.type_.boxed,
                vec = if arg.type_.vec { format!(", {}", arg.type_.vec_boxed) } else { String::new() },
            );
        }
    }

    out
}

fn process_tl_scheme() -> (Vec<TlCombinator>, HashMap<String, TlCombinator>) {
    let mut tl_scheme_file = File::open("./scheme.tl").expect("Could not open scheme file");
    let mut tl_scheme_contents = String::new();

    tl_scheme_file
        .read_to_string(&mut tl_scheme_contents)
        .expect("Could not read scheme file");
    let mut i = 0;

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

fn process_tl_line(line: &str) -> TlCombinator {
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

fn parse_args(argstr: &str) -> Vec<TlArg> {
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

fn process_type(type_str: &str) -> TlType {
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

fn is_boxed(argument_type: &str) -> bool {
    let i = argument_type.find(|c| c == '.' || c == '<').map(|m| m + 1).unwrap_or(0);
    (argument_type.as_bytes()[i] as char).is_uppercase()
}

fn filter_variant(variant: &str, type_name: &str) -> String {
    // Nice job past me, this works really well
    lazy_static! {
        static ref WORD_RE: Regex = Regex::new(r"[A-Z]+[a-z0-9]*")
            .expect("Error compiling variant filter regex");
    }

    let var_matches = WORD_RE
        .find_iter(variant)
        .map(|m| m.as_str())
        .collect::<Vec<&str>>();

    let type_matches = WORD_RE
        .find_iter(type_name)
        .map(|m| m.as_str())
        .collect::<Vec<&str>>();

    let mut v_index = 0;
    let mut t_index = 0;

    let v_len = var_matches.len();
    let t_len = type_matches.len();

    let mut unique = String::new();

    if t_index + 1 < t_len && var_matches[v_index] == type_matches[t_index + 1] {
        t_index += 1;
    }

    loop {
        if var_matches[v_index] == type_matches[t_index] {
            if t_index + 1 < t_len {
                t_index += 1;
            } else {
                while v_index + 1 < v_len {
                    v_index += 1;
                    unique.push_str(var_matches[v_index]);
                }
            }

            if v_index + 1 < v_len {
                v_index += 1;
            } else {
                break;
            }
        } else {
            unique.push_str(var_matches[v_index]);
            if v_index + 1 < v_len {
                v_index += 1;
            } else {
                break;
            }
        }
    }

    let mut res = if unique != "" {
        unique
    } else {
        type_matches[t_len - 1].to_string()
    };

    if res == "Self" {
        res.push('_');
    }

    res
}

fn fix_reserved_name(name: &str) -> String {
    match name {
        "self" | "Self" | "type" | "loop" | "static" | "final" => name.to_owned() + "_",
        _ => name.to_string(),
    }
}

fn is_vec(type_: &str) -> (bool, &str) {
    if type_.len() > 8 && &type_[0..7].to_lowercase() == "vector<" {
        (true, &type_[7..type_.len() - 1])
    } else {
        (false, &type_)
    }
}

fn is_primitive(type_: &str) -> Option<&'static str> {
    match type_ {
        "#" => Some("u32"),
        "int" => Some("i32"),
        "long" => Some("i64"),
        "double" => Some("f64"),
        "int128" => Some("[u8; 16]"),
        "int256" => Some("[u8; 32]"),
        "string" => Some("String"),
        "bytes" => Some("Vec<u8>"),
        _ => None,
    }
}

fn normalize_name(name: &str) -> String {
    lazy_static! {
        static ref WORD_RE: Regex = Regex::new(r"[A-Z]+[a-z0-9]*|[a-z]+[a-z0-9]*")
            .expect("Error compiling variant filter regex");
    }

    WORD_RE
        .find_iter(name)
        .map(|s| uppercase_first(s.as_str()))
        .collect::<String>()
}

fn uppercase_first(s: &str) -> String {
    s[..1].to_uppercase() + s[1..].to_lowercase().as_str()
}
