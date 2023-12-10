use bytes::Bytes;

use crate::soda::utils::{self};

pub enum CacheType {
    TMDB,
    FANART,
}

pub(crate) fn cache_remove(cache_type: &CacheType, key: &str) {
    sled_cache_remove(cache_path(cache_type), key)
}

fn cache_path(cache_type: &CacheType) -> String {
    match cache_type {
        CacheType::TMDB => utils::get_tmdb_http_cache_path(),
        CacheType::FANART => utils::get_fanart_http_cache_path(),
    }
}

pub(crate) fn cache_get(cache_type: &CacheType, key: &str) -> Option<String> {
    sled_ache_get(cache_path(cache_type), key)
}

pub(crate) fn cache_save_bytes(cache_type: &CacheType, key: &str, value: &Bytes) {
    sled_cache_save_bytes(cache_path(cache_type), key, value)
}

pub(crate) fn cache_get_bytes(cache_type: &CacheType, key: &str) -> Option<Bytes> {
    sled_cache_get_bytes(cache_path(cache_type), key)
}

fn sled_cache_remove(cache_path: String, url: &str) {
    let tree = sled::open(cache_path).expect("open");
    tree.remove(url).expect("remove");
    tree.flush().expect("flush error");
    tracing::info!("cache remove key =  {:?}", url);
}

fn sled_cache_get_bytes(cache_path: String, key: &str) -> Option<Bytes> {
    let tree = sled::open(cache_path).expect("open");
    let key = key;
    match tree.get(key) {
        Ok(value) => match value {
            Some(value) => {
                let value = value.as_ref();
                let value = bytes::Bytes::from(value.to_vec());
                tracing::info!("cache hit key = {:?}", key);
                return Some(value);
            }
            None => {
                tracing::info!("cache not hit key = {}", key);
                return None;
            }
        },
        Err(e) => {
            tracing::info!("cache error key = {}", key);
            return None;
        }
    }
}

fn sled_cache_save_bytes(cache_path: String, key: &str, value: &Bytes) {
    let tree = sled::open(cache_path).expect("open");
    let key = key;
    tree.insert(key, value.to_vec()).expect("insert");
    tree.flush().expect("flush error");
    tracing::info!("cache success key =  {:?}", key);
}

fn sled_ache_get(cache_path: String, key: &str) -> Option<String> {
    let tree = sled::open(cache_path).expect("open");
    match tree.get(key) {
        Ok(value) => match value {
            Some(value) => {
                let value = value.as_ref();
                let value = String::from_utf8(value.to_vec()).unwrap();
                tracing::info!("cache hit key = {:?}", key);
                return Some(value);
            }
            None => {
                tracing::info!("cache not hit key = {}", key);
                return None;
            }
        },
        Err(e) => {
            tracing::info!("cache error key = {}", key);
            return None;
        }
    }
}
