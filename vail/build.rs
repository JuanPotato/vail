#[macro_use]
extern crate lazy_static;

extern crate regex;

use regex::Regex;

use std::collections::HashMap;

use std::fs::File;
use std::io::Read;
use std::fmt::Write as FmtWrite;
use std::io::Write as IoWrite;
use std::path::Path;
use std::i64;
use std::env;

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
    // let out_dir = "./src/";
    let dest_path = Path::new(&out_dir).join("tl.rs");
    println!("{:?}", dest_path);
    let mut tl_output = File::create(&dest_path).unwrap();
    let mut tl_scheme_file = File::open("./scheme.tl").unwrap();
    let mut tl_scheme_contents = String::new();

    let _ = tl_scheme_file.read_to_string(&mut tl_scheme_contents);

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

            let item = TlItem {
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

    // println!("{:#?}", type_counts);

    let mut done = Vec::new();

    for ref cons in &constructors {
        if *type_counts.get(&cons.item_type).unwrap() == 1 && cons.item_type == cons.name {
            // It's going to be a struct
            
            let struct_output = write_struct(&cons);
            
            tl_output.write_all(struct_output.as_bytes());
        } else {
            // Its going to be an enum
            if done.contains(&&cons.item_type) {
                continue;
            }

            let enum_output = write_enum(&cons, &constructors);
            tl_output.write_all(enum_output.as_bytes());

            done.push(&cons.item_type);
        }
    }

    done.clear();

    let _ = write!(tl_output,
       "#[derive(Debug)]\n\
        enum TlType {{").unwrap();

    for ref cons in &constructors {
        if done.contains(&&cons.item_type) {
            continue;
        }

        let _ = write!(tl_output,
            "\n    {0}(Box<{0}>),",
            cons.item_type).unwrap();

        done.push(&cons.item_type);
    }

    let _ = write!(tl_output,
        "\n}}").unwrap();

    let _ = tl_output.flush();
}

/*
fn write_func(tl_func: &TlItem) -> String {

}
*/

fn write_enum(tl_enum: &TlItem, constructors: &[TlItem]) -> String {
    let mut output = String::new();

    write!(output,
       "#[derive(Debug, TlType)]\n\
        pub enum {} {{",
        tl_enum.item_type).unwrap();

    for similar_cons in constructors.iter() {
        if similar_cons.item_type != tl_enum.item_type {
            continue;
        }

        let unique = filter_variant(&similar_cons.name, &tl_enum.item_type);

        write!(output,
            "\n    #[tl_id = \"{:x}\"]\
            \n    {}",
            similar_cons.id, filter_arg_name(&unique)).unwrap();

        if let Some(ref args) = similar_cons.args {
            write!(output, " {{").unwrap();

            for arg in args {
                let mut arg_type = tl_type_to_rust(&arg.arg_type);
                let arg_name = filter_arg_name(&arg.name);

                if arg_type == tl_enum.item_type {
                    arg_type = format!("Box<{}>", arg_type);
                }

                if arg.flag_bit != -1 {
                    if arg_type != "bool" {
                        write!(output,
                            "\n        #[flag_bit = \"{}\"]\
                             \n        {}: Option<{}>,",
                            arg.flag_bit,
                            arg_name,
                            arg_type).unwrap();
                    } else {
                        write!(output,
                            "\n        #[flag_bit = \"{}\"]\
                             \n        {}: {},",
                            arg.flag_bit,
                            arg_name,
                            arg_type).unwrap();
                    }
                } else {
                    write!(output, "\n        {}: {},",
                        arg_name,
                        arg_type).unwrap();
                }
            }

            write!(output, "\n    }},\n").unwrap();
        } else {
            write!(output, ",\n").unwrap();
        }
    }

    write!(output, "}}\n\n").unwrap();

    output
}

fn write_struct(tl_struct: &TlItem) -> String {
    let mut output = String::new();
    
    write!(output,
       "#[derive(Debug, TlType)]\n\
        #[tl_id = \"{:x}\"]\n\
        pub struct {}",
        tl_struct.id as u32, tl_struct.name).unwrap();

    if let Some(ref args) = tl_struct.args {
        let _ = write!(output, " {{").unwrap();

        for arg in args {
            let arg_type = tl_type_to_rust(&arg.arg_type);
            if arg.flag_bit != -1 {
                if arg_type != "bool" {
                    write!(output,
                        "\n    #[flag_bit = \"{}\"]\
                         \n    pub {}: Option<{}>,",
                        arg.flag_bit,
                        filter_arg_name(&arg.name),
                        arg_type).unwrap();
                } else {
                    write!(output,
                        "\n    #[flag_bit = \"{}\"]\
                         \n    pub {}: {},",
                        arg.flag_bit,
                        filter_arg_name(&arg.name),
                        arg_type).unwrap();
                }
            } else {
                write!(output, "\n    {}: {},",
                    filter_arg_name(&arg.name),
                    tl_type_to_rust(&arg.arg_type)).unwrap();
            }
        }

        write!(output, "\n}}\n\n").unwrap();
    } else {
        write!(output, ";\n\n").unwrap();
    }

    output
}

fn filter_variant(variant: &str, type_name: &str) -> String {
    lazy_static! {
        static ref WORD_RE: Regex = Regex::new(r"[A-Z]+[a-z0-9]*").unwrap();
    }

    let var_matches = WORD_RE.find_iter(variant)
                              .map(|m| m.as_str())
                              .collect::<Vec<&str>>();

    let type_matches = WORD_RE.find_iter(type_name)
                              .map(|m| m.as_str())
                              .collect::<Vec<&str>>();

    let mut v_index = 0;
    let mut t_index = 0;

    let v_len = var_matches.len();
    let t_len = type_matches.len();

    let mut unique = String::new();

    if t_index + 1 < t_len
        && var_matches[v_index] == type_matches[t_index+1] {
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

    if unique != "" {
        unique
    } else {
        type_matches[t_len-1].to_string()
    }
}


fn filter_arg_name(s: &str) -> String { // This isnt nice
    match s {
        "type" => "ttype",
        "self" => "sself",
        "Self" => "SSelf",
        "final" => "ffinal",
        "loop" => "lloop",
        _ => s,
    }.to_string()
}

fn tl_type_to_rust(s: &str) -> String {
    let mut do_box = false;

    let new_s = match s {
        "#"      => "u32",
        "True" |
        "Bool"   => "bool",
        "String" => "String",
        "Int"  |
        "int"    => "i32",
        "Vector<Int>" |
        "Vector<int>" => "Vec<i32>",
        "Long" |
        "long"   => "i64",
        "Vector<Long>" |
        "Vector<long>" => "Vec<i64>",
        "Float"  => "f32",
        "Double" => "f64",
        "Bytes"  => "Vec<u8>",
        _ => {
            do_box = true;
            s
        },
    }.to_string().replace("Vector", "Vec");

    if do_box && !new_s.contains("Vec") {
        format!("Box<{}>", new_s)
    } else {
        new_s
    }
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
