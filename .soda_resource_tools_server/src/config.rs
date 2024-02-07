use std::io::{Read, Write};
use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub(crate) struct Config {
    /// 超级管理员
    pub(crate) admin_user: String,
    /// 超级管理员初始密码
    pub(crate) admin_password: String,
    /// TOKEN过期时间
    pub(crate) access_token_expire_millis: i64,
    /// 电视剧重命名格式
    pub(crate) tv_rename_format: String,
    /// 电影重命名格式
    pub(crate) movie_rename_format: String,
}

impl Config {
    pub(crate) fn new() -> Config {
        return Config {
            admin_user: "admin".to_string(),
            admin_password: "password".to_string(),
            access_token_expire_millis: 7 * 24 * 60 * 60 * 1000,
            tv_rename_format: "".to_string(),
            movie_rename_format: "".to_string(),
        };
    }

    pub(crate) fn save(&self) {
        let toml_config = TomlConfig { tv_rename_format: self.tv_rename_format.clone(), movie_rename_format: self.movie_rename_format.clone() };
        save_toml_config(toml_config);
    }

    pub(crate) fn update(&mut self) {
        if let Some(toml_config) = get_toml_config() {
            self.movie_rename_format = toml_config.movie_rename_format.clone();
            self.tv_rename_format = toml_config.tv_rename_format.clone();
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct TomlConfig {
    /// 电视剧重命名格式
    pub(crate) tv_rename_format: String,
    /// 电影重命名格式
    pub(crate) movie_rename_format: String,
}

pub(crate) fn get_toml_config() -> Option<TomlConfig> {
    return if Path::new("config/config.toml").exists() {
        let mut buffer = String::new();

        std::fs::OpenOptions::new()
            .read(true)
            .open("config/config.toml")
            .expect("Couldn't open file")
            .read_to_string(&mut buffer).unwrap();

        let config: TomlConfig = toml::from_str(&buffer).unwrap();

        tracing::info!("{:?}", config);

        Some(config)
    } else {
        None
    };
}

pub(crate) fn save_toml_config(config: TomlConfig) {
    tracing::info!("{:?}", config);
    let toml = toml::to_string_pretty(&config).unwrap();
    std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("config/config.toml")
        .expect("Couldn't open file")
        .write_all(toml.as_bytes())
        .unwrap();
}
