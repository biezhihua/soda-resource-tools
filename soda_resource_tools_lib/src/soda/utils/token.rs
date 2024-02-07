use std::collections::BTreeMap;

use hmac::digest::KeyInit;
use hmac::Hmac;
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;

/// JWT https://github.com/mikkyang/rust-jwt
#[derive(Debug, Eq, PartialEq)]
pub struct Token {
    pub key: String,
    pub expire_time_millis: i64,
}

/// 根据ID和过期时间创建token
pub fn create_token(sign_key: &str, token: Token) -> String {
    //
    let mut claims: BTreeMap<String, String> = BTreeMap::new();
    claims.insert("key".to_string(), token.key.to_string());
    claims.insert("expire_time_millis".to_string(), token.expire_time_millis.to_string());

    //
    let hmac: Hmac<Sha256> = Hmac::new_from_slice(sign_key.as_bytes()).unwrap();
    return claims.sign_with_key(&hmac).unwrap().clone().to_string();
}

pub fn verification_token(sign_key: &str, token: &str) -> Option<Token> {
    let hmac: Hmac<Sha256> = Hmac::new_from_slice(sign_key.as_bytes()).unwrap();

    let claims: BTreeMap<String, String> = token.verify_with_key(&hmac).unwrap();

    let expire_time_millis = claims["expire_time_millis"].to_string();
    let key = claims["key"].to_string();

    return match expire_time_millis.parse::<i64>() {
        Ok(expire_time_millis) => Some(Token { key, expire_time_millis }),
        Err(_) => None,
    };
}

#[cfg(test)]
mod test {
    use crate::soda::utils::token::{create_token, verification_token, Token};

    #[test]
    fn test_create_token() {
        let token = create_token(
            "soda",
            Token {
                key: "time".to_string(),
                expire_time_millis: 60 * 24 * 7 * 1000,
            },
        );
        println!("access_token = {}", token);
        assert_eq!(
            token,
            "eyJhbGciOiJIUzI1NiJ9.eyJleHBpcmVfdGltZV9taWxsaXMiOiIxMDA4MDAwMCIsImtleSI6InRpbWUifQ.qK5Ff-aqKPoo15R3afHo5C4dV0XJhMEhsAwrR-UANwM"
        );
    }

    #[test]
    fn test_verification_token() {
        let token = verification_token(
            "soda",
            "eyJhbGciOiJIUzI1NiJ9.eyJleHBpcmVfdGltZV9taWxsaXMiOiIxMDA4MDAwMCIsImtleSI6InRpbWUifQ.qK5Ff-aqKPoo15R3afHo5C4dV0XJhMEhsAwrR-UANwM",
        )
        .unwrap();
        assert_eq!(
            token,
            Token {
                key: "time".to_string(),
                expire_time_millis: 60 * 24 * 7 * 1000
            }
        );
    }
}
