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

#[proc_macro_derive(Factory, attributes(skip))]
pub fn fy_derive(input: TokenStream) -> TokenStream {
    let (input, name, data) = preamble(parse_macro_input!(input as DeriveInput));
    let fields = data
        .fields
        .iter()
        .filter(|f| !f.attrs.iter().any(|a| a.path().is_ident("skip")))
        .collect::<Vec<&Field>>();

    let names = fields.iter().map(|f| f.ident.clone());
    let types = fields.iter().map(|f| f.ty.clone());

    let (i, t, w) = input.generics.split_for_impl();

    let implimentation = quote! {

        impl #i #name #t #w {

            pub fn new() -> Self {
                Self::default()
            }

            #(pub fn #names(mut self, value: impl Into<#types>) -> Self {
                self.#names = value.into();
                self
            })*
        }

    };

    TokenStream::from(implimentation)
}
