use serde_json::Value;

use self::entity::FanartTV;

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
        MTInfo::MOVIE(_) => todo!(),
        MTInfo::TV(tv) => match tv {
            super::entity::TVType::TMDB(info) => {
                if let Some(tvdb_id) = info.tv.tvdb_id() {
                    match request_fanart_tv(&tvdb_id) {
                        Ok(fanart_tv) => {
                            info.fanart_tv = Some(fanart_tv);
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

// https://fanart.tv/
// https://fanarttv.docs.apiary.io/#introduction/api_key-and-client_key
///_movie_url: str = f'https://webservice.fanart.tv/v3/movies/%s?api_key={settings.FANART_API_KEY}'
// _tv_url: str = f'https://webservice.fanart.tv/v3/tv/%s?api_key={settings.FANART_API_KEY}'
fn request_fanart_tv(tvdb_id: &str) -> Result<FanartTV, SodaError> {
    let url = format!("https://webservice.fanart.tv/v3/tv/{}?api_key={}", tvdb_id, get_api_key());
    let json = blocking_request_value_with_cache(super::cache::CacheType::TMDB, "GET", &url)?;

    match serde_json::from_value(json) {
        Ok(value) => Ok(value),
        Err(e) => {
            if e.is_data() {
                tracing::error!("数据类型错误 {}", e);
            } else if e.is_syntax() {
                tracing::error!("语法错误 {}", e);
            } else if e.is_io() {
                tracing::error!("IO 错误 {}", e);
            } else if e.is_eof() {
                tracing::error!("意外的文件结束 {}", e);
            }
            Err(SodaError::Json(e))
        }
    }
}

#[cfg(test)]
mod fanart_tests {
    use super::request_fanart_tv;

    // #[test]
    fn test_request_fanart_tv() {
        let ret = request_fanart_tv(&384640.to_string()).unwrap();
        println!("{:?}", ret);
    }
}
