use std::num::NonZeroU32;

use data_encoding::HEXLOWER;
use rand::Rng;
use ring::{digest, pbkdf2};

/// https://rust-lang-nursery.github.io/rust-cookbook/cryptography/encryption.html
/// https://aqrun.com/rust/rust-encryption/

/// 加密算法
static PBKDF2_ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA256;
/// 算法输出长度
const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;

/// 生成密码随机salt 64位字符串
pub fn generate_salt() -> String {
    // 32位u8数组
    let mut salt = [0u8; CREDENTIAL_LEN];
    // 填充随机数
    rand::thread_rng().fill(&mut salt[..]);
    // 转为16进制字符串
    let str_salt = HEXLOWER.encode(&salt);

    str_salt
}

///
/// 密码序列化
///
pub fn encrypt_password(salt: &str, password: &str) -> String {
    // 算法迭代次数
    let n_iter: NonZeroU32 = NonZeroU32::new(100_000).unwrap();
    let mut to_store: [u8; CREDENTIAL_LEN] = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(PBKDF2_ALG, n_iter, salt.as_bytes(), password.as_bytes(), &mut to_store);
    // 将类型[u8; CREDENTIAL_LEN] 转为 字符串
    let str_result = HEXLOWER.encode(&to_store);

    str_result
}

///
/// 较验密码
///
pub fn verify_password(password: &str, actual_password: &str, salt: &str) -> Result<bool, String> {
    // 将存储的字符串密码解析为u8数组
    let mut actual_password_decode: Vec<u8> = Vec::new();

    if let Ok(res) = HEXLOWER.decode(actual_password.as_bytes()) {
        actual_password_decode = res;
    }

    let n_iter: NonZeroU32 = NonZeroU32::new(100_000).unwrap();

    // 较验密码是否匹配
    match pbkdf2::verify(PBKDF2_ALG, n_iter, salt.as_bytes(), password.as_bytes(), actual_password_decode.as_slice()) {
        Ok(_) => Ok(true),
        _ => Err("Failed".to_string()),
    }
}

#[cfg(test)]
mod test {
    use super::{encrypt_password, generate_salt, verify_password};

    /// 生成64位随机salt
    #[test]
    fn test_generate_salt() {
        let salt = generate_salt();
        assert_eq!(salt.len(), 64);
    }

    /// 测试密码加密
    #[test]
    fn test_encrypt_password() {
        let salt = "030c8d02eea6e5e5219096bd076c41e58e955632d59beb7d44fa18e3fbccb0bd";
        let password = "abc123";
        let res = encrypt_password(salt, password);
        assert_eq!(res, "76b0f1985d149b9d3770b3ef3c8d59b5ec9ec32ad0499e0882a25c567ccf99d6");
    }

    /// 测试密码验证正确
    #[test]
    fn test_password_verify() {
        let salt = "030c8d02eea6e5e5219096bd076c41e58e955632d59beb7d44fa18e3fbccb0bd";
        let actual_pass = "76b0f1985d149b9d3770b3ef3c8d59b5ec9ec32ad0499e0882a25c567ccf99d6";
        let password = "abc123";
        let res = verify_password(password, actual_pass, salt);
        assert!(res.is_ok());
    }

    /// 测试密码验证不正确
    #[test]
    fn test_password_verify_false() {
        let salt = "030c8d02eea6e5e5219096bd076c41e58e955632d59beb7d44fa18e3fbccb0bd";
        let actual_pass = "76b0f1985d149b9d3770b3ef3c8d59b5ec9ec32ad0499e0882a25c567ccf99d6";
        let password = "abc1231";
        let res = verify_password(password, actual_pass, salt);
        assert!(res.is_err());
    }
}
