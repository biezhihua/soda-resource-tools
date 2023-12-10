use std::f32::consts::E;

use reqwest::Response;
use serde_json::Value;

use crate::soda::entity::SodaError;

use super::{
    entity::{TmdbEpisode, TmdbSeason, TmdbTV},
    request,
};

/// https://developer.themoviedb.org/reference/tv-series-details
/// https://api.themoviedb.org/3/tv/{series_id}
/// ```json
/// {
///     "adult": false,
///     "backdrop_path": "/y78FlnxoM2o0Rwo70qNvjudcgDH.jpg",
///     "created_by": [
///       {
///         "id": 3388975,
///         "credit_id": "61e7a714083547001b07b8c5",
///         "name": "Wang Yu",
///         "gender": 2,
///         "profile_path": null
///       }
///     ],
///     "episode_run_time": [
///       19
///     ],
///     "first_air_date": "2020-07-25",
///     "genres": [
///       {
///         "id": 16,
///         "name": "动画"
///       },
///       {
///         "id": 10765,
///         "name": "Sci-Fi & Fantasy"
///       }
///     ],
///     "homepage": "https://www.bilibili.com/bangumi/media/md28223043",
///     "id": 106449,
///     "in_production": true,
///     "languages": [
///       "zh"
///     ],
///     "last_air_date": "2023-11-25",
///     "last_episode_to_air": {
///       "id": 4608892,
///       "name": "星海飞驰1",
///       "overview": "韩立结丹成功后，会了会新认识的邻居，拉拢一下朋友关系。暗地里，韩立带着曲魂去市场上出售手中的妖丹材料，换取自己所需的东西，想炼制本命法宝【青竹蜂云剑】。怎料遇上了文思月小姑娘，而且她看似有意想介绍韩立直接去跟妙音门的主人做交易。",
///       "vote_average": 0,
///       "vote_count": 0,
///       "air_date": "2023-11-25",
///       "episode_number": 77,
///       "episode_type": "standard",
///       "production_code": "",
///       "runtime": null,
///       "season_number": 1,
///       "show_id": 106449,
///       "still_path": "/iCuKcnEKjFs9ml0k12X45pblhVy.jpg"
///     },
///     "name": "凡人修仙传",
///     "next_episode_to_air": {
///       "id": 4608893,
///       "name": "星海飞驰2",
///       "overview": "",
///       "vote_average": 0,
///       "vote_count": 0,
///       "air_date": "2023-12-02",
///       "episode_number": 78,
///       "episode_type": "standard",
///       "production_code": "",
///       "runtime": null,
///       "season_number": 1,
///       "show_id": 106449,
///       "still_path": null
///     },
///     "networks": [
///       {
///         "id": 1605,
///         "logo_path": "/mtmMg3PD4YGfrlmqpEiO6NL2ch9.png",
///         "name": "bilibili",
///         "origin_country": "CN"
///       }
///     ],
///     "number_of_episodes": 110,
///     "number_of_seasons": 1,
///     "origin_country": [
///       "CN"
///     ],
///     "original_language": "zh",
///     "original_name": "凡人修仙传",
///     "overview": "看机智的凡人小子韩立如何稳健发展、步步为营，战魔道、夺至宝、驰骋星海、快意恩仇，成为纵横三界的强者。他日仙界重相逢，一声道友尽沧桑。",
///     "popularity": 65.722,
///     "poster_path": "/q4wDrarB2SdYnVnMensJ9zL7vV3.jpg",
///     "production_companies": [
///       {
///         "id": 123270,
///         "logo_path": "/4KxfGr13VXMr44og11hzpdwfQDQ.png",
///         "name": "bilibili",
///         "origin_country": "CN"
///       },
///       {
///         "id": 71194,
///         "logo_path": null,
///         "name": "Original Force Animation",
///         "origin_country": "CN"
///       },
///       {
///         "id": 164932,
///         "logo_path": null,
///         "name": "万维猫动画",
///         "origin_country": ""
///       },
///       {
///         "id": 164933,
///         "logo_path": null,
///         "name": "猫片Mopi",
///         "origin_country": ""
///       }
///     ],
///     "production_countries": [
///       {
///         "iso_3166_1": "CN",
///         "name": "China"
///       }
///     ],
///     "seasons": [
///       {
///         "air_date": "2020-07-25",
///         "episode_count": 110,
///         "id": 157272,
///         "name": "第 1 季",
///         "overview": "平凡少年韩立出生贫困，为了让家人过上更好的生活，自愿前去七玄门参加入门考核，最终被墨大夫收入门下。墨大夫一开始对韩立悉心培养、传授医术，让韩立对他非常感激，但随着一同入门的弟子张铁失踪，韩立才发现了墨大夫的真面目。墨大夫试图夺舍韩立，最终却被韩立反杀。通过墨大夫的遗书韩立得知了一个全新世界：修仙界的存在。在帮助七玄门抵御外敌之后，韩立离开了七玄门，前去墨大夫的家中寻找暖阳宝玉解毒，并帮助墨家人打败了敌人。通过墨大夫之女墨彩环的口中得知太南小会地址，韩立为追寻修仙人的足迹决定前往太南小会，拜别家人后，韩立来到太南谷，结识了一众修仙人士，正式展开了自己的修仙之旅。",
///         "poster_path": "/uHJ91ht8ylNgmFWAnxJOG79rCtU.jpg",
///         "season_number": 1,
///         "vote_average": 10
///       }
///     ],
///     "spoken_languages": [
///       {
///         "english_name": "Mandarin",
///         "iso_639_1": "zh",
///         "name": "普通话"
///       }
///     ],
///     "status": "Returning Series",
///     "tagline": "",
///     "type": "Scripted",
///     "vote_average": 8.9,
///     "vote_count": 23,
///     "credits": {
///       "cast": [
///         {
///           "adult": false,
///           "gender": 2,
///           "id": 3082032,
///           "known_for_department": "Acting",
///           "name": "Wenqing Qian",
///           "original_name": "Wenqing Qian",
///           "popularity": 1.22,
///           "profile_path": "/42V5un2Vw23rfU4NWWi1VLox31D.jpg",
///           "character": "Han Li (voice)",
///           "credit_id": "60952c0ce2bca8003d189ed1",
///           "order": 0
///         },
///         {
///           "adult": false,
///           "gender": 1,
///           "id": 2164942,
///           "known_for_department": "Acting",
///           "name": "Ireine Song",
///           "original_name": "Ireine Song",
///           "popularity": 5.277,
///           "profile_path": "/q97r6UwDSuNeWk0eq3ddXKBCspz.jpg",
///           "character": "Han Yun Zhi (voice)",
///           "credit_id": "60952beb025764003f47d87b",
///           "order": 1
///         },
///         {
///           "adult": false,
///           "gender": 1,
///           "id": 2463716,
///           "known_for_department": "Acting",
///           "name": "Li Shimeng",
///           "original_name": "Li Shimeng",
///           "popularity": 5.274,
///           "profile_path": "/i7hKjBsuxiMHuj3v68yWhbLXNbi.jpg",
///           "character": "Nangong Wan (voice)",
///           "credit_id": "626658ec7fcab3116727f84d",
///           "order": 2
///         },
///         {
///           "adult": false,
///           "gender": 1,
///           "id": 2104118,
///           "known_for_department": "Acting",
///           "name": "Xu Jiaqi",
///           "original_name": "Xu Jiaqi",
///           "popularity": 1.286,
///           "profile_path": "/1CzUSgTj3NPZ2WkT2j9c1fFzJGL.jpg",
///           "character": "Dong Xuaner (voice)",
///           "credit_id": "626659d87caa472d77dae243",
///           "order": 3
///         }
///       ],
///       "crew": []
///     },
///     "external_ids": {
///       "imdb_id": "tt12879782",
///       "freebase_mid": null,
///       "freebase_id": null,
///       "tvdb_id": 384640,
///       "tvrage_id": null,
///       "wikidata_id": null,
///       "facebook_id": null,
///       "instagram_id": null,
///       "twitter_id": null
///     }
/// }
/// ```
pub(crate) fn tv_details(tmdb_id: &str) -> Result<TmdbTV, SodaError> {
    let action = format!("/tv/{}", tmdb_id);

    let params = format!("append_to_response={}", "credits,external_ids");

    let response = request::tmdb_request(&action, &params, "GET")?;

    match serde_json::from_value(response) {
        Ok(value) => Ok(value),
        Err(e) => Err(SodaError::String(format!("tv_details json error {:?}", e))),
    }
}

