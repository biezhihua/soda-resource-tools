use std::fs;
use std::path::Path;

use super::entity::{MTInfo, MTMetadata, TransferType};

pub(crate) fn transfer(target_dir: &str, transfer_mode: &TransferType, rename_format: &str, file_metadata: &MTMetadata, src_path: &str) -> Option<String> {
    // 生成转移文件路径
    if let Some(target_path) = get_mt_target_path(target_dir, rename_format, file_metadata) {
        tracing::info!("target_path {:?}", target_path);

        // 转移文件
        let transfer_file = transfer_mt_file(file_metadata, src_path, &target_path, transfer_mode.clone());
        if let Some(transferred_path) = transfer_file {
            tracing::info!("transfer file success");
            return Some(transferred_path);
        }
        return None;
    } else {
        tracing::info!("get target path error");
        return None;
    }
}

/// 生成转移文件路径
pub(crate) fn get_mt_target_path(target_dir: &str, rename_format: &str, metadata: &MTMetadata) -> Option<String> {
    if let Some(transfer_path) = gen_mt_rename_path(rename_format, metadata) {
        let ret = Path::new(target_dir).join(transfer_path).to_string_lossy().to_string().replace("/", &std::path::MAIN_SEPARATOR.to_string());
        return Some(ret);
    } else {
        return None;
    }
}

pub(crate) fn transfer_mt_file(mt_meta: &MTMetadata, src_file_path: &str, target_file_path: &str, transfer_mode: TransferType) -> Option<String> {
    // check src_file_path is valid
    if !Path::new(src_file_path).exists() {
        tracing::info!("src_file_path {:?} is not exists", src_file_path);
        return None;
    }

    let target_path = Path::new(target_file_path);

    // check transfer_path parent path is valid and create it
    if let Some(parent_path) = Path::new(&target_path).parent() {
        let parent_path = parent_path.to_str().unwrap().to_string();
        fs::create_dir_all(parent_path).unwrap_or_else(|err| {
            tracing::error!("Failed to create directory: {}", err);
        });
    }

    // check target_path is exists and remove it
    if target_path.exists() {
        // fs::remove_file(target_path).unwrap();
        tracing::info!("target_path is exists, skip it");
        return Some(target_file_path.to_string());
    }

    // transfer file
    return transfer_by_mode(transfer_mode, src_file_path, target_file_path);
}

/// 生成转移文件路径
pub(crate) fn gen_mt_rename_path(rename_format: &str, mt_meta: &MTMetadata) -> Option<String> {
    let mut result = rename_format.to_string();

    // $title_cn$.$title_en$.$year$.$season$$episode$.$resolution$.$source$.$video_codec$.$audio_codec$.$extension$
    if mt_meta.title_cn.is_empty() {
        result = result.replace("$title_cn$", "");
        result = result.replace("..", ".");
    } else {
        result = result.replace("$title_cn$", &mt_meta.title_cn);
    }

    if mt_meta.title_en.is_empty() {
        result = result.replace("$title_en$", "");
        result = result.replace("..", ".");
    } else {
        let mut title_en = mt_meta.title_en.clone();
        title_en = title_en.replace(" ", ".");
        title_en = title_en.replace("-", ".");
        result = result.replace("$title_en$", &title_en);
    }

    match &mt_meta.release_year {
        None => {
            result = result.replace("$release_year$", "");
            result = result.replace("..", ".");
        }
        Some(year) => {
            result = result.replace("$release_year$", year);
        }
    }

    match &mt_meta.year {
        None => {
            result = result.replace("$year$", "");
            result = result.replace("..", ".");
        }
        Some(year) => {
            result = result.replace("$year$", year);
        }
    }

    if !mt_meta.season.is_empty() && !mt_meta.episode.is_empty() {
        result = result.replace("$season$", &mt_meta.season);
        result = result.replace("$episode$", &mt_meta.episode);
    } else if mt_meta.season.is_empty() && !mt_meta.episode.is_empty() {
        result = result.replace("$season$", "");
        result = result.replace("$episode$", &mt_meta.episode);
    } else if !mt_meta.season.is_empty() && mt_meta.episode.is_empty() {
        result = result.replace("$season$", &mt_meta.season);   
        result = result.replace("$episode$", "");
    } else if mt_meta.season.is_empty() && mt_meta.episode.is_empty() {
        result = result.replace(".$season$$episode$.", ".");
    }

    if mt_meta.resolution.is_empty() {
        result = result.replace("$resolution$", "");
        result = result.replace("..", ".");
    } else {
        result = result.replace("$resolution$", &mt_meta.resolution);
    }

    if mt_meta.source.is_empty() {
        result = result.replace("$source$", "");
        result = result.replace("..", ".");
    } else {
        result = result.replace("$source$", &mt_meta.source);
    }

    if mt_meta.video_codec.is_empty() {
        result = result.replace("$video_codec$", "");
        result = result.replace("..", ".");
    } else {
        result = result.replace("$video_codec$", &mt_meta.video_codec);
    }

    if mt_meta.audio_codec.is_empty() {
        result = result.replace("$audio_codec$", "");
        result = result.replace("..", ".");
    } else {
        result = result.replace("$audio_codec$", &mt_meta.audio_codec);
    }

    if mt_meta.extension.is_empty() {
        result = result.replace("$release_group$", "");
    } else {
        result = result.replace("$release_group$", &mt_meta.release_group);
    }

    if mt_meta.extension.is_empty() {
        result = result.replace("$extension$", "");
    } else {
        result = result.replace("$extension$", &mt_meta.extension);
    }

    return Some(result);
}

