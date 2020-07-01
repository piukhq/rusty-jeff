extern crate base64;
extern crate openssl;

use openssl::pkey::Private;
use openssl::rsa::{Padding, Rsa};

use std::convert::TryInto;
use std::str;
use std::string::String;

pub fn rsa_decrypt_base64(priv_key_pem: &str, base64_text: &str) -> Result<String, &'static str> {
    let text_to_decrypt = base64::decode(base64_text).map_err(|_| "failed to decode base64")?;

    let priv_key: Rsa<Private> = Rsa::private_key_from_pem(priv_key_pem.as_bytes())
        .map_err(|_| "failed to parse private key PEM")?;
    let data_len = std::cmp::max(text_to_decrypt.len(), priv_key.size().try_into().unwrap());
    let mut decrypted_data: Vec<u8> = vec![0; data_len];
    priv_key
        .private_decrypt(&text_to_decrypt, &mut decrypted_data, Padding::PKCS1_OAEP)
        .map_err(|_| "failed to decrypt text")?;

    Ok(str::from_utf8(&decrypted_data)
        .map_err(|_| "failed to parse UTF-8 from decrypted data")?
        .trim_matches(char::from(0))
        .into())
}
