/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use TlArg;
use TlCombinator;

use build_utils::{indent_code, filter_variant};
use std::fmt::Write;

use std::collections::HashMap;

pub fn des_struct(cons: &TlCombinator) -> String {
    let mut gen_self = String::new();

    writeln!(gen_self, "Ok({} {{", &cons.name).unwrap();

    for arg in &cons.args {
        writeln!(gen_self, "    {},", &arg.name).unwrap();
    }

    writeln!(gen_self, "}})").unwrap();

    format!( 
"impl Deserializable for {name} {{
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {{
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x{id:08x});

        Self::deserialize_bare(buf, id)
    }}

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<{name}> {{
        {args}\n\
        {gen_self}    \
    }}
}}\n\n",
        name = &cons.name,
        args = des_args(&cons.args, 8),
        id = cons.id,
        gen_self = indent_code(&gen_self, 8),
    )
}

pub fn des_enum(
    group: &[&str],
    constructors: &HashMap<String, TlCombinator>,
    type_name: &str,
) -> String {
    let mut out = format!(
"impl Deserializable for {name} {{\n    \
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {{
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }}

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<{name}> {{\n        \
        match id {{",

        name = type_name
    );

    for variant in group {
        let cons = constructors.get(*variant).unwrap();
        let variant_name = filter_variant(&cons.name, &cons.type_.name);

        let mut gen_self = String::new();

        writeln!(gen_self, "Ok({}::{} {{", type_name, &variant_name).unwrap();

        for arg in &cons.args {
            writeln!(gen_self, "    {},", &arg.name).unwrap();
        }

        writeln!(gen_self, "}})").unwrap();

        write!(
            out,
            "\n            \
                0x{id:08x}_u32 => {{\n\
               {args}            \n\
               {gen_self}            \
            }},\n",
            id = cons.id,
            args = des_args(&cons.args, 16),
            gen_self = indent_code(&gen_self, 16),
        ).unwrap();
    }

    write!(
        out,"
            _ => {{
                panic!(\"ID not recognized lmao\");
            }},\n",
    ).unwrap();


    writeln!(out,
"        }}
    }}
}}\n").unwrap();
    out
}

pub fn des_args(args: &[TlArg], indent: usize) -> String {
    let mut out = String::new();

    for arg in args {
        let add_boxing = !arg.type_.primitive && !arg.type_.vec;

        if let Some(bit) = arg.bit {
            writeln!(out, "\nlet {name} = if flags & 1 << {bit} != 0 {{",
                name = &arg.name,
                bit = bit,
            ).unwrap();


            out.push_str("    Some(");

            if add_boxing {
                out.push_str("Box::new(");
            }

            out.push_str("buf.deserialize()?");

            out.push(')');

            if add_boxing {
                out.push_str(")\n");
            }

            writeln!(out, "}} else {{\n    None\n}};").unwrap();
        } else {
            write!(
                out,
                "let {name} = ",
                name = &arg.name,
            ).unwrap();

            if add_boxing {
                out.push_str("Box::new(");
            }

            out.push_str("buf.deserialize()?");

            if add_boxing {
                out.push(')');
            }

            out.push_str(";\n");
        }
    }

    indent_code(&out, indent)
}

