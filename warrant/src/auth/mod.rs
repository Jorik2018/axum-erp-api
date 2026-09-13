mod claims;
mod extractor;

pub use claims::Claims;
pub use extractor::AuthUser;

use jsonwebtoken::{Algorithm, DecodingKey, Validation};

pub struct JwtService {
    key: DecodingKey,
    validation: Validation,
}

impl JwtService {
    pub fn new(public_key: &[u8], issuer: Option<String>) -> Result<Self, jsonwebtoken::errors::Error> {
        let key = DecodingKey::from_rsa_pem(public_key)?;
        let mut validation = Validation::new(Algorithm::RS256);
        validation.validate_exp = true;
        if let Some(issuer) = issuer {
            validation.set_issuer(&[issuer]);
        }
        Ok(Self { key, validation })
    }

    pub fn validate(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let data = jsonwebtoken::decode::<Claims>(token, &self.key, &self.validation)?;
        Ok(data.claims)
    }
}
