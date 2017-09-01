/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

extern crate quote;
extern crate proc_macro;
extern crate syn;

use proc_macro::TokenStream;
use quote::ToTokens;
use std::fmt::Write;

#[derive(Debug)]
struct RustField {
    name: String,
    type_: String,
    flag: i64,
}

#[derive(Debug)]
enum RustType {
    Struct {
        name: String,
        tl_id: u32,
        args: Vec<RustField>,
    },

    Enum {
        name: String,
        variants: Vec<RustVariant>,
    },
}

#[derive(Debug)]
struct RustVariant {
    name: String,
    tl_id: u32,
    args: Vec<RustField>,
}

impl From<syn::Field> for RustField {
    fn from(f: syn::Field) -> RustField {
        RustField::from(&f)
    }
}

impl<'a> From<&'a syn::Field> for RustField {
    fn from(f: &'a syn::Field) -> RustField {
        let name = if let Some(ref i) = f.ident {
            i.as_ref().to_string()
        } else {
            String::default()
        };

        let mut tokens = quote::Tokens::new();

        let type_ = match f.ty {
            syn::Ty::Path(_, ref path) => {
                path.to_tokens(&mut tokens);

                tokens.into_string()
            }
            _ => "This shouldn't happen, and I'm sorry.".to_string(),
        };

        let flag_bit = get_attr(&f.attrs, "flag_bit");

        RustField {
            name: name.clone(),
            type_: type_,
            flag: flag_bit.parse::<i64>().unwrap_or(0),
        }
    }
}

impl From<syn::MacroInput> for RustType {
    fn from(input: syn::MacroInput) -> RustType {
        RustType::from(&input)
    }
}

impl<'a> From<&'a syn::MacroInput> for RustType {
    fn from(input: &'a syn::MacroInput) -> RustType {
        let name = input.ident.as_ref().to_string();

        match input.body {
            syn::Body::Enum(ref variants) => {
                let m_variants = variants
                    .into_iter()
                    .map(|v| v.into())
                    .collect::<Vec<RustVariant>>();

                RustType::Enum {
                    name: name,
                    variants: m_variants,
                }
            }
            syn::Body::Struct(ref v_data) => {
                let args = match v_data {
                    &syn::VariantData::Struct(ref fields) => {
                        fields
                            .into_iter()
                            .map(|f| f.into())
                            .collect::<Vec<RustField>>()
                    }
                    &syn::VariantData::Tuple(ref fields) => {
                        fields
                            .into_iter()
                            .map(|f| f.into())
                            .collect::<Vec<RustField>>()
                    }
                    &syn::VariantData::Unit => Vec::default(),
                };

                let tl_id = get_attr(&input.attrs, "tl_id");

                RustType::Struct {
                    name: name.clone(),
                    tl_id: u32::from_str_radix(&tl_id, 16)
                        .expect(&format!("Could not parse tl_id ({}) of {}", &tl_id, &name)),
                    args: args,
                }
            }
        }
    }
}

impl From<syn::Variant> for RustVariant {
    fn from(v: syn::Variant) -> RustVariant {
        RustVariant::from(&v)
    }
}

impl<'a> From<&'a syn::Variant> for RustVariant {
    fn from(v: &'a syn::Variant) -> RustVariant {
        let name = v.ident.as_ref().to_string();

        let args = match &v.data {
            &syn::VariantData::Struct(ref fields) => {
                fields
                    .into_iter()
                    .map(|f| f.into())
                    .collect::<Vec<RustField>>()
            }
            &syn::VariantData::Tuple(ref fields) => {
                fields
                    .into_iter()
                    .map(|f| f.into())
                    .collect::<Vec<RustField>>()
            }
            &syn::VariantData::Unit => Vec::default(),
        };

        let tl_id = get_attr(&v.attrs, "tl_id");

        RustVariant {
            name: name.clone(),
            tl_id: u32::from_str_radix(&tl_id, 16)
                .expect(&format!("Could not parse tl_id ({}) of {}", &tl_id, &name)),
            args: args,
        }
    }
}

#[proc_macro_derive(ToTlType)]
pub fn to_tltype(input: TokenStream) -> TokenStream {
    let s = input.to_string();

    let ast = syn::parse_macro_input(&s).unwrap();
    let simp = RustType::from(ast);

    let gen = impl_to_tltype(&simp);

    gen.parse().unwrap()
}

