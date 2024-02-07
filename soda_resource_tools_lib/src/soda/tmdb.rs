use std::collections::HashMap;
use std::f32::consts::E;
use std::fs;

use once_cell::sync::Lazy;
use regex::Regex;
use serde::de::value;
use serde::Deserialize;
use serde_json::Value;
use tracing::debug;

use crate::soda::entity::{MTMetadata, MTType};
use crate::soda::extension_option::OptionExtensions;

use self::entity::{TmdbEpisode, TmdbMovie, TmdbSeason, TmdbTV};

use super::entity::{MTInfo, SodaError};
use super::LIB_CONFIG;

pub(crate) mod entity;
mod movie;
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

pub fn search_tv_by_name(title_cn: &str, title_en: &str) -> Result<Value, SodaError> {
    tracing::debug!("search_tv_by_name title_cn = {:?} title_en = {:?} ", title_cn, title_en);

    let mut season_title = "";

    // 先使用英文标题搜索
    if !title_en.is_empty() {
        season_title = title_en;
        if let Ok(tv) = find_tv_by_language(season_title, "", title_cn, title_en, "en-US") {
            return Ok(tv);
        }
        if let Ok(tv) = find_tv_by_language(season_title, "", title_cn, title_en, "zh-CN") {
            return Ok(tv);
        }
    }

    // 再使用中文标题搜索
    if !title_cn.is_empty() {
        season_title = title_cn;
        // 非第一季不使用年份搜索
        if let Ok(tv) = find_tv_by_language(season_title, "", title_cn, title_en, "en-US") {
            return Ok(tv);
        }
        if let Ok(tv) = find_tv_by_language(season_title, "", title_cn, title_en, "zh-CN") {
            return Ok(tv);
        }
    }
    return Err(SodaError::Str("search_tv_by_name not found"));
}

pub fn search_tv_by_season(title_cn: &str, title_en: &str, season_number: i64, season_year: &str) -> Result<Value, SodaError> {
    tracing::debug!(
        "search_tv_by_season title_cn = {:?} title_en = {:?} season_number = {:?} season_year = {:?}",
        title_cn,
        title_en,
        season_number,
        season_year
    );

    let mut season_title = "";

    // 先使用英文标题搜索
    if !title_en.is_empty() {
        season_title = title_en;
        if season_number == 1 {
            // 第一季使用年份准确搜索
            if let Ok(tv) = find_tv_by_language(season_title, season_year, title_cn, title_en, "en-US") {
                return Ok(tv);
            }
            if let Ok(tv) = find_tv_by_language(season_title, season_year, title_cn, title_en, "zh-CN") {
                return Ok(tv);
            }
        } else {
            // 非第一季不使用年份搜索
            if let Ok(tv) = find_tv_by_language(season_title, "", title_cn, title_en, "en-US") {
                return Ok(tv);
            }
            if let Ok(tv) = find_tv_by_language(season_title, "", title_cn, title_en, "zh-CN") {
                return Ok(tv);
            }
        }
    }

    // 再使用中文标题搜索
    if !title_cn.is_empty() {
        season_title = title_cn;
        if season_number == 1 {
            // 第一季使用年份准确搜索
            if let Ok(tv) = find_tv_by_language(season_title, season_year, title_cn, title_en, "en-US") {
                return Ok(tv);
            }
            if let Ok(tv) = find_tv_by_language(season_title, season_year, title_cn, title_en, "zh-CN") {
                return Ok(tv);
            }
        } else {
            // 非第一季不使用年份搜索
            if let Ok(tv) = find_tv_by_language(season_title, "", title_cn, title_en, "en-US") {
                return Ok(tv);
            }
            if let Ok(tv) = find_tv_by_language(season_title, "", title_cn, title_en, "zh-CN") {
                return Ok(tv);
            }
        }
    }

    return Err(SodaError::Str("search_tv_by_season not found"));
}

