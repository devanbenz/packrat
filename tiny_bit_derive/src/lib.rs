use proc_macro::TokenStream;
use syn::__private::quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(TinyBitSerializer)]
pub fn tiny_bit_serializer(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl #name {
            pub fn serialize(&self) -> Vec<u8> {
                let mut kv_bytes: Vec<u8> = vec![];

                let key = &self.key;
                let key_len = key.len() as u8;
                kv_bytes.push(key_len);
                for b in key.bytes() {
                    kv_bytes.push(b);
                }

                let value = &self.value;
                let value_len = value.len() as u8;
                kv_bytes.push(value_len);
                for b in value.bytes() {
                    kv_bytes.push(b);
                }

                kv_bytes
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(TinyBitDeserializer)]
pub fn tiny_bit_deserializer(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl #name {
            pub fn deserialize(bytes: Vec<u8>) -> Result<Vec<(String, String)>, std::string::FromUtf8Error>{
                let mut str_vec: Vec<String> = vec![];
                let mut tup_vec: Vec<(String, String)> = vec![];
                let mut v_iter = bytes.into_iter();

                loop {
                    if let Some(bit) = v_iter.next() {
                        let mut str = String::new();
                        for _ in 0..bit.to_owned() {
                            if let Some(utf_char) = v_iter.next() {
                                let a = String::from_utf8(vec![utf_char])?;
                                str.push(a.parse().expect("cannot parse to char"));
                            }
                        }

                        if str_vec.len() == 2 {
                            let value = str_vec.pop().expect("could not get value");
                            let key = str_vec.pop().expect("could not get key");

                            tup_vec.push((key, value));
                            str_vec.push(str);
                        } else {
                            str_vec.push(str);
                        }
                    } else {
                        if str_vec.len() != 0 {
                            let value = str_vec.pop().expect("could not get value");
                            let key = str_vec.pop().expect("could not get key");
                            tup_vec.push((key, value));
                        }
                        break;
                    }
                }
                Ok(tup_vec)
            }
        }
    };

    TokenStream::from(expanded)
}
