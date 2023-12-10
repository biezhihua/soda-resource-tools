use std::path::Path;

/// 获取Windows所有盘符
pub fn get_windows_drives() -> Vec<String> {
    let mut result = Vec::new();
    for drive in 'A'..='Z' {
        let drive_path_str = format!("{}:", drive.to_string());
        let path_obj = Path::new(&drive_path_str);
        if path_obj.exists() {
            result.push(drive_path_str);
        }
    }
    return result;
}

pub fn get_linux_drive() -> String {
    return "/".to_string();
}

pub fn is_windows_os() -> bool {
    return std::env::consts::OS == "windows";
}

pub fn is_linux_os() -> bool {
    return std::env::consts::OS == "linux";
}

pub fn is_macos() -> bool {
    return std::env::consts::OS == "macos";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[test]
    fn test_get_windows_drives() {
        let windows_drives = get_windows_drives();
        // 一般C盘都有
        assert_eq!(windows_drives.contains(&"C:".to_string()), true);
    }
}
