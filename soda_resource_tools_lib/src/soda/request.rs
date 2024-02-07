use bytes::Bytes;
use reqwest::{blocking::Response, Error};
use serde::de::value;
use serde_json::Value;
use std::fs;

use crate::soda::utils;

use super::{
    cache::{self, CacheType},
    entity::SodaError,
    extension_result::ResultExtensions,
};

/// 请求URL并返回JSON
/// 如果有缓存，优先使用缓存。
///     如果缓存内容错误，则移除缓存结果。
///     如果缓存内容解析JSON成功，则返回
/// 没有缓存则请求网络，并缓存结果。
///     如果请求网络失败，则返回错误。
///     如果请求网络成功，则解析成JSON缓存后返回。
pub(crate) fn blocking_request_value_with_cache(cache_type: CacheType, method: &str, url: &str) -> Result<Value, SodaError> {
    tracing::debug!("request url = {:?}", url);
    let cache = cache::cache_get(&cache_type, url);
    if cache.is_some() {
        let cache_text = cache.unwrap();
        match serde_json::from_str(&cache_text) {
            Ok(value) => {
                return Ok(value);
            }
            Err(e) => {
                tracing::error!("cache error text =  {:?}", cache_text);
                tracing::error!("cache json error {:?}", e);
                crate::soda::cache::cache_remove(&cache_type, url);
            }
        }
    }

    let response = blocking_request(method, url)?;

    let response_bytes = response.bytes().on_err_inspect(|e| {
        tracing::error!("response bytes error {:?}", e);
    })?;

    let content = String::from_utf8(response_bytes.to_vec()).unwrap();

    match serde_json::from_str(&content) {
        Ok(value) => {
            crate::soda::cache::cache_save_bytes(&cache_type, url, &response_bytes);
            return Ok(value);
        }
        Err(e) => {
            tracing::error!("response json error text =  {:?}", content);
            tracing::error!("response json error {:?}", e);
            return Err(SodaError::String(format!("response json error e = {:?} ", e)));
        }
    }
}

pub(crate) fn blocking_request(method: &str, url: &str) -> Result<reqwest::blocking::Response, SodaError> {
    Ok(if method == "GET" {
        blocking_request_get(url)?
    } else {
        blocking_request_post(url)?
    })
}

pub(crate) fn blocking_request_post(url: &str) -> Result<reqwest::blocking::Response, SodaError> {
    let response = reqwest::blocking::Client::new().post(url).send().on_err_inspect(|e| {
        tracing::error!("request error {:?}", e);
    })?;
    Ok(response)
}

pub(crate) fn blocking_request_get(url: &str) -> Result<reqwest::blocking::Response, SodaError> {
    let response = reqwest::blocking::get(url).on_err_inspect(|e| {
        tracing::error!("request error {:?}", e);
    })?;
    Ok(response)
}

pub(crate) fn blocking_get_request_and_download_file(url: &str, image_path: &std::path::PathBuf) {
    match blocking_request_bytes_with_cache(CacheType::FANART, "GET", url) {
        Ok(response_bytes) => {
            let mut out = fs::File::create(image_path).unwrap();
            std::io::copy(&mut response_bytes.to_vec().as_slice(), &mut out).unwrap();
        }
        Err(_) => {}
    };
}

fn blocking_request_bytes_with_cache(cache_type: CacheType, method: &str, url: &str) -> Result<Bytes, SodaError> {
    let cache = cache::cache_get_bytes(&cache_type, url);
    if cache.is_some() {
        return Ok(cache.unwrap());
    }

    let response = blocking_request(method, url)?;

    let response_bytes = response.bytes().on_err_inspect(|e| {
        tracing::error!("response bytes error {:?}", e);
    })?;

    crate::soda::cache::cache_save_bytes(&cache_type, url, &response_bytes);

    Ok(response_bytes)
}
