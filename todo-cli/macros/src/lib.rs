extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Fields, Data};



#[proc_macro_derive(Create)]
pub fn create_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_identifier = &input.ident;
    let fields = match &input.data {
        Data::Struct(data) => &data.fields,
        _ => panic!("Create can only be derived for structs"),
    };
  
   let mut field_printing = Vec::new();

   
   for field in fields {
       let field_name = field.ident.as_ref().unwrap();
       let print_code = quote! {
           println!("Field '{}' has value '{:?}'", stringify!(#field_name), self.#field_name);
       };
       field_printing.push(print_code);
   }

        

    
    let expanded = quote! {
        impl #struct_identifier{
            pub fn create(self) -> Self{
                #( #field_printing )*

                self
            }
        }
    };
    
    TokenStream::from(expanded)
}


