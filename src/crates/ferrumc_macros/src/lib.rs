extern crate proc_macro;

use proc_macro::TokenStream;

use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(Decode)]
pub fn decode_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Used to store all field decoding statements
    let mut field_statements = Vec::new();

    // Check if our struct has named fields
    if let syn::Data::Struct(syn::DataStruct { fields: syn::Fields::Named(fields), .. }) = input.data {
        for field in fields.named {
            // Get the identifier of the field
            let ident = field.ident.unwrap();
            // Generate a statement to decode this field from the bytes
            let type_name = field.ty;
            let statement = quote! {
                #ident: Box::into_inner(#type_name::decode(bytes)?),
            };
            field_statements.push(statement);
        }
    }

    // Get the identifier of our struct
    let name = input.ident;

    // Generate the implementation
    let expanded = quote! {
        impl #name {
            pub fn decode<T>(bytes: &mut T) -> Result<Self, Error>
            where
                T: Read,
            {
                Ok(Self {
                    #(#field_statements)*
                })
                
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}