/// 查询某季的所有信信息
/// https://developer.themovie  db.org/reference/tv-season-details
/// https://api.themoviedb.org/3/tv/{series_id}/season/{season_number}
/// ```json
/// {
///     "_id": "5f1c2a7e0bb0760036ee5f2d",
///     "air_date": "2020-07-25",
///     "episodes": [
///         {
///             "air_date": "2020-07-25",
///             "episode_number": 1,
///             "episode_type": "standard",
///             "id": 2364606,
///             "name": "风起天南1：七玄门",
///             "overview": "韩立出生于贫困家庭，家中有父母和一个小妹。为减轻家里负担，自愿去参加七玄门的入门考核。路上结识到好友张铁，但是入门考核并不那么容易。",
///             "production_code": "",
///             "runtime": 20,
///             "season_number": 1,
///             "show_id": 106449,
///             "still_path": "/cdX94fHQcM2LaOROyvmFcTiGUcZ.jpg",
///             "vote_average": 10,
///             "vote_count": 2,
///             "crew": [],
///             "guest_stars": []
///         },
///         {
///             "air_date": null,
///             "episode_number": 110,
///             "episode_type": "standard",
///             "id": 4650767,
///             "name": "第 110 集",
///             "overview": "",
///             "production_code": "",
///             "runtime": null,
///             "season_number": 1,
///             "show_id": 106449,
///             "still_path": null,
///             "vote_average": 0,
///             "vote_count": 0,
///             "crew": [],
///             "guest_stars": []
///         }
///     ],
///     "name": "第 1 季",
///     "overview": "平凡少年韩立出生贫困，为了让家人过上更好的生活，自愿前去七玄门参加入门考核，最终被墨大夫收入门下。墨大夫一开始对韩立悉心培养、传授医术，让韩立对他非常感激，但随着一同入门的弟子张铁失踪，韩立才发现了墨大夫的真面目。墨大夫试图夺舍韩立，最终却被韩立反杀。通过墨大夫的遗书韩立得知了一个全新世界：修仙界的存在。在帮助七玄门抵御外敌之后，韩立离开了七玄门，前去墨大夫的家中寻找暖阳宝玉解毒，并帮助墨家人打败了敌人。通过墨大夫之女墨彩环的口中得知太南小会地址，韩立为追寻修仙人的足迹决定前往太南小会，拜别家人后，韩立来到太南谷，结识了一众修仙人士，正式展开了自己的修仙之旅。",
///     "id": 157272,
///     "poster_path": "/uHJ91ht8ylNgmFWAnxJOG79rCtU.jpg",
///     "season_number": 1,
///     "vote_average": 10,
///     "credits": {
///         "cast": [
///             {
///                 "adult": false,
///                 "gender": 2,
///                 "id": 3082032,
///                 "known_for_department": "Acting",
///                 "name": "Wenqing Qian",
///                 "original_name": "Wenqing Qian",
///                 "popularity": 1.22,
///                 "profile_path": "/42V5un2Vw23rfU4NWWi1VLox31D.jpg",
///                 "character": "Han Li (voice)",
///                 "credit_id": "60952c0ce2bca8003d189ed1",
///                 "order": 0
///             },
///             {
///                 "adult": false,
///                 "gender": 1,
///                 "id": 2164942,
///                 "known_for_department": "Acting",
///                 "name": "Ireine Song",
///                 "original_name": "Ireine Song",
///                 "popularity": 5.277,
///                 "profile_path": "/q97r6UwDSuNeWk0eq3ddXKBCspz.jpg",
///                 "character": "Han Yun Zhi (voice)",
///                 "credit_id": "60952beb025764003f47d87b",
///                 "order": 1
///             },
///             {
///                 "adult": false,
///                 "gender": 1,
///                 "id": 2463716,
///                 "known_for_department": "Acting",
///                 "name": "Li Shimeng",
///                 "original_name": "Li Shimeng",
///                 "popularity": 5.274,
///                 "profile_path": "/i7hKjBsuxiMHuj3v68yWhbLXNbi.jpg",
///                 "character": "Nangong Wan (voice)",
///                 "credit_id": "626658ec7fcab3116727f84d",
///                 "order": 2
///             },
///             {
///                 "adult": false,
///                 "gender": 1,
///                 "id": 2104118,
///                 "known_for_department": "Acting",
///                 "name": "Xu Jiaqi",
///                 "original_name": "Xu Jiaqi",
///                 "popularity": 1.286,
///                 "profile_path": "/1CzUSgTj3NPZ2WkT2j9c1fFzJGL.jpg",
///                 "character": "Dong Xuaner (voice)",
///                 "credit_id": "626659d87caa472d77dae243",
///                 "order": 3
///             }
///         ],
///         "crew": []
///     }
/// }
/// ```
pub(crate) fn tv_season_detail(tmdb_id: i64, season_number: i64) -> Result<TmdbSeason, SodaError> {
    let action = format!("/tv/{}/season/{}", tmdb_id, season_number);

    let params = format!("append_to_response={}", "credits");

    let response = request::tmdb_request(&action, &params, "GET")?;

    match serde_json::from_value(response) {
        Ok(value) => Ok(value),
        Err(e) => Err(SodaError::String(format!("tv_season_detail json error {:?}", e))),
    }
}

