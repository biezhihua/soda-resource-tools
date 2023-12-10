use self::token::Tokens;
use crate::soda::entity::MTMetadata;
use crate::soda::global::REGEX_MT_EXT;
use crate::soda::utils;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::time::SystemTime;

use super::LIB_CONFIG;

pub(crate) mod strong_match_token;
mod token;

/// 识别影音媒体信息
pub(crate) fn mt_metadata(path: &str) -> Option<MTMetadata> {
    tracing::info!("path = {:?}", path);
    let title = Path::new(path).file_name()?.to_str()?.to_string();
    let start_time = SystemTime::now();
    if let Some(tokens) = token::build_tokens(&title) {
        if let Some(meta) = gen_mt_metadata(&title, tokens) {
            let end_time = SystemTime::now();
            tracing::info!("title = {:?} time = {:?}", title, (end_time.duration_since(start_time)).unwrap());
            return Some(meta);
        }
    }
    tracing::error!("mt_metadata failed, path {:?}", path);
    utils::write_meta_file_path(&path);
    None
}

/// 生成影音媒体元数据
fn gen_mt_metadata(title: &str, tokens: Tokens) -> Option<MTMetadata> {
    let tokens = tokens.tokens;
    if !tokens.is_empty() {
        let mut metadata = MTMetadata::empty(title);

        if let Some(title_cn) = tokens.get("title_cn") {
            metadata.title_cn = title_cn.clone();
        }

        if let Some(title_en) = tokens.get("title_en") {
            metadata.title_en = title_en.clone();
        }

        if let Some(format) = tokens.get("format") {
            metadata.extension = format.clone();
        }

        if let Some(video_codec) = tokens.get("video_codec") {
            metadata.video_codec = video_codec.clone();
        }

        if let Some(source) = tokens.get("source") {
            metadata.source = source.clone();
        }

        if let Some(resolution) = tokens.get("resolution") {
            metadata.resolution = resolution.clone();
        }

        if let Some(year) = tokens.get("year") {
            metadata.year = Some(year.clone());
        }

        if let Some(season) = tokens.get("season") {
            metadata.season = season.clone();
        }

        if let Some(episode) = tokens.get("episode") {
            metadata.episode = episode.clone();
        }

        if let Some(audio_codec) = tokens.get("audio_codec") {
            metadata.audio_codec = audio_codec.clone();
        }

        if let Some(release_group) = tokens.get("release_group") {
            metadata.release_group = release_group.clone();
        }

        if let Some(special) = tokens.get("special") {
            metadata.special = special.clone();

            let config = LIB_CONFIG.lock().unwrap();
            if config.metadata_skip_special {
                tracing::error!("skip special = {:?}", special);
                return None;
            }
        }

        return Some(metadata);
    }
    None
}
