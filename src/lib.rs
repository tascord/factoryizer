use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Field, Ident};

fn preamble(input: DeriveInput) -> (DeriveInput, Ident, DataStruct) {
    let name = input.clone().ident;
    let data = match input.clone().data {
        Data::Struct(data) => data,
        _ => panic!("Factory can only be implemented for structs"),
    };

    (input, name, data)
}

#[proc_macro_derive(Factory, attributes(skip, into))]
pub fn fy_derive(input: TokenStream) -> TokenStream {
    let (input, name, data) = preamble(parse_macro_input!(input as DeriveInput));
    let into = input.attrs.iter().any(|a| a.path().is_ident("into"));
    let fields = data
        .fields
        .iter()
        .filter(|f| !f.attrs.iter().any(|a| a.path().is_ident("skip")))
        .collect::<Vec<&Field>>();

    let names = fields.iter().map(|f| f.ident.clone());
    let types = fields.iter().map(|f| f.ty.clone());

    let implimentation = if into {
        quote! {

            impl #name {

                pub fn new() -> Self {
                    Self::default()
                }

                #(pub fn #names<T>(&mut self, value: T) -> &mut Self
                where T: Into<#types> {
                    self.#names = Into::into(value);
                    self
                })*
            }

        }
    } else {
        quote! {

            impl #name {

                pub fn new() -> Self {
                    Self::default()
                }

                #(pub fn #names(mut self, value: #types) -> Self {
                    self.#names = value;
                    self
                })*
            }

        }
    };

    TokenStream::from(implimentation)
}
