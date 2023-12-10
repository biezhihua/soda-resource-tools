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
pub(crate) fn search_tv(name: &str, year: Option<&str>, first_release_year: Option<&str>, include_adult: Option<bool>, page: usize) -> Result<Vec<Value>, SodaError> {
    let action = "/search/tv";

    let mut params = format!("query={}&page={}", encode(name), page);

    first_release_year.is_some_then(|value| params.push_str(&format!("&first_air_date_year={}", value)));

    year.is_some_then(|value| params.push_str(&format!("&year={}", value)));

    include_adult.is_some_then(|value| params.push_str(&format!("&include_adult={}", value)));

    // 使用英文搜索更准确
    params.push_str("&language=en-US");

    match request::tmdb_request(action, &params, "GET") {
        Ok(response) => Result::Ok(response.get("results").ok_or("get results error")?.as_array().unwrap().clone()),
        Err(e) => Result::Err(SodaError::String(format!("search_tv error {:?}", e))),
    }
}
