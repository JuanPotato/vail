#[macro_use]
extern crate lazy_static;

extern crate regex;

use regex::Regex;

use std::collections::HashMap;

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
    flag_bit: i64,
}

#[derive(Debug)]
struct TlItem {
    name: String,
    id: u32,
    item_type: String,
    args: Option<Vec<Arg>>,
}

// Notes

// # is a u32, only used for the flags attribute
// args are flags.1?true which means if the 2nd bit (0 indexed) is 1,
//   then there will be a true object. In this case if the bit is 1
//   then we just assume it is true and go on rather than looking for a true id 
// X, !X, and Type are generic, so anything
// int is i32
// long is i64
// float is f32
// double is f64


fn main() {
    let item_regex = Regex::new(r"^(?P<name>[\w\.]+)#(?P<id>[0-9a-f]+) (?P<args>[\w <>:#?.{}!]*)= (?P<type>[\w<.>]+);").unwrap();

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("tl.rs");
    let mut tl_output = File::create(&dest_path).unwrap();
    let mut tl_scheme_file = File::open("./scheme.tl").unwrap();
    let mut tl_scheme_contents = String::new();

    tl_scheme_file.read_to_string(&mut tl_scheme_contents);

    let mut constructors: Vec<TlItem> = Vec::new();
    let mut functions: Vec<TlItem> = Vec::new();
    let mut type_counts: HashMap<String, i64> = HashMap::new(); // how many types there are of each type

    let mut is_function = false;
    for line in tl_scheme_contents.lines() {
        if line == "---functions---" {
            is_function = true;
            continue;
        }

        if let Some(captures) = item_regex.captures(line) {
            let name = dot_to_camel(captures.name("name").unwrap().as_str());
            let id = u32::from_str_radix(captures.name("id").unwrap().as_str(), 16).unwrap();
            let args = captures.name("args");
            let item_type = dot_to_camel(captures.name("type").unwrap().as_str());

            let mut item = TlItem {
                name: name,
                id: id,
                item_type: item_type.clone(),
                args: parse_args(args),
            };

            if is_function {
                functions.push(item);
            } else {
                constructors.push(item);
                let counter = type_counts.entry(item_type).or_insert(0);
                *counter += 1;
            }
        }
    }

    println!("{:#?}", type_counts);

    // for mut ref cons in constructors {

    // }
}

fn parse_args(capture: Option<regex::Match>) -> Option<Vec<Arg>> {
    lazy_static! {
        static ref ARG_RE: Regex = Regex::new(r"(?P<name>\w+):(?:flags\.(?P<flag_bit>\d+)\?)?(?P<type>[\w<.>#]+) ?").unwrap();
    }
    match capture {
        Some(cap) => {
            let mut args: Vec<Arg> = Vec::new();
            
            for piece in cap.as_str().trim().split_whitespace() {
                if piece.contains('{') {
                    // This arg was something like {X:Type} which we take into account already
                    continue;
                }
                if let Some(capture) = ARG_RE.captures(piece) {
                    let name = capture.name("name").unwrap().as_str().to_string();
                    let arg_type = dot_to_camel(capture.name("type").unwrap().as_str());
                    
                    let flag_bit = if let Some(bit) = capture.name("flag_bit") {
                        bit.as_str().parse::<i64>().unwrap()
                    } else {
                        -1
                    };

                    args.push(Arg {
                        name: name,
                        arg_type: arg_type,
                        flag_bit: flag_bit,
                    });
                }
            }

            if args.len() == 0 {
                None
            } else {
                Some(args)
            }
        },
        None => None,
    }
}

fn upper_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn dot_to_camel(s: &str) -> String {
    s.split('.').map(|a| upper_first(a)).collect::<String>()
}
