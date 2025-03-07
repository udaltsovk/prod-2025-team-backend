use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::config;

static TOKEN_LIFETIME: usize = 60 * 60 * 24 * 3 as usize;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub sub: String,
}

pub fn new(id: Uuid) -> String {
    let current_time = Utc::now().timestamp() as usize;
    let mut header = Header::new(Algorithm::HS256);
    header.kid = Some("client".to_owned());
    encode(
        &header,
        &Claims {
            exp: current_time + TOKEN_LIFETIME,
            iat: current_time,
            sub: id.to_string(),
        },
        &EncodingKey::from_secret(config::JWT_SECRET.as_bytes()),
    )
    .unwrap()
}

pub fn parse(token: String) -> Option<Claims> {
    match decode(
        &token,
        &DecodingKey::from_secret(config::JWT_SECRET.as_bytes()),
        &Validation::default(),
    ) {
        Err(..) => None,
        Ok(data) => Some(data.claims),
    }
}
