use jsonwebtoken::DecodingKey;
use std::fs;

pub fn load_decoding_key() -> DecodingKey {
    let pem = fs::read("E://projects/publicKey.pem")
        .expect("No se pudo leer public key");

    DecodingKey::from_rsa_pem(&pem).expect("Clave pública inválida")
}