/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::fmt::Write;
use std::collections::HashMap;
use build_utils::{indent_code, filter_variant};

use TlArg;
use TlCombinator;


pub fn write_struct(cons: &TlCombinator) -> String {
    if cons.args.len() != 0 {
        format!(
"#[derive(Debug, Clone, PartialEq)]\n\
pub struct {name} {{\n\
    {args}\
}}\n\n",
name = &cons.name,
args = write_args(&cons.args, 4, true)
)
        } else {
            format!(
"#[derive(Debug, Clone, PartialEq)]\n\
    pub struct {name};\n\n",
    name = &cons.name
    )
        }
}

pub fn write_enum(
    group: &[&str],
    constructors: &HashMap<String, TlCombinator>,
    type_name: &str,
    ) -> String {
    let mut out = format!(
        "#[derive(Debug, Clone, PartialEq)]\n\
         pub enum {name} {{",
         name = type_name
         );

    for variant in group {
        let cons = constructors.get(*variant).unwrap();
        let variant_name = filter_variant(&cons.name, &cons.type_.name);

        if cons.args.len() != 0 {
            write!(
                out,
                "\n    {name} {{\n\
                {args}    \
                 }},\n",
                 name = &variant_name,
                 args = write_args(&cons.args, 8, false)
                 ).unwrap();
        } else {
            writeln!(out, "\n    {name},", name = &variant_name).unwrap();
        }
    }

    write!(out, "}}\n\n").unwrap();
    out
}

pub fn write_args(args: &[TlArg], indent: usize, do_pub: bool) -> String {
    let mut out = String::new();

    for arg in args {
        let mut adjusted_type = arg.type_.name.clone();

        if !arg.type_.boxed && !arg.type_.primitive {
            adjusted_type = format!("Bare<{}>", adjusted_type);
        }

        if arg.type_.vec {
            adjusted_type = format!("Vec<{}>", adjusted_type);

            if !arg.type_.vec_boxed {
                adjusted_type = format!("Bare<{}>", adjusted_type);
            }
        }


        if !arg.type_.primitive && !arg.type_.vec {
            adjusted_type = format!("Box<{}>", adjusted_type);
        }

        if arg.bit.is_some() {
            adjusted_type = format!("Option<{}>", adjusted_type);
        }

        writeln!(
            out,
            "{pu}{name}: {typ},",
            pu = if do_pub { "pub " } else { "" },
            name = &arg.name,
            typ = adjusted_type,
            ).unwrap();
    }

    indent_code(&out, indent)
}


