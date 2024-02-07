use bytes::Bytes;
use once_cell::sync::Lazy;
use sled::Db;

use crate::soda::utils::{self};
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;

pub enum CacheType {
    TMDB,
    FANART,
}

pub(crate) static TMDB_CACHE: Lazy<Mutex<Db>> = Lazy::new(|| Mutex::new(sled::open(cache_path(&CacheType::TMDB)).expect("open")));

pub(crate) static FANART_CACHE: Lazy<Mutex<Db>> = Lazy::new(|| Mutex::new(sled::open(cache_path(&CacheType::FANART)).expect("open")));

pub(crate) fn cache_remove(cache_type: &CacheType, key: &str) {
    let cache_db = cache_db(cache_type);
    sled_cache_remove(cache_db, key)
}

pub(crate) fn cache_get(cache_type: &CacheType, key: &str) -> Option<String> {
    let cache_db = cache_db(cache_type);
    sled_cache_get(cache_db, key)
}

pub(crate) fn cache_save_bytes(cache_type: &CacheType, key: &str, value: &Bytes) {
    let cache_db = cache_db(cache_type);
    sled_cache_save_bytes(cache_db, key, value)
}

pub(crate) fn cache_get_bytes(cache_type: &CacheType, key: &str) -> Option<Bytes> {
    let cache_db = cache_db(cache_type);
    sled_cache_get_bytes(cache_db, key)
}

fn cache_db(cache_type: &CacheType) -> MutexGuard<'_, Db> {
    match cache_type {
        CacheType::TMDB => TMDB_CACHE.lock().unwrap(),
        CacheType::FANART => FANART_CACHE.lock().unwrap(),
    }
}

fn cache_path(cache_type: &CacheType) -> String {
    match cache_type {
        CacheType::TMDB => utils::get_tmdb_http_cache_path(),
        CacheType::FANART => utils::get_fanart_http_cache_path(),
    }
}

fn sled_cache_remove(db: MutexGuard<'_, Db>, url: &str) {
    db.remove(url).expect("remove");
    db.flush().expect("flush error");
    tracing::debug!("cache remove success key =  {:?}", url);
}

fn sled_cache_get_bytes(db: MutexGuard<'_, Db>, key: &str) -> Option<Bytes> {
    let key = key;
    match db.get(key) {
        Ok(value) => match value {
            Some(value) => {
                let value = value.as_ref();
                let value = bytes::Bytes::from(value.to_vec());
                tracing::debug!("hit cache success key = {:?}", key);
                return Some(value);
            }
            None => {
                return None;
            }
        },
        Err(e) => {
            tracing::error!("cache error key = {}", key);
            return None;
        }
    }
}

fn sled_cache_save_bytes(db: MutexGuard<'_, Db>, key: &str, value: &Bytes) {
    let key = key;
    db.insert(key, value.to_vec()).expect("insert");
    db.flush().expect("flush error");
    tracing::debug!("cache success key =  {:?}", key);
}

fn sled_cache_get(db: MutexGuard<'_, Db>, key: &str) -> Option<String> {
    match db.get(key) {
        Ok(value) => match value {
            Some(value) => {
                let value = value.as_ref();
                let value = String::from_utf8(value.to_vec()).unwrap();
                tracing::debug!("hit cache success key = {:?}", key);
                return Some(value);
            }
            None => {
                return None;
            }
        },
        Err(e) => {
            tracing::error!("cache error key = {}", key);
            return None;
        }
    }
}
