use std::fs;
use std::path::Path;

use regex::Regex;

use crate::soda::global::REGEX_MT_EXT;

use super::entity::ResourceType;
use super::utils;

pub(crate) fn get_level1_sub_dirs<F>(directory_path: &str, callback: &mut F) -> std::io::Result<()>
where
    F: FnMut(&Path),
{
    let entries = fs::read_dir(directory_path)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            callback(&path)
        }
    }
    Ok(())
}

pub(crate) fn get_level1_sub_files_filter_format<F>(directory_path: &str, suffix: &Regex, callback: &mut F) -> std::io::Result<()>
where
    F: FnMut(&Path),
{
    for entry in fs::read_dir(directory_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(file_name) = path.file_name() {
                if let Some(file_name_str) = file_name.to_str() {
                    if suffix.is_match(file_name_str) {
                        callback(&path);
                    }
                }
            }
        }
    }

    Ok(())
}

fn find_mt_with_ext<F>(dir_path: &str, callback: &mut F)
where
    F: FnMut(String),
{
    let path = Path::new(dir_path);
    if path.is_file() {
        if let Some(file_name) = path.file_name() {
            if let Some(file_name_str) = file_name.to_str() {
                if (&REGEX_MT_EXT).is_match(file_name_str) {
                    callback(path.to_string_lossy().to_string());
                    return;
                }
            }
        }
    }

    let mut callback_file = |path: &Path| {
        callback(path.to_string_lossy().to_string());
    };
    if let Err(e) = get_level1_sub_files_filter_format(dir_path, &REGEX_MT_EXT, &mut callback_file) {
        tracing::error!("Error reading files: {}", e);
    }

    let mut callback_dir = |path: &Path| {
        // 如果是蓝光目录，不再递归查找
        if utils::is_bluray_dir(dir_path) {
            callback(dir_path.to_string());
        } else {
            find_mt_with_ext(&path.to_string_lossy().to_string(), callback);
        }
    };
    if let Err(e) = get_level1_sub_dirs(dir_path, &mut callback_dir) {
        tracing::error!("Error reading directory: {}", e);
    }
}

/// 寻找配置目录下的可整理资源
pub fn find<F>(resource_type: &ResourceType, src_dir_path: &str, callback: &mut F)
where
    F: FnMut(String),
{
    match resource_type {
        ResourceType::MT => {
            find_mt_with_ext(src_dir_path, callback);
        }
    }
}
