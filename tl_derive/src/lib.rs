#[macro_use]
extern crate quote;
extern crate proc_macro;
extern crate syn;

use proc_macro::TokenStream;

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
        variants: Vec<RustType>,
    }
}

impl From<syn::Field> for RustField {
    fn from(f: syn::Field) -> RustField {
        let name = if let Some(i) = f.ident {
            i.as_ref().to_string()
        } else {
            String::default()
        };

        let type_ = match f.ty {
            syn::Ty::Path(_, path) => {
                let segs = path.segments.iter()
                                        .map(|m| m.ident.as_ref())
                                        .collect::<Vec<&str>>();

                let mut s = String::from(segs[0]);

                for seg in &segs[1..] {
                    s.push_str(seg);
                }

                s
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

impl<'a> From<&'a syn::Field> for RustField {
    fn from(f: &'a syn::Field) -> RustField {
        let name = if let Some(ref i) = f.ident {
            i.as_ref().to_string()
        } else {
            String::default()
        };

        let type_ = match f.ty {
            syn::Ty::Path(_, ref path) => {
                let segs = path.segments.iter()
                                        .map(|m| m.ident.as_ref())
                                        .collect::<Vec<&str>>();

                let mut s = String::from(segs[0]);

                for seg in &segs[1..] {
                    s.push_str(seg);
                }

                s
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
        let name = input.ident.as_ref().to_string();

        match input.body {
            syn::Body::Enum(variants) => {
                let m_variants = Vec::new();

                variants.into_iter().map(|v| v.into()).collect::<Vec<RustType>>();

                RustType::Enum {
                    name: name,
                    variants: m_variants,
                }
            }
            syn::Body::Struct(v_data) => {
                let args = match v_data {
                    syn::VariantData::Struct(fields) => {
                        fields.into_iter()
                              .map(|f| f.into())
                              .collect::<Vec<RustField>>()
                    }
                    syn::VariantData::Tuple(fields) => {
                        fields.into_iter()
                              .map(|f| f.into())
                              .collect::<Vec<RustField>>()
                    }
                    syn::VariantData::Unit => {
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

impl<'a> From<&'a syn::MacroInput> for RustType {
    fn from(input: &'a syn::MacroInput) -> RustType {
        let name = input.ident.as_ref().to_string();

        match input.body {
            syn::Body::Enum(ref variants) => {
                let m_variants = variants.into_iter().map(|v| v.into()).collect::<Vec<RustType>>();

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

impl From<syn::Variant> for RustType {
    fn from(v: syn::Variant) -> RustType {
        let name = v.ident.as_ref().to_string();

        let args = match v.data {
            syn::VariantData::Struct(fields) => {
                fields.into_iter()
                      .map(|f| f.into())
                      .collect::<Vec<RustField>>()
            }
            syn::VariantData::Tuple(fields) => {
                fields.into_iter()
                      .map(|f| f.into())
                      .collect::<Vec<RustField>>()
            }
            syn::VariantData::Unit => {
                Vec::default()
            }
        };

        let tl_id = get_attr(&v.attrs, "tl_id");

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

impl<'a> From<&'a syn::Variant> for RustType {
    fn from(v: &'a syn::Variant) -> RustType {
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



#[proc_macro_derive(TlType, attributes(tl_id, flag_bit))]
pub fn tl_type(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();
    
    // Parse the string representation
    let ast = syn::parse_macro_input(&s).unwrap();
    println!("{:#?}", ast);
    println!("{:#?}", RustType::from(&ast));

    // Build the impl
    let gen = impl_tl_type(&ast);
    
    // Return the generated impl
    gen.parse().unwrap()
}

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

fn impl_tl_type(ast: &syn::MacroInput) -> quote::Tokens {
    let tl_id = get_attr(&ast.attrs, "tl_id");

    println!("{:?}", tl_id);

    let name = &ast.ident;
    quote! {
        impl From<#name> for TlType {
            fn from(x: #name) -> TlType {
                TlType::#name(x.into())
            }
        }
    } // TODO Serializer serde
}
