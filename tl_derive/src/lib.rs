#[macro_use]
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
    }
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
            _ => "This shouldn't happen, and im sorry.".to_string()
        };

        let flag_bit = get_attr(&f.attrs, "flag_bit");

        RustField {
            name: name.clone(),
            type_: type_,
            flag: flag_bit
                .parse::<i64>()
                .unwrap_or(-1),
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
                let m_variants = variants.into_iter().map(|v| v.into()).collect::<Vec<RustVariant>>();

                RustType::Enum {
                    name: name,
                    variants: m_variants,
                }
            }
            syn::Body::Struct(ref v_data) => {
                let args = match v_data {
                    &syn::VariantData::Struct(ref fields) => {
                        fields.into_iter()
                              .map(|f| f.into())
                              .collect::<Vec<RustField>>()
                    }
                    &syn::VariantData::Tuple(ref fields) => {
                        fields.into_iter()
                              .map(|f| f.into())
                              .collect::<Vec<RustField>>()
                    }
                    &syn::VariantData::Unit => {
                        Vec::default()
                    }
                };

                let tl_id = get_attr(&input.attrs, "tl_id");

                RustType::Struct {
                    name: name.clone(),
                    tl_id: u32::from_str_radix(&tl_id, 16)
                        .expect(&format!("Could not parse tl_id ({}) of {}",
                                         &tl_id,
                                         &name)),
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
                fields.into_iter()
                      .map(|f| f.into())
                      .collect::<Vec<RustField>>()
            }
            &syn::VariantData::Tuple(ref fields) => {
                fields.into_iter()
                      .map(|f| f.into())
                      .collect::<Vec<RustField>>()
            }
            &syn::VariantData::Unit => {
                Vec::default()
            }
        };

        let tl_id = get_attr(&v.attrs, "tl_id");

        RustVariant {
            name: name.clone(),
            tl_id: u32::from_str_radix(&tl_id, 16)
                .expect(&format!("Could not parse tl_id ({}) of {}",
                                 &tl_id,
                                 &name)),
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
//
//
// ```
// let a = GetMessages::new().something("yeah").build();
// mtproto.send(a);
// ```
// fn send<T>(obj: &T) where T can Serialize
//
// or 
//
// ```
// let a = GetMessages::new().something("yeah");
// mtproto.send(a)
// ```
// fn send<T>(obj: &T) where T can IntoSerialize
//
//
// or
//
// ```
// GetMessages::new().something("yeah").send(mtproto);
// ```
// trait TlFunction { fn send(mtp: &Mtproto) -> Result<(), Error> }
//

// #[proc_macro_derive(TlFunc, attributes(return_type))]
// pub fn tl_func(input: TokenStream) -> TokenStream {
//     let s = input.to_string();

//     let ast = syn::parse_macro_input(&s).unwrap();
//     let simp = RustType::from(ast);

//     let gen = impl_func(&simp);

//     gen.parse().unwrap()
// }

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
        "impl From<{name}> for TlType {{\n    \
            fn from(x: {name}) -> TlType {{\n        \
                TlType::{name}(x.into())\n    \
            }}\n\
        }}\n",
    name = match *t {
        RustType::Struct { ref name, .. } => name,
        RustType::Enum { ref name, .. } => name
    })
}

// fn impl_func(t: &RustType) -> String {
//     match *t {
//         RustType::Struct { ref name, tl_id, ref args } => {
//             // Begin Serialize
//             // End Serialize

//             buf
//         }

//         RustType::Enum { ref variants, ref name } => { panic!("This should never happen") }
//     }
// }

