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

        RustField {
            name: name,
            type_: type_,
            flag: get_attr(&f.attrs, "flag_bit").parse::<i64>().unwrap(),
        }
    }
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
        args: Vec<RustType>,
    }
}

impl From<syn::MacroInput> for RustType {
    fn from(input: syn::MacroInput) -> RustType {
        match input.body {
            syn::Body::Enum(variants) => {
                RustType::Enum(Vec::default())
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


            }
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