fn find_tv_by_language(title: &str, season_year: &str, title_cn: &str, title_en: &str, language: &str) -> Result<Value, SodaError> {
    // 先使用年份搜索
    let mut tvs = search::search_tv_with_language(title, None, Some(season_year), None, 1, language)?;

    // 如果没有搜索到结果则移除年份再次搜索
    if tvs.len() == 0 {
        tvs = search::search_tv_with_language(title, None, None, None, 1, language)?;
        return find_tv_by_name_and_year(&tvs, title_cn, title_en, "");
    }
    // 如果搜索结果只有一个，那么直接使用这个作为查询结果
    else if tvs.len() == 1 {
        return Ok(tvs.get(0).ok_or("get tv error")?.clone());
    }
    // 如果搜索到结果则使用年份搜索
    else {
        return find_tv_by_name_and_year(&tvs, title_cn, title_en, season_year);
    }
}

pub fn search_movie(title_cn: &str, title_en: &str, year: Option<&str>) -> Result<Value, SodaError> {
    let mut title = "";

    // 先使用英文标题搜索
    if !title_en.is_empty() {
        title = title_en;
        if let Ok(tv) = find_movie_by_language(title, year, title_cn, title_en, "en-US") {
            return Ok(tv);
        }
        if let Ok(tv) = find_movie_by_language(title, year, title_cn, title_en, "zh-CN") {
            return Ok(tv);
        }
    }

    // 再使用中文标题搜索
    if !title_cn.is_empty() {
        title = title_cn;
        if let Ok(tv) = find_movie_by_language(title, year, title_cn, title_en, "en-US") {
            return Ok(tv);
        }
        if let Ok(tv) = find_movie_by_language(title, year, title_cn, title_en, "zh-CN") {
            return Ok(tv);
        }
    }

    return Err(SodaError::Str("search_movie not found"));
}

fn find_movie_by_language(title: &str, year: Option<&str>, title_cn: &str, title_en: &str, language: &str) -> Result<Value, SodaError> {
    // 先使用年份搜索
    let mut movies = search::search_movie_with_language(title, year, language)?;

    // 如果没有搜索到结果则移除年份再次搜索
    if movies.len() == 0 {
        movies = search::search_tv_with_language(title, None, None, None, 1, language)?;
        return find_movie_by_name_and_year(&movies, title_cn, title_en, None);
    }
    // 如果搜索结果只有一个，那么直接使用这个作为查询结果
    else if movies.len() == 1 {
        return Ok(movies.get(0).ok_or("get movies error")?.clone());
    }
    // 如果搜索到结果则使用年份搜索
    else {
        return find_movie_by_name_and_year(&movies, title_cn, title_en, year);
    }
}