/// 查询集的详细信息
/// https://developer.themoviedb.org/reference/tv-episode-details
/// https://api.themoviedb.org/3/tv/{series_id}/season/{season_number}/episode/{episode_number}
/// ```json
/// {
///   "air_date": "2020-07-25",
///   "crew": [],
///   "episode_number": 1,
///   "guest_stars": [],
///   "name": "风起天南1：七玄门",
///   "overview": "韩立出生于贫困家庭，家中有父母和一个小妹。为减轻家里负担，自愿去参加七玄门的入门考核。路上结识到好友张铁，但是入门考核并不那么容易。",
///   "id": 2364606,
///   "production_code": "",
///   "runtime": 20,
///   "season_number": 1,
///   "still_path": "/cdX94fHQcM2LaOROyvmFcTiGUcZ.jpg",
///   "vote_average": 10,
///   "vote_count": 2,
///   "credits": {
///     "cast": [
///       {
///         "adult": false,
///         "gender": 2,
///         "id": 3082032,
///         "known_for_department": "Acting",
///         "name": "Wenqing Qian",
///         "original_name": "Wenqing Qian",
///         "popularity": 1.22,
///         "profile_path": "/42V5un2Vw23rfU4NWWi1VLox31D.jpg",
///         "character": "Han Li (voice)",
///         "credit_id": "60952c0ce2bca8003d189ed1",
///         "order": 0
///       }
///     ],
///     "crew": [],
///     "guest_stars": []
///   }
/// }
/// ```
pub(crate) fn tv_season_episode_detail(tmdb_id: i64, season_number: i64, episode_number: i64) -> Result<TmdbEpisode, SodaError> {
    let action = format!("/tv/{}/season/{}/episode/{}", tmdb_id, season_number, episode_number);

    let params = format!("append_to_response={}", "credits");

    let response = request::tmdb_request(&action, &params, "GET")?;

    match serde_json::from_value(response) {
        Ok(value) => Ok(value),
        Err(e) => Err(SodaError::String(format!("tv_season_episode_detail json error {:?}", e))),
    }
}
