use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::span::{self, Id};

use crate::soda::extension_option::OptionExtensions;

use super::tmdb::entity::{TmdbCast, TmdbCrew, TmdbEpisode, TmdbGenre, TmdbSeason, TmdbSeasonInfo, TmdbTV, TmdbTVInfo};
use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum SodaError {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
    Request(reqwest::Error),
    Json(serde_json::Error),
    Str(&'static str),
    String(String),
}

impl Display for SodaError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            SodaError::Io(ref err) => write!(f, "IO error: {}", err),
            SodaError::Parse(ref err) => write!(f, "Parse error: {}", err),
            SodaError::Request(ref err) => write!(f, "Request error: {}", err),
            SodaError::Json(ref err) => write!(f, "Json error: {}", err),
            SodaError::Str(ref err) => write!(f, "Biz error: {}", err),
            SodaError::String(ref err) => write!(f, "Biz error: {}", err),
        }
    }
}

impl From<String> for SodaError {
    fn from(err: String) -> Self {
        SodaError::String(err)
    }
}

impl From<&'static str> for SodaError {
    fn from(err: &'static str) -> Self {
        SodaError::Str(err)
    }
}

impl From<std::io::Error> for SodaError {
    fn from(err: std::io::Error) -> Self {
        SodaError::Io(err)
    }
}

impl From<std::num::ParseIntError> for SodaError {
    fn from(err: std::num::ParseIntError) -> Self {
        SodaError::Parse(err)
    }
}

impl From<reqwest::Error> for SodaError {
    fn from(err: reqwest::Error) -> Self {
        SodaError::Request(err)
    }
}

