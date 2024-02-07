use serde_json::Value;

use self::entity::{FanartMovie, FanartTV};

use super::{
    entity::{MTInfo, SodaError},
    extension_option::OptionExtensions,
    extension_result::ResultExtensions,
    request::blocking_request_value_with_cache,
};

pub(crate) mod entity;

///  https:/// webservice.fanart.tv/v3/tv/384640?api_key=7b7fb929ae578432f178b94e75a6e663
///  ```json
///  {
///      "name": "A Record of a Mortal's Journey to Immortality",
///      "thetvdb_id": "384640",
///      "tvbanner": [
///          {
///              "id": "177328",
///              "url": "http://assets.fanart.tv/fanart/tv/384640/tvbanner/a-record-of-a-mortals-journey-to-immortality-64edc522e8e99.jpg",
///              "lang": "zh",
///              "likes": "3"
///          }
///      ],
///      "hdclearart": [
///          {
///              "id": "177350",
///              "url": "http://assets.fanart.tv/fanart/tv/384640/hdclearart/a-record-of-a-mortals-journey-to-immortality-64edd2671641b.png",
///              "lang": "zh",
///              "likes": "3"
///          },
///          {
///              "id": "177349",
///              "url": "http://assets.fanart.tv/fanart/tv/384640/hdclearart/a-record-of-a-mortals-journey-to-immortality-64edd263f3ad9.png",
///              "lang": "zh",
///              "likes": "3"
///          },
///          {
///              "id": "177348",
///              "url": "http://assets.fanart.tv/fanart/tv/384640/hdclearart/a-record-of-a-mortals-journey-to-immortality-64edd2593d29b.png",
///              "lang": "zh",
///              "likes": "2"
///          }
///      ],
///      "characterart": [
///          {
///              "id": "177347",
///              "url": "http://assets.fanart.tv/fanart/tv/384640/characterart/a-record-of-a-mortals-journey-to-immortality-64edd0301d1b6.png",
///              "lang": "00",
///              "likes": "3"
///          },
///          {
///              "id": "177346",
///              "url": "http://assets.fanart.tv/fanart/tv/384640/characterart/a-record-of-a-mortals-journey-to-immortality-64edd02d7574b.png",
///              "lang": "00",
///              "likes": "3"
///          },
///          {
///              "id": "177345",
///              "url": "http://assets.fanart.tv/fanart/tv/384640/characterart/a-record-of-a-mortals-journey-to-immortality-64edd02986d2c.png",
///              "lang": "00",
///              "likes": "3"
///          },
///          {
///              "id": "177344",
///              "url": "http://assets.fanart.tv/fanart/tv/384640/characterart/a-record-of-a-mortals-journey-to-immortality-64edd026ad9e5.png",
///              "lang": "00",
///              "likes": "3"
///          }
///      ],
///      "hdtvlogo": [
///          {
///              "id": "177342",
///              "url": "http://assets.fanart.tv/fanart/tv/384640/hdtvlogo/a-record-of-a-mortals-journey-to-immortality-64edc71454b04.png",
///              "lang": "zh",
///              "likes": "3"
///          },
///          {
///              "id": "177332",
///              "url": "http://assets.fanart.tv/fanart/tv/384640/hdtvlogo/a-record-of-a-mortals-journey-to-immortality-64edc544c01bd.png",
///              "lang": "zh",
///              "likes": "3"
///          }
///      ],
///      "tvthumb": [
///          {
///              "id": "177331",
///              "url": "http://assets.fanart.tv/fanart/tv/384640/tvthumb/a-record-of-a-mortals-journey-to-immortality-64edc535c458c.jpg",
///              "lang": "zh",
///              "likes": "3"
///          },
///          {
///              "id": "177330",
///              "url": "http://assets.fanart.tv/fanart/tv/384640/tvthumb/a-record-of-a-mortals-journey-to-immortality-64edc533d5e98.jpg",
///              "lang": "zh",
///              "likes": "3"
///          },
///          {
///              "id": "177329",
///              "url": "http://assets.fanart.tv/fanart/tv/384640/tvthumb/a-record-of-a-mortals-journey-to-immortality-64edc531da82e.jpg",
///              "lang": "zh",
///              "likes": "3"
///          }
///      ],
///      "showbackground": [
///          {
///              "id": "177327",
///              "url": "http://assets.fanart.tv/fanart/tv/384640/showbackground/a-record-of-a-mortals-journey-to-immortality-64edc5050d06f.jpg",
///              "lang": "",
///              "likes": "3",
///              "season": "all"
///          }
///      ],
///      "seasonposter": [
///          {
///              "id": "177335",
///              "url": "http://assets.fanart.tv/fanart/tv/384640/seasonposter/a-record-of-a-mortals-journey-to-immortality-64edc58a0f59a.jpg",
///              "lang": "zh",
///              "likes": "2",
///              "season": "0"
///          },
///          {
///              "id": "177336",
///              "url": "http://assets.fanart.tv/fanart/tv/384640/seasonposter/a-record-of-a-mortals-journey-to-immortality-64edc5b101a5e.jpg",
///              "lang": "zh",
///              "likes": "2",
///              "season": "1"
///          },
///          {
///              "id": "177337",
///              "url": "http://assets.fanart.tv/fanart/tv/384640/seasonposter/a-record-of-a-mortals-journey-to-immortality-64edc5cb463e7.jpg",
///              "lang": "zh",
///              "likes": "2",
///              "season": "2"
///          },
///          {
///              "id": "177338",
///              "url": "http://assets.fanart.tv/fanart/tv/384640/seasonposter/a-record-of-a-mortals-journey-to-immortality-64edc5ce1bf57.jpg",
///              "lang": "zh",
///              "likes": "2",
///              "season": "2"
///          },
///          {
///              "id": "177339",
///              "url": "http://assets.fanart.tv/fanart/tv/384640/seasonposter/a-record-of-a-mortals-journey-to-immortality-64edc5d0c7ffe.jpg",
///              "lang": "zh",
///              "likes": "2",
///              "season": "2"
///          },
///          {
///              "id": "177340",
///              "url": "http://assets.fanart.tv/fanart/tv/384640/seasonposter/a-record-of-a-mortals-journey-to-immortality-64edc5e7206f9.jpg",
///              "lang": "zh",
///              "likes": "2",
///              "season": "3"
///          },
///          {
///              "id": "177341",
///              "url": "http://assets.fanart.tv/fanart/tv/384640/seasonposter/a-record-of-a-mortals-journey-to-immortality-64edc5f892600.jpg",
///              "lang": "zh",
///              "likes": "2",
///              "season": "5"
///          }
///      ],
///      "tvposter": [
///          {
///              "id": "177334",
///              "url": "http://assets.fanart.tv/fanart/tv/384640/tvposter/a-record-of-a-mortals-journey-to-immortality-64edc577d8a33.jpg",
///              "lang": "zh",
///              "likes": "2"
///          },
///          {
///              "id": "177333",
///              "url": "http://assets.fanart.tv/fanart/tv/384640/tvposter/a-record-of-a-mortals-journey-to-immortality-64edc575441f4.jpg",
///              "lang": "zh",
///              "likes": "2"
///          }
///      ]
///  }
/// ```
pub(crate) fn obtain_images(mt_info: &mut MTInfo) {
    match mt_info {
        MTInfo::MOVIE(movie) => match movie {
            super::entity::MovieType::TMDB(info) => {
                let tmdb_id = info.movie.tmdb_id();
                if !tmdb_id.is_empty() {
                    match request_fanart_movies(&tmdb_id) {
                        Ok(fanart) => {
                            info.fanart = Some(fanart);
                        }
                        Err(e) => {
                            tracing::error!("request_fanart_movie error {:?}", e);
                        }
                    }
                }
            }
        },
        MTInfo::TV(tv) => match tv {
            super::entity::TVType::TMDB(info) => {
                if let Some(tvdb_id) = info.tv.tvdb_id() {
                    match request_fanart_tv(&tvdb_id) {
                        Ok(fanart) => {
                            info.fanart = Some(fanart);
                        }
                        Err(e) => {
                            tracing::error!("request_fanart_tv error {:?}", e);
                        }
                    }
                }
            }
        },
    }
}