#[proc_macro_derive(ToTlFunc)]
pub fn to_tlfunc(input: TokenStream) -> TokenStream {
    let s = input.to_string();

    let ast = syn::parse_macro_input(&s).unwrap();
    let simp = RustType::from(ast);

    let gen = impl_to_tlfunc(&simp);

    gen.parse().unwrap()
}

#[proc_macro_derive(TlSer, attributes(tl_id, flag_bit))]
pub fn tl_ser(input: TokenStream) -> TokenStream {
    let s = input.to_string();

    let ast = syn::parse_macro_input(&s).unwrap();
    let simp = RustType::from(ast);

    let gen = impl_ser(&simp);

    gen.parse().unwrap()
}

#[proc_macro_derive(TlDes, attributes(tl_id, flag_bit))]
pub fn tl_des(input: TokenStream) -> TokenStream {
    let s = input.to_string();

    let ast = syn::parse_macro_input(&s).unwrap();
    let simp = RustType::from(ast);

    let gen = impl_des(&simp);

    gen.parse().unwrap()
}

// For functions we need to provide a set of builder functions.
// ^ will be using derive_builder 0.4
//
// ```
// let a = GetMessages::new().something("yeah").build();
// mtproto.send(a);
// ```
// fn send<T>(obj: &T) where T can Serialize
//

fn get_attr(attrs: &[syn::Attribute], key: &str) -> String {
    let mut val = String::new();

    for attr in attrs {
        if let syn::MetaItem::NameValue(ref ident, ref lit) = attr.value {
            if ident.as_ref() == key {
                if let &syn::Lit::Str(ref s, _) = lit {
                    val = s.clone();
                }
            }
        }
    }

    val
}

fn impl_to_tltype(t: &RustType) -> String {
    format!(
        r#"
impl From<{name}> for TlType {{
    fn from(x: {name}) -> Self {{
        TlType::{name}(x.into())
    }}
}}"#,
        name = match *t {
            RustType::Struct { ref name, .. } => name,
            RustType::Enum { ref name, .. } => name,
        }
    )
}

fn impl_to_tlfunc(t: &RustType) -> String {
    format!(
        r#"
impl From<{name}> for TlFunc {{
    fn from(x: {name}) -> Self {{
        TlFunc::{name}(x.into())
    }}
}}"#,
        name = match *t {
            RustType::Struct { ref name, .. } => name,
            RustType::Enum { ref name, .. } => name,
        }
    )
}

