use soda_resource_tools_lib::soda::utils::{encrypt, token};
use soda_resource_tools_lib::soda::utils::token::Token;

/// 验证密码
pub fn verify_password(password: &str, actual_password: &str) -> bool {
    return match encrypt::verify_password(password, actual_password, "soda") {
        Ok(is_ok) => {
            is_ok
        }
        Err(_) => {
            false
        }
    };
}

/// 密码加密
pub fn encrypt_password(password: &str) -> String {
    return encrypt::encrypt_password("soda", password);
}

/// 创建token
pub(crate) fn create_token(token: Token) -> String {
    let expire_time = token.expire_time_millis;
    let now_time = chrono::offset::Utc::now().timestamp_millis();
    let final_time = now_time + expire_time;
    tracing::info!("now_time = {}, expire_time = {}, final_time = {}", expire_time,now_time, final_time);
    return token::create_token("soda", Token {
        key: token.key,
        expire_time_millis: final_time,
    });
}

/// 验证token是否有效和过期
pub(crate) fn verification_token(token: String) -> Option<Token> {
    if let Some(ret) = token::verification_token("soda", &token) {
        let now_time = chrono::offset::Utc::now().timestamp_millis();
        let expire_time = ret.expire_time_millis;
        tracing::info!("now_time = {}, expire_time = {}",now_time,expire_time );
        return if now_time <= expire_time {
            Some(ret)
        } else {
            None
        };
    }
    return None;
}