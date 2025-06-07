use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr, Token};
use syn::parse::{Parse, ParseStream};

use cryptix_1988::{XOR_KEY, AES_KEY, IV, B64_CHARSET};

struct EncryptInput {
    string: LitStr,
    _comma: Token![,],
    method: LitStr,
}

impl Parse for EncryptInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(EncryptInput {
            string: input.parse()?,
            _comma: input.parse()?,
            method: input.parse()?,
        })
    }
}

#[proc_macro]
pub fn cx<'a>(input: TokenStream) -> TokenStream {
    let EncryptInput { string, method, .. } = parse_macro_input!(input as EncryptInput);

    let plain = string.value();
    let method = method.value();

    let encrypted_bytes = match method.as_str() {
        "xor" => xor_encrypt(plain.as_bytes(), XOR_KEY),
        "aes" => aes_encrypt(plain.as_bytes(), AES_KEY),
        "b64" => base64_encode(plain.as_bytes(), B64_CHARSET),
        _ => panic!("Unsupported encryption method: {}", method),
    };

    let hex_str = hex::encode(&encrypted_bytes);

    let output = quote! {
        {
            cryptix_1988::decrypt(#method, #hex_str)
        }
    };

    output.into()
}

fn xor_encrypt<'a>(data: &[u8], key: &[u8]) -> Vec<u8> {
    data.iter()
        .enumerate()
        .map(|(i, &b)| b ^ key[i % key.len()])
        .collect()
}

fn aes_encrypt<'a>(data: &[u8], key: &[u8]) -> Vec<u8> {

    use aes::Aes256;
    use cbc::Encryptor;
    use cipher::{KeyIvInit, BlockEncryptMut};
    use block_padding::{Pkcs7};

    let cipher = Encryptor::<Aes256>::new(key.into(), &IV.into());

    let block_size = 16;
    let mut buffer = Vec::with_capacity(data.len() + block_size);
    buffer.extend_from_slice(data);
    buffer.resize(data.len() + block_size, 0); 

    let encrypted = cipher
        .encrypt_padded_mut::<Pkcs7>(&mut buffer, data.len())
        .expect("PKCS7 padding failed");

    encrypted.to_vec()
}

fn base64_encode<'a>(data: &[u8], _key: &str) -> Vec<u8> {

    use base64::prelude::*;
    use base64::{
        alphabet,
        engine::{self},
    };

    let alphabet = alphabet::Alphabet::new(
        "+/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
    ).expect("Invalid base64 alphabet");

    let crazy_config = engine::GeneralPurposeConfig::new()
        .with_decode_allow_trailing_bits(true)
        .with_encode_padding(true)
        .with_decode_padding_mode(engine::DecodePaddingMode::RequireNone);

    let crazy_engine = engine::GeneralPurpose::new(&alphabet, crazy_config);

    let encoded = crazy_engine.encode(data);

    encoded.into_bytes()

}