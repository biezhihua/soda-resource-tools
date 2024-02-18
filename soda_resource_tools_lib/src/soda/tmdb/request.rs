use serde_json::Value;

use crate::soda::entity::SodaError;
use crate::soda::extension_option::OptionExtensions;
use crate::soda::extension_result::ResultExtensions;
use crate::soda::request::blocking_request;
use crate::soda::{cache, request, utils, LIB_CONFIG};

pub(crate) fn tmdb_request(action: &str, params: &str, method: &str) -> Result<Value, SodaError> {
    let api_key = get_api_key()
        .on_none_inspect(|| {
            tracing::debug!("TheMovieDb API Key 未设置");
        })
        .unwrap();

    let url = if params.contains("language") {
        format!("https://{}/3{}?api_key={}&{}", get_api_domain(), action, api_key, params)
    } else {
        format!(
            "https://{}/3{}?api_key={}&{}&language={}",
            get_api_domain(),
            action,
            api_key,
            params,
            get_api_language()
        )
    };

    let json = request::blocking_request_value_with_cache(cache::CacheType::TMDB, method, &url)?;

    Ok(json)
}

fn get_api_key() -> Option<String> {
    let config = LIB_CONFIG.lock().unwrap();
    return Some(config.tmdb_api_key.clone());
}

fn get_api_domain() -> String {
    return "api.themoviedb.org".to_string();
}

fn get_api_language() -> String {
    //return "en-US".to_string();
    return "zh".to_string();
}
