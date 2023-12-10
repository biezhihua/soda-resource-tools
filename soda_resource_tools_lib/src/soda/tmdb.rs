use once_cell::sync::Lazy;
use regex::Regex;
use serde::de::value;
use serde_json::Value;
use tracing::info;

use crate::soda::entity::{MTMetadata, MTType};
use crate::soda::extension_option::OptionExtensions;

use self::entity::{TmdbEpisode, TmdbSeason, TmdbTV};

use super::entity::{MTInfo, SodaError};

pub(crate) mod entity;
mod request;
pub(crate) mod scraper;
mod search;
mod tv;

/// # TMDB图片地址，无需修改需保留默认值，如果默认地址连通性不好可以尝试修改为：`static-mdb.v.geilijiasu.com`
/// TMDB_IMAGE_DOMAIN=image.tmdb.org
/// # TMDB API地址，无需修改需保留默认值，也可配置为`api.tmdb.org`或其它中转代理服务地址，能连通即可
/// TMDB_API_DOMAIN=api.themoviedb.org
///

fn get_info(tmdb_id: &str) {}

fn get_movie_detail(tmdb_id: &str) {}

pub fn search_tv_by_season(title_cn: &str, title_en: &str, season_year: &str) -> Result<Value, SodaError> {
    tracing::info!("search tv title_cn = {:?} title_en = {:?} season_year = {}", title_cn, title_en, season_year);

    // search tv 优先使用英文标题
    let title = if title_en.is_empty() { title_cn } else { title_en };

    let tvs = search::search_tv(title, Some(season_year), None, None, 1)?;

    return find_tv_by_name(&tvs, title);
}

/// 根据名称搜索电视剧获取基础信息
fn find_tv_by_name(tvs: &Vec<Value>, name: &str) -> Result<Value, SodaError> {
    for tv in tvs {
        static TV_NAME_REPLACE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[ \.-]").unwrap());

        let id = tv.get("id").ok_or("get id error")?.as_i64().unwrap();
        let name = name.to_lowercase();
        let name = TV_NAME_REPLACE_REGEX.replace_all(&name, "");
        let tv_name = tv.get("name").ok_or("get name error")?.as_str().unwrap().to_lowercase();
        let tv_name = TV_NAME_REPLACE_REGEX.replace_all(&tv_name, "");
        let tv_original_name = tv.get("original_name").ok_or("get original_name error")?.as_str().unwrap().to_lowercase();
        let tv_original_name = TV_NAME_REPLACE_REGEX.replace_all(&tv_original_name, "");
        if tv_name == name || tv_original_name == name {
            return Result::Ok(tv.clone());
        }
    }
    return Err(SodaError::String(format!("find tv on tvs failed name = {} tvs = {:?}", name, tvs)));
}

pub fn search_tv_by_name(name: &str) -> Result<Value, SodaError> {
    tracing::info!("search tv name = {:?}", name);

    let tvs = search::search_tv(name, None, None, None, 1)?;

    return find_tv_by_name(&tvs, name);
}

pub(crate) fn search_movie(name: &str, year: &str, season: &str) -> Option<Value> {
    if name.is_empty() {
        return None;
    }
    return None;
}

/// 识别影视资源
pub(crate) fn recognize_mt(meta: &mut MTMetadata) -> Option<MTInfo> {
    let mut mt_info: Option<MTInfo> = None;

    if meta.is_tv() {
        // 根据名称搜索电视剧获取基础信息
        let mut tv_value = match &meta.year {
            None => search_tv_by_name(meta.title()),
            Some(year) => search_tv_by_season(&meta.title_cn, &meta.title_en, year),
        };

        // 根据TMDBID获取详细信息
        match tv_value {
            Ok(value) => {
                if let Some(value) = value.get("id") {
                    let tmdb_id = value.as_i64().unwrap().to_string();
                    // 获取电视剧详细信息
                    match tmdb_tv_details(&tmdb_id) {
                        Ok(tv_detail) => mt_info = Some(MTInfo::new(tv_detail)),
                        Err(e) => {
                            tracing::error!("search tv detail failed, name = {:?} error = {:?}", meta.title(), e);
                        }
                    }
                } else {
                    tracing::error!("get tv id failed, name = {:?}", meta.title());
                }
            }
            Err(e) => {
                tracing::error!("search tv failed, name = {:?} error = {:?}", meta.title(), e);
            }
        }
    } else if meta.is_movie() {
        // search_movie(meta.title(), &meta.year.clone().unwrap(), &meta.season);
    }

    mt_info.is_some_mut_then(|info| {
        meta.merge(info);

        if let Some(season_number) = meta.season_number() {
            // 获取电视剧季的详细信息
            match tmdb_tv_season_detail(info.tmdb_id(), season_number) {
                Ok(season_detail) => info.insert_tv_season(season_number, season_detail),
                Err(e) => tracing::error!("search tv season detail failed, name = {:?} season = {:?} error = {:?}", meta.title(), season_number, e),
            }

            // 获取电视剧集的详细信息
            if let Some(episode_number) = meta.episode_number() {
                match tmdb_tv_season_episode_detail(info.tmdb_id(), season_number, episode_number) {
                    Ok(episode_detail) => info.insert_tv_season_episode(season_number, episode_number, episode_detail),
                    Err(e) => tracing::error!("search tv season episode detail failed, name = {:?} season = {:?} episode = {:?} error = {:?}", meta.title(), season_number, episode_number, e),
                }
            } else {
                tracing::error!("episode is empty");
            }
        } else {
            tracing::error!("season is empty");
        }
    });

    return mt_info;
}

/// 查询电视剧集的详细信息
fn tmdb_tv_season_episode_detail(tmdb_id: i64, season_number: i64, episode_number: i64) -> Result<TmdbEpisode, SodaError> {
    tracing::info!("search tv tmdb_id = {:?} season = {:?} episode = {:?}", tmdb_id, season_number, episode_number);
    return tv::tv_season_episode_detail(tmdb_id, season_number, episode_number);
}

/// 查询电视剧季的详细信息
fn tmdb_tv_season_detail(tmdb_id: i64, season_number: i64) -> Result<TmdbSeason, SodaError> {
    tracing::info!("search tv tmdb_id = {:?} season = {:?}", tmdb_id, season_number);
    return tv::tv_season_detail(tmdb_id, season_number);
}

/// 查询电视剧详细信息
fn tmdb_tv_details(tmdb_id: &str) -> Result<TmdbTV, SodaError> {
    tracing::info!("search tv tmdb_id = {:?}", tmdb_id);
    return tv::tv_details(tmdb_id);
}

#[cfg(test)]
mod tmdb_tests {

    use super::*;
    use crate::soda::*;
    use tracing_subscriber::fmt::format::FmtSpan;
    use tracing_subscriber::EnvFilter;

    #[test]
    fn test_recognize_mt_1() {
        init_tracing();
        let mut metadata = meta::mt_metadata("凡人修仙传.The.Mortal.Ascention.2020.S01E01.2160p.WEB-DL.H.264.AAC-OurTV.mp4").unwrap();
        let value = tmdb::recognize_mt(&mut metadata).unwrap();
        assert_eq!("凡人修仙传", value.original_title());
    }

    /// 初始化日志配置
    fn init_tracing() {
        tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env().add_directive("soda=info".parse().unwrap())).with_span_events(FmtSpan::FULL).init();
    }
}
