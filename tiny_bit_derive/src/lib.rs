use proc_macro::TokenStream;
use syn::__private::quote::quote;
use syn::{parse_macro_input, parse_quote, DeriveInput, Generics};

#[proc_macro_derive(TinyBitSerializer)]
pub fn tiny_bit_serializer(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = generic_debug_constraint(&input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            pub fn serialize(&self) -> Vec<u8> {
                format!("{:?}", &self).into_bytes()
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(TinyBitDeserializer)]
pub fn tiny_bit_deserializer(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = generic_debug_constraint(&input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            pub fn deserialize(bytes: Vec<u8>) {
                println!("{:?}", std::str::from_utf8(&bytes).unwrap());
            }
        }
    };

    TokenStream::from(expanded)
}

fn generic_debug_constraint(generics: &Generics) -> Generics {
    let mut new_generics = generics.clone();
    for param in new_generics.type_params_mut() {
        param.bounds.push(parse_quote!(std::fmt::Debug));
    }

    new_generics
}
