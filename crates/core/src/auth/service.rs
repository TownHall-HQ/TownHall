use std::fmt::Display;

use argon2::verify_encoded;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use pxid::Pxid;
use serde::{Deserialize, Serialize};

use super::error::{AuthError, Result};

const JWT_AUDIENCE: &str = "TownHall";
const TOKEN_DURATION: Duration = Duration::days(30);

/// JWT Token Abstaction
#[derive(Debug)]
pub struct Token {
    pub(crate) raw: String,
    pub(crate) claims: Claims,
}

impl Token {
    /// Retrieves the token's user ID
    pub fn user_id(&self) -> Pxid {
        self.claims.uid
    }

    /// Retrieves the internal JWT String
    pub fn token_string(&self) -> String {
        self.raw.to_string()
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}

#[derive(Clone)]
pub struct AuthService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    validation: Validation,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub uid: Pxid,
    pub iat: usize,
}

impl AuthService {
    pub fn new(jwt_secret: &str) -> Self {
        let encoding_key = EncodingKey::from_secret(jwt_secret.as_bytes());
        let decoding_key = DecodingKey::from_secret(jwt_secret.as_bytes());
        let mut validation = Validation::new(Algorithm::HS256);

        validation.set_audience(JWT_AUDIENCE.as_bytes());

        Self {
            encoding_key,
            decoding_key,
            validation,
        }
    }

    pub fn sign_token(&self, uid: Pxid) -> Result<Token> {
        let iat = Utc::now().timestamp() as usize;
        let exp = Utc::now()
            .checked_add_signed(TOKEN_DURATION)
            .ok_or(AuthError::SignTokenError)?
            .timestamp() as usize;
        let claims = Claims { exp, iat, uid };
        let jwt = encode(&Header::default(), &claims, &self.encoding_key)?;

        Ok(Token { raw: jwt, claims })
    }

    pub fn verify_token(&self, token: &Token) -> Result<Claims> {
        let token_data = decode::<Claims>(&token.raw, &self.decoding_key, &self.validation)?;

        Ok(token_data.claims)
    }

    pub fn validate_password(&self, encoded: &str, raw: &str) -> bool {
        let raw = raw.as_bytes();

        verify_encoded(encoded, raw).unwrap()
    }

    pub fn parse_jwt(&self, jwt: &str) -> Result<Token> {
        let claims = Self::decode_token(jwt, &self.decoding_key, &self.validation)?;

        Ok(Token {
            raw: jwt.to_string(),
            claims,
        })
    }

    pub(crate) fn decode_token(
        token: &str,
        decoding_key: &DecodingKey,
        validation: &Validation,
    ) -> Result<Claims> {
        let token_data = decode::<Claims>(token, decoding_key, validation)?;

        Ok(token_data.claims)
    }
}
