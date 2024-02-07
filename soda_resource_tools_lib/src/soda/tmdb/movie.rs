use std::f32::consts::E;

use reqwest::Response;
use serde_json::Value;

use crate::soda::entity::SodaError;

use super::{
    entity::{TmdbEpisode, TmdbMovie, TmdbSeason, TmdbTV},
    request,
};

/// https://developer.themoviedb.org/reference/movie-details
/// https://api.themoviedb.org/3/movie/{movie_id}
/// ```json
/// {
///     "adult": false,
///     "backdrop_path": "/4HodYYKEIsGOdinkGi2Ucz6X9i0.jpg",
///     "belongs_to_collection": {
///       "id": 573436,
///       "name": "蜘蛛侠：平行宇宙（系列）",
///       "poster_path": "/eD4bGQNfmqExIAzKdvX5gDHhI2.jpg",
///       "backdrop_path": "/oC5S5pdfzPiQC4us05uDMT7v5Ng.jpg"
///     },
///     "budget": 100000000,
///     "genres": [
///       {
///         "id": 16,
///         "name": "动画"
///       },
///       {
///         "id": 28,
///         "name": "动作"
///       },
///       {
///         "id": 12,
///         "name": "冒险"
///       },
///       {
///         "id": 878,
///         "name": "科幻"
///       }
///     ],
///     "homepage": "",
///     "id": 569094,
///     "imdb_id": "tt9362722",
///     "original_language": "en",
///     "original_title": "Spider-Man: Across the Spider-Verse",
///     "overview": "讲述了新生代蜘蛛侠迈尔斯（沙梅克·摩尔 Shameik Moore 配音）携手蜘蛛格温（海莉·斯坦菲尔德 Hailee Steinfeld 配音），穿越多元宇宙踏上更宏大的冒险征程的故事。面临每个蜘蛛侠都会失去至亲的宿命，迈尔斯誓言打破命运魔咒，找到属于自己的英雄之路。而这个决定和蜘蛛侠2099（奥斯卡·伊萨克 Oscar Is aac 配音）所领军的蜘蛛联盟产生了极大冲突，一场以一敌百的蜘蛛侠大内战即将拉响！",
///     "popularity": 357.459,
///     "poster_path": "/jBmJZDiJ2h6DimGROpxWlPh1xIo.jpg",
///     "production_companies": [
///       {
///         "id": 5,
///         "logo_path": "/71BqEFAF4V3qjjMPCpLuyJFB9A.png",
///         "name": "Columbia Pictures",
///         "origin_country": "US"
///       },
///       {
///         "id": 2251,
///         "logo_path": "/5ilV5mH3gxTEU7p5wjxptHvXkyr.png",
///         "name": "Sony Pictures Animation",
///         "origin_country": "US"
///       },
///       {
///         "id": 77973,
///         "logo_path": "/9y5lW86HnxKUZOFencYk3TIIRCM.png",
///         "name": "Lord Miller",
///         "origin_country": "US"
///       },
///       {
///         "id": 84041,
///         "logo_path": "/nw4kyc29QRpNtFbdsBHkRSFavvt.png",
///         "name": "Pascal Pictures",
///         "origin_country": "US"
///       },
///       {
///         "id": 14439,
///         "logo_path": null,
///         "name": "Arad Productions",
///         "origin_country": "US"
///       },
///       {
///         "id": 7505,
///         "logo_path": "/837VMM4wOkODc1idNxGT0KQJlej.png",
///         "name": "Marvel Entertainment",
///         "origin_country": "US"
///       }
///     ],
///     "production_countries": [
///       {
///         "iso_3166_1": "US",
///         "name": "United States of America"
///       }
///     ],
///     "release_date": "2023-05-31",
///     "revenue": 690500000,
///     "runtime": 140,
///     "spoken_languages": [
///       {
///         "english_name": "English",
///         "iso_639_1": "en",
///         "name": "English"
///       },
///       {
///         "english_name": "Hindi",
///         "iso_639_1": "hi",
///         "name": "हिन्दी"
///       },
///       {
///         "english_name": "Italian",
///         "iso_639_1": "it",
///         "name": "Italiano"
///       },
///       {
///         "english_name": "Spanish",
///         "iso_639_1": "es",
///         "name": "Español"
///       }
///     ],
///     "status": "Released",
///     "tagline": "",
///     "title": "蜘蛛侠：纵横宇宙",
///     "video": false,
///     "vote_average": 8.376,
///     "vote_count": 5554,
///     "credits": {
///       "cast": [
///         {
///           "adult": false,
///           "gender": 2,
///           "id": 587506,
///           "known_for_department": "Acting",
///           "name": "Shameik Moore",
///           "original_name": "Shameik Moore",
///           "popularity": 22.61,
///           "profile_path": "/uJNaSTsfBOvtFWsPP23zNthknsB.jpg",
///           "cast_id": 705,
///           "character": "Miles Morales / Spider-Man (voice)",
///           "credit_id": "6489a4f8e375c000e251ab48",
///           "order": 0
///         }
///       ]
///     },
///     "external_ids": {
///       "imdb_id": "tt9362722",
///       "wikidata_id": "Q76448600",
///       "facebook_id": "SpiderVerseMovie",
///       "instagram_id": "spiderversemovie",
///       "twitter_id": "SpiderVerse"
///     }
/// }
/// ```
pub(crate) fn movie_details(tmdb_id: &str) -> Result<TmdbMovie, SodaError> {
    let action = format!("/movie/{}", tmdb_id);

    let params = format!("append_to_response={}", "credits,external_ids");

    let response = request::tmdb_request(&action, &params, "GET")?;

    match serde_json::from_value(response) {
        Ok(value) => Ok(value),
        Err(e) => Err(SodaError::String(format!("movie_details json error {:?}", e))),
    }
}
