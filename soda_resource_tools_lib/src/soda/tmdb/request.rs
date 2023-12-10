use serde_json::Value;

use crate::soda::entity::SodaError;
use crate::soda::extension_option::OptionExtensions;
use crate::soda::extension_result::ResultExtensions;
use crate::soda::request::blocking_request;
use crate::soda::{cache, request, utils};

pub(crate) fn tmdb_request(action: &str, params: &str, method: &str) -> Result<Value, SodaError> {
    tracing::info!("action = {} params = {} method = {}", action, params, method);

    let api_key = get_api_key()
        .on_none_inspect(|| {
            tracing::info!("TheMovieDb API Key 未设置");
        })
        .unwrap();

    let url = if params.contains("language") { format!("https://{}/3{}?api_key={}&{}", get_api_domain(), action, api_key, params) } else { format!("https://{}/3{}?api_key={}&{}&language={}", get_api_domain(), action, api_key, params, get_api_language()) };

    let json = request::blocking_request_value_with_cache(cache::CacheType::TMDB, method, &url)?;

    Ok(json)
}

fn get_api_key() -> Option<String> {
    return Some("6f5b96d0d7253117c44963a0ce8aa6f2".to_string());
}

fn get_api_domain() -> String {
    return "api.themoviedb.org".to_string();
}

fn get_api_language() -> String {
    //return "en-US".to_string();
    return "zh".to_string();
}