fn impl_ser(t: &RustType) -> String {
    match *t {
        RustType::Struct { ref name, tl_id, ref args } => {
            // Begin Serialize
            let mut buf = format!(
                "impl Serialize<{name}> for Cursor<Vec<u8>> {{\n    \
                    fn serialize(&mut self, obj: &{name}) -> Result<(), io::Error> {{\n        \
                        <Self as Serialize<u32>>::serialize(self, &{tl_id}u32)?;\n",
                        name = name, tl_id = tl_id);

            for arg in args {
                if arg.name == "flags" {
                    buf.push_str("        <Self as Serialize<u32>>::serialize(self, &(0");

                    for arg in args {
                        if arg.flag == -1 { continue; }
                        if arg.type_ != "bool" {
                            write!(buf, " | if obj.{name}.is_some() {{ 1 }} else {{ 0 }} << {flag}",
                                name = arg.name, flag = arg.flag).unwrap();
                        } else {
                            write!(buf, " | if obj.{name} {{ 1 }} else {{ 0 }} << {flag}",
                                name = arg.name, flag = arg.flag).unwrap();
                        }
                    }

                    buf.push_str(" as u32));");
                    continue;
                }

                if arg.flag != -1 {
                    if arg.type_ != "bool" {
                        write!(buf,
                            "        if let Some(ref opt) = obj.{name} {{\n            \
                                         <Self as Serialize<{type_}>>::serialize(self, opt)?;\n            \
                                     }}\n",
                            type_ = &arg.type_[9..arg.type_.len()-2], name = arg.name).unwrap();
                    }
                    // if it is a boolean type, it isnt going to be serialized
                    // it just depends on the flags
                } else {
                    write!(buf,
                        "        <Self as Serialize<{type_}>>::serialize(self, &obj.{name})?;\n",
                        type_ = arg.type_, name = arg.name).unwrap();
                }
            }

            buf.push_str("Ok(())    }\n}\n\n");
            // End Serialize


            buf
        }

        RustType::Enum { ref variants, ref name } => {
            // Begin Serialize
            let mut buf = format!(
                "impl Serialize<{name}> for Cursor<Vec<u8>> {{\n    \
                    fn serialize(&mut self, obj: &{name}) -> Result<(), io::Error> {{\n        \
                        match obj {{", name = name);

            for variant in variants {
                write!(buf, "\n &{}::{}", name, variant.name).unwrap();

                if variant.args.len() > 0 {
                    buf.push_str(" {");

                    for arg in &variant.args {
                        if arg.type_ != "bool" {
                            write!(buf, " ref {}, ", arg.name).unwrap();
                        } else {
                            write!(buf, " {}, ", arg.name).unwrap();
                        }
                    }

                    buf.push_str("} ");
                }

                write!(buf, " => {{\n
                        <Self as Serialize<u32>>::serialize(self, &{tl_id}u32)?;\n",
                        tl_id = variant.tl_id).unwrap();

                for arg in &variant.args {
                    if arg.name == "flags" {
                        buf.push_str("        <Self as Serialize<u32>>::serialize(self, &(0");

                        for arg in &variant.args {
                            if arg.flag == -1 { continue; }
                            if arg.type_ != "bool" {
                                write!(buf, " | if {name}.is_some() {{ 1 }} else {{ 0 }} << {flag}",
                                    name = arg.name, flag = arg.flag).unwrap();
                            } else {
                                write!(buf, " | if {name} {{ 1 }} else {{ 0 }} << {flag}",
                                    name = arg.name, flag = arg.flag).unwrap();
                            }
                        }

                        buf.push_str(" as u32))?;");
                        continue;
                    }

                    if arg.flag != -1 {
                        if arg.type_ != "bool" {
                            write!(buf,
                                "        if let &Some(ref opt) = {name} {{\n            \
                                             <Self as Serialize<{type_}>>::serialize(self, opt)?;\n            \
                                         }}\n",
                                type_ = &arg.type_[9..arg.type_.len()-2], name = arg.name).unwrap();
                        }
                    } else {
                        write!(buf,
                            "        <Self as Serialize<{type_}>>::serialize(self, &{name})?;\n",
                            type_ = arg.type_, name = arg.name).unwrap();
                    }
                }

                buf.push_str("Ok(()) }\n")
            }

            buf.push_str("        }\n    }\n}\n");
            // End Serialize


            buf
        }
    }
}

