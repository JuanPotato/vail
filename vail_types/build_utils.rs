/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use regex::Regex;

#[macro_export]
macro_rules! optional {
    ($var:expr, $default:expr) => (
        if $var { $default } else { "" }
    );

    ($var:expr, $default:expr, $else:expr) => (
        if $var { $default } else { $else }
    )
}

pub fn indent_code(code: &str, indent_level: usize) -> String {
    let mut indented_code = String::new();
    let indent = format!("{:indent$}", "", indent=indent_level);

    for line in code.lines() {
        if line.len() > 0 {
            indented_code.push_str(&indent);
        }

        indented_code.push_str(line);
        indented_code.push('\n');
    }

    return indented_code;
}


// Name fixing
pub fn filter_variant(variant: &str, type_name: &str) -> String {
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

pub fn fix_reserved_name(name: &str) -> String {
    match name {
        "self" | "Self" | "type" | "loop" | "static" | "final" => name.to_owned() + "_",
        _ => name.to_string(),
    }
}

pub fn normalize_name(name: &str) -> String {
    lazy_static! {
        static ref WORD_RE: Regex = Regex::new(r"[A-Z]+[a-z0-9]*|[a-z]+[a-z0-9]*")
            .expect("Error compiling variant filter regex");
    }

    WORD_RE
        .find_iter(name)
        .map(|s| uppercase_first(s.as_str()))
        .collect::<String>()
}

pub fn uppercase_first(s: &str) -> String {
    s[..1].to_uppercase() + s[1..].to_lowercase().as_str()
}


// Is things
pub fn is_boxed(argument_type: &str) -> bool {
    let i = argument_type.find(|c| c == '.' || c == '<').map(|m| m + 1).unwrap_or(0);
    (argument_type.as_bytes()[i] as char).is_uppercase()
}


pub fn is_vec(type_: &str) -> (bool, &str) {
    if type_.len() > 8 && &type_[0..7].to_lowercase() == "vector<" {
        (true, &type_[7..type_.len() - 1])
    } else {
        (false, &type_)
    }
}

pub fn is_primitive(type_: &str) -> Option<&'static str> {
    match type_ {
        "#" => Some("u32"),
        "int" => Some("i32"),
        "long" => Some("i64"),
        "double" => Some("f64"),
        "int128" => Some("i128"),
        "int256" => Some("[u8; 32]"),
        "string" => Some("String"),
        "bytes" => Some("Vec<u8>"),
        _ => None,
    }
}