fn transfer_by_mode(mode: TransferType, src_file: &str, target_file: &str) -> Option<String> {
    match mode {
        TransferType::HardLink => {
            return match fs::hard_link(src_file, target_file) {
                Ok(()) => Some(target_file.to_string()),
                Err(e) => {
                    tracing::info!("Failed to create hard link: {}", e);
                    None
                }
            };
        }
        TransferType::SymbolLink => {
            return match create_symlink(src_file, target_file) {
                Ok(()) => Some(target_file.to_string()),
                Err(e) => {
                    tracing::info!("Failed to create soft link: {}", e);
                    None
                }
            };
        }
        TransferType::Copy => {
            return match fs::copy(src_file, target_file) {
                Ok(_) => Some(target_file.to_string()),
                Err(e) => {
                    tracing::info!("Failed to move file: {}", e);
                    None
                }
            };
        }
        TransferType::Move => {
            return match fs::copy(src_file, target_file) {
                Ok(_) => {
                    fs::remove_file(src_file).unwrap_or_else(|e| {
                        tracing::info!("Failed to remove file: {}", e);
                    });
                    Some(target_file.to_string())
                }
                Err(e) => {
                    tracing::info!("Failed to move file: {}", e);
                    None
                }
            };
        }
    }
}

fn create_symlink(target_path: &str, symlink_path: &str) -> Result<(), std::io::Error> {
    // 使用条件编译来根据平台选择不同的符号链接函数
    #[cfg(target_os = "windows")]
    {
        // 在 Windows 上使用 std::os::windows::fs::symlink_file
        std::os::windows::fs::symlink_file(target_path, symlink_path)?;
    }
    #[cfg(not(target_os = "windows"))]
    {
        // 在 Unix-like 系统上使用 std::os::unix::fs::symlink
        std::os::unix::fs::symlink(target_path, symlink_path)?;
    }

    Ok(())
}

#[cfg(test)]
mod transfer_tests {
    use crate::soda::meta::{self};

    use super::*;

    #[test]
    fn test_generate_transfer_path() {
        let metadata = meta::mt_metadata("凡人修仙传.The.Mortal.Ascention.2020.S01E01.2160p.WEB-DL.H264.AAC-OurTV.mp4").unwrap();
        let ret = gen_mt_rename_path("$title_cn$.$title_en$.$year$.$season$$episode$.$resolution$.$source$.$video_codec$.$audio_codec$.$extension$", &metadata);
        assert_eq!(ret.unwrap(), "凡人修仙传.The.Mortal.Ascention.2020.S01E01.2160p.WEB-DL.H.264.AAC.mp4")
    }

    #[test]
    fn test_generate_transfer_path1() {
        let metadata = meta::mt_metadata("凡人修仙传.The.Mortal.Ascention.2020.S01E01.2160p.WEB-DL.H264.AAC-OurTV.mp4").unwrap();
        let ret = gen_mt_rename_path("$title_cn$.$title_en$.$year$/$title_cn$.$title_en$.$year$.$season$.$resolution$.$source$.$video_codec$.$audio_codec$/$title_cn$.$title_en$.$year$.$season$$episode$.$resolution$.$source$.$video_codec$.$audio_codec$.$extension$", &metadata);
        assert_eq!(ret.unwrap(), "凡人修仙传.The.Mortal.Ascention.2020/凡人修仙传.The.Mortal.Ascention.2020.S01.2160p.WEB-DL.H.264.AAC/凡人修仙传.The.Mortal.Ascention.2020.S01E01.2160p.WEB-DL.H.264.AAC.mp4")
    }

    #[test]
    fn test_generate_transfer_path2() {
        let metadata = meta::mt_metadata("凡人修仙传.The.Mortal.Ascention.2020.S01E01.2160p.mp4").unwrap();
        let ret = gen_mt_rename_path("$title_cn$.$title_en$.$year$/$title_cn$.$title_en$.$year$.$season$.$resolution$.$source$.$video_codec$.$audio_codec$/$title_cn$.$title_en$.$year$.$season$$episode$.$resolution$.$source$.$video_codec$.$audio_codec$.$extension$", &metadata);
        assert_eq!(ret.unwrap(), "凡人修仙传.The.Mortal.Ascention.2020/凡人修仙传.The.Mortal.Ascention.2020.S01.2160p/凡人修仙传.The.Mortal.Ascention.2020.S01E01.2160p.mp4")
    }

    #[test]
    fn test_generate_transfer_path3() {
        let metadata = meta::mt_metadata("Friends.S01E02.1080p.BluRay.Remux.AVC.AC3-WhaleHu.mkv").unwrap();
        let ret = gen_mt_rename_path("$title_cn$.$title_en$.$year$.$season$$episode$.$resolution$.$source$.$video_codec$.$audio_codec$.$extension$", &metadata);
        assert_eq!(ret.unwrap(), "Friends.S01E02.1080p.BluRay.Remux.H.264.AC3.mkv")
    }

    #[test]
    fn test_generate_transfer_path4() {
        let metadata = meta::mt_metadata("The.Long.Season.E01.2023.1080p.WEBrip.NF.x265.10bit.AC3￡cXcY@FRDS.mkv").unwrap();
        let ret = gen_mt_rename_path("$title_cn$.$title_en$.$year$.$season$$episode$.$resolution$.$source$.$video_codec$.$audio_codec$.$extension$", &metadata);
        assert_eq!(ret.unwrap(), "The.Long.Season.2023.E01.1080p.WEBrip.H.265.AC3.mkv")
    }
}
