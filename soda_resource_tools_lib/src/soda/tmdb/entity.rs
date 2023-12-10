use std::collections::HashMap;

use crate::soda::fanart::entity::FanartTV;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TmdbSeasonInfo {
    pub tv_season: TmdbSeason,
    pub tv_episodes: HashMap<i64, TmdbEpisode>,
}
impl TmdbSeasonInfo {
    pub(crate) fn new(tmdb_season: TmdbSeason) -> TmdbSeasonInfo {
        return TmdbSeasonInfo { tv_season: tmdb_season, tv_episodes: HashMap::new() };
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TmdbTVInfo {
    pub tv: TmdbTV,
    pub fanart_tv: Option<FanartTV>,
    pub tv_seasons: HashMap<i64, TmdbSeasonInfo>,
}

impl TmdbTVInfo {
    pub(crate) fn new(tmdb_tv: TmdbTV) -> TmdbTVInfo {
        return TmdbTVInfo { tv: tmdb_tv, tv_seasons: HashMap::new(), fanart_tv: None };
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TmdbTV {
    pub backdrop_path: Option<String>,
    pub first_air_date: Option<String>,
    pub genres: Option<Vec<TmdbGenre>>,
    pub id: i64,
    pub name: Option<String>,
    pub original_language: Option<String>,
    pub original_name: Option<String>,
    pub overview: Option<String>,
    pub popularity: Option<f64>,
    pub poster_path: Option<String>,
    pub seasons: Option<Vec<TmdbSeason>>,
    pub vote_average: Option<f64>,
    pub vote_count: Option<i64>,
    pub credits: Option<TmdbCredits>,
    pub external_ids: Option<TmdbExternalIds>,
}
impl TmdbTV {
    pub(crate) fn new(tmdb_info: Value) -> TmdbTV {
        let result: Result<TmdbTV, serde_json::Error> = serde_json::from_value(tmdb_info);
        match result {
            Ok(result) => {
                return result;
            }
            Err(e) => {
                // 打印错误信息
                if e.is_data() {
                    tracing::error!("TmdbTV new 数据类型错误 {}", e);
                } else if e.is_syntax() {
                    tracing::error!("TmdbTV new 语法错误 {}", e);
                } else if e.is_io() {
                    tracing::error!("TmdbTV new IO 错误 {}", e);
                } else if e.is_eof() {
                    tracing::error!("TmdbTV new 意外的文件结束 {}", e);
                }
                unreachable!("解析失败");
            }
        }
    }

    pub(crate) fn actors(&self) -> Option<Vec<&TmdbCast>> {
        let mut ret = Vec::new();
        match &self.credits {
            Some(credits) => match &credits.cast {
                Some(casts) => {
                    for cast in casts {
                        match cast.known_for_department() {
                            "Acting" => {
                                ret.push(cast);
                            }
                            _ => {}
                        }
                    }
                }
                None => return None,
            },
            None => return None,
        }
        return Some(ret);
    }

    pub(crate) fn directors(&self) -> Option<Vec<&TmdbCrew>> {
        let mut ret = Vec::new();
        match &self.credits {
            Some(credits) => match &credits.crew {
                Some(crews) => {
                    for crew in crews {
                        let job = crew.job();
                        if job == "Director" || job == "Writer" || job == "Editor" || job == "Producer" {
                            ret.push(crew);
                        }
                    }
                }
                None => return None,
            },
            None => return None,
        }
        return Some(ret);
    }

    pub(crate) fn year(&self) -> &str {
        &(self.first_air_date()[0..4])
    }

    pub(crate) fn tmdb_id(&self) -> String {
        self.id.to_string()
    }

    pub(crate) fn tvdb_id(&self) -> Option<String> {
        if let Some(external_ids) = &self.external_ids {
            return Some(external_ids.tvdb_id().to_string());
        }
        return None;
    }

    pub(crate) fn imdb_id(&self) -> Option<&str> {
        if let Some(external_ids) = &self.external_ids {
            return Some(external_ids.imdb_id());
        }
        return None;
    }

    pub(crate) fn overview(&self) -> &str {
        match &self.overview {
            Some(overview) => overview.as_str(),
            None => "",
        }
    }

    pub(crate) fn name(&self) -> &str {
        match &self.name {
            Some(name) => name.as_str(),
            None => "",
        }
    }

    pub(crate) fn original_language(&self) -> &str {
        match &self.original_language {
            Some(original_language) => original_language.as_str(),
            None => "",
        }
    }

    pub(crate) fn first_air_date(&self) -> &str {
        match &self.first_air_date {
            Some(first_air_date) => first_air_date.as_str(),
            None => "",
        }
    }

    pub(crate) fn poster_path(&self) -> &str {
        match &self.poster_path {
            Some(poster_path) => poster_path.as_str(),
            None => "",
        }
    }

    pub(crate) fn backdrop_path(&self) -> &str {
        match &self.backdrop_path {
            Some(backdrop_path) => backdrop_path.as_str(),
            None => "",
        }
    }

    pub(crate) fn original_name(&self) -> &str {
        match &self.original_name {
            Some(original_name) => original_name.as_str(),
            None => "",
        }
    }

    pub(crate) fn vote_average(&self) -> String {
        match &self.vote_average {
            Some(vote_average) => vote_average.to_string(),
            None => "".to_string(),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TmdbExternalIds {
    pub imdb_id: Option<String>,
    pub tvdb_id: Option<i64>,
}
impl TmdbExternalIds {
    pub(crate) fn imdb_id(&self) -> &str {
        match &self.imdb_id {
            Some(imdb_id) => imdb_id.as_str(),
            None => "",
        }
    }

    pub(crate) fn tvdb_id(&self) -> i64 {
        match &self.tvdb_id {
            Some(tvdb_id) => tvdb_id.clone(),
            None => 0,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TmdbGenre {
    pub id: Option<i64>,
    pub name: Option<String>,
}
impl TmdbGenre {
    pub(crate) fn name(&self) -> &str {
        match &self.name {
            Some(name) => name.as_str(),
            None => "",
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TmdbCredits {
    pub cast: Option<Vec<TmdbCast>>,
    pub crew: Option<Vec<TmdbCrew>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TmdbCrew {
    pub adult: Option<bool>,
    pub gender: Option<i64>,
    pub id: Option<i64>,
    pub known_for_department: Option<String>,
    pub name: Option<String>,
    pub original_name: Option<String>,
    pub popularity: Option<f64>,
    pub profile_path: Option<String>,
    pub credit_id: Option<String>,
    pub department: Option<String>,
    pub job: Option<String>,
}
impl TmdbCrew {
    fn job(&self) -> &str {
        match &self.job {
            Some(job) => job.as_str(),
            None => "",
        }
    }

    pub(crate) fn name(&self) -> &str {
        match &self.name {
            Some(name) => name.as_str(),
            None => "",
        }
    }

    pub(crate) fn id(&self) -> String {
        match &self.id {
            Some(id) => id.to_string(),
            None => "".to_string(),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TmdbCast {
    pub adult: Option<bool>,
    pub gender: Option<i64>,
    pub id: Option<i64>,
    pub known_for_department: Option<String>,
    pub name: Option<String>,
    pub original_name: Option<String>,
    pub popularity: Option<f64>,
    pub profile_path: Option<String>,
    pub character: Option<String>,
    pub credit_id: Option<String>,
    pub order: Option<i64>,
}

impl TmdbCast {
    pub(crate) fn thumb(&self) -> String {
        match &self.profile_path {
            Some(profile_path) => format!("https://www.themoviedb.org/t/p/w138_and_h175_face{}", profile_path),
            None => "".to_string(),
        }
    }

    pub(crate) fn profile(&self) -> String {
        match &self.id {
            Some(id) => format!("https://www.themoviedb.org/person/{}", id.to_string()),
            None => "".to_string(),
        }
    }

    fn known_for_department(&self) -> &str {
        match &self.known_for_department {
            Some(known_for_department) => known_for_department.as_str(),
            None => "",
        }
    }

    pub(crate) fn id(&self) -> String {
        match &self.id {
            Some(id) => id.to_string(),
            None => "".to_string(),
        }
    }

    pub(crate) fn character(&self) -> &str {
        match &self.character {
            Some(character) => character.as_str(),
            None => "",
        }
    }

    pub(crate) fn name(&self) -> &str {
        match &self.name {
            Some(name) => name.as_str(),
            None => "",
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TmdbEpisode {
    pub air_date: Option<String>,
    pub episode_number: Option<i64>,
    pub episode_type: Option<String>,
    pub id: Option<i64>,
    pub name: Option<String>,
    pub overview: Option<String>,
    pub production_code: Option<String>,
    pub runtime: Option<i64>,
    pub season_number: Option<i64>,
    pub show_id: Option<i64>,
    pub still_path: Option<String>,
    pub vote_average: Option<f64>,
    pub vote_count: Option<i64>,
    pub credits: Option<TmdbCredits>,
}
impl TmdbEpisode {
    pub(crate) fn tmdb_id_str(&self) -> String {
        match &self.id {
            Some(id) => id.to_string(),
            None => "".to_string(),
        }
    }

    pub(crate) fn title(&self) -> &str {
        match &self.name {
            Some(name) => name.as_str(),
            None => "",
        }
    }

    pub(crate) fn overview(&self) -> &str {
        match &self.overview {
            Some(overview) => overview.as_str(),
            None => "",
        }
    }

    pub(crate) fn air_date(&self) -> &str {
        match &self.air_date {
            Some(air_date) => air_date.as_str(),
            None => "",
        }
    }

    pub(crate) fn year(&self) -> &str {
        match &self.air_date {
            Some(air_date) => &air_date[0..4],
            None => "",
        }
    }

    pub(crate) fn vote_average(&self) -> f64 {
        match &self.vote_average {
            Some(vote_average) => *vote_average,
            None => 0.0,
        }
    }

    fn episode_number(&self) -> i64 {
        match &self.episode_number {
            Some(episode_number) => *episode_number,
            None => 0,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TmdbSeason {
    pub id: Option<i64>,
    pub air_date: Option<String>,
    pub episodes: Option<Vec<TmdbEpisode>>,
    pub name: Option<String>,
    pub overview: Option<String>,
    pub poster_path: Option<String>,
    pub season_number: Option<i64>,
    pub vote_average: Option<f64>,
}
impl TmdbSeason {
    pub(crate) fn overview(&self) -> &str {
        match &self.overview {
            Some(overview) => overview.as_str(),
            None => "",
        }
    }

    pub(crate) fn title(&self) -> String {
        match &self.season_number {
            Some(season_number) => format!("第 {} 季", season_number.to_string()).to_string(),
            None => "".to_string(),
        }
    }

    pub(crate) fn air_date(&self) -> &str {
        match &self.air_date {
            Some(air_date) => air_date.as_str(),
            None => "",
        }
    }

    pub(crate) fn year(&self) -> &str {
        match &self.air_date {
            Some(air_date) => &air_date[0..4],
            None => "",
        }
    }

    pub(crate) fn season_number(&self) -> String {
        match &self.season_number {
            Some(season_number) => season_number.to_string(),
            None => "".to_string(),
        }
    }
}