fn impl_des(t: &RustType) -> String {
    match *t {
        RustType::Struct { ref name, tl_id, ref args } => {
            // Begin Deserialize
            let mut buf = format!(
                "impl Deserialize<{name}> for Cursor<Vec<u8>> {{\n    \
                    fn _deserialize(&mut self) -> Result<{name}, io::Error> {{\n        \
                        let is_tl_id = self.deserialize::<u32>()? == {tl_id};\n",
                        name = name, tl_id = tl_id);

            for arg in args {
                let mut is_box = false;
                let mut fil_type = arg.type_.as_str();
                
                if fil_type.len() > 9 && &fil_type[0..9] == "Option < " {
                    fil_type = &fil_type[9..fil_type.len()-2];
                }
                    
                if fil_type.len() > 6 && &fil_type[0..6] == "Box < " {
                    fil_type = &fil_type[6..fil_type.len()-2];
                    is_box = true;
                }
                    
                if arg.flag != -1 {
                    if arg.type_ != "bool" {
                        write!(buf,
                          "        let {name} = if flags & (1 << {flag}) != 0 {{\n            \
                                       Some({opt_box1}self.deserialize::<{type_}>()?{opt_box2})\n            \
                                   }} else {{\n            \
                                       None\n            \
                                   }};\n",
                          type_ = fil_type, name = arg.name, flag = arg.flag, opt_box1 = if is_box { "Box::new(" } else { "" }, opt_box2 = if is_box { ")" } else { "" }).unwrap();
                    } else {
                        write!(buf,
                            "        let {name} = flags & (1 << {flag}) != 0;\n",
                            name = arg.name, flag = arg.flag).unwrap();
                    }
                } else {
                    write!(buf,
                        "        let {name} = self.deserialize::<{type_}>()?{opt_box};\n",
                        type_ = fil_type, name = arg.name, opt_box = if is_box { ".into()" } else { "" }).unwrap();
                }
            }

            write!(buf, "Ok({name} {{", name = name).unwrap();

            for arg in args {
                write!(buf, "{name}: {name}, \n", name = arg.name).unwrap();
            }
            
            buf.push_str("})    }\n}\n\n");
            // End Deserialize


            buf
        }

        RustType::Enum { ref variants, ref name } => {
            // Begin Deserialize
            let mut buf = format!(
                "impl Deserialize<{name}> for Cursor<Vec<u8>> {{\n    \
                    fn _deserialize(&mut self) -> Result<{name}, io::Error> {{\n        \
                        let tl_id = self.deserialize::<u32>()?;
                        match tl_id {{", name = name);

            for variant in variants {
                write!(buf, "\n {0}u32 => {{\n", variant.tl_id).unwrap();

                for arg in &variant.args {
                    let mut is_box = false;
                    let mut fil_type = arg.type_.as_str();

                    if fil_type.len() > 9 && &fil_type[0..9] == "Option < " {
                        fil_type = &fil_type[9..fil_type.len()-2];
                    }

                    if fil_type.len() > 6 && &fil_type[0..6] == "Box < " {
                        fil_type = &fil_type[6..fil_type.len()-2];
                        is_box = true;
                    }
                    
                    if arg.flag != -1 {
                        if arg.type_ != "bool" {
                            write!(buf,
                              "        let {name} = if flags & (1 << {flag}) != 0 {{\n            \
                                           Some({opt_box1}self.deserialize::<{type_}>()?{opt_box2})\n            \
                                       }} else {{\n            \
                                           None\n            \
                                       }};\n",
                              type_ = fil_type, name = arg.name, flag = arg.flag, opt_box1 = if is_box { "Box::new(" } else { "" }, opt_box2 = if is_box { ")" } else { "" }).unwrap();

                        } else {
                            write!(buf,
                                "        let {name} = flags & (1 << {flag}) != 0;\n",
                                name = arg.name, flag = arg.flag).unwrap();
                        }
                    } else {
                        write!(buf,
                            "        let {name} = self.deserialize::<{type_}>()?{opt_box};\n",
                            type_ = fil_type, name = arg.name, opt_box = if is_box { ".into()" } else { "" }).unwrap();
                    }
                }


                write!(buf, "Ok({}::{} {{\n", name, variant.name).unwrap();
                
                for arg in &variant.args {
                    write!(buf, "{name}: {name}, \n", name = arg.name).unwrap();
                }
                
                buf.push_str("}) }\n")
            }
            
            buf.push_str("_ => {Err(io::Error::new(io::ErrorKind::NotFound, \"You gave me an id for a Type, that id was not for that type. Oh no\"))}");

            buf.push_str("        }\n    }\n}\n");
            // End Deserialize


            buf
        }
    }
}
