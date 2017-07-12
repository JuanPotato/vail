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

// Ok im sorry, I had to somehow make a distinction between a flag bit that was
// for a bool that you had to read, (Bool) or a bool that was determined by the
// bit itself, if 1 then true (True). So my solution was to make all the normal
// bit flags to be one more than what they were. 0 -> 1, etc. But the exception
// being the (True) bool would be a negative bit flag to indicate that it is to
// be treated differently than a bool with a positive bit flag. Since -1 cannot
// be used to represent being mandatory, 0 is used instead. Yeah, its not great

// Any awkward wording above is because i tried to make all lines only 80 chars

/*
When you write a really long comment and make it your goal to make each line be
exactly 80 characters including the note afterwards pointing out that any weird
wording in the comment was due to you trying to make each line be 80 characters

Oh no, someone help me I have gone too meta and I need help to escape. pls help
*/

fn main() {
    let item_regex = Regex::new(
        r"^(?P<name>[\w\.]+)#(?P<id>[0-9a-f]+) (?P<args>[\w <>:#?.{}!]*)= (?P<type>[\w<.>]+);",
    ).expect("Error while compiling item regex");

    let out_dir = env::var("OUT_DIR").expect("Could not find OUT_DIR environment variable");
    // let out_dir = "./src/";
    let cons_out_path = Path::new(&out_dir).join("constructors.rs");
    let func_out_path = Path::new(&out_dir).join("functions.rs");

    let mut cons_out = File::create(&cons_out_path).expect("Could not create constructors file");
    let mut func_out = File::create(&func_out_path).expect("Could not create functions file");

    let mut tl_scheme_file = File::open("./scheme.tl").expect("Could not open scheme file");
    let mut tl_scheme_contents = String::new();

    tl_scheme_file
        .read_to_string(&mut tl_scheme_contents)
        .expect("Could not read scheme file");

    let mut constructors: Vec<TlItem> = Vec::new();
    let mut functions: Vec<TlItem> = Vec::new();
    // how many types there are of each type
    let mut type_counts: HashMap<String, i64> = HashMap::new();

    let mut is_function = false;
    for line in tl_scheme_contents.lines() {
        match line {
            "---functions---" => {
                is_function = true;
                continue;
            }

            "---types---" => {
                is_function = false;
                continue;
            }

            _ => {}
        }

        if let Some(captures) = item_regex.captures(line) {
            let mut name = dot_to_camel(
                captures
                    .name("name")
                    .expect("Could not get capture `name`")
                    .as_str(),
            );
            name = snake_to_camel(name.as_str());

            let id = u32::from_str_radix(
                captures
                    .name("id")
                    .expect("Could not get capture `id`")
                    .as_str(),
                16,
            ).expect("Could not parse tl_id as u32");
            let args = captures.name("args");

            let mut item_type = dot_to_camel(
                captures
                    .name("type")
                    .expect("Could not get capture `type`")
                    .as_str(),
            );
            item_type = snake_to_camel(item_type.as_str());


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

    // Write the constructors
    let mut done = Vec::new();

    write!(cons_out, "// Constructors").unwrap();
    for ref cons in &constructors {
        if *type_counts.get(&cons.item_type).unwrap() == 1 && cons.item_type == cons.name {
            // It's going to be a struct

            let struct_output = write_struct(&cons, false);

            cons_out.write_all(struct_output.as_bytes()).unwrap();
        } else {
            // Its going to be an enum
            if done.contains(&&cons.item_type) {
                continue;
            }

            let enum_output = write_enum(&cons, &constructors);
            cons_out.write_all(enum_output.as_bytes()).unwrap();

            done.push(&cons.item_type);
        }
    }

    done.clear();

    // Write the TlType enum
    write!(
        cons_out,
        r#"

#[derive(Debug)]
pub enum TlType {{"#
    ).expect("Could not write to constructor file");

    for ref cons in &constructors {
        if done.contains(&&cons.item_type) {
            continue;
        }

        write!(
            cons_out,
            r#"
    {0}(Box<{0}>),"#,
            cons.item_type
        ).expect("Could not write to constructor file");

        done.push(&cons.item_type);
    }

    write!(
        cons_out,
        r#"
}}"#
    ).expect("Could not write to constructor file");

    done.clear();


    // Implement Serialize and Deserialize for TlType that just points to the
    // variant's function

    // Serialize for TlType
    write!(
        cons_out,
        r#"

impl Serialize<TlType> for Cursor<Vec<u8>> {{
    fn serialize(&mut self, cons: &TlType) -> Result<(), io::Error> {{
        match *cons {{"#
    ).expect("Could not write to constructors file");

    for ref cons in &constructors {
        if *type_counts.get(&cons.item_type).unwrap() == 1 && cons.item_type == cons.name {
            // It's going to be a struct
            write!(
                cons_out,
                r#"
            TlType::{0}(ref obj) => <Self as Serialize<{0}>>::serialize(self, &obj)?,"#,
                cons.item_type
            ).expect("Could not write to constructors file");
        } else {
            // Its going to be an enum
            if done.contains(&&cons.item_type) {
                continue;
            }

            write!(
                cons_out,
                r#"
            TlType::{0}(ref obj) => <Self as Serialize<{0}>>::serialize(self, &obj)?,"#,
                cons.item_type
            ).expect("Could not write to constructors file");

            done.push(&cons.item_type);
        }
    }

    write!(
        cons_out,
        r#"
        }}

        Ok(())
    }}
}}"#
    ).expect("Could not write to constructors file");

    done.clear();

    // Deserialize for TlType
    write!(
        cons_out,
        r#"

impl Deserialize<TlType> for Cursor<Vec<u8>> {{
    fn _deserialize(&mut self, received_tl_id: u32) -> Result<TlType, io::Error> {{
        let received_tl_id = if received_tl_id == 0 {{
            self.deserialize::<u32>(0)?
        }} else {{
            received_tl_id
        }};
        match received_tl_id {{"#
    ).expect("Could not write to constructors file");

    for ref cons in &constructors {
        if *type_counts.get(&cons.item_type).unwrap() == 1 && cons.item_type == cons.name {
            // It's going to be a struct
            write!(
                cons_out,
                r#"
            0x{0:x} => Ok(self.deserialize::<{1}>(received_tl_id)?.into()),"#,
                cons.id,
                cons.item_type
            ).expect("Could not write to constructors file");
        } else {
            // Its going to be an enum
            if done.contains(&&cons.item_type) {
                continue;
            }

            let mut add_or = false;

            write!(cons_out, "\n            ").unwrap();

            for similar_cons in constructors.iter() {
                if similar_cons.item_type != cons.item_type {
                    continue;
                }

                if add_or {
                    write!(cons_out, r#"| "#).unwrap();
                }

                write!(cons_out, r#"0x{0:x} "#, similar_cons.id)
                    .expect("Could not write to constructors file");
                add_or = true;
            }

            write!(
                cons_out,
                r#"=> Ok(self.deserialize::<{}>(received_tl_id)?.into()),"#,
                cons.item_type
            ).expect("Could not write to constructors file");

            done.push(&cons.item_type);
        }
    }

    write!(
        cons_out,
        r#"

            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Uknown tl_id: {{}}", received_tl_id)
            ))
         }}
    }}
}}
"#
    ).expect("Could not write to constructors file");

    cons_out
        .flush()
        .expect("Error while flushing constructor file");


    write!(cons_out, "\n\n// Functions").unwrap();

    // Write the functions
    for ref func in &functions {
        let output = write_struct(&func, true);
        func_out.write_all(output.as_bytes()).unwrap();
    }

    // Write the TlFunc enum
    write!(
        func_out,
        r#"

#[derive(Debug)]
pub enum TlFunc {{"#
    ).expect("Could not write to functions file");

    for ref func in &functions {
        write!(
            func_out,
            r#"
    {0}(Box<{0}>),"#,
            func.name
        ).expect("Could not write to functions file");
    }

    write!(
        func_out,
        r#"
}}"#
    ).expect("Could not write to functions file");

    // Serialize for TlFunc
    write!(
        func_out,
        r#"

impl Serialize<TlFunc> for Cursor<Vec<u8>> {{
    fn serialize(&mut self, func: &TlFunc) -> Result<(), io::Error> {{
        match *func {{"#
    ).expect("Could not write to functions file");

    for ref func in &functions {
        write!(
            func_out,
            r#"
            TlFunc::{0}(ref obj) => <Self as Serialize<{0}>>::serialize(self, &obj)?,"#,
            func.name
        ).expect("Could not write to functions file");
    }

    write!(
        func_out,
        r#"
        }}

        Ok(())
    }}
}}
"#
    ).expect("Could not write to functions file");

    func_out
        .flush()
        .expect("Error while flushing functions file");
}

fn write_enum(tl_enum: &TlItem, constructors: &[TlItem]) -> String {
    let mut output = String::new();

    write!(
        output,
        r#"

#[derive(Debug, ToTlType, TlSer, TlDes)]
pub enum {} {{"#,
        tl_enum.item_type
    ).unwrap();

    for similar_cons in constructors.iter() {
        if similar_cons.item_type != tl_enum.item_type {
            continue;
        }

        let unique = filter_variant(&similar_cons.name, &tl_enum.item_type);

        write!(
            output,
            r#"
    #[tl_id = "{:x}"]
    {}"#,
            similar_cons.id,
            filter_arg_name(&unique)
        ).unwrap();

        if let Some(ref args) = similar_cons.args {
            write!(output, r#" {{"#).unwrap();

            for arg in args {
                let mut arg_type = tl_type_to_rust(&arg.arg_type);
                let arg_name = filter_arg_name(&arg.name);

                if arg_type == tl_enum.item_type {
                    arg_type = format!("Box<{}>", arg_type);
                }

                if arg.flag_bit < 0 {
                    write!(
                        output,
                        r#"
        #[flag_bit = "{}"]
        {}: {},"#,
                        arg.flag_bit,
                        arg_name,
                        arg_type
                    ).unwrap();
                } else if arg.flag_bit > 0 {
                    write!(
                        output,
                        r#"
        #[flag_bit = "{}"]
        {}: Option<{}>,"#,
                        arg.flag_bit,
                        arg_name,
                        arg_type
                    ).unwrap();
                } else {
                    write!(
                        output,
                        r#"
        {}: {},"#,
                        arg_name,
                        arg_type
                    ).unwrap();
                }
            }

            write!(
                output,
                r#"
    }},"#
            ).unwrap();
        } else {
            write!(output, r#","#).unwrap();
        }
    }

    write!(
        output,
        r#"
}}"#
    ).unwrap();

    output
}

fn write_struct(tl_struct: &TlItem, func: bool) -> String {
    let mut output = String::new();

    if func {
        write!(
            output,
            r#"

#[derive(Debug, ToTlFunc, TlSer)]
#[tl_id = "{:x}"]
pub struct {}"#,
            tl_struct.id as u32,
            tl_struct.name
        ).expect("Error writing struct to string");
    } else {
        write!(
            output,
            r#"

#[derive(Debug, ToTlType, TlSer, TlDes)]
#[tl_id = "{:x}"]
pub struct {}"#,
            tl_struct.id as u32,
            tl_struct.name
        ).expect("Error writing struct to string");
    }

    if let Some(ref args) = tl_struct.args {
        write!(output, r#" {{"#).expect("Error writing struct to string");

        for arg in args {
            let arg_type = tl_type_to_rust(&arg.arg_type);

            if arg.flag_bit < 0 {
                write!(
                    output,
                    r#"
    #[flag_bit = "{}"]
    pub {}: {},"#,
                    arg.flag_bit,
                    filter_arg_name(&arg.name),
                    arg_type
                ).expect("Error writing struct to string");
            } else if arg.flag_bit > 0 {
                write!(
                    output,
                    r#"
    #[flag_bit = "{}"]
    pub {}: Option<{}>,"#,
                    arg.flag_bit,
                    filter_arg_name(&arg.name),
                    arg_type
                ).expect("Error writing struct to string");
            } else {
                write!(
                    output,
                    r#"
    pub {}: {},"#,
                    filter_arg_name(&arg.name),
                    tl_type_to_rust(&arg.arg_type)
                ).expect("Error writing struct to string");
            }
        }

        write!(
            output,
            r#"
}}"#
        ).expect("Error writing struct to string");
    } else {
        write!(output, r#";"#).expect("Error writing struct to string");
    }

    output
}

fn filter_variant(variant: &str, type_name: &str) -> String {
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

    if unique != "" {
        unique
    } else {
        type_matches[t_len - 1].to_string()
    }
}


fn filter_arg_name(s: &str) -> String {
    // This isnt nice
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
        "#" => "u32",
        "True" | "bool" | "Bool" => s,
        "Vector<string>" | "Vector<String>" => "Vec<String>",
        "string" | "String" => "String",
        "Int" | "int" => "i32",
        "Int128" | "Int256" => s,
        "Vector<Int>" | "Vector<int>" => "Vec<i32>",
        "Long" | "long" => "i64",
        "Vector<Long>" | "Vector<long>" => "Vec<i64>",
        "Float" => "f32",
        "Double" => "f64",
        "Bytes" => "Vec<u8>",
        _ => {
            do_box = true;
            s
        }
    }.to_string()
        .replace("Vector", "Vec");

    if do_box && !new_s.contains("Vec") {
        format!("Box<{}>", new_s)
    } else {
        new_s
    }
}

fn parse_args(capture: Option<regex::Match>) -> Option<Vec<Arg>> {
    lazy_static! {
        static ref ARG_RE: Regex = Regex::new(
            r"(?P<name>[{\w]+):(?:flags\.(?P<flag_bit>\d+)\?)?(?P<type>[\w<.>#!]+)",
        ).expect("Error compiling parse args regex");
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
                    let name = capture
                        .name("name")
                        .expect("Error getting `name` capture")
                        .as_str()
                        .to_string();
                    let mut arg_type = dot_to_camel(
                        capture
                            .name("type")
                            .expect("Error getting `type` capture")
                            .as_str(),
                    );
                    arg_type = snake_to_camel(arg_type.as_str());


                    if arg_type == "!X" {
                        arg_type = String::from("TlFunc")
                    }

                    let mut flag_bit = if let Some(bit) = capture.name("flag_bit") {
                        bit.as_str()
                            .parse::<i64>()
                            .expect("Error parsing bit_flag into an int") +
                            1
                    } else {
                        0
                    };

                    if arg_type == "True" {
                        flag_bit *= -1;
                        arg_type = String::from("bool");
                    }

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
        }
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

fn snake_to_camel(s: &str) -> String {
    let string = s.split('_').map(|a| upper_first(a)).collect::<String>();

    string
        .split('<')
        .map(|a| upper_first(a))
        .collect::<Vec<String>>()
        .join("<")
}
