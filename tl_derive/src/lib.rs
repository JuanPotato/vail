#[macro_use]
extern crate quote;
extern crate proc_macro;
extern crate syn;

use proc_macro::TokenStream;

#[proc_macro_derive(TlType, attributes(tl_id, flag_bit))]
pub fn tl_type(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();
    
    // Parse the string representation
    let ast = syn::parse_macro_input(&s).unwrap();
    // println!("{:#?}", ast);

    // Build the impl
    let gen = impl_tl_type(&ast);
    
    // Return the generated impl
    gen.parse().unwrap()
}

fn get_attr(ast: &syn::MacroInput, key: &str) -> String {
    let mut val = String::new();

    for attr in &ast.attrs {
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
    let tl_id = get_attr(ast, "tl_id");

    print!("{:?}", tl_id);

    let name = &ast.ident;
    quote! {
        impl From<#name> for TlType {
            fn from(x: #name) -> TlType {
                TlType::#name(x.into())
            }
        }
    } // TODO Serializer serde
}
