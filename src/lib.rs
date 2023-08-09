extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{Attribute, parse_macro_input, DeriveInput, Data};

#[proc_macro_derive(Transformable, attributes(transform))]
pub fn transformable(input: TokenStream) -> TokenStream {
    // Parse the string representation
    let mut input = parse_macro_input!(input as DeriveInput);

    // Build the impl
    let name = input.ident;
    let transform_name = match &mut input.data {
        Data::Struct(ref mut struct_body) => {
            let field = struct_body.fields.iter_mut().find(|field| get_transform_field(&field.attrs).is_some())
                .expect("No field has the #[transform] attribute.");

            field.attrs.remove(get_transform_field(&field.attrs).unwrap());
            
            field.ident.clone().expect("Tuple structs are not supported.")
        },
        _ => panic!("Only structs are supported.")
    };

    quote! {
        impl Transformable for #name {
            fn transform(&self) -> &Transform {
                &self.#transform_name
            }
            fn transform_mut(&mut self) -> &mut Transform {
                &mut self.#transform_name
            }
        }
    }.into()
}

fn get_transform_field(attributes: &[Attribute]) -> Option<usize> {
    attributes
        .iter()
        .enumerate()
        .find(|(_, attr)| attr.path().is_ident("transform"))
        .map(|(index, _)| index)
}