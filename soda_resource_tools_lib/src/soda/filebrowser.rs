use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::time::UNIX_EPOCH;

use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use serde::{Deserialize, Serialize};

use crate::soda::utils::system;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileItem {
    /// 类型 dir / file
    #[serde(rename = "type")]
    pub file_type: String,
    /// 文件路径
    pub path: String,
    /// 文件名
    pub name: String,
    /// 文件后缀
    pub extension: String,
    /// 文件大小
    pub size: u64,
    /// 文件大小格式化
    pub format_size: String,
    /// 修改时间
    pub modify_time: u64,
    /// 格式化修改时间
    pub format_modify_time: String,
}

impl FileItem {
    pub(crate) fn create(path_obj: &Path) -> FileItem {
        let is_dir = path_obj.is_dir();
        let file_type = if is_dir { "dir".to_string() } else { "file".to_string() };
        let path = if system::is_windows_os() { path_obj.to_str().unwrap().to_string().replace("\\", "/") } else { path_obj.to_str().unwrap().to_string() };
        let name = path_obj.file_name().unwrap().to_str().unwrap().to_string();
        let extension = Self::get_extension(path_obj, is_dir);
        let size = if is_dir { 0 } else { path_obj.metadata().unwrap().len() };
        let modify_time = path_obj.metadata().unwrap().modified().unwrap().duration_since(UNIX_EPOCH).unwrap().as_secs();
        FileItem {
            file_type,
            path,
            name,
            extension,
            size,
            format_size: FileItem::format_file_size(size),
            modify_time,
            format_modify_time: FileItem::format_file_modify_time(modify_time as i64),
        }
    }

    fn format_file_size(file_size_in_bytes: u64) -> String {
        const KB: f64 = 1024.0;
        const MB: f64 = KB * 1024.0;
        const GB: f64 = MB * 1024.0;

        if file_size_in_bytes < KB as u64 {
            format!("{} B", file_size_in_bytes)
        } else if file_size_in_bytes < (MB as u64) {
            format!("{:.2} KB", file_size_in_bytes as f64 / KB)
        } else if file_size_in_bytes < (GB as u64) {
            format!("{:.2} MB", file_size_in_bytes as f64 / MB)
        } else {
            format!("{:.2} GB", file_size_in_bytes as f64 / GB)
        }
    }

    fn get_extension(path_obj: &Path, is_dir: bool) -> String {
        let extension = if is_dir { "".to_string() } else { path_obj.extension().unwrap_or(OsStr::new("")).to_str().unwrap_or("").to_string() };
        extension
    }

    pub(crate) fn create_for_linux_drive(path_obj: &Path) -> FileItem {
        let is_dir = path_obj.is_dir();
        let file_type = if is_dir { "dir".to_string() } else { "file".to_string() };
        let path = path_obj.to_str().unwrap().to_string();
        let name = path_obj.to_str().unwrap().to_string();
        let extension = Self::get_extension(path_obj, is_dir);
        let size = if is_dir { 0 } else { path_obj.metadata().unwrap().len() };
        let modify_time = path_obj.metadata().unwrap().modified().unwrap().duration_since(UNIX_EPOCH).unwrap().as_secs();
        FileItem {
            file_type,
            path,
            name,
            extension,
            size,
            format_size: FileItem::format_file_size(size),
            modify_time,
            format_modify_time: FileItem::format_file_modify_time(modify_time as i64),
        }
    }

    pub(crate) fn create_for_windows_drive(path_obj: &Path) -> FileItem {
        let is_dir = path_obj.is_dir();
        let file_type = if is_dir { "dir".to_string() } else { "file".to_string() };
        let path = path_obj.to_str().unwrap().to_string() + "/";
        let name = path_obj.to_str().unwrap().to_string();
        let extension = Self::get_extension(path_obj, is_dir);
        let size = if is_dir { 0 } else { path_obj.metadata().unwrap().len() };
        let modify_time = path_obj.metadata().unwrap().modified().unwrap().duration_since(UNIX_EPOCH).unwrap().as_secs();
        FileItem {
            file_type,
            path,
            name,
            extension,
            size,
            format_size: FileItem::format_file_size(size),
            modify_time,
            format_modify_time: FileItem::format_file_modify_time(modify_time as i64),
        }
    }

    fn format_file_modify_time(modify_time: i64) -> String {
        let time = NaiveDateTime::from_timestamp_opt(modify_time, 0).unwrap();
        let dt: DateTime<Local> = Local.from_local_datetime(&time).unwrap();
        return dt.format("%Y-%m-%d %H:%M:%S").to_string();
    }
}

/// 获取Windows或Linux指定路径下的所有文件和文件夹信息
///
pub fn list_sub_path(path: String, filter_extension: Vec<String>) -> Vec<FileItem> {
    let mut ret_items = Vec::new();
    let path_obj = Path::new(&path);
    if path_obj.is_dir() {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let item = FileItem::create(&entry.path());
                    // 如果是文件，需要过滤文件格式
                    if entry.file_type().unwrap().is_file() {
                        // 为空不过滤
                        if filter_extension.is_empty() {
                            ret_items.push(item);
                        }
                        // 符合条件不过滤
                        else if filter_extension.contains(&item.extension) {
                            ret_items.push(item);
                        }
                        // 其余情况过滤掉
                        else {
                            continue;
                        }
                    } else {
                        ret_items.push(item);
                    }
                }
            }
        }
    }
    return ret_items;
}

