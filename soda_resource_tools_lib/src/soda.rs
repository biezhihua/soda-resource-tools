use std::{
    collections::HashMap,
    fs,
    path::{self, Path},
    sync::Mutex,
};

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::soda::entity::{MTInfo, MTMetadata, RenameStyle, TransferType};

use self::{
    entity::{LibConfig, MetaContext, NamesMap, ResourceType, ScrapeConfig, SodaError},
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
pub mod meta;
pub(crate) mod request;
pub(crate) mod scraper;
pub(crate) mod tmdb;
pub(crate) mod transfer;
pub(crate) mod utils;
pub(crate) mod watcher;

pub(crate) static LIB_CONFIG: Lazy<Mutex<LibConfig>> = Lazy::new(|| Mutex::new(LibConfig::new()));

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

/// 刮削源目录资源到目标目录。
///
/// 刮削要分成多个步骤
/// 1. 找到要整理的资源
/// 2. 本地识别要整理的资源
/// 3. 远程识别要整理的资源
/// 4. 转移文件
/// 5. 刮削要整理的资源
///
/// https://post.smzdm.com/p/aox8wp36/
/// https://emby.media/support/articles/Movie-Naming.html
/// https://support.emby.media/support/solutions/articles/44001159110-tv-naming
///
pub fn scrape(
    resource_type: ResourceType,
    transfer_type: TransferType,
    scrape_config: ScrapeConfig,
    src_directory: String,
    target_directory: String,
) {
    tracing::debug!(
        "scrape_src_to_target resource_type {:?}, transfer_mode {:?}, src_directory {:?}, target_directory {:?}",
        resource_type,
        transfer_type,
        src_directory,
        target_directory
    );

    let mut paths: Vec<String> = Vec::new();

    // 找到资源
    let mut callback = |path: String| {
        tracing::debug!("find path = {}", path);
        paths.push(path);
    };
    finder::find(&resource_type, &src_directory, &mut callback);

    tracing::info!(target:"soda::info", "资源数量: {}", paths.len());

    // paths按照父路径分组
    let mut paths_parent_group = HashMap::new();
    for ele in &paths {
        let parent_path = Path::new(&ele).parent().unwrap().to_str().unwrap().to_string();
        paths_parent_group.insert(parent_path, ele);
    }

    tracing::info!(target:"soda::info", "资源组数量: {}", paths_parent_group.len());

    // 缓存信息
    let mut mt_infos: HashMap<String, MTInfo> = HashMap::new();

    // 上下文
    let mut meta_context = MetaContext::new();

    // 识别资源
    for src_path in paths {
        match resource_type {
            ResourceType::MT => {
                // 初始化上下文
                if meta_context.init(&src_path) {
                    // 刮削
                    match scrape_mt(
                        &mut meta_context,
                        &mut mt_infos,
                        src_path,
                        &scrape_config,
                        &target_directory,
                        &transfer_type,
                    ) {
                        Ok(_) => {}
                        Err(e) => {
                            tracing::error!(target: "soda::info","刮削失败 e = {}", e);
                            tracing::error!("recognize_scrape_mt failed e = {}", e);
                            // scrape_context.error = true;
                        }
                    };
                }
            }
        }
    }
}

fn scrape_mt(
    meta_context: &mut MetaContext,
    mt_infos: &mut HashMap<String, MTInfo>,
    src_path: String,
    scrape_config: &ScrapeConfig,
    target_directory: &String,
    transfer_type: &TransferType,
) -> Result<(), SodaError> {
    tracing::info!(target:"soda::info", "开始刮削: {}", src_path);

    // 创建meta
    let mut mt_meta = meta::create_metadata_mt(meta_context)?;

    // 识别成功但是没有关键信息，那么取父目录的信息重新识别一次
    if mt_meta.title_cn.is_empty() && mt_meta.title_en.is_empty() {
        let root_src_path = Path::new(&src_path).parent().unwrap().parent().unwrap();
        let root_path = root_src_path.to_str().unwrap().to_string();
        meta_context.init(&root_path);

        let mut root_mt_meta = meta::create_metadata_mt(meta_context)?;

        if root_mt_meta.title_cn.is_empty() && root_mt_meta.title_en.is_empty() {
            return Err(SodaError::Str("title_cn and title_en is empty"));
        }

        mt_meta.title_cn = root_mt_meta.title_cn;
        mt_meta.title_en = root_mt_meta.title_en;
    }

    // 如果是电视剧，但是缺少季信息，那么默认为第一季
    if !mt_meta.episode.is_empty() && mt_meta.season.is_empty() {
        mt_meta.season = format!("S{:02}", 1);
        tracing::debug!("set season = {:?}", mt_meta.season);
    }

    name_mapping(&mut mt_meta);

    tracing::info!(target:"soda::info", "资源名解析成功: {}", serde_json::to_string(&mt_meta)?);

    if !scrape_config.enable_recognize {
        return Err(SodaError::Str("enable_recognize is false"));
    };

    let mut final_target_path = String::new();

    // 远程识别要整理的资源
    match tmdb::recognize_mt(mt_infos, &mut mt_meta) {
        Ok(mt_info_key) => {
            // 远程识别要整理的资源

            let mut mt_info = mt_infos.get_mut(&mt_info_key).ok_or(SodaError::Str("mt_info is empty"))?;

            tracing::info!(target:"soda::info", "tmdb_id = {}, tvdb_id = {}, imdb_id = {}, title = {}",mt_info.tmdb_id(),mt_info.tvdb_id().unwrap_or("0".to_string()),mt_info.imdb_id().unwrap_or("0"), mt_info.title());

            // 更新媒体图片
            if scrape_config.enable_scrape_image {
                fanart::obtain_images(&mut mt_info);
            }

            // 选择重命名格式
            let rename_style = LIB_CONFIG.lock().unwrap().rename_style.clone();

            // 选择重命名格式
            let rename_format = if mt_meta.is_movie() {
                LIB_CONFIG.lock().unwrap().transfer_rename_format_movie.clone()
            } else {
                LIB_CONFIG.lock().unwrap().transfer_rename_format_tv.clone()
            };

            // 生成转移文件路径
            let transfer_target_path = transfer::gen_mt_transfer_target_path(&target_directory, rename_style, &rename_format, &mt_meta);

            final_target_path = transfer_target_path.clone();

            // 转移文件
            transfer::transfer_mt_file(&mt_meta, &src_path, &transfer_target_path, &transfer_type)?;

            // 刮削信息
            if scrape_config.enable_scrape_write {
                scraper::scrape_metadata(scrape_config, &mt_meta, mt_info, &transfer_target_path);
            }
        }
        Err(error) => {
            tracing::error!(target: "soda::info","识别失败 e = {}", error);

            // 选择重命名格式
            let rename_style = LIB_CONFIG.lock().unwrap().rename_style.clone();

            // 选择重命名格式
            let rename_format_str = if mt_meta.is_movie() {
                LIB_CONFIG.lock().unwrap().transfer_rename_format_movie.clone()
            } else {
                LIB_CONFIG.lock().unwrap().transfer_rename_format_tv.clone()
            };

            // 选择重命名格式
            let rename_format = if mt_meta.is_movie() {
                LIB_CONFIG.lock().unwrap().transfer_rename_format_movie.clone()
            } else {
                LIB_CONFIG.lock().unwrap().transfer_rename_format_tv.clone()
            };

            // 生成转移文件路径
            let transfer_target_path = transfer::gen_mt_transfer_target_path(&target_directory, rename_style, &rename_format, &mt_meta);

            final_target_path = transfer_target_path.clone();

            // 转移文件
            transfer::transfer_mt_file(&mt_meta, &src_path, &transfer_target_path, &transfer_type)?;
        }
    }

    tracing::debug!("scrape_mt success mt_meta = {:?}", mt_meta);

    tracing::info!(target:"soda::info", "刮削结束: src={} target={}", src_path, final_target_path);

    return Ok(());
}

/// 处理名字映射，有些中文名或者英文名不准确，需要映射
fn name_mapping(mt_meta: &mut MTMetadata) {
    // 处理名字映射
    for name in &NAMES_MAP.names {
        tracing::debug!("name_mapping name = {}", serde_json::to_string(&name).unwrap());
        tracing::debug!(
            "name_mapping mt_meta.title_en = {:?} mt_meta.title_cn = {:?} mt.meta.year = {:?}",
            mt_meta.title_en,
            mt_meta.title_en,
            mt_meta.year
        );
        //
        if !name.src.release_year.is_empty() && name.src.release_year == mt_meta.year {
            if !name.src.title_cn.is_empty() && name.src.title_cn == mt_meta.title_cn {
                if !name.target.title_cn.is_empty() {
                    mt_meta.title_cn = name.target.title_cn.clone();
                    tracing::debug!(
                        "name_mapping 1 name.src.title_cn = {}, mt_meta.title_cn = {}",
                        name.src.title_cn,
                        mt_meta.title_cn
                    );
                }
                if !name.target.title_en.is_empty() {
                    mt_meta.title_en = name.target.title_en.clone();
                    tracing::debug!(
                        "name_mapping 2 name.src.title_en = {}, mt_meta.title_en = {}",
                        name.src.title_en,
                        mt_meta.title_en
                    );
                }
            }

            //
            if !name.src.title_en.is_empty() && name.src.title_en == mt_meta.title_en {
                if !name.target.title_cn.is_empty() {
                    mt_meta.title_cn = name.target.title_cn.clone();
                    tracing::debug!(
                        "name_mapping 1 name.src.title_cn = {}, mt_meta.title_cn = {}",
                        name.src.title_cn,
                        mt_meta.title_cn
                    );
                }
                if !name.target.title_en.is_empty() {
                    mt_meta.title_en = name.target.title_en.clone();
                    tracing::debug!(
                        "name_mapping 2 name.src.title_en = {}, mt_meta.title_en = {}",
                        name.src.title_en,
                        mt_meta.title_en
                    );
                }
            }
        } else {
            // 映射英文名
            if !name.src.title_en.is_empty() && name.src.title_en == mt_meta.title_en && !name.target.title_en.is_empty() {
                mt_meta.title_en = name.target.title_en.clone();
                tracing::debug!(
                    "name_mapping 3 name.src.title_en = {}, mt_meta.title_en = {}",
                    name.src.title_en,
                    mt_meta.title_en
                );
            }

            // 映射中文名
            if !name.src.title_cn.is_empty() && name.src.title_cn == mt_meta.title_cn && !name.target.title_cn.is_empty() {
                mt_meta.title_cn = name.target.title_cn.clone();
                tracing::debug!(
                    "name_mapping 4 name.src.title_cn = {}, mt_meta.title_cn = {}",
                    name.src.title_cn,
                    mt_meta.title_cn
                );
            }
        }
    }
}

static NAMES_MAP: Lazy<NamesMap> = Lazy::new(|| {
    let config = LIB_CONFIG.lock().unwrap();

    let content = if !config.strong_match_name_map.is_empty() {
        config.strong_match_name_map.clone()
    } else if !config.strong_match_name_map_path.is_empty() {
        fs::read_to_string(config.strong_match_name_map_path.as_str()).unwrap()
    } else {
        unreachable!("strong_match_name_map and strong_match_name_map_path is empty")
    };

    serde_json::from_str(&content).unwrap()
});
