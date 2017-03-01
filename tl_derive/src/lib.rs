#[macro_use]
extern crate quote;
extern crate proc_macro;
extern crate syn;

use proc_macro::TokenStream;

#[proc_macro_derive(TlType, attributes(tl_id, flag_num))]
pub fn hello_world(input: TokenStream) -> TokenStream {
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

fn impl_tl_type(ast: &syn::MacroInput) -> quote::Tokens {
    let tl_id: String;
    // for attr in &ast.attributes {
    //     if 
    // }
    let name = &ast.ident;
    quote! {
        impl From<#name> for Numbers {
            fn from(x: #name) -> Numbers {
                Numbers::#name(x.into())
            }
        }
    } // TODO Serializer serde
}
