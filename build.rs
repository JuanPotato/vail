extern crate regex;

use regex::Regex;

use std::env;
use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::path::Path;
use std::i64;

#[derive(Debug)]
struct Arg {
    name: String,
    arg_type: String,
    bit_flag: i64,
}

#[derive(Debug)]
struct TlItem {
    name: String,
    id: i64,
    item_type: String,
    args: Option<Vec<Arg>>,
    function: bool,
}

fn main() {
    let item_regex = Regex::new(r"^(?P<name>\w+)(?:\.(?P<sub>\w+))?#(?P<id>[0-9a-f]+) (?P<args>.*)= (?P<type>[\w<.>]+);").unwrap();
    let args_regex = Regex::new(r"(?P<name>\w+):(?:flags\.(?P<bit_flag>\d+)\?)?(?P<type>[\w<.>#]+) ?").unwrap();

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("tl.rs");
    let mut tl_output = File::create(&dest_path).unwrap();
    let mut tl_scheme_file = File::open("./scheme.tl").unwrap();
    let mut tl_scheme_contents = String::new();

    tl_scheme_file.read_to_string(&mut tl_scheme_contents);

    let constructors: Vec<TlItem> = Vec::new();
    let functions: Vec<TlItem> = Vec::new();

    let mut is_function = false;
    for line in tl_scheme_contents.lines() {
        if line == "---functions---" {
            is_function = true;
            continue;
        }

        if let Some(captures) = item_regex.captures(line) {
            let name = upper_first(captures.name("name").unwrap().as_str());
            let sub = captures.name("sub");
            let id = i64::from_str_radix(captures.name("id").unwrap().as_str(), 16).unwrap();
            let args = captures.name("args");
            let item_type = captures.name("type").unwrap().as_str().to_string();

            let mut item = TlItem {
                name: if sub.is_some() { name + sub.unwrap().as_str() } else { name },
                id: id,
                item_type: item_type,
                args: None,
                function: is_function,
            };
        }
    }
}

fn upper_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(c).collect(),
    }
}