/// 根据名称搜索电影获取基础信息
///
// {
// "page": 1,
// "results": [
//     {
//     "adult": false,
//     "backdrop_path": "/4HodYYKEIsGOdinkGi2Ucz6X9i0.jpg",
//     "genre_ids": [
//         16,
//         28,
//         12,
//         878
//     ],
//     "id": 569094,
//     "original_language": "en",
//     "original_title": "Spider-Man: Across the Spider-Verse",
//     "overview": "讲述了新生代蜘蛛侠迈尔斯（沙梅克·摩尔 Shameik Moore 配音）携手蜘蛛格温（海莉·斯坦菲尔德 Hailee Steinfeld 配音），穿越多元宇宙踏上更宏大的冒险征程的故事。面临每个蜘蛛侠都会失去至亲的宿命，迈尔斯誓言打破命运魔咒，找到属于自己的英雄之路。而这个决定和蜘蛛侠2099（奥斯卡·伊萨克 Oscar Is aac 配音）所领军的蜘蛛联盟产生了极大冲突，一场以一敌百的蜘蛛侠大内战即将拉响！",
//     "popularity": 357.459,
//     "poster_path": "/jBmJZDiJ2h6DimGROpxWlPh1xIo.jpg",
//     "release_date": "2023-05-31",
//     "title": "蜘蛛侠：纵横宇宙",
//     "video": false,
//     "vote_average": 8.376,
//     "vote_count": 5548
//     }
// ],
// "total_pages": 1,
// "total_results": 1
// }
// curl --request GET \
//      --url 'https://api.themoviedb.org/3/search/movie?query=Spider%20Man%20Across%20the%20Spider%20Verse&include_adult=false&language=zh-CN&page=1&year=2023' \
//      --header 'Authorization: Bearer eyJhbGciOiJIUzI1NiJ9.eyJhdWQiOiI2ZjViOTZkMGQ3MjUzMTE3YzQ0OTYzYTBjZThhYTZmMiIsInN1YiI6IjVjYmQzOTlmYzNhMzY4MTM2OTg1ZmM4ZSIsInNjb3BlcyI6WyJhcGlfcmVhZCJdLCJ2ZXJzaW9uIjoxfQ.0ogVgNoOHpOswqSpl5Zg-_zxWEvAL2CO1TwIerbRbeo' \
//      --header 'accept: application/json'
fn find_movie_by_name_and_year(movies: &Vec<Value>, title_cn: &str, title_en: &str, year: Option<&str>) -> Result<Value, SodaError> {
    tracing::debug!(
        "find_movie_by_name_and_year title_cn = {:?} title_en = {:?} year = {:?}",
        title_cn,
        title_en,
        year
    );

    static MOVIE_NAME_REPLACE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[ ,\.\-'\?:\|]").unwrap());

    let title_cn = title_cn.to_lowercase();
    let title_cn = MOVIE_NAME_REPLACE_REGEX.replace_all(&title_cn, "");

    let title_en = title_en.to_lowercase();
    let title_en = MOVIE_NAME_REPLACE_REGEX.replace_all(&title_en, "");

    // 如果有年份则根据年份搜索
    if let Some(year) = year {
        for movie in movies {
            tracing::debug!("find_movie_by_name_and_year movie = {:?}", movie);

            let id = movie.get("id").ok_or("get id error")?.as_i64().unwrap();

            let movie_name = movie.get("title").ok_or("get name error")?.as_str().unwrap().to_lowercase();
            let movie_name = MOVIE_NAME_REPLACE_REGEX.replace_all(&movie_name, "");
            let movie_original_title = movie
                .get("original_title")
                .ok_or("get original_title error")?
                .as_str()
                .unwrap()
                .to_lowercase();
            let movie_original_title = MOVIE_NAME_REPLACE_REGEX.replace_all(&movie_original_title, "");

            let is_name = movie_name == title_cn || movie_original_title == title_cn || movie_name == title_en || movie_original_title == title_en;

            let release_year = movie.get("release_date").ok_or("get release_date error")?.as_str().unwrap().to_string();
            if release_year.len() > 4 {
                let release_year = (release_year[0..4]).to_string();
                if release_year == year && is_name {
                    return Result::Ok(movie.clone());
                }
            }
        }
    }

    // 根据名称搜索
    for movie in movies {
        let id = movie.get("id").ok_or("get id error")?.as_i64().unwrap();

        tracing::debug!("find_movie_by_name_and_year movie = {:?}", movie);

        let movie_name: String = movie.get("title").ok_or("get name error")?.as_str().unwrap().to_lowercase();
        let movie_name = MOVIE_NAME_REPLACE_REGEX.replace_all(&movie_name, "");
        let movie_original_title = movie
            .get("original_title")
            .ok_or("get original_title error")?
            .as_str()
            .unwrap()
            .to_lowercase();
        let movie_original_title = MOVIE_NAME_REPLACE_REGEX.replace_all(&movie_original_title, "");

        tracing::debug!(
            "find_movie_by_name_and_year title_cn = {:?} title_en = {:?} movie_name = {:?} movie_original_title = {:?}",
            title_cn,
            title_en,
            movie_name,
            movie_original_title
        );

        let is_name = movie_name == title_cn || movie_original_title == title_cn || movie_name == title_en || movie_original_title == title_en;
        if is_name {
            return Result::Ok(movie.clone());
        }
    }

    return Err(SodaError::String(format!(
        "find movie on movies failed tvs = {}",
        serde_json::to_string(&movies).unwrap()
    )));
}

/// 根据名称搜索电视剧获取基础信息
///
fn find_tv_by_name_and_year(tvs: &Vec<Value>, title_cn: &str, title_en: &str, first_release_year: &str) -> Result<Value, SodaError> {
    static TV_NAME_REPLACE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[ ,\.\-'\?:\|]").unwrap());

    let title_cn = title_cn.to_lowercase();
    let title_cn = TV_NAME_REPLACE_REGEX.replace_all(&title_cn, "");

    let title_en = title_en.to_lowercase();
    let title_en = TV_NAME_REPLACE_REGEX.replace_all(&title_en, "");

    // 如果有年份则根据年份搜索
    if !first_release_year.is_empty() {
        for tv in tvs {
            let id = tv.get("id").ok_or("get id error")?.as_i64().unwrap();

            let tv_name = tv.get("name").ok_or("get name error")?.as_str().unwrap().to_lowercase();
            let tv_name = TV_NAME_REPLACE_REGEX.replace_all(&tv_name, "");
            let tv_original_name = tv.get("original_name").ok_or("get original_name error")?.as_str().unwrap().to_lowercase();
            let tv_original_name = TV_NAME_REPLACE_REGEX.replace_all(&tv_original_name, "");

            let is_name = tv_name == title_cn || tv_original_name == title_cn || tv_name == title_en || tv_original_name == title_en;

            let release_year = tv.get("first_air_date").ok_or("get first_air_date error")?.as_str().unwrap().to_string();
            if release_year.len() > 4 {
                let release_year = (release_year[0..4]).to_string();
                if release_year == first_release_year && is_name {
                    return Result::Ok(tv.clone());
                }
            }
        }
    }

    // 根据名称搜索
    for tv in tvs {
        let id = tv.get("id").ok_or("get id error")?.as_i64().unwrap();

        let tv_name = tv.get("name").ok_or("get name error")?.as_str().unwrap().to_lowercase();
        let tv_name = TV_NAME_REPLACE_REGEX.replace_all(&tv_name, "");
        let tv_original_name = tv.get("original_name").ok_or("get original_name error")?.as_str().unwrap().to_lowercase();
        let tv_original_name = TV_NAME_REPLACE_REGEX.replace_all(&tv_original_name, "");

        let is_name = tv_name == title_cn || tv_original_name == title_cn || tv_name == title_en || tv_original_name == title_en;
        if is_name {
            return Result::Ok(tv.clone());
        }
    }

    return Err(SodaError::String(format!(
        "find tv on tvs failed tvs = {}",
        serde_json::to_string(&tvs).unwrap()
    )));
}

/// 识别影视资源
pub(crate) fn recognize_mt(mt_infos: &mut HashMap<String, MTInfo>, mt_meta: &mut MTMetadata) -> Result<String, SodaError> {
    if mt_meta.is_tv() {
        return recognize_mt_tv(mt_infos, mt_meta);
    } else if mt_meta.is_movie() {
        return recognize_mt_movie(mt_infos, mt_meta);
    }
    return Err(SodaError::Str("not support recognize mt"));
}

fn recognize_mt_movie(mt_infos: &mut HashMap<String, MTInfo>, mt_meta: &mut MTMetadata) -> Result<String, SodaError> {
    // 缓存信息
    let key: String = format!("{}-{}", &mt_meta.title_en, &mt_meta.title_cn);

    let mt_info = if let Some(cache_mt_info) = mt_infos.get_mut(&key) {
        tracing::debug!("recognize_mt_movie cache mt_info = {:?}", cache_mt_info);
        cache_mt_info
    } else {
        // 根据名称搜索电视剧获取基础信息
        let mut movie_value = if !mt_meta.year.is_empty() {
            // 按按照AKA查询 as known as
            if !mt_meta.aka_title_en_first.is_empty() && !mt_meta.aka_title_en_second.is_empty() {
                if let Ok(ret) = search_movie(&mt_meta.title_cn, &mt_meta.aka_title_en_first, Some(&mt_meta.year)) {
                    Ok(ret)
                } else {
                    search_movie(&mt_meta.title_cn, &mt_meta.aka_title_en_second, Some(&mt_meta.year))
                }
            }
            // 再按照普通名字查询
            else {
                search_movie(&mt_meta.title_cn, &mt_meta.title_en, Some(&mt_meta.year))
            }
        } else {
            search_movie(&mt_meta.title_cn, &mt_meta.title_en, None)
        }?;

        // 根据TMDBID获取电视剧详细信息
        let id_value = movie_value
            .get("id")
            .ok_or(SodaError::String(format!("get movie id failed, name = {:?}", mt_meta.title())))?;
        let tmdb_id = id_value.as_i64().unwrap().to_string();
        let mut mt_info = MTInfo::new_movie(tmdb_movie_details(&tmdb_id)?);
        mt_infos.insert(key.clone(), mt_info);

        mt_infos.get_mut(&key).unwrap()
    };

    // 合并元数据
    mt_meta.merge_movie(mt_info);

    return Ok(key);
}

/// 识别影视资源
pub(crate) fn recognize_mt_tv(mt_infos: &mut HashMap<String, MTInfo>, mt_meta: &mut MTMetadata) -> Result<String, SodaError> {
    // 缓存信息
    let key: String = format!("{}-{}", &mt_meta.title_en, &mt_meta.title_cn);

    let mt_info = if let Some(cache_mt_info) = mt_infos.get_mut(&key) {
        tracing::debug!("recognize_mt_tv cache mt_info = {:?}", cache_mt_info);
        cache_mt_info
    } else {
        // 根据名称搜索电视剧获取基础信息
        let mut tv_value = if !mt_meta.year.is_empty() && !mt_meta.season.is_empty() {
            let season_number = mt_meta.season_number().unwrap();
            search_tv_by_season(&mt_meta.title_cn, &mt_meta.title_en, season_number, &mt_meta.year)
        } else {
            search_tv_by_name(&mt_meta.title_cn, &mt_meta.title_en)
        }?;

        // 根据TMDBID获取电视剧详细信息
        let id_value = tv_value
            .get("id")
            .ok_or(SodaError::String(format!("get tv id failed, name = {:?}", mt_meta.title())))?;
        let tmdb_id = id_value.as_i64().unwrap().to_string();
        let mut mt_info = MTInfo::new_tv(tmdb_tv_details(&tmdb_id)?);
        mt_infos.insert(key.clone(), mt_info);

        mt_infos.get_mut(&key).unwrap()
    };

    // 合并元数据
    mt_meta.merge_tv(mt_info);

    // 获取电视剧季的详细信息
    let season_number = mt_meta.season_number().ok_or(SodaError::Str("season is empty"))?;
    let season_detail = tmdb_tv_season_detail(mt_info.tmdb_id(), season_number)?;
    mt_meta.merge_season(&season_detail);
    mt_info.insert_tv_season(season_number, season_detail);

    // 获取电视剧集的详细信息
    let episode_number = mt_meta.episode_number().ok_or(SodaError::Str("episode is empty"))?;
    let episode_detail = tmdb_tv_season_episode_detail(mt_info.tmdb_id(), season_number, episode_number)?;
    mt_info.insert_tv_season_episode(season_number, episode_number, episode_detail);

    return Ok(key);
}

/// 查询电视剧集的详细信息
fn tmdb_tv_season_episode_detail(tmdb_id: i64, season_number: i64, episode_number: i64) -> Result<TmdbEpisode, SodaError> {
    tracing::debug!(
        "tmdb_tv_season_episode_detail tmdb_id = {:?} season = {:?} episode = {:?}",
        tmdb_id,
        season_number,
        episode_number
    );
    return tv::tv_season_episode_detail(tmdb_id, season_number, episode_number);
}

/// 查询电视剧季的详细信息
fn tmdb_tv_season_detail(tmdb_id: i64, season_number: i64) -> Result<TmdbSeason, SodaError> {
    tracing::debug!("tmdb_tv_season_detail tmdb_id = {:?} season = {:?}", tmdb_id, season_number);
    return tv::tv_season_detail(tmdb_id, season_number);
}

/// 查询电视剧详细信息
fn tmdb_tv_details(tmdb_id: &str) -> Result<TmdbTV, SodaError> {
    tracing::debug!("tmdb_tv_details tmdb_id = {:?}", tmdb_id);
    return tv::tv_details(tmdb_id);
}

/// 查询电影详细信息
fn tmdb_movie_details(tmdb_id: &str) -> Result<TmdbMovie, SodaError> {
    tracing::debug!("tmdb_movie_details tmdb_id = {:?}", tmdb_id);
    return movie::movie_details(tmdb_id);
}

#[cfg(test)]
mod tmdb_tests {}