pub fn list_root_path() -> Vec<FileItem> {
    return match std::env::consts::OS {
        "linux" => list_root_path_in_linux(),
        "windows" => list_root_path_in_windows(),
        "macos" => list_root_path_in_linux(),
        _ => Vec::new(),
    };
}

/// 获取Windows所有盘符的描述信息
fn list_root_path_in_windows() -> Vec<FileItem> {
    let mut ret_items = Vec::new();
    for drive in system::get_windows_drives() {
        let path_obj = Path::new(&drive);
        if path_obj.exists() {
            ret_items.push(FileItem::create_for_windows_drive(path_obj));
        }
    }
    return ret_items;
}

/// 获取Linux根目录描述信息
fn list_root_path_in_linux() -> Vec<FileItem> {
    let mut ret_items = Vec::new();
    ret_items.push(FileItem::create_for_linux_drive(Path::new(&system::get_linux_drive())));
    return ret_items;
}

#[cfg(test)]
mod filebrowser_tests {
    use std::env;

    use super::*;

    #[cfg(target_os = "windows")]
    #[test]
    fn test_list_root_path_in_windows() {
        println!("{:?}", env::current_dir());
        {
            let root = list_root_path_in_windows();
            println!("{:?}", root);
            let filtered: Vec<FileItem> = root.into_iter().filter(|it| it.name == "C:").collect();
            let item = filtered[0].clone();
            assert_eq!(item.file_type, "dir");
            assert_eq!(item.path, "C:/");
            assert_eq!(item.name, "C:");
            assert_eq!(item.extension, "");
            assert_eq!(item.size, 0);
        }
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_list_root_path_in_linux() {
        println!("{:?}", env::current_dir());
        {
            let root = list_root_path_in_linux();
            println!("{:?}", root);
            let filtered: Vec<FileItem> = root.into_iter().filter(|it| it.name == "/").collect();
            let item = filtered[0].clone();
            assert_eq!(item.file_type, "dir");
            assert_eq!(item.path, "/");
            assert_eq!(item.name, "/");
            assert_eq!(item.extension, "");
            assert_eq!(item.size, 0);
        }
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn test_list_sub_path_in_windows() {
        println!("{:?}", env::current_dir());
        {
            let root = list_sub_path(env::current_dir().unwrap().to_str().unwrap().to_string(), vec![]);
            println!("{:?}", root);
            let filtered: Vec<FileItem> = root.into_iter().filter(|it| it.name == "Cargo.toml").collect();
            let item = filtered[0].clone();
            assert_eq!(item.file_type, "file");
            assert_eq!(item.path.contains("soda_resource_tools_lib/Cargo.toml"), true);
            assert_eq!(item.name, "Cargo.toml");
            assert_eq!(item.extension, "toml");
        }

        {
            let root = list_sub_path(env::current_dir().unwrap().to_str().unwrap().to_string(), vec!["toml".to_string()]);
            println!("{:?}", root);
            let filtered: Vec<FileItem> = root.into_iter().filter(|it| it.name == "Cargo.toml").collect();
            let item = filtered[0].clone();
            assert_eq!(item.file_type, "file");
            assert_eq!(item.path.contains("soda_resource_tools_lib/Cargo.toml"), true);
            assert_eq!(item.name, "Cargo.toml");
            assert_eq!(item.extension, "toml");
        }

        {
            let root = list_sub_path(env::current_dir().unwrap().to_str().unwrap().to_string(), vec!["md".to_string()]);
            println!("{:?}", root);
            let filtered: Vec<FileItem> = root.into_iter().filter(|it| it.name == "README.md").collect();
            let item = filtered[0].clone();
            assert_eq!(item.file_type, "file");
            assert_eq!(item.path.contains("soda_resource_tools_lib/README.md"), true);
            assert_eq!(item.name, "README.md");
            assert_eq!(item.extension, "md");
        }
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_list_sub_path_in_linux() {
        println!("{:?}", env::current_dir());
        {
            let root = list_sub_path(env::current_dir().unwrap().to_str().unwrap().to_string(), vec![]);
            println!("{:?}", root);
            let filtered: Vec<FileItem> = root.into_iter().filter(|it| it.name == "Cargo.toml").collect();
            let item = filtered[0].clone();
            assert_eq!(item.file_type, "file");
            assert_eq!(item.path.contains("soda_resource_tools_lib/Cargo.toml"), true);
            assert_eq!(item.name, "Cargo.toml");
            assert_eq!(item.extension, "toml");
        }

        {
            let root = list_sub_path(env::current_dir().unwrap().to_str().unwrap().to_string(), vec!["toml".to_string()]);
            println!("{:?}", root);
            let filtered: Vec<FileItem> = root.into_iter().filter(|it| it.name == "Cargo.toml").collect();
            let item = filtered[0].clone();
            assert_eq!(item.file_type, "file");
            assert_eq!(item.path.contains("soda_resource_tools_lib/Cargo.toml"), true);
            assert_eq!(item.name, "Cargo.toml");
            assert_eq!(item.extension, "toml");
        }

        {
            let root = list_sub_path(env::current_dir().unwrap().to_str().unwrap().to_string(), vec!["md".to_string()]);
            println!("{:?}", root);
            let filtered: Vec<FileItem> = root.into_iter().filter(|it| it.name == "README.md").collect();
            let item = filtered[0].clone();
            assert_eq!(item.file_type, "file");
            assert_eq!(item.path.contains("soda_resource_tools_lib/README.md"), true);
            assert_eq!(item.name, "README.md");
            assert_eq!(item.extension, "md");
        }
    }
}
