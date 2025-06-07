
pub const XOR_KEY: &[u8] = b"my_super_secret_key"; 
pub const AES_KEY: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ-01337"; 
pub const IV: [u8; 16] = [51, 201, 144, 12, 200, 112, 234, 177, 222, 42, 94, 33, 105, 14, 99, 80]; 
pub const B64_CHARSET: &str = "+/ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"; 

pub fn decrypt<'a>(method: &str, encrypted_hex: &str) -> String {
    let encrypted_bytes = hex::decode(encrypted_hex).expect("Invalid hex string");

    match method {
        "xor" => xor_decrypt(&encrypted_bytes, XOR_KEY),
        "aes" => aes_decrypt(&encrypted_bytes, AES_KEY), 
        "b64" => b64_decode(&encrypted_bytes, B64_CHARSET), 
        _ => panic!("Unsupported decrypt method: {}", method),
    }
}

fn xor_decrypt<'a>(data: &[u8], key: &[u8]) -> String {
    let decrypted: Vec<u8> = data.iter()
        .enumerate()
        .map(|(i, &b)| b ^ key[i % key.len()])
        .collect();
    String::from_utf8(decrypted).expect("Invalid UTF-8")
}

fn aes_decrypt<'a>(data: &[u8], key: &[u8]) -> String {

    use aes::Aes256;
    use cbc::Decryptor;
    use cipher::{KeyIvInit, BlockDecryptMut};
    use block_padding::Pkcs7;

    let key = &key[..32.min(key.len())]; 

    let cipher = Decryptor::<Aes256>::new_from_slices(key, &IV)
        .expect("Invalid AES-256 key/IV");

    let mut buffer = data.to_vec();

    let decrypted = cipher
        .decrypt_padded_mut::<Pkcs7>(&mut buffer)
        .expect("AES decrypt failed");

    String::from_utf8(decrypted.to_vec()).expect("Invalid UTF-8")
}

fn b64_decode<'a>(data: &[u8], _key: &str) -> String {

    use base64::prelude::*;
    use base64::{
        alphabet,
        engine::{self, general_purpose},
    };

    let trimmed: Vec<u8> = data
        .iter()
        .rev()
        .skip_while(|&&x| x == b'=') 
        .copied()
        .collect::<Vec<u8>>()
        .into_iter()
        .rev()
        .collect();

    let payload = String::from_utf8(trimmed).unwrap();

    let alphabet = alphabet::Alphabet::new(B64_CHARSET)
        .expect("Invalid base64 alphabet");

    let crazy_engine = engine::GeneralPurpose::new(&alphabet, general_purpose::NO_PAD);

    let custom_b64_value = crazy_engine.decode(payload).unwrap();
	
    String::from_utf8(custom_b64_value).unwrap()
}