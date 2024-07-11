extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Fields, Data};



#[proc_macro_derive(Create)]
pub fn create_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_identifier = &input.ident;

    let expanded = match &input.data {
        Data::Struct(data_struct) => {
            match &data_struct.fields {
                Fields::Named(fields_named) => {
                    let field_names: Vec<_> = fields_named.named.iter().map(|f| {
                        let ident = f.ident.as_ref().unwrap().to_string();
                        quote! { #ident }
                    }).collect();

                    let field_values: Vec<_> = fields_named.named.iter().map(|f| {
                        let ident = f.ident.as_ref().unwrap();
                        quote! { &self.#ident as &dyn rusqlite::ToSql }
                    }).collect();

                    quote! {
                        impl Create for #struct_identifier {
                            fn table_name() -> &'static str {
                                stringify!(#struct_identifier).to_lowercase().as_str()
                            }

                            fn field_names() -> &'static str {
                                Box::leak(format!("{}", stringify!(#(#field_names),*)).into_boxed_str())
                            }

                            fn field_values(&self) -> Vec<&dyn rusqlite::ToSql> {
                                vec![#(#field_values),*]
                            }
                        }
                    }
                },
                _ => panic!("Create can only be derived for structs with named fields"),
            }
        },
        _ => panic!("Create can only be derived for structs"),
    };

    TokenStream::from(expanded)
}
