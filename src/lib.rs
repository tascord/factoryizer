use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Ident};

fn preamble(input: DeriveInput) -> (DeriveInput, Ident, DataStruct) {
    let name = input.clone().ident;
    let data = match input.clone().data {
        Data::Struct(data) => data,
        _ => panic!("Factory can only be implemented for structs"),
    };

    (input, name, data)
}

#[proc_macro_derive(Factory)]
pub fn fy_derive(input: TokenStream) -> TokenStream {
    let (_input, name, data) = preamble(parse_macro_input!(input as DeriveInput));
    let fields = data.fields;
    let names = fields.iter().map(|f| f.ident.clone());
    let types = fields.iter().map(|f| f.ty.clone());

    let implimentation = quote! {

        impl #name {

            pub fn new() -> Self {
                Self::default()
            }

            #(pub fn #names(&mut self, value: #types) -> &mut Self {
                self.#names = value;
                self
            })*
        }

    };

    TokenStream::from(implimentation)
}
