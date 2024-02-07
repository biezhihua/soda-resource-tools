use serde_json::Value;
use urlencoding::encode;

use crate::soda::{entity::SodaError, extension_option::OptionExtensions, tmdb::request};

// success
// curl --request GET \
//      --url 'https://api.themoviedb.org/3/search/tv?query=%E5%87%A1%E4%BA%BA%E4%BF%AE%E4%BB%99%E4%BC%A0&include_adult=false&language=en-US&page=1' \
//      --header 'Authorization: Bearer eyJhbGciOiJIUzI1NiJ9.eyJhdWQiOiI2ZjViOTZkMGQ3MjUzMTE3YzQ0OTYzYTBjZThhYTZmMiIsInN1YiI6IjVjYmQzOTlmYzNhMzY4MTM2OTg1ZmM4ZSIsInNjb3BlcyI6WyJhcGlfcmVhZCJdLCJ2ZXJzaW9uIjoxfQ.0ogVgNoOHpOswqSpl5Zg-_zxWEvAL2CO1TwIerbRbeo' \
//      --header 'accept: application/json'
// {
//   "page": 1,
//   "results": [
//     {
//       "adult": false,
//       "backdrop_path": "/xurwNaNCr38iKqEwtFVTw3dQM1h.jpg",
//       "genre_ids": [
//         16,
//         10765
//       ],
//       "id": 106449,
//       "origin_country": [
//         "CN"
//       ],
//       "original_language": "zh",
//       "original_name": "凡人修仙传",
//       "overview": "A poor and ordinary boy from a village joins a minor sect in Jiang Hu and becomes an Unofficial Disciple by chance. How will Han Li, a commoner by birth, establish a foothold for himself in his sect? With his mediocre aptitude, he must successfully traverse the treacherous path of cultivation and avoid the notice of those who may do him harm. This is a story of an ordinary mortal who, against all odds, clashes with devilish demons and ancient celestials in order to find his own path towards immortality.",
//       "popularity": 68.575,
//       "poster_path": "/9bDt042m2IaDfpSTEyvhPWNlTY3.jpg",
//       "first_air_date": "2020-07-25",
//       "name": "A Record of a Mortal's Journey to Immortality",
//       "vote_average": 8.9,
//       "vote_count": 23
//     }
//   ],
//   "total_pages": 1,
//   "total_results": 1
// }
// error
// curl --request GET \
//      --url 'https://api.themoviedb.org/3/search/tv?include_adult=false&language=en-US&page=1' \
//      --header 'Authorization: Bearer eyJhbGciOiJIUzI1NiJ9.eyJhdWQiOiI2ZjViOTZkMGQ3MjUzMTE3YzQ0OTYzYTBjZThhYTZmMiIsInN1YiI6IjVjYmQzOTlmYzNhMzY4MTM2OTg1ZmM4ZSIsInNjb3BlcyI6WyJhcGlfcmVhZCJdLCJ2ZXJzaW9uIjoxfQ.0ogVgNoOHpOswqSpl5Zg-_zxWEvAL2CO1TwIerbRbeo' \
//      --header 'accept: application/json'
// {
//   "page": 1,
//   "results": [],
//   "total_pages": 1,
//   "total_results": 0
// }
/// https://developer.themoviedb.org/reference/search-tv
pub(crate) fn search_tv_with_language(
    name: &str,
    year: Option<&str>,
    first_release_year: Option<&str>,
    include_adult: Option<bool>,
    page: usize,
    language: &str,
) -> Result<Vec<Value>, SodaError> {
    let action = "/search/tv";

    let mut params = format!("query={}&page={}", encode(name), page);

    first_release_year.is_some_then(|value| params.push_str(&format!("&first_air_date_year={}", value)));

    year.is_some_then(|value| params.push_str(&format!("&year={}", value)));

    include_adult.is_some_then(|value| params.push_str(&format!("&include_adult={}", value)));

    // 使用英文搜索更准确
    params.push_str(&format!("&language={}", language));

    match request::tmdb_request(action, &params, "GET") {
        Ok(response) => Result::Ok(response.get("results").ok_or("get results error")?.as_array().unwrap().clone()),
        Err(e) => {
            tracing::error!("search_tv_with_language error {:?}", e);
            Result::Err(SodaError::String(format!("search_tv_with_language error {:?}", e)))
        }
    }
}

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
pub(crate) fn search_movie_with_language(name: &str, year: Option<&str>, language: &str) -> Result<Vec<Value>, SodaError> {
    let action = "/search/movie";

    let mut params = format!("query={}&page={}", encode(name), 1);

    year.is_some_then(|value| params.push_str(&format!("&year={}", value)));

    // 使用英文搜索更准确
    params.push_str(&format!("&language={}", language));

    match request::tmdb_request(action, &params, "GET") {
        Ok(response) => Result::Ok(response.get("results").ok_or("get results error")?.as_array().unwrap().clone()),
        Err(e) => {
            tracing::error!("search_movie_with_language error {:?}", e);
            Result::Err(SodaError::String(format!("search_movie_with_language error {:?}", e)))
        }
    }
}

#[cfg(test)]
mod search_test {
    use super::search_movie_with_language;

    #[test]
    fn test_search_movie_with_language() {
        let value = search_movie_with_language("Spider Man Across the Spider Verse", Some("2023"), "en-US").unwrap();
    }
}
