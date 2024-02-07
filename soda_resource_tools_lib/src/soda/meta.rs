use crate::soda::entity::MTMetadata;
use crate::soda::global::REGEX_MT_EXT;
use crate::soda::utils;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::path::Path;
use std::time::SystemTime;

use super::entity::*;
use super::LIB_CONFIG;

pub(crate) mod strong_match_token;
mod token;

/// 识别影音媒体信息
pub fn create_metadata_mt(meta_context: &mut MetaContext) -> Result<MTMetadata, SodaError> {
    let cur_path = meta_context.cur_path.to_string();
    let cur_file_name = meta_context.cur_file_name.to_string();
    tracing::debug!("create metadata path = {:?}", cur_path);
    let start_time = SystemTime::now();
    if let Some(tokens) = token::tokens(meta_context) {
        if let Some(meta) = gen_metadata_mt(&cur_file_name, tokens) {
            let end_time = SystemTime::now();
            tracing::debug!("meta = {:?} ", meta);
            tracing::debug!(
                "create metadata success title = {:?} time = {:?}",
                cur_file_name,
                (end_time.duration_since(start_time)).unwrap()
            );
            return Ok(meta);
        }
    }

    tracing::error!(target:"soda::info", "识别媒体信息失败: {}", cur_path);
    tracing::error!(target:"soda::metadata", "识别媒体信息失败: {}", cur_path);
    return Err(SodaError::String(format!("create metadata failed, path = {:?}", cur_path)));
}

/// 生成影音媒体元数据
fn gen_metadata_mt(title: &str, token: Token) -> Option<MTMetadata> {
    let tokens = token.tokens;
    if !tokens.is_empty() {
        let mut meta = MTMetadata::empty(title);

        if let Some(value) = tokens.get(KEY_TITLE_CN) {
            meta.title_cn = value.clone();
        }

        if let Some(value) = tokens.get(KEY_TITLE_EN) {
            meta.title_en = value.clone();
        }

        if let Some(value) = tokens.get(KEY_AKA_TITLE_EN) {
            meta.aka_title_en = value.clone();
        }

        if let Some(value) = tokens.get(KEY_AKA_TITLE_EN_FIRST) {
            meta.aka_title_en_first = value.clone();
        }

        if let Some(value) = tokens.get(KEY_AKA_TITLE_EN_SECOND) {
            meta.aka_title_en_second = value.clone();
        }

        if let Some(value) = tokens.get(KEY_EXTENSION) {
            meta.extension = value.clone();
        }

        if let Some(value) = tokens.get(KEY_VIDEO_CODEC) {
            meta.video_codec = value.clone();
        }

        if let Some(value) = tokens.get(KEY_SOURCE) {
            meta.source = value.clone();
        }

        if let Some(value) = tokens.get(KEY_RESOLUTION) {
            meta.resolution = value.clone();
        }

        if let Some(value) = tokens.get(KEY_YEAR) {
            meta.year = value.clone();
        }

        if let Some(value) = tokens.get(KEY_SEASON) {
            meta.season = value.clone();
        }

        if let Some(value) = tokens.get(KEY_EPISODE) {
            meta.episode = value.clone();
        }

        if let Some(value) = tokens.get(KEY_AUDIO_CODEC) {
            meta.audio_codec = value.clone();
        }

        if let Some(value) = tokens.get(KEY_RELEASE_GROUP) {
            meta.release_group = value.clone();
        }

        if let Some(special) = tokens.get(KEY_SPECIAL) {
            meta.special = special.clone();

            let config = LIB_CONFIG.lock().unwrap();
            if config.metadata_skip_special {
                tracing::error!("skip special = {:?}", special);
                return None;
            }
        }
        return Some(meta);
    }
    None
}