impl From<serde_json::Error> for SodaError {
    fn from(err: serde_json::Error) -> Self {
        SodaError::Json(err)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibConfig {
    /// 缓存路径
    pub cache_path: String,
    /// 强匹配规则路径
    pub strong_match_rules_path: String,
    /// 强匹配正则规则路径
    pub strong_match_regex_rules_path: String,
    /// 是否启用强匹配缓存
    pub strong_match_regex_enable_cache: bool,
    /// 是否跳过特典
    pub metadata_skip_special: bool,
    /// 影视 - 电视剧 重命名格式
    pub transfer_rename_format_tv: String,
    /// 影视 - 电影 重命名格式
    pub transfer_rename_format_movie: String,
}

impl LibConfig {
    pub fn update(&mut self, config: LibConfig) {
        tracing::info!("update config = {:?}", config);
        self.cache_path = config.cache_path;
        self.strong_match_rules_path = config.strong_match_rules_path;
        self.strong_match_regex_rules_path = config.strong_match_regex_rules_path;
        self.strong_match_regex_enable_cache = config.strong_match_regex_enable_cache;
        self.metadata_skip_special = config.metadata_skip_special;
        self.transfer_rename_format_tv = config.transfer_rename_format_tv;
        self.transfer_rename_format_movie = config.transfer_rename_format_movie;
    }

    pub fn new() -> LibConfig {
        let current_path = std::env::current_dir().unwrap();
        return LibConfig {
            cache_path: current_path.join("cache").to_str().unwrap().to_string(),
            strong_match_rules_path: current_path.join("config").join("mt_strong_match_rules.json").to_str().unwrap().to_string(),
            strong_match_regex_rules_path: current_path.join("config").join("mt_strong_match_regex_rules.json").to_str().unwrap().to_string(),
            transfer_rename_format_tv: "$title_cn$.$title_en$.$release_year$/$title_cn$.$title_en$.$year$.$season$.$resolution$.$source$.$video_codec$.$audio_codec$/$title_cn$.$title_en$.$year$.$season$$episode$.$resolution$.$source$.$video_codec$.$audio_codec$.$extension$".to_string(),
            transfer_rename_format_movie: "$title_cn$.$title_en$.$year$.$resolution$.$source$.$video_codec$.$audio_codec$.$extension$".to_string(),
            strong_match_regex_enable_cache: false,
            metadata_skip_special: false,
        };
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrapeConfig {
    /// 是否刮削图片
    pub enable_scrape_image: bool,
    /// 是否识别媒体资源
    pub enable_recognize: bool,
}

impl ScrapeConfig {
    pub fn new() -> ScrapeConfig {
        return ScrapeConfig { enable_scrape_image: true, enable_recognize: true };
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MTInfo {
    MOVIE(MovieType),
    TV(TVType),
}

impl MTInfo {
    pub(crate) fn new(tmdb_tv: TmdbTV) -> MTInfo {
        MTInfo::TV(TVType::TMDB(TmdbTVInfo::new(tmdb_tv)))
    }

    pub(crate) fn title(&self) -> &str {
        match self {
            MTInfo::MOVIE(movie) => match movie {
                MovieType::TMDB() => "",
            },
            MTInfo::TV(tv) => match tv {
                TVType::TMDB(tv) => tv.tv.name(),
            },
        }
    }

    pub(crate) fn original_title(&self) -> &str {
        match self {
            MTInfo::MOVIE(movie) => match movie {
                MovieType::TMDB() => "",
            },
            MTInfo::TV(tv) => match tv {
                TVType::TMDB(tv) => tv.tv.original_name(),
            },
        }
    }

    /// TMDB ID
    pub(crate) fn tmdb_id(&self) -> i64 {
        match self {
            MTInfo::MOVIE(movie) => match movie {
                MovieType::TMDB() => -1,
            },
            MTInfo::TV(tv) => match tv {
                TVType::TMDB(tv) => tv.tv.id.clone(),
            },
        }
    }

    pub(crate) fn tvdb_id(&self) -> Option<i64> {
        match self {
            MTInfo::TV(tv) => match tv {
                TVType::TMDB(tv) => {
                    if let Some(external_ids) = &tv.tv.external_ids {
                        return Some(external_ids.tvdb_id());
                    }
                    return None;
                }
            },
            _ => None,
        }
    }

    pub(crate) fn imdb_id(&self) -> Option<&str> {
        match self {
            MTInfo::MOVIE(movie) => match movie {
                MovieType::TMDB() => None,
            },
            MTInfo::TV(tv) => match tv {
                TVType::TMDB(tv) => {
                    if let Some(external_ids) = &tv.tv.external_ids {
                        return Some(external_ids.imdb_id());
                    }
                    return None;
                }
            },
        }
    }

    pub(crate) fn insert_tv_season_episode(&mut self, season_number: i64, episode_number: i64, tmdb_episode: TmdbEpisode) {
        match self {
            MTInfo::TV(tv) => match tv {
                TVType::TMDB(tv) => match tv.tv_seasons.get_mut(&season_number) {
                    Some(season_info) => {
                        season_info.tv_episodes.insert(episode_number, tmdb_episode);
                    }
                    None => {
                        tracing::error!("tv_seasons not found season_number = {}", season_number);
                    }
                },
            },
            _ => {}
        }
    }

    pub(crate) fn insert_tv_season(&mut self, season_number: i64, tmdb_season: TmdbSeason) {
        match self {
            MTInfo::TV(tv) => match tv {
                TVType::TMDB(tv) => {
                    tv.tv_seasons.insert(season_number, TmdbSeasonInfo::new(tmdb_season));
                }
            },
            _ => {}
        }
    }

    pub(crate) fn tmdb_id_str(&self) -> String {
        match self {
            MTInfo::MOVIE(movie) => match movie {
                MovieType::TMDB() => "".to_string(),
            },
            MTInfo::TV(tv) => match tv {
                TVType::TMDB(tv) => tv.tv.id.to_string(),
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MovieType {
    TMDB(),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TVType {
    TMDB(TmdbTVInfo),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MTType {
    MOVIE,
    TV,
}

/// 资源类型
#[derive(Debug)]
pub enum ResourceType {
    /// 影视 MT
    MT,
}

/// The metadata parsed from the soda file name.
///
/// eg: 凡人修仙传.The.Mortal.Ascention.2020.S01E01.2160p.WEB-DL.H264.AAC-OurTV.mp4
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MTMetadata {
    /// origin_title
    ///
    /// eg: 凡人修仙传.The.Mortal.Ascention.2020.S01E01.2160p.WEB-DL.H264.AAC-OurTV.mp4
    ///
    pub origin_title: String,

    /// chinese title
    ///
    /// eg: 凡人修仙传
    ///a
    pub title_cn: String,

    /// english title
    ///
    /// The.Mortal.Ascention
    ///
    pub title_en: String,

    /// year
    ///
    /// 2020
    ///
    pub year: Option<String>,

    /// 发布年，TMDB补充
    pub release_year: Option<String>,

    /// season
    ///
    /// eg: S01
    ///
    /// "S01E01" is a common format used to represent the season and episode number of a television show or series.
    /// Here's the breakdown of its meaning:
    /// - "S01": This indicates Season 1 of the show. It represents the specific season or series of episodes that the particular episode belongs to.
    /// - "E01": This indicates Episode 1 within the specified season. It represents the sequential order of the episode within the season.
    ///
    pub season: String,

    /// episode
    ///
    /// eg: E01
    ///
    pub episode: String,

    /// resolution
    ///
    /// eg: 2160p
    ///
    /// "1080P" refers to a video resolution commonly known as Full HD or 1080p. The "1080" represents the vertical resolution of the video, which is 1080 pixels. The "P" stands for progressive scan, indicating that the video is displayed progressively line by line.
    ///
    /// Other similar text that indicates video resolutions include:
    /// - 720P: This refers to a video resolution of 1280x720 pixels, also known as HD or 720p.
    /// - 4K: This refers to a video resolution of approximately 3840x2160 pixels, also known as Ultra HD or 4K UHD.
    /// - 8K: This refers to a video resolution of approximately 7680x4320 pixels, providing even higher resolution than 4K.
    /// - all resolution text:
    ///     - 240p: 426 x 240 pixels
    ///     - 360p: 640 x 360 pixels
    ///     - 480p: 854 x 480 pixels
    ///     - 720p: 1280 x 720 pixels
    ///     - 1080p: 1920 x 1080 pixels
    ///     - 1440p (2K): 2560 x 1440 pixels
    ///     - 2160p (4K UHD): 3840 x 2160 pixels
    ///     - 4320p (8K UHD): 7680 x 4320 pixels
    ///
    /// what is  the meaning of UHD
    /// UHD stands for "Ultra High Definition." It is a video resolution standard that provides a higher pixel count and improved image quality compared to traditional high-definition (HD) resolutions. UHD typically refers to a resolution of 3840 × 2160 pixels, commonly known as 4K UHD. This higher resolution allows for more detailed and sharper images, providing a more immersive viewing experience for video content.
    ///
    /// These terms are used to describe the clarity and detail of the video image, with higher numbers indicating higher resolutions and finer details.
    ///
    pub resolution: String,

    /// source
    ///
    /// eg: WEB-DL/AMZN WEB-DL
    ///
    /// "WEB-DL" stands for Web Download or Web-Digital Copy. It refers to a video file that has been obtained by directly downloading it from an online source, typically a streaming platform or a digital distribution service. WEB-DL files are often of high quality and can be in various video formats, such as MP4 or MKV, and may include audio codecs like AAC or AC3.
    ///
    /// Other common video formats that you may come across include:
    ///
    /// - BluRay or BD: Refers to a video sourced from a Blu-ray disc, known for its high-quality video and audio.
    /// - DVDRip: Refers to a video that has been ripped or copied from a DVD source.
    /// - HDTVRip: Similar to HDTV, it represents a video that has been ripped or captured from an HDTV broadcast.
    /// - BRRip/BDRip: These terms indicate a video that has been ripped or copied from a Blu-ray disc source, similar to BluRay.
    /// - HDRip: Refers to a video that has been ripped or captured from a source with HDR (High Dynamic Range) content, offering enhanced contrast and color.
    /// - CAM: Represents a video recorded using a handheld camera in a theater during a movie screening. The quality is generally lower in CAM recordings.
    /// - HDTV: Stands for High Definition Television, indicating that the video has been captured or broadcasted in high-definition format.
    ///
    /// - REMUX: "REMUX" is a term commonly used in the video industry, referring to the process of remultiplexing or remuxing video and audio streams without re-encoding them. In other words, it involves extracting the video and audio streams from the original source and repackaging them into a new container file without any compression or re-encoding. This helps to preserve the original quality and characteristics of the video and audio while reducing file size.
    ///     - The purpose of REMUX is typically to create compressed versions while maintaining high quality from high-definition sources, such as Blu-ray. By remuxing, the video and audio can be preserved with minimal loss in quality compared to re-encoding.
    ///     - It's important to note that REMUX is not a specific video encoding format but rather a process of remultiplexing or remuxing video and audio streams. Therefore, the file format of a REMUXed file can vary and depends on the container format used for repackaging, such as MKV, MP4, etc.
    ///
    /// - BluRay.Remux: "BluRay.Remux" refers to a specific type of video release. Let's break down the components:
    ///     - "BluRay": This indicates that the source of the video is a Blu-ray disc, which is a high-definition optical disc format.
    ///     - "Remux": It stands for "remultiplexing" and refers to a process where the video and audio streams from the original Blu-ray disc are extracted and then combined into a new container format, without any re-encoding. The purpose of a remux is to preserve the original video and audio quality while reducing the file size by removing unnecessary data.
    ///
    /// - BDRemux refers to a type of video file that has been created by remuxing (remultiplexing) the content from a Blu-ray Disc (BD) without any loss of quality. It is a process that involves extracting the original video, audio, and subtitle streams from a Blu-ray Disc and then packaging them into a new container format, typically in an MKV (Matroska) or M2TS (MPEG-2 Transport Stream) format.
    ///     - The term "BDRemux" indicates that the video file retains the original video and audio streams directly from the Blu-ray source, without re-encoding or compressing the content. As a result, BDRemux files provide the highest quality available, preserving the original video and audio fidelity of the Blu-ray Disc.
    ///     - BDRemux files are typically large in size because they maintain the original video and audio bitrates and quality. They are preferred by enthusiasts and collectors who desire the best possible video and audio experience from Blu-ray content.
    ///     - It's worth noting that BDRemux files require compatible soda players or devices that can handle the specific container format and codecs used in the remuxed file.
    ///
    /// It's important to note that the availability and usage of these formats may vary, and some may be more common in certain contexts or regions.
    ///
    pub source: String,

    ///
    /// format
    ///
    /// eg: .mp4
    ///
    /// ".mp4" is a file extension that indicates the video is encoded in the MPEG-4 Part 14 format. MPEG-4 is a widely used video compression standard that provides efficient video encoding and is compatible with various devices and platforms. MP4 files can contain both video and audio data.
    ///
    /// Here are some common video format suffixes (file extensions) and their corresponding formats:
    ///
    /// - .mp4: MPEG-4 Part 14 video format
    /// - .avi: Audio Video Interleave format
    /// - .mkv: Matroska multimedia container format
    /// - .mov: QuickTime movie format
    /// - .wmv: Windows Media Video format
    /// - .flv: Flash Video format
    /// - .webm: WebM multimedia container format
    /// - .m4v: MPEG-4 video format (similar to .mp4)
    /// - .3gp: 3GPP multimedia format (commonly used for mobile devices)
    /// - .m2ts: MPEG-2 Transport Stream format (often used for Blu-ray Discs)
    /// - .ogv: Ogg Video format
    /// - .rmvb: RealMedia Variable Bitrate format
    ///
    /// These are just a few examples, and there are many more video formats available. The choice of video format depends on factors such as compatibility, quality, and intended use.
    ///
    pub extension: String,

    /// video_codec
    ///
    /// eg: H264/H265/x264
    ///
    /// H.264, also known as AVC (Advanced Video Coding), is a widely used video compression standard. It is a popular video codec that efficiently compresses video data while maintaining good visual quality. H.264 is supported by a wide range of devices and platforms, making it suitable for various applications such as streaming, video conferencing, and video storage.
    ///
    /// Here are some commonly used video encoder formats:
    ///
    /// - H.264/AVC: Advanced Video Coding, widely used for high-quality video compression.
    /// - H.265/HEVC: High-Efficiency Video Coding, a successor to H.264, providing better compression efficiency and improved video quality.
    /// - VP9: Developed by Google, a video codec designed to provide high-quality video compression with better performance than older codecs like H.264.
    /// - AV1: A royalty-free video codec developed by the Alliance for Open Media (AOMedia), designed to provide efficient compression and high video quality.
    /// - MPEG-2: An older video compression standard commonly used for DVD video and broadcast television.
    /// - MPEG-4: A versatile video compression standard that includes various codecs such as MPEG-4 Part 2 (DivX, Xvid) and MPEG-4 Part 10 (AVC/H.264).
    /// - VC-1: A video codec developed by Microsoft, used in formats like Blu-ray and Windows Media Video (WMV).
    ///
    /// These are some of the commonly used video encoder formats, each with its own characteristics, compression efficiency, and compatibility with different devices and platforms. The choice of encoder format depends on factors such as video quality requirements, device support, and intended usage.
    pub video_codec: String,

    /// audio_codec
    ///
    /// eg: AAC/Atmos.DDP5.1/DTS.2Audios/DD+2.0(Dolby Digital Plus 2.0)
    ///
    /// AAC stands for Advanced Audio Coding. It is a widely used audio encoding format that provides high-quality audio compression. AAC is known for its efficiency in compressing audio data while maintaining good sound quality.
    ///
    /// Here are some commonly used audio encoder formats:
    ///
    /// - MP3 (MPEG-1 Audio Layer 3): One of the most popular audio encoding formats, known for its widespread compatibility and good balance between file size and audio quality.
    /// - AAC (Advanced Audio Coding): Designed to offer better sound quality than MP3 at similar bit rates. It is commonly used for audio streaming, online music services, and various multimedia applications.
    /// - AC3 (Dolby Digital): Developed by Dolby Laboratories, AC3 is a widely used audio coding format for surround sound in movies, DVDs, and digital broadcasting.
    /// - FLAC (Free Lossless Audio Codec): A lossless audio compression format that allows for bit-perfect audio reproduction while reducing file size without any loss in quality. FLAC files are often used for high-quality audio archiving and playback.
    /// - Opus: A highly versatile and efficient audio codec suitable for a wide range of applications, including real-time communication, music streaming, and internet telephony.
    /// - DTS (Digital Theater Systems): A popular audio codec used for surround sound in home theater systems and Blu-ray discs.
    /// - PCM (Pulse Code Modulation): Represents uncompressed audio in raw form, without any compression or encoding. PCM is commonly used in audio CDs and as an intermediate format for audio processing.
    ///
    /// These are some of the commonly used audio encoder formats, each with its own characteristics, compression techniques, and applications. The choice of audio format depends on factors such as desired audio quality, file size considerations, and compatibility with playback devices.
    ///
    /// special format: AVC.FLAC2.0
    /// - AVC.FLAC2.0 refers to the video and audio formats used in a soda file.
    /// - AVC stands for Advanced Video Coding, which is commonly known as H.264. It is a video compression standard widely used for high-quality video encoding. AVC provides efficient compression while maintaining good video quality.
    /// - FLAC stands for Free Lossless Audio Codec. It is an audio compression format that allows for lossless compression, meaning the audio can be compressed without any loss in quality. FLAC is known for its high audio fidelity and is often used for archiving or preserving audio quality.
    /// - 2.0 refers to the audio channels or audio configuration. In this case, 2.0 indicates stereo audio, which means there are two audio channels: left and right.
    /// - So, AVC.FLAC2.0 signifies that the video in the soda file is encoded using AVC (H.264) video compression, and the audio is encoded in FLAC format with stereo (2.0) audio channels.
    ///
    pub audio_codec: String,

    /// source_or_group
    ///
    /// eg: OurTV/PTerWEB/lolice-mix/7³ACG@OurBits
    ///
    ///
    pub release_group: String,

    /// 特典
    /// IT狂人说明书(幕后特辑).The.IT.Crowd.Manual.2014.720p.HDTV.x265.AC3￡cXcY@FRDS
    /// The.IT.Crowd.Manual.2014.720p.HDTV.x265.10bit.AC3￡cXcY@FRDS.mkv
    pub special: String,
}

impl MTMetadata {
    pub fn empty(title: &str) -> MTMetadata {
        return MTMetadata {
            origin_title: title.to_string(),
            title_cn: "".to_string(),
            title_en: "".to_string(),
            year: None,
            season: "".to_string(),
            episode: "".to_string(),
            resolution: "".to_string(),
            source: "".to_string(),
            extension: "".to_string(),
            video_codec: "".to_string(),
            audio_codec: "".to_string(),
            release_group: "".to_string(),
            special: "".to_string(),
            release_year: None,
        };
    }

    pub fn is_movie(&self) -> bool {
        return self.season.is_empty() && self.episode.is_empty();
    }

    pub fn is_tv(&self) -> bool {
        return !self.is_movie();
    }

    pub fn is_empty(&self) -> bool {
        return self.title_cn.is_empty() && self.title_en.is_empty() && self.year.is_none() && self.season.is_empty() && self.episode.is_empty() && self.resolution.is_empty() && self.source.is_empty() && self.extension.is_empty() && self.video_codec.is_empty() && self.audio_codec.is_empty() && self.release_group.is_empty();
    }

    pub fn title(&self) -> &str {
        return if self.title_cn.is_empty() { &self.title_en } else { &self.title_cn };
    }

    pub fn season_number(&self) -> Option<i64> {
        if self.season.is_empty() {
            return None;
        }
        let season_number = self.season.split("S").collect::<Vec<&str>>().get(1).unwrap().to_string().parse::<i64>().expect("season number parse error");
        return Some(season_number);
    }

    pub(crate) fn episode_number(&self) -> Option<i64> {
        if self.episode.is_empty() {
            return None;
        }
        let episode_number = self.episode.split("E").collect::<Vec<&str>>().get(1).unwrap().to_string().parse::<i64>().expect("episode number parse error");
        return Some(episode_number);
    }

    pub(crate) fn merge(&mut self, info: &mut MTInfo) {
        if let MTInfo::TV(tv) = info {
            if let TVType::TMDB(tv) = tv {
                if self.is_tv() {
                    // 如果是TV但是有集无季需要补充信息
                    if let Some(seasons) = &tv.tv.seasons {
                        if seasons.len() == 1 {
                            if let Some(season) = seasons.get(0) {
                                if let Some(season_number) = season.season_number {
                                    self.season = format!("S{:02}", season_number);
                                    tracing::info!("merge season info, name = {} season = {}", self.title(), self.season);
                                }
                            }
                        }
                    }

                    // 如果没有中文名，则合并中文名
                    if self.title_cn.is_empty() && !tv.tv.name().is_empty() {
                        let new_title = tv.tv.name().to_string();
                        tracing::info!("merge title_cn, old_title = {} new_title = {}", self.title_cn, new_title);
                        self.title_cn = new_title;
                    }

                    // 如果没有英文名，则合并英文名
                    if self.title_en.is_empty() && !tv.tv.original_name().is_empty() {
                        let new_title = tv.tv.original_name().to_string();
                        tracing::info!("merge title_en, old_title = {} new_title = {}", self.title_en, new_title);
                        self.title_en = new_title;
                    }

                    // 补充发布年
                    if self.release_year.is_none() && !tv.tv.first_air_date().is_empty() && tv.tv.first_air_date().len() > 4 {
                        self.release_year = Some(tv.tv.first_air_date()[0..4].to_string());
                        tracing::info!("merge release_year, release_year = {:?} first_air_date = {:?}", self.release_year, tv.tv.first_air_date());
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum TransferType {
    /// what is meaning of HardLink?
    ///
    /// A hard link is a file system feature that allows multiple file entries (directory entries) to point to the same underlying data on disk. In other words, it creates multiple references to the same file. Each hard link is essentially a directory entry that points directly to the inode (index node) of the file on the file system.
    ///
    /// Unlike symbolic links (soft links), hard links are not separate files that store the path to the original file. Instead, all hard links are equal and point to the same data on disk. Any changes made to the file content or metadata through one hard link are immediately visible through all other hard links pointing to the same file.
    ///
    /// From the perspective of the file system, there is no distinction between the original file and its hard links. They all share the same file data and are considered equivalent references to that data. If any hard link is deleted, the file data remains intact as long as at least one hard link still exists.
    ///
    /// It's worth noting that hard links can only be created within the same file system, as they rely on the same inode structure.
    HardLink,
    /// what is meaning of soft link?
    /// A symbolic link, also known as a soft link or symlink, is a special type of file that acts as a pointer or reference to another file or directory. Unlike a hard link, a symbolic link is a separate file that contains the path to the target file or directory.
    ///
    /// When you access a symbolic link, the operating system transparently redirects you to the target file or directory. It essentially provides an indirect way to access a file or directory, even if it is located in a different location or has a different name.
    ///
    /// Symbolic links are commonly used for various purposes, such as creating shortcuts, organizing file systems, and providing flexibility in file and directory management. They can be created across different file systems and even across networked machines.
    ///
    /// One important characteristic of symbolic links is that they continue to work even if the target file or directory is moved or renamed. This can be both advantageous and potentially risky, as deleting or moving a target file or directory may result in a broken symbolic link.
    ///
    /// In summary, symbolic links provide a flexible way to reference files or directories by creating a separate file that points to the target. They allow for easy management and management of files and directories within a file system.
    SymbolLink,
    /// 复制
    Copy,
    /// 移动
    Move,
}

#[cfg(test)]
mod entity_tests {}