fn get_api_key() -> String {
    "fe74657e137cca7ca521c12e44fd6292".to_string()
}

/// https://fanart.tv/
/// https://fanarttv.docs.apiary.io/#introduction/api_key-and-client_key
///_movie_url: str = f'https://webservice.fanart.tv/v3/movies/%s?api_key={settings.FANART_API_KEY}'
/// _tv_url: str = f'https://webservice.fanart.tv/v3/tv/%s?api_key={settings.FANART_API_KEY}'
/// https://webservice.fanart.tv/v3/tv/384640?api_key=7b7fb929ae578432f178b94e75a6e663
fn request_fanart_tv(tvdb_id: &str) -> Result<FanartTV, SodaError> {
    let url = format!("https://webservice.fanart.tv/v3/tv/{}?api_key={}", tvdb_id, get_api_key());
    let json = blocking_request_value_with_cache(super::cache::CacheType::TMDB, "GET", &url)?;

    match serde_json::from_value(json) {
        Ok(value) => Ok(value),
        Err(e) => Err(SodaError::Json(e)),
    }
}

/// https://fanarttv.docs.apiary.io/#reference/movies/get-movies/get-images-for-movie
/// https://webservice.fanart.tv/v3/movies/372058?api_key=7b7fb929ae578432f178b94e75a6e663
/// ```json
/// {
///     "name": "Your Name.",
///     "tmdb_id": "372058",
///     "imdb_id": "tt5311514",
///     "hdmovielogo": [
///         {
///             "id": "179358",
///             "url": "https://assets.fanart.tv/fanart/movies/372058/hdmovielogo/your-name-587d8cb5bef81.png",
///             "lang": "en",
///             "likes": "7"
///         },
///         {
///             "id": "130708",
///             "url": "https://assets.fanart.tv/fanart/movies/372058/hdmovielogo/your-name-566af0546b9d8.png",
///             "lang": "ja",
///             "likes": "2"
///         },
///         {
///             "id": "173213",
///             "url": "https://assets.fanart.tv/fanart/movies/372058/hdmovielogo/your-name-5845e9e8eed81.png",
///             "lang": "en",
///             "likes": "2"
///         },
///         {
///             "id": "173195",
///             "url": "https://assets.fanart.tv/fanart/movies/372058/hdmovielogo/your-name-5845db34263d4.png",
///             "lang": "en",
///             "likes": "2"
///         },
///         {
///             "id": "212924",
///             "url": "https://assets.fanart.tv/fanart/movies/372058/hdmovielogo/your-name-5a391e5c3871f.png",
///             "lang": "ru",
///             "likes": "1"
///         },
///         {
///             "id": "276373",
///             "url": "https://assets.fanart.tv/fanart/movies/372058/hdmovielogo/your-name-5dbd5727a99c8.png",
///             "lang": "ja",
///             "likes": "0"
///         },
///         {
///             "id": "373543",
///             "url": "https://assets.fanart.tv/fanart/movies/372058/hdmovielogo/your-name-62e91ca365271.png",
///             "lang": "zh",
///             "likes": "0"
///         }
///     ],
///     "moviebackground": [
///         {
///             "id": "173208",
///             "url": "https://assets.fanart.tv/fanart/movies/372058/moviebackground/your-name-5845e4f97e1b1.jpg",
///             "lang": "",
///             "likes": "4"
///         },
///         {
///             "id": "191798",
///             "url": "https://assets.fanart.tv/fanart/movies/372058/moviebackground/your-name-58f8e1b587811.jpg",
///             "lang": "",
///             "likes": "4"
///         },
///         {
///             "id": "130714",
///             "url": "https://assets.fanart.tv/fanart/movies/372058/moviebackground/your-name-566af6943c991.jpg",
///             "lang": "",
///             "likes": "3"
///         },
///         {
///             "id": "173241",
///             "url": "https://assets.fanart.tv/fanart/movies/372058/moviebackground/your-name-58461266f328d.jpg",
///             "lang": "",
///             "likes": "2"
///         },
///         {
///             "id": "173204",
///             "url": "https://assets.fanart.tv/fanart/movies/372058/moviebackground/your-name-5845e453ddf5f.jpg",
///             "lang": "",
///             "likes": "2"
///         },
///         {
///             "id": "173201",
///             "url": "https://assets.fanart.tv/fanart/movies/372058/moviebackground/your-name-5845e0146e532.jpg",
///             "lang": "",
///             "likes": "2"
///         },
///         {
///             "id": "130713",
///             "url": "https://assets.fanart.tv/fanart/movies/372058/moviebackground/your-name-566af4cd73697.jpg",
///             "lang": "",
///             "likes": "1"
///         },
///         {
///             "id": "173206",
///             "url": "https://assets.fanart.tv/fanart/movies/372058/moviebackground/your-name-5845e48055152.jpg",
///             "lang": "",
///             "likes": "1"
///         },
///         {
///             "id": "173207",
///             "url": "https://assets.fanart.tv/fanart/movies/372058/moviebackground/your-name-5845e4a0c6d41.jpg",
///             "lang": "",
///             "likes": "1"
///         }
///     ],
///     "movieposter": [
///         {
///             "id": "173198",
///             "url": "https://assets.fanart.tv/fanart/movies/372058/movieposter/your-name-5845df3ac3d16.jpg",
///             "lang": "00",
///             "likes": "4"
///         },
///         {
///             "id": "130709",
///             "url": "https://assets.fanart.tv/fanart/movies/372058/movieposter/your-name-566af06fcb6f5.jpg",
///             "lang": "ja",
///             "likes": "3"
///         },
///         {
///             "id": "173200",
///             "url": "https://assets.fanart.tv/fanart/movies/372058/movieposter/your-name-5845dfaf75329.jpg",
///             "lang": "en",
///             "likes": "3"
///         },
///         {
///             "id": "216544",
///             "url": "https://assets.fanart.tv/fanart/movies/372058/movieposter/your-name-5a6c6dd7aa9b0.jpg",
///             "lang": "en",
///             "likes": "3"
///         },
///         {
///             "id": "216542",
///             "url": "https://assets.fanart.tv/fanart/movies/372058/movieposter/your-name-5a6c6dbf2e633.jpg",
///             "lang": "en",
///             "likes": "2"
///         },
///         {
///             "id": "212110",
///             "url": "https://assets.fanart.tv/fanart/movies/372058/movieposter/your-name-5a2db632e8087.jpg",
///             "lang": "en",
///             "likes": "2"
///         },
///         {
///             "id": "216543",
///             "url": "https://assets.fanart.tv/fanart/movies/372058/movieposter/your-name-5a6c6dcbcc520.jpg",
///             "lang": "en",
///             "likes": "2"
///         },
///         {
///             "id": "216545",
///             "url": "https://assets.fanart.tv/fanart/movies/372058/movieposter/your-name-5a6c6de35a75a.jpg",
///             "lang": "en",
///             "likes": "2"
///         },
///         {
///             "id": "397624",
///             "url": "https://assets.fanart.tv/fanart/movies/372058/movieposter/your-name-648f55f3ed34e.jpg",
///             "lang": "00",
///             "likes": "2"
///         },
///         {
///             "id": "173199",
///             "url": "https://assets.fanart.tv/fanart/movies/372058/movieposter/your-name-5845df4683e35.jpg",
///             "lang": "ja",
///             "likes": "2"
///         },
///         {
///             "id": "212925",
///             "url": "https://assets.fanart.tv/fanart/movies/372058/movieposter/your-name-5a391e78462c3.jpg",
///             "lang": "ru",
///             "likes": "1"
///         },
///         {
///             "id": "130710",
///             "url": "https://assets.fanart.tv/fanart/movies/372058/movieposter/your-name-566af0a3b7870.jpg",
///             "lang": "ja",
///             "likes": "1"
///         },
///         {
///             "id": "212116",
///             "url": "https://assets.fanart.tv/fanart/movies/372058/movieposter/your-name-5a2dc19f656cf.jpg",
///             "lang": "en",
///             "likes": "1"
///         },
///         {
///             "id": "212109",
///             "url": "https://assets.fanart.tv/fanart/movies/372058/movieposter/your-name-5a2db6271f5b9.jpg",
///             "lang": "en",
///             "likes": "1"
///         },
///         {
///             "id": "212108",
///             "url": "https://assets.fanart.tv/fanart/movies/372058/movieposter/your-name-5a2db61a609f5.jpg",
///             "lang": "en",
///             "likes": "1"
///         },
///         {
///             "id": "173212",
///             "url": "https://assets.fanart.tv/fanart/movies/372058/movieposter/your-name-5845e8716ddd9.jpg",
///             "lang": "00",
///             "likes": "1"
///         },
///         {
///             "id": "303081",
///             "url": "https://assets.fanart.tv/fanart/movies/372058/movieposter/your-name-5efcd1142897c.jpg",
///             "lang": "ja",
///             "likes": "0"
///         }
///     ],
///     "moviedisc": [
///         {
///             "id": "210331",
///             "url": "https://assets.fanart.tv/fanart/movies/372058/moviedisc/your-name-5a11496732508.png",
///             "lang": "es",
///             "likes": "3",
///             "disc": "1",
///             "disc_type": "bluray"
///         },
///         {
///             "id": "210332",
///             "url": "https://assets.fanart.tv/fanart/movies/372058/moviedisc/your-name-5a1149794b5e2.png",
///             "lang": "en",
///             "likes": "2",
///             "disc": "1",
///             "disc_type": "bluray"
///         },
///         {
///             "id": "130779",
///             "url": "https://assets.fanart.tv/fanart/movies/372058/moviedisc/your-name-566b968bcf999.png",
///             "lang": "ja",
///             "likes": "2",
///             "disc": "1",
///             "disc_type": "bluray"
///         },
///         {
///             "id": "130778",
///             "url": "https://assets.fanart.tv/fanart/movies/372058/moviedisc/your-name-566b966fd82af.png",
///             "lang": "ja",
///             "likes": "1",
///             "disc": "1",
///             "disc_type": "bluray"
///         },
///         {
///             "id": "212114",
///             "url": "https://assets.fanart.tv/fanart/movies/372058/moviedisc/your-name-5a2db6715f8f0.png",
///             "lang": "en",
///             "likes": "1",
///             "disc": "1",
///             "disc_type": "bluray"
///         },
///         {
///             "id": "212113",
///             "url": "https://assets.fanart.tv/fanart/movies/372058/moviedisc/your-name-5a2db6659fd6a.png",
///             "lang": "en",
///             "likes": "1",
///             "disc": "1",
///             "disc_type": "bluray"
///         },
///         {
///             "id": "212112",
///             "url": "https://assets.fanart.tv/fanart/movies/372058/moviedisc/your-name-5a2db65a262f2.png",
///             "lang": "en",
///             "likes": "1",
///             "disc": "1",
///             "disc_type": "bluray"
///         },
///         {
///             "id": "212111",
///             "url": "https://assets.fanart.tv/fanart/movies/372058/moviedisc/your-name-5a2db643ddc80.png",
///             "lang": "en",
///             "likes": "1",
///             "disc": "1",
///             "disc_type": "bluray"
///         }
///     ],
///     "moviethumb": [
///         {
///             "id": "210334",
///             "url": "https://assets.fanart.tv/fanart/movies/372058/moviethumb/your-name-5a114afcd27c4.jpg",
///             "lang": "en",
///             "likes": "2"
///         },
///         {
///             "id": "173796",
///             "url": "https://assets.fanart.tv/fanart/movies/372058/moviethumb/your-name-584c67e0de0f1.jpg",
///             "lang": "en",
///             "likes": "2"
///         },
///         {
///             "id": "212929",
///             "url": "https://assets.fanart.tv/fanart/movies/372058/moviethumb/your-name-5a391ecd68ca8.jpg",
///             "lang": "ru",
///             "likes": "1"
///         },
///         {
///             "id": "368161",
///             "url": "https://assets.fanart.tv/fanart/movies/372058/moviethumb/your-name-628592f53d488.jpg",
///             "lang": "en",
///             "likes": "0"
///         }
///     ],
///     "moviebanner": [
///         {
///             "id": "173211",
///             "url": "https://assets.fanart.tv/fanart/movies/372058/moviebanner/your-name-5845e7135b5c2.jpg",
///             "lang": "en",
///             "likes": "2"
///         },
///         {
///             "id": "212928",
///             "url": "https://assets.fanart.tv/fanart/movies/372058/moviebanner/your-name-5a391eb4b3f91.jpg",
///             "lang": "ru",
///             "likes": "1"
///         },
///         {
///             "id": "346384",
///             "url": "https://assets.fanart.tv/fanart/movies/372058/moviebanner/your-name-6149cf35f09e0.jpg",
///             "lang": "ja",
///             "likes": "0"
///         }
///     ],
///     "hdmovieclearart": [
///         {
///             "id": "212926",
///             "url": "https://assets.fanart.tv/fanart/movies/372058/hdmovieclearart/your-name-5a391e8f8c516.png",
///             "lang": "en",
///             "likes": "1"
///         },
///         {
///             "id": "212927",
///             "url": "https://assets.fanart.tv/fanart/movies/372058/hdmovieclearart/your-name-5a391ea639497.png",
///             "lang": "ru",
///             "likes": "1"
///         },
///         {
///             "id": "130711",
///             "url": "https://assets.fanart.tv/fanart/movies/372058/hdmovieclearart/your-name-566af0dd41b1d.png",
///             "lang": "ja",
///             "likes": "1"
///         }
///     ]
/// }
/// ```
fn request_fanart_movies(tvdb_id: &str) -> Result<FanartMovie, SodaError> {
    let url = format!("https://webservice.fanart.tv/v3/movies/{}?api_key={}", tvdb_id, get_api_key());
    let json = blocking_request_value_with_cache(super::cache::CacheType::TMDB, "GET", &url)?;

    match serde_json::from_value(json) {
        Ok(value) => Ok(value),
        Err(e) => Err(SodaError::Json(e)),
    }
}

#[cfg(test)]
mod fanart_tests {
    use super::request_fanart_tv;

    // #[test]
    // fn test_request_fanart_tv() {
    //     let ret = request_fanart_tv(&384640.to_string()).unwrap();
    //     println!("{:?}", ret);
    // }
}
