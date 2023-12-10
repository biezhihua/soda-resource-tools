use std::path::PathBuf;
use std::{fs, path::Path};

pub mod encrypt;
pub mod system;
pub mod time;
pub mod token;

use std::fs::OpenOptions;
use std::io::Write;

use super::LIB_CONFIG;

pub fn is_bluray_dir(path: &str) -> bool {
    let path = Path::new(path);
    if !path.exists() {
        return false;
    }

    if !path.is_dir() {
        return false;
    }

    // check path dir must have BDMV and CERTIFICATE folders
    let mut has_bdmv = false;
    let mut has_certificate = false;
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let entry_path = entry.path();
        let entry_path_str = entry_path.to_str().unwrap();
        if entry_path_str.eq("BDMV") {
            has_bdmv = true;
        } else if entry_path_str.eq("CERTIFICATE") {
            has_certificate = true;
        }
    }

    if !has_bdmv || !has_certificate {
        return false;
    }

    return true;
}

pub(crate) fn str_replace_extension(file_path: &str, new_extension: &str) -> String {
    let path = Path::new(file_path);
    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
    let new_file_name = format!("{}.{}", stem, new_extension);
    path.with_file_name(new_file_name).to_str().unwrap().to_string()
}

pub(crate) fn append_to_file(file_path: &str, text: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new().write(true).append(true).create(true).open(file_path)?;
    writeln!(file, "{}", text)?;
    Ok(())
}

pub(crate) fn write_meta_file_path(path: &str) {
    let cache_path = get_cache_path();
    let log_file_path = cache_path.join("meta_error.log").to_str().unwrap().to_string();
    match append_to_file(&log_file_path, path) {
        Ok(_) => {}
        Err(e) => println!("write content error : {:?}", e),
    }
}

pub(crate) fn write_request_error(msg: String) {
    let cache_path = get_cache_path();
    let log_file_path = cache_path.join("request_error.log").to_str().unwrap().to_string();
    match append_to_file(&log_file_path, &msg) {
        Ok(_) => {}
        Err(e) => println!("write content error : {:?}", e),
    }
}

pub(crate) fn get_cache_path() -> PathBuf {
    let cache_path = &LIB_CONFIG.lock().unwrap().cache_path;
    let cache_path = Path::new(cache_path);
    let cache_path = cache_path.to_path_buf();
    if !cache_path.exists() {
        fs::create_dir_all(cache_path.clone()).unwrap();
    }
    cache_path
}

pub(crate) fn get_tmdb_http_cache_path() -> String {
    let cache_path = get_cache_path();
    let http_cache = cache_path.join("imdb_http_cache").to_str().unwrap().to_string();
    http_cache
}


pub(crate) fn get_fanart_http_cache_path() -> String {
    let cache_path = get_cache_path();
    let http_cache = cache_path.join("fanart_http_cache").to_str().unwrap().to_string();
    http_cache
}
