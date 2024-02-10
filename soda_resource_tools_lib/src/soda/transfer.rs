use std::path::Path;
use std::{fs, path::PathBuf};

use crate::soda::entity::EmbyRenameStyle;

use super::entity::{MTInfo, MTMetadata, RenameStyle, SodaError, TransferType};

pub(crate) fn transfer(
    target_path: &str,
    transfer_mode: &TransferType,
    rename_style: Option<RenameStyle>,
    rename_format: &str,
    mt_meta: &MTMetadata,
    src_path: &str,
) -> Result<String, SodaError> {
    // 生成转移文件路径
    let target_path = gen_mt_transfer_target_path(target_path, rename_style, rename_format, mt_meta);
    tracing::debug!("target_path {:?}", target_path);

    // 转移文件
    let transferred_path = transfer_mt_file(mt_meta, src_path, &target_path, transfer_mode)?;
    return Ok(transferred_path);
}

/// 生成转移文件路径
pub(crate) fn gen_mt_transfer_target_path(target_path: &str, rename_style: Option<RenameStyle>, rename_format: &str, mt_meta: &MTMetadata) -> String {
    tracing::debug!(
        "gen_mt_transfer_target_path target_path = {:?} rename_style = {:?}, rename_format = {:?}, mt_meta = {:?}",
        target_path,
        rename_style,
        rename_format,
        mt_meta
    );

    if let Some(rename_style) = rename_style {
        let transfer_path = gen_mt_rename_path2(rename_style, mt_meta);

        let path = Path::new(target_path).join(transfer_path).to_string_lossy().to_string();

        tracing::debug!("transfer_path after {:?}", path);

        return path;
    } else {
        let transfer_path: String = gen_mt_rename_path(rename_format, mt_meta);
        tracing::debug!("transfer_path before {:?}", transfer_path);

        let path = Path::new(target_path)
            .join(transfer_path)
            .to_string_lossy()
            .to_string()
            .replace("/", &std::path::MAIN_SEPARATOR.to_string());

        tracing::debug!("transfer_path after {:?}", path);
        return path;
    }
}

/// 转移文件
pub(crate) fn transfer_mt_file(mt_meta: &MTMetadata, src_path: &str, target_path: &str, transfer_type: &TransferType) -> Result<String, SodaError> {
    // check src_file_path is valid
    if !Path::new(src_path).exists() {
        return Err(SodaError::String(format!("src_file_path {:?} is not exists", src_path)));
    }

    let target_path = Path::new(target_path);
    let target_path_str = target_path.to_str().unwrap().to_string();

    // check transfer_path parent path is valid and create it
    if let Some(parent_path) = Path::new(&target_path).parent() {
        let parent_path = parent_path.to_str().unwrap().to_string();
        fs::create_dir_all(parent_path)?;
    }

    // check target_path is exists and remove it
    if target_path.exists() {
        // fs::remove_file(target_path).unwrap();
        tracing::debug!("target_path is exists, skip it");
        return Ok(target_path_str);
    }

    // transfer file
    return transfer_by_mode(transfer_type, src_path, &target_path_str);
}

/// 生成转移文件路径
fn gen_mt_rename_path2(rename_style: RenameStyle, mt_meta: &MTMetadata) -> PathBuf {
    tracing::debug!("gen_mt_rename_path2 rename_style = {:?}, mt_meta = {:?}", rename_style, mt_meta);

    match rename_style {
        RenameStyle::Emby => {
            if mt_meta.is_movie() {
                return EmbyRenameStyle::EmbyMovie.rename(mt_meta);
            } else if mt_meta.is_tv() {
                return EmbyRenameStyle::EmbyTV.rename(mt_meta);
            }
        }
    }
    unreachable!("gen_mt_rename_path2 = {:?}", rename_style);
}

