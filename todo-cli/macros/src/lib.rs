extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Fields, Data};



#[proc_macro_derive(Create)]
pub fn create_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_identifier = &input.ident;
   
    let expanded = quote! {
        impl #struct_identifier{
            pub fn create(self) -> Self{
                  println!("Create trait for struct {:?}", self);

                self
            }
        }
    };
    
    TokenStream::from(expanded)
}


