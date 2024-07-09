extern crate proc_macro;
use quote::quote;
use syn::Data;
use proc_macro::TokenStream;

#[proc_macro_derive(Create)]
pub fn create(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);
    let struct_identifier = &input.ident;
    let expanded = quote!{
        impl Create for #struct_identifier{
            // INSERT INTO TABLE #name #fields VALUES #values;
            fn get_fields(){}
            fn get_name() -> &str{
                stringify!(#struct_identifier).to_lowercase().as_str()
            }
            fn get_value() {}
            fn execute_query(){
                // INSERT INTO #name #fields VALUES #values;
                // let sql = format!("INSERT INTO {} {} VALUES {}", Self::get_name(), Self::get_fields(), Self::get_value());
                // let con = database::get_connection();
                // database::execute_query(&con, &sql);
            }
        }
    };
    TokenStream::from(expanded)

}

