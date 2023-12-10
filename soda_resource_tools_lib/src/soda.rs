use std::sync::Mutex;

use once_cell::sync::Lazy;

use crate::soda::entity::{MTInfo, MTMetadata, TransferType};

use self::{
    entity::{LibConfig, ResourceType, ScrapeConfig},
    meta::strong_match_token,
};

pub(crate) mod cache;
pub(crate) mod dom;
pub mod entity;
pub(crate) mod extension_option;
pub(crate) mod extension_result;
pub(crate) mod fanart;
pub(crate) mod filebrowser;
pub(crate) mod finder;
pub(crate) mod global;
pub(crate) mod meta;
pub(crate) mod request;
pub(crate) mod scraper;
pub(crate) mod tmdb;
pub(crate) mod transfer;
pub(crate) mod utils;
pub(crate) mod watcher;

pub(crate) static LIB_CONFIG: Lazy<Mutex<LibConfig>> = Lazy::new(|| {
    let config = LibConfig::new();
    tracing::info!("LIB_CONFIG = {:?}", config);
    Mutex::new(config)
});

pub fn get_lib_config() -> LibConfig {
    let config = LIB_CONFIG.lock().unwrap();
    return config.clone();
}

/// 初始化配置
pub fn init_lib_config() {
    let mut config = LIB_CONFIG.lock().unwrap();

    strong_match_token::init();
}

/// 更新配置
pub fn update_lib_config(new_config: LibConfig) {
    let mut config = LIB_CONFIG.lock().unwrap();
    config.update(new_config);
}

pub fn create_mt_metadata(title: &str) -> Option<MTMetadata> {
    return meta::mt_metadata(title);
}

/// 刮削源目录资源到目标目录。
///
/// 刮削要分成多个步骤
/// 1. 找到要整理的资源
/// 2. 本地识别要整理的资源
/// 3. 远程识别要整理的资源
/// 4. 转移文件
/// 5. 刮削要整理的资源
pub fn scrape(resource_type: ResourceType, transfer_type: TransferType, scrape_config: ScrapeConfig, src_directory: String, target_directory: String) {
    init_lib_config();

    tracing::info!("scrape_src_to_target resource_type {:?}, transfer_mode {:?}, src_directory {:?}, target_directory {:?}", resource_type, transfer_type, src_directory, target_directory);

    // 找到要整理的资源
    finder::find(&resource_type, &src_directory, |path: String| {
        tracing::info!("find resource path = {:?}", path);

        match resource_type {
            ResourceType::MT => {
                // 本地识别要整理的资源
                if let Some(mut mt_meta) = meta::mt_metadata(&path) {
                    tracing::info!("meta mt_meta {:?}", mt_meta);

                    let mut mt_info: Option<MTInfo> = if scrape_config.enable_recognize {
                        // 远程识别要整理的资源
                        if let Some(mut info) = tmdb::recognize_mt(&mut mt_meta) {
                            tracing::info!("recognize mt_info {:?}", info.original_title());
                            Some(info)
                        } else {
                            tracing::error!("remote recognize resource failed, mt_meta {:?}", mt_meta);
                            None
                        }
                    } else {
                        tracing::info!("remote recognize resource disabled, mt_meta {:?}", mt_meta);
                        None
                    };

                    // 更新媒体图片
                    if let Some(mt_info) = &mut mt_info {
                        if scrape_config.enable_scrape_image {
                            fanart::obtain_images(mt_info);
                        }
                    }

                    // 选择重命名格式
                    let rename_format = if mt_meta.is_movie() { LIB_CONFIG.lock().unwrap().transfer_rename_format_movie.clone() } else { LIB_CONFIG.lock().unwrap().transfer_rename_format_tv.clone() };

                    tracing::info!("rename_format {:?}", rename_format);

                    // 转移文件
                    if let Some(transferred_path) = transfer::transfer(&target_directory, &transfer_type, &rename_format, &mt_meta, &path) {
                        if let Some(mt_info) = &mut mt_info {
                            // 刮削要整理的资源
                            scraper::scrape_metadata(&scrape_config, &mt_meta, &mt_info, &transferred_path);
                        }
                    };
                }
            }
        }
    });
}