/// 生成转移文件路径
pub(crate) fn gen_mt_rename_path(rename_format: &str, mt_meta: &MTMetadata) -> String {
    tracing::debug!("gen_mt_rename_path rename_format = {:?}, mt_meta = {:?}", rename_format, mt_meta);

    let mut result = rename_format.to_string();

    // $title_cn$.$title_en$.$year$.$season$$episode$.$resolution$.$source$.$video_codec$.$audio_codec$.$extension$
    if mt_meta.title_cn.is_empty() {
        result = result.replace("$title_cn$.", "");
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

    if mt_meta.year.is_empty() {
        result = result.replace("$year$", "");
        result = result.replace("..", ".");
    } else {
        result = result.replace("$year$", &mt_meta.year);
    }

    if !mt_meta.season.is_empty() && !mt_meta.episode.is_empty() {
        result = result.replace("$season$", &mt_meta.season_number_format());
        result = result.replace("$episode$", &mt_meta.episode_number_format());
    } else if mt_meta.season.is_empty() && !mt_meta.episode.is_empty() {
        result = result.replace("$season$", "");
        result = result.replace("$episode$", &mt_meta.episode_number_format());
    } else if !mt_meta.season.is_empty() && mt_meta.episode.is_empty() {
        result = result.replace("$season$", &mt_meta.season_number_format());
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

    if mt_meta.release_group.is_empty() {
        result = result.replace("$release_group$", "");
        result = result.replace(".-", "");
    } else {
        result = result.replace("$release_group$", &mt_meta.release_group);
        result = result.replace(".-", "-");
    }

    if mt_meta.extension.is_empty() {
        result = result.replace("$extension$", "");
    } else {
        result = result.replace("$extension$", &mt_meta.extension);
    }

    result = result.replace("./", "/");
    result = result.replace(".\\", "\\");

    tracing::debug!("gen_mt_rename_path result = {:?}", result);

    return result;
}

fn transfer_by_mode(mode: &TransferType, src_file: &str, target_file: &str) -> Result<String, SodaError> {
    tracing::debug!(
        "transfer_by_mode mode = {:?},src_file_exist = {},  src_file = {:?}, target_file = {:?}",
        mode,
        Path::new(src_file).exists(),
        src_file,
        target_file
    );
    match mode {
        TransferType::HardLink => {
            return match fs::hard_link(src_file, target_file) {
                Ok(()) => Ok(target_file.to_string()),
                Err(e) => Err(SodaError::String(format!("Failed to create hard link: {}", e))),
            };
        }
        TransferType::SymbolLink => {
            return match create_symlink(src_file, target_file) {
                Ok(()) => Ok(target_file.to_string()),
                Err(e) => Err(SodaError::String(format!("Failed to create soft link: {}", e))),
            };
        }
        TransferType::Copy => {
            return match fs::copy(src_file, target_file) {
                Ok(_) => Ok(target_file.to_string()),
                Err(e) => Err(SodaError::String(format!("Failed to move file: {}", e))),
            };
        }
        TransferType::Move => {
            return match fs::copy(src_file, target_file) {
                Ok(_) => {
                    fs::remove_file(src_file).unwrap_or_else(|e| {
                        tracing::error!("Failed to remove file: {}", e);
                    });
                    Ok(target_file.to_string())
                }
                Err(e) => Err(SodaError::String(format!("Failed to move file: {}", e))),
            };
        }
    }
}

fn create_symlink(src_path: &str, target_path: &str) -> Result<(), std::io::Error> {
    // 使用条件编译来根据平台选择不同的符号链接函数
    #[cfg(target_os = "windows")]
    {
        // 在 Windows 上使用 std::os::windows::fs::symlink_file
        std::os::windows::fs::symlink_file(src_path, target_path)?;
    }
    #[cfg(not(target_os = "windows"))]
    {
        // 在 Unix-like 系统上使用 std::os::unix::fs::symlink
        std::os::unix::fs::symlink(src_path, target_path)?;
    }

    Ok(())
}

#[cfg(test)]
mod transfer_tests {
    use tracing::level_filters::LevelFilter;
    use tracing_subscriber::{fmt::format::FmtSpan, EnvFilter};

    use crate::soda::{
        entity::MetaContext,
        meta::{self},
    };

    use crate::soda::fs::metadata;

    use super::*;

    static mut IS_TRACING_INIT: bool = false;

    /// 初始化日志配置
    fn init_tracing() {
        unsafe {
            if !IS_TRACING_INIT {
                IS_TRACING_INIT = true;
                tracing_subscriber::fmt()
                    .with_env_filter(EnvFilter::from_default_env().add_directive(LevelFilter::DEBUG.into()))
                    .with_span_events(FmtSpan::FULL)
                    .init();
            } else {
            }
        }
    }

    #[test]
    fn test_generate_transfer_path() {
        init_tracing();

        let mut scrape_context = MetaContext::new();
        scrape_context.init("凡人修仙传.The.Mortal.Ascention.2020.S01E01.2160p.WEB-DL.H264.AAC-OurTV.mp4");
        let metadata = meta::create_metadata_mt(&mut scrape_context).unwrap();
        let ret = gen_mt_rename_path(
            "$title_cn$.$title_en$.$year$.$season$$episode$.$resolution$.$source$.$video_codec$.$audio_codec$.$extension$",
            &metadata,
        );
        assert_eq!(ret, "凡人修仙传.The.Mortal.Ascention.2020.S01E01.2160p.WEB-DL.H.264.AAC.mp4")
    }

    #[test]
    fn test_generate_transfer_path1() {
        init_tracing();

        let mut scrape_context = MetaContext::new();
        scrape_context.init("凡人修仙传.The.Mortal.Ascention.2020.S01E01.2160p.WEB-DL.H264.AAC-OurTV.mp4");
        let metadata = meta::create_metadata_mt(&mut scrape_context).unwrap();
        let ret = gen_mt_rename_path("$title_cn$.$title_en$.$year$/$title_cn$.$title_en$.$year$.$season$.$resolution$.$source$.$video_codec$.$audio_codec$/$title_cn$.$title_en$.$year$.$season$$episode$.$resolution$.$source$.$video_codec$.$audio_codec$.$extension$", &metadata);
        assert_eq!(ret, "凡人修仙传.The.Mortal.Ascention.2020/凡人修仙传.The.Mortal.Ascention.2020.S01.2160p.WEB-DL.H.264.AAC/凡人修仙传.The.Mortal.Ascention.2020.S01E01.2160p.WEB-DL.H.264.AAC.mp4")
    }

    #[test]
    fn test_generate_transfer_path2() {
        init_tracing();

        let mut scrape_context = MetaContext::new();
        scrape_context.init("凡人修仙传.The.Mortal.Ascention.2020.S01E01.2160p.mp4");
        let metadata = meta::create_metadata_mt(&mut scrape_context).unwrap();
        let ret = gen_mt_rename_path("$title_cn$.$title_en$.$year$/$title_cn$.$title_en$.$year$.$season$.$resolution$.$source$.$video_codec$.$audio_codec$/$title_cn$.$title_en$.$year$.$season$$episode$.$resolution$.$source$.$video_codec$.$audio_codec$.$extension$", &metadata);
        assert_eq!(ret, "凡人修仙传.The.Mortal.Ascention.2020/凡人修仙传.The.Mortal.Ascention.2020.S01.2160p/凡人修仙传.The.Mortal.Ascention.2020.S01E01.2160p.mp4")
    }

    #[test]
    fn test_generate_transfer_path3() {
        init_tracing();

        let mut scrape_context = MetaContext::new();
        scrape_context.init("Friends.S01E02.1080p.BluRay.Remux.AVC.AC3-WhaleHu.mkv");
        let metadata = meta::create_metadata_mt(&mut scrape_context).unwrap();
        let ret = gen_mt_rename_path(
            "$title_cn$.$title_en$.$year$.$season$$episode$.$resolution$.$source$.$video_codec$.$audio_codec$.$extension$",
            &metadata,
        );
        assert_eq!(ret, "Friends.S01E02.1080p.BluRay.Remux.H.264.AC3.mkv")
    }

    #[test]
    fn test_generate_transfer_path4() {
        init_tracing();

        let mut scrape_context = MetaContext::new();
        scrape_context.init("The.Long.Season.E01.2023.1080p.WEBrip.x265.10bit.AC3￡cXcY@FRDS.mkv");
        let metadata = meta::create_metadata_mt(&mut scrape_context).unwrap();
        let ret = gen_mt_rename_path(
            "$title_cn$.$title_en$.$year$.$season$$episode$.$resolution$.$source$.$video_codec$.$audio_codec$.$extension$",
            &metadata,
        );
        assert_eq!(ret, "The.Long.Season.2023.E01.1080p.WEBrip.H.265.AC3.mkv")
    }

    #[test]
    fn test_generate_transfer_path5() {
        init_tracing();

        let mut scrape_context = MetaContext::new();
        scrape_context.init("大侠霍元甲.Fearless.2020.E01.2160P.WEB-DL.H265.AAC-HDHWEB.mp4");
        let metadata = meta::create_metadata_mt(&mut scrape_context).unwrap();
        let ret = gen_mt_rename_path(
            "$title_cn$.$title_en$.$year$.$season$$episode$.$resolution$.$source$.$video_codec$.$audio_codec$.$extension$",
            &metadata,
        );
        assert_eq!(ret, "大侠霍元甲.Fearless.2020.E01.2160p.WEB-DL.H.265.AAC.mp4")
    }
}
