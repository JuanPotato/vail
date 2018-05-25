/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use TlArg;
use TlCombinator;

use build_utils::{indent_code, filter_variant};
use std::fmt::Write;

use std::collections::HashMap;

pub fn ser_struct(cons: &TlCombinator) -> String {
    format!(
"impl Serializable for {name} {{
    #[allow(unused_mut)]
    fn serialize_into<B: Write>(&self, buf: &mut B, boxed: bool) -> Result<()> {{
        if boxed {{
            buf.serialize(&0x{id:08x}_u32, false)?;
        }}\n\
        {args}
        Ok(())
    }}
 }}\n\n",
        name = &cons.name,
        args = ser_args(&cons.args, 8, true),
        id = cons.id
    )
}

pub fn ser_enum(
    group: &[&str],
    constructors: &HashMap<String, TlCombinator>,
    type_name: &str,
) -> String {
    let mut out = format!(
"impl Serializable for {name} {{
    fn serialize_into<B: Write>(&self, buf: &mut B, boxed: bool) -> Result<()> {{
        if boxed {{
            match self {{",

        name = type_name
    );

    for variant in group {
        let cons = constructors.get(*variant).unwrap();
        let variant_name = filter_variant(&cons.name, &cons.type_.name);

        write!(
            out,
            "\n                \
                {type_name}::{name} {{ .. }} => buf.serialize(&0x{id:08x}_u32, false)?,",
            type_name = type_name,
            name = &variant_name,
            id = cons.id,
        ).unwrap();
    }

    write!(out, "\n            \
            }};\n        \
        }}\n\n        \

        match self {{"
        ).unwrap();


    for variant in group {
        let cons = constructors.get(*variant).unwrap();
        let variant_name = filter_variant(&cons.name, &cons.type_.name);

        write!(
            out,
            "\n            \
                {type_name}::{name} {{",
            type_name = type_name,
            name = &variant_name,
        ).unwrap();

        for arg in cons.args.iter() {
            if arg.name == "flags" {
                write!(out, "\n                {}: _,", arg.name).unwrap();
            } else { 
                write!(out, "\n                ref {},", arg.name).unwrap();
            }
        }

        write!(out, "\n            }} => {{\n\
               {args}            \
            }},\n",
            args = ser_args(&cons.args, 16, false)).unwrap();

    }


    writeln!(out, "        }}\n\n        \
             Ok(())\n    \
         }}\n}}\n").unwrap();
    out
}

pub fn ser_args(args: &[TlArg], indent: usize, do_obj: bool) -> String {
    let mut out = String::new();

    for arg in args {
        if arg.name == "flags" {
            write!(out, "{}", ser_flags_var(args, 0, do_obj)).unwrap();
            continue;
        }

        let mut as_ref = !arg.type_.primitive && !arg.type_.vec;

        let as_ref_func = match arg.type_.name.as_ref() /* heh */ {
            "String" | "string" => ".as_bytes()",
            "Vec<u8>" => ".as_slice()",
            _ => ".as_ref()"
        };

        let vec_boxed = format!(", {}", arg.type_.vec_boxed);

        if arg.bit.is_some() {
            writeln!(out, "\nif let Some(ref value) = {obj}{name} {{",
                obj = optional!(do_obj, "self."),
                name = &arg.name,
            ).unwrap();

            writeln!(out, "    buf.serialize{vec_func}(value{as_ref}, {boxed}{vec})?;",
                vec_func = optional!(arg.type_.vec, "_vec"),
                as_ref = optional!(as_ref, as_ref_func),
                boxed = arg.type_.boxed,
                vec = optional!(arg.type_.vec, &vec_boxed),
            ).unwrap();

            writeln!(out, "}}\n").unwrap();
        } else {
            write!(
                out,
                "buf.serialize{vec_func}({reff}{obj}{name}{as_ref}, {boxed}{vec})?;\n",
                vec_func = optional!(arg.type_.vec, "_vec"),
                reff = optional!(!as_ref && do_obj, "&"),
                obj = optional!(do_obj, "self."),
                name = &arg.name,
                as_ref = optional!(as_ref, as_ref_func),
                boxed = arg.type_.boxed,
                vec = optional!(arg.type_.vec, &vec_boxed),
            ).unwrap();
        }
    }

    indent_code(&out, indent)
}

pub fn ser_flags_var(args: &[TlArg], indent: usize, do_obj: bool) -> String {
    let mut out = String::new();

    writeln!(out, "let mut ser_flags: u32 = 0;\n").unwrap();

    for arg in args {
        if let Some(bit) = arg.bit {
            writeln!(out, "if {obj}{name}.is_some() {{", name = arg.name, obj = optional!(do_obj, "self.")).unwrap();
            writeln!(out, "    ser_flags |= 1 << {bit};", bit = bit).unwrap();
            writeln!(out, "}}\n"                                     ).unwrap();
        }
    }

    writeln!(out, "buf.serialize(&ser_flags, false)?;").unwrap();
    
    indent_code(&out, indent)
}

