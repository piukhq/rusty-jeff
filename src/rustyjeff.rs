extern crate base64;
extern crate openssl;

use openssl::rsa::{Padding, Rsa};
use rayon::prelude::*;

use std::convert::TryInto;
use std::str;
use std::string::String;

pub fn rsa_decrypt_base64(
    priv_key_pem: &str,
    field_values: Vec<&str>,
) -> Result<Vec<String>, &'static str> {
    let priv_key = Rsa::private_key_from_pem(priv_key_pem.as_bytes())
        .map_err(|_| "failed to parse private key PEM")?;
    let buf_size: usize = priv_key.size().try_into().unwrap();

    field_values
        .par_iter()
        .map(|v| {
            let mut decrypted_data = vec![0; buf_size];
            base64::decode(v)
                .map_err(|_| "failed to decode base64")
                .and_then(|value| {
                    priv_key
                        .private_decrypt(&value, &mut decrypted_data, Padding::PKCS1_OAEP)
                        .map_err(|_| "failed to decrypt text")
                        .and_then(|size| {
                            Ok(String::from_utf8_lossy(&decrypted_data[0..size]).into_owned())
                        })
                })
        })
        .collect()
}
