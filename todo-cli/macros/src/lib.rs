extern crate proc_macro;
extern crate rusqlite;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Fields, Data};
use rusqlite::{Connection, Result, params};

#[proc_macro_derive(Create)]
pub fn create(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_identifier = &input.ident;

    let sql_code = match input.data {
        Data::Struct(ref data_struct) => {
            let fields = match data_struct.fields {
                Fields::Named(ref fields_named) => &fields_named.named,
                _ => panic!("Unsupported data type")
            };

            

            let field_names: Vec<_> = fields.iter().map(|f| f.ident.as_ref().unwrap()).collect();
            let field_names_str: Vec<_> = field_names.iter().map(|f| f.to_string()).collect();
            let field_values: Vec<_> = field_names.iter().map(|f| quote! { &self.#f }).collect();

            let joined_field_names = field_names_str.join(",");
            let placeholders = vec!["?"; field_names.len()].join(",");

            quote! {
                impl #struct_identifier {
                    pub fn create(&self, con: &Connection) -> Result<i64> {
                        let sql = format!(
                            "INSERT INTO {} ({}) VALUES ({})",
                            stringify!(#struct_identifier),
                            #joined_field_names,
                            #placeholders
                        );

                        let params = params![#(#field_values),*];
                        match con.execute(&sql, params) {
                            Ok(_) => Ok(con.last_insert_rowid()),
                            Err(err) => Err(err),
                        }
                    }
                }
            }
        }
        _ => unimplemented!(),
    };

    TokenStream::from(sql_code)
}

#[proc_macro_derive(Read)]
pub fn read(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_identifier = &input.ident;

    let sql_code = match input.data {
        Data::Struct(ref data_struct) => {
            println!("{:?}", data_struct);
            quote!{
                impl #struct_identifier {
                    println!("Hello, world!");
                }
            }
        }
        _ => unimplemented!(),
    };

    TokenStream::from(sql_code)
}

#[proc_macro_derive(Update)]
pub fn update(input: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[proc_macro_derive(Delete)]
pub fn delete(input: TokenStream) -> TokenStream {
    TokenStream::new()
}