fn impl_ser(t: &RustType) -> String {
    match *t {
        RustType::Struct {
            ref name,
            tl_id,
            ref args,
        } => {
            // Begin Serialize
            let mut buf = if args.len() > 0 {
                format!(
                    r#"
impl Serialize<{name}> for Cursor<Vec<u8>> {{
    fn serialize(&mut self, obj: &{name}) -> Result<(), io::Error> {{
        <Self as Serialize<u32>>::serialize(self, &{tl_id}u32)?;"#,
                    name = name,
                    tl_id = tl_id
                )
            } else {
                format!(
                    r#"
impl Serialize<{name}> for Cursor<Vec<u8>> {{
    fn serialize(&mut self, _: &{name}) -> Result<(), io::Error> {{
        <Self as Serialize<u32>>::serialize(self, &{tl_id}u32)?;"#,
                    name = name,
                    tl_id = tl_id
                )
            };

            for arg in args {
                if arg.name == "flags" {
                    buf.push_str(
                        r#"
        <Self as Serialize<u32>>::serialize(self, &(0"#,
                    );

                    for arg in args {
                        if arg.flag < 0 {
                            write!(
                                buf,
                                r#" | if obj.{name} {{ 1 }} else {{ 0 }} << {flag}"#,
                                name = arg.name,
                                flag = arg.flag.abs() - 1
                            ).unwrap();
                        } else if arg.flag > 0 {
                            write!(
                                buf,
                                r#" | if obj.{name}.is_some() {{ 1 }} else {{ 0 }} << {flag}"#,
                                name = arg.name,
                                flag = arg.flag - 1
                            ).unwrap();
                        }
                    }

                    buf.push_str(r#" as u32))?;"#);
                    continue;
                }

                if arg.flag > 0 {
                    write!(
                        buf,
                        r#"
        if let Some(ref opt) = obj.{name} {{
            <Self as Serialize<{type_}>>::serialize(self, opt)?;
        }}"#,
                        type_ = &arg.type_[9..arg.type_.len() - 2],
                        name = arg.name
                    ).unwrap();
                } else if arg.flag == 0 {
                    write!(
                        buf,
                        r#"
        <Self as Serialize<{type_}>>::serialize(self, &obj.{name})?;"#,
                        type_ = arg.type_,
                        name = arg.name
                    ).unwrap();
                }
            }

            buf.push_str(
                r#"
        Ok(())
    }
}"#,
            );
            // End Serialize

            buf
        }

        RustType::Enum {
            ref variants,
            ref name,
        } => {
            // Begin Serialize
            let mut buf = format!(
                r#"
impl Serialize<{name}> for Cursor<Vec<u8>> {{
    fn serialize(&mut self, obj: &{name}) -> Result<(), io::Error> {{
        match obj {{"#,
                name = name
            );

            for variant in variants {
                write!(
                    buf,
                    r#"
            &{}::{}"#,
                    name,
                    variant.name
                ).unwrap();

                if variant.args.len() > 0 {
                    buf.push_str(r#" {"#);

                    let mut add_dotdot = false;

                    for arg in &variant.args {
                        if arg.type_ != "bool" {
                            if arg.name == "flags" {
                                add_dotdot = true;
                            } else {
                                write!(buf, r#" ref {}, "#, arg.name).unwrap();
                            }
                        } else {
                            write!(buf, r#" {}, "#, arg.name).unwrap();
                        }
                    }

                    if add_dotdot {
                        buf.push_str(r#".. } "#);
                    } else {
                        buf.push_str(r#"} "#);
                    }
                }

                write!(
                    buf,
                    r#" => {{
                <Self as Serialize<u32>>::serialize(self, &{tl_id}u32)?;"#,
                    tl_id = variant.tl_id
                ).unwrap();

                for arg in &variant.args {
                    if arg.name == "flags" {
                        buf.push_str(
                            r#"
                <Self as Serialize<u32>>::serialize(self, &(0"#,
                        );

                        for arg in &variant.args {
                            if arg.flag < 0 {
                                write!(
                                    buf,
                                    r#" | if {name} {{ 1 }} else {{ 0 }} << {flag}"#,
                                    name = arg.name,
                                    flag = arg.flag.abs() - 1
                                ).unwrap();
                            } else if arg.flag > 0 {
                                write!(
                                    buf,
                                    r#" | if {name}.is_some() {{ 1 }} else {{ 0 }} << {flag}"#,
                                    name = arg.name,
                                    flag = arg.flag - 1
                                ).unwrap();
                            }
                        }

                        buf.push_str(r#" as u32))?;"#);
                        continue;
                    }

                    if arg.flag > 0 {
                        write!(
                            buf,
                            r#"
                if let &Some(ref opt) = {name} {{
                    <Self as Serialize<{type_}>>::serialize(self, opt)?;
                }}"#,
                            type_ = &arg.type_[9..arg.type_.len() - 2],
                            name = arg.name
                        ).unwrap();
                    } else if arg.flag == 0 {
                        write!(
                            buf,
                            r#"
                <Self as Serialize<{type_}>>::serialize(self, &{name})?;"#,
                            type_ = arg.type_,
                            name = arg.name
                        ).unwrap();
                    }
                }

                buf.push_str(
                    r#"
                Ok(())
            }"#,
                )
            }

            buf.push_str(
                r#"
        }
    }
}"#,
            );
            // End Serialize

            buf
        }
    }
}

fn impl_des(t: &RustType) -> String {
    match *t {
        RustType::Struct {
            ref name,
            tl_id,
            ref args,
        } => {
            // Begin Deserialize
            let mut buf = format!(
                r#"
impl Deserialize<{name}> for Cursor<Vec<u8>> {{
    fn _deserialize(&mut self, received_tl_id: u32) -> Result<{name}, io::Error> {{
        let received_tl_id = if received_tl_id == 0 {{
            self.deserialize::<u32>(0)?
        }} else {{
            received_tl_id
        }};
        if received_tl_id != {tl_id} {{
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!(
                    "Incorrect tl_id for {name}. Expected {tl_id}, received {{}}",
                    received_tl_id
                )
            ));
        }}"#,
                name = name,
                tl_id = tl_id
            );

            for arg in args {
                let mut is_box = false;
                let mut fil_type = arg.type_.as_str();

                if fil_type.len() > 9 && &fil_type[0..9] == "Option < " {
                    fil_type = &fil_type[9..fil_type.len() - 2];
                }

                if fil_type.len() > 6 && &fil_type[0..6] == "Box < " {
                    fil_type = &fil_type[6..fil_type.len() - 2];
                    is_box = true;
                }

                if arg.flag > 0 {
                    write!(
                        buf,
                        r#"
        let {name} = if flags & (1 << {flag}) != 0 {{
            Some({opt_box1}self.deserialize::<{type_}>(0)?{opt_box2})
        }} else {{
            None
        }};"#,
                        type_ = fil_type,
                        name = arg.name,
                        flag = arg.flag - 1,
                        opt_box1 = if is_box { "Box::new(" } else { "" },
                        opt_box2 = if is_box { ")" } else { "" }
                    ).unwrap();
                } else if arg.flag < 0 {
                    write!(
                        buf,
                        r#"
        let {name} = flags & (1 << {flag}) != 0;"#,
                        name = arg.name,
                        flag = arg.flag.abs() - 1
                    ).unwrap();
                } else {
                    write!(
                        buf,
                        r#"
        let {name} = self.deserialize::<{type_}>(0)?{opt_box};"#,
                        type_ = fil_type,
                        name = arg.name,
                        opt_box = if is_box { ".into()" } else { "" }
                    ).unwrap();
                }
            }

            write!(
                buf,
                r#"
        Ok({name} {{"#,
                name = name
            ).unwrap();

            for arg in args {
                write!(
                    buf,
                    r#"
            {name}: {name},"#,
                    name = arg.name
                ).unwrap();
            }

            buf.push_str(
                r#"
        })
    }
}"#,
            );
            // End Deserialize

            buf
        }

        RustType::Enum {
            ref variants,
            ref name,
        } => {
            // Begin Deserialize
            let mut buf = format!(
                r#"
impl Deserialize<{name}> for Cursor<Vec<u8>> {{
    fn _deserialize(&mut self, received_tl_id: u32) -> Result<{name}, io::Error> {{
        let received_tl_id = if received_tl_id == 0 {{
            self.deserialize::<u32>(0)?
        }} else {{
            received_tl_id
        }};
        match received_tl_id {{"#,
                name = name
            );

            for variant in variants {
                write!(
                    buf,
                    r#"
            {0}u32 => {{"#,
                    variant.tl_id
                ).unwrap();

                for arg in &variant.args {
                    let mut is_box = false;
                    let mut fil_type = arg.type_.as_str();

                    if fil_type.len() > 9 && &fil_type[0..9] == "Option < " {
                        fil_type = &fil_type[9..fil_type.len() - 2];
                    }

                    if fil_type.len() > 6 && &fil_type[0..6] == "Box < " {
                        fil_type = &fil_type[6..fil_type.len() - 2];
                        is_box = true;
                    }

                    // Check if primitive and then set whether we need to read the id to deserialize
                    if arg.flag > 0 {
                        write!(
                            buf,
                            r#"
                let {name} = if flags & (1 << {flag}) != 0 {{
                    Some({opt_box1}self.deserialize::<{type_}>(0)?{opt_box2})
                }} else {{
                    None
                }};"#,
                            type_ = fil_type,
                            name = arg.name,
                            flag = arg.flag - 1,
                            opt_box1 = if is_box { "Box::new(" } else { "" },
                            opt_box2 = if is_box { ")" } else { "" }
                        ).unwrap();
                    } else if arg.flag < 0 {
                        write!(
                            buf,
                            r#"
                let {name} = flags & (1 << {flag}) != 0;"#,
                            name = arg.name,
                            flag = arg.flag.abs() - 1
                        ).unwrap();
                    } else {
                        write!(
                            buf,
                            r#"
                let {name} = self.deserialize::<{type_}>(0)?{opt_box};"#,
                            type_ = fil_type,
                            name = arg.name,
                            opt_box = if is_box { ".into()" } else { "" }
                        ).unwrap();
                    }
                }


                write!(
                    buf,
                    r#"
                Ok({}::{} {{"#,
                    name,
                    variant.name
                ).unwrap();

                for arg in &variant.args {
                    write!(
                        buf,
                        r#"
                    {name}: {name},"#,
                        name = arg.name
                    ).unwrap();
                }

                buf.push_str(
                    r#"
                })
            }"#,
                )
            }

            // buf.push_str(r#"
            // _ => {
            // Err(io::Error::new(
            // io::ErrorKind::NotFound,
            // "You gave me an id for a Type, that id was not for that type. Oh no"
            // ))
            // }"#);
            write!(
                buf,
                r#"
            _ => {{
                Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("No variant of {name} found with tl_id of {{}}", received_tl_id)
                ))
            }}"#,
                name = name
            ).unwrap();

            buf.push_str(
                r#"
        }
    }
}"#,
            );
            // End Deserialize

            buf
        }
    }
}
