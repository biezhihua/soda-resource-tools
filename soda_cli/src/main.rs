use std::{
    env::current_dir,
    fs,
    path::{Path, PathBuf},
};

use directories::ProjectDirs;
use serde_json::Value;
use soda_resource_tools_lib::soda::{
    self,
    entity::{RenameStyle, ResourceType, ScrapeConfig, SodaError, TransferType},
};
use tracing_appender::non_blocking::NonBlocking;
use tracing_subscriber::{filter, fmt::time::ChronoLocal, layer::SubscriberExt, util::SubscriberInitExt, Layer};

use clap::builder::TypedValueParser as _;
use clap::Parser;
use clap::Subcommand;

#[derive(Debug, Parser)]
#[command(name = "soda_clix", version = "0.1.1", author = "biezhihua", about = "A media scrape CLI", long_about = None)]
struct Cli {
    /// 开发模式
    #[arg(long)]
    dev: Option<bool>,

    /// 日志路径
    #[arg(long,value_hint = clap::ValueHint::DirPath)]
    log_path: Option<std::path::PathBuf>,

    /// 日志级别
    #[arg(long, default_value = "debug", value_parser = clap::builder::PossibleValuesParser::new(["trace", "debug", "info", "warn", "error"]))]
    log_level: Option<String>,

    /// 缓存路径
    #[arg(long,value_hint = clap::ValueHint::DirPath)]
    cache_path: Option<std::path::PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// 刮削资源
    #[command(arg_required_else_help = true)]
    Scrape {
        /// 媒体类型
        /// mt: 电影和电视剧
        #[arg(
        long,
        default_value_t = ResourceType::MT,
        value_parser = clap::builder::PossibleValuesParser::new(["mt"])
            .map(|s| s.parse::<ResourceType>().unwrap()),
        )]
        resource_type: ResourceType,

        /// 媒体从源目录转移到输出目录的方式
        /// hard_link: 硬链接
        /// symbol_link: 符号链接
        /// copy: 复制
        /// move: 移动
        #[arg(
            long,
            default_value_t = TransferType::HardLink,
            value_parser = clap::builder::PossibleValuesParser::new(["hard_link", "symbol_link", "copy", "move"])
                .map(|s| s.parse::<TransferType>().unwrap()),
            )]
        transfer_type: TransferType,

        /// 刮削图片
        /// true: 刮削图片
        /// false: 不刮削图片
        #[arg(long, default_value_t = true)]
        scrape_image: bool,

        /// 重命名格式
        /// emby: Emby格式
        #[arg( long, default_value_t = RenameStyle::Emby, value_parser = clap::builder::PossibleValuesParser::new(["emby"])
                .map(|s| s.parse::<RenameStyle>().unwrap()),
        )]
        rename_style: RenameStyle,

        /// 媒体源目录
        #[arg(long,value_hint = clap::ValueHint::DirPath)]
        src_dir: Option<std::path::PathBuf>,

        /// 媒体刮削输出目录
        /// 刮削后的文件输出目录，如果不指定则默认为src_dir
        #[arg(long,value_hint = clap::ValueHint::DirPath)]
        target_dir: Option<std::path::PathBuf>,
    },
}

fn main() -> Result<(), SodaError> {
    // 解析命令行参数
    let args = Cli::parse();

    // 开发模式
    let dev = args.dev.unwrap_or(false);

    // 获取配置文件目录
    let proj_dirs = ProjectDirs::from("com", "biezhihua", "soda").unwrap();

    // 创建配置文件目录
    let config_dir = proj_dirs.config_dir();
    fs::create_dir_all(config_dir)?;

    // 创建缓存文件目录
    let mut cache_dir = args.cache_path.unwrap_or(proj_dirs.cache_dir().join("cache"));
    if dev {
        cache_dir = current_dir()?.parent().unwrap().join("soda_resource_tools_lib").join("cache");
    }
    let cache_dir = Path::new(&cache_dir);
    fs::create_dir_all(cache_dir)?;

    // 创建日志文件目录
    let log_dir = if dev {
        cache_dir.join("log")
    } else {
        args.log_path.unwrap_or(proj_dirs.cache_dir().join("log"))
    };
    let log_dir = Path::new(&log_dir);
    clean_dir(&log_dir.to_path_buf());
    fs::create_dir_all(log_dir)?;

    // 配置日志
    let log_level = args.log_level.unwrap_or("info".to_string());

    // 配置日志
    let all_file_appender = tracing_appender::rolling::never(log_dir, "all.log");
    let (all_log, _guard) = tracing_appender::non_blocking(all_file_appender);

    // 配置日志
    let metadata_file_appender = tracing_appender::rolling::never(log_dir, "metadata.log");
    let (metadata_log, _guard) = tracing_appender::non_blocking(metadata_file_appender);

    // 日志初始化
    init_tracing(log_level, all_log, metadata_log);

    tracing::info!(target:"soda::info", "配置文件目录: {}", config_dir.to_str().unwrap());
    tracing::info!(target:"soda::info", "缓存文件目录: {}", cache_dir.to_str().unwrap());
    tracing::info!(target:"soda::info", "日志文件目录: {}", log_dir.to_str().unwrap());

    // 检查网络
    check_internet()?;

    match args.command {
        Commands::Scrape {
            resource_type,
            transfer_type,
            src_dir,
            target_dir,
            scrape_image,
            rename_style,
        } => {
            // 开发者配置
            if dev {
                let lib_dir = current_dir()?.parent().unwrap().join("soda_resource_tools_lib");
                init_lib_config_dev(&lib_dir, &rename_style);
            }
            // Release配置
            else {
                if let Ok(()) = init_config(&config_dir) {
                    tracing::info!(target:"soda::info", "初始化配置文件成功");
                } else {
                    tracing::error!(target:"soda::info", "初始化配置文件失败，请检查网络后重试");
                    return Err(SodaError::Str("初始化配置文件失败，请检查网络后重试"));
                }

                let local_soda_config_path = config_dir.join(SODA_CONFIG_JSON);
                let local_soda_config: Value = serde_json::from_str(&fs::read_to_string(&local_soda_config_path)?)?;
                tracing::info!(target:"soda::info", "配置文件: {}", local_soda_config);

                if !local_soda_config.get("enable_cli").unwrap().as_bool().unwrap() {
                    tracing::error!(target:"soda::info", "配置文件中enable_cli为false，不允许使用soda_cli");
                    return Ok(());
                }

                init_lib_config(&local_soda_config, &config_dir, &cache_dir, &rename_style);
            }

            if src_dir.is_some() {
                let src_dir = src_dir.clone().unwrap();
                let target_dir = if target_dir.is_some() { target_dir.unwrap() } else { src_dir.clone() };
                if src_dir.exists() {
                    if !target_dir.exists() {
                        fs::create_dir_all(target_dir.clone()).unwrap();
                        tracing::info!(target:"soda::info", "创建媒体刮削输出目录: {}", target_dir.to_str().unwrap());
                    }
                    let src_dir = src_dir.to_str().unwrap().to_string();
                    let target_dir = target_dir.to_str().unwrap().to_string();
                    scrape_mt(resource_type, transfer_type, src_dir, target_dir, scrape_image);
                } else {
                    tracing::error!(target:"soda::info", "媒体源目录不存在")
                }
            }
        }
    }

    return Ok(());
}

fn check_internet() -> Result<(), SodaError> {
    tracing::info!(target:"soda::info", "开始检查网络");

    tracing::info!(target:"soda::info", "开始访问: https://raw.githubusercontent.com/biezhihua/soda-resource-tools/main/soda_cli_config/soda_config.json" );
    let _ = reqwest::blocking::get(raw_github_json())?;

    tracing::info!(target:"soda::info", "开始访问: https://api.themoviedb.org" );
    let _ = reqwest::blocking::get(api_themoviedb())?;

    tracing::info!(target:"soda::info", "开始访问: https://webservice.fanart.tv" );
    let _ = reqwest::blocking::get(api_fanart())?;

    return Ok(());
}

fn init_lib_config(local_soda_config: &Value, config_dir: &Path, cache_dir: &Path, rename_style: &RenameStyle) {
    let bin_path_name = local_soda_config.get("bin").unwrap().as_str().unwrap();
    let soda_config_bin_path = config_dir.join(bin_path_name);
    let soda_config_bin_content = fs::read_to_string(&soda_config_bin_path).unwrap();
    let soda_config_bin_json: Value = serde_json::from_str(&soda_config_bin_content).unwrap();

    let mt_strong_match_rules_tv = soda_config_bin_json.get("mt_strong_match_rules_tv").unwrap();
    let mt_strong_match_rules_movie = soda_config_bin_json.get("mt_strong_match_rules_movie").unwrap();
    let mt_strong_match_regex_rules = soda_config_bin_json.get("mt_strong_match_regex_rules").unwrap();
    let mt_strong_match_name_map = soda_config_bin_json.get("mt_strong_match_name_map").unwrap();

    let mut config = soda::get_lib_config();
    config.cache_path = cache_dir.to_str().unwrap().to_string();
    config.strong_match_name_map = serde_json::to_string(mt_strong_match_name_map).unwrap();
    config.strong_match_regex_rules = serde_json::to_string(mt_strong_match_regex_rules).unwrap();
    config.strong_match_rules_tv = serde_json::to_string(mt_strong_match_rules_tv).unwrap();
    config.strong_match_rules_tv_path = "".to_string();
    config.strong_match_rules_movie = serde_json::to_string(mt_strong_match_rules_movie).unwrap();
    config.strong_match_rules_movie_path = "".to_string();
    config.strong_match_regex_rules_path = "".to_string();
    config.strong_match_name_map_path = "".to_string();
    config.metadata_skip_special = true;
    config.rename_style = Some(rename_style.clone());
    soda::update_lib_config(config);
    tracing::info!(target:"soda::info", "配置更新成功");
}

fn init_lib_config_dev(lib_dir: &Path, rename_style: &RenameStyle) {
    let mut config = soda::get_lib_config();
    config.cache_path = lib_dir.join("cache").to_str().unwrap().to_string();
    config.strong_match_rules_tv_path = lib_dir.join("config").join("mt_strong_match_rules_tv.json").to_str().unwrap().to_string();
    config.strong_match_rules_movie_path = lib_dir
        .join("config")
        .join("mt_strong_match_rules_movie.json")
        .to_str()
        .unwrap()
        .to_string();
    config.strong_match_regex_rules_path = lib_dir
        .join("config")
        .join("mt_strong_match_regex_rules.json")
        .to_str()
        .unwrap()
        .to_string();
    config.strong_match_name_map_path = lib_dir.join("config").join("mt_strong_match_name_map.json").to_str().unwrap().to_string();
    config.rename_style = Some(rename_style.clone());
    config.metadata_skip_special = true;
    soda::update_lib_config(config);
    tracing::info!(target:"soda::info", "配置更新成功");
}

fn init_config(config_dir: &Path) -> Result<(), SodaError> {
    tracing::info!(target:"soda::info", "开始从Github获取配置文件");

    let soda_config_url = format!("{}/{}", raw_github(), SODA_CONFIG_JSON);

    let remote_soda_config: Value = reqwest::blocking::get(soda_config_url)?.json()?;

    tracing::info!(target:"soda::info", "获取配置文件成功: {}", remote_soda_config);

    let local_soda_config_path = config_dir.join(SODA_CONFIG_JSON);
    if !local_soda_config_path.exists() {
        update_soda_config(&local_soda_config_path, &remote_soda_config, config_dir)?;
    } else {
        let local_soda_config: Value = serde_json::from_str(&fs::read_to_string(&local_soda_config_path)?)?;
        let local_version = local_soda_config.get("version").unwrap().as_i64().unwrap();
        let remote_version = remote_soda_config.get("version").unwrap().as_i64().unwrap();
        if local_version < remote_version {
            update_soda_config(&local_soda_config_path, &remote_soda_config, config_dir)?;
        }
    }
    return Ok(());
}

const SODA_CONFIG_JSON: &'static str = "soda_config.json";

fn raw_github() -> &'static str {
    return "https://raw.githubusercontent.com/biezhihua/soda-resource-tools/main/soda_cli_config";
}

fn raw_github_json() -> String {
    return format!("{}/{}", raw_github(), SODA_CONFIG_JSON);
}

fn api_themoviedb() -> &'static str {
    return "https://api.themoviedb.org";
}

fn api_fanart() -> &'static str {
    return "https://webservice.fanart.tv";
}

fn update_soda_config(
    local_soda_config_path: &std::path::PathBuf,
    remote_soda_config: &Value,
    config_dir: &std::path::Path,
) -> Result<(), SodaError> {
    // write config to local
    fs::write(local_soda_config_path, remote_soda_config.to_string())?;

    let bin = remote_soda_config
        .get("bin")
        .ok_or(SodaError::Str("bin字段不存在"))?
        .as_str()
        .ok_or(SodaError::Str("bin字段不是字符串"))?;

    let bin_url = format!("{}/{}", raw_github(), bin);

    tracing::info!(target:"soda::info", "开始从Github获取bin文件: {}", bin_url);

    let mut local_bin_file = fs::File::create(config_dir.join(bin))?;

    reqwest::blocking::get(bin_url)?.copy_to(&mut local_bin_file)?;

    return Ok(());
}

fn init_tracing(log_level: String, all_log: NonBlocking, metadata_log: NonBlocking) {
    let filter = match log_level.as_str() {
        "trace" => filter::LevelFilter::TRACE,
        "debug" => filter::LevelFilter::DEBUG,
        "info" => filter::LevelFilter::INFO,
        "warn" => filter::LevelFilter::WARN,
        "error" => filter::LevelFilter::ERROR,
        _ => filter::LevelFilter::INFO,
    };
    tracing_subscriber::registry()
        .with(
            // 收集INFO级别以上的soda_cli日志输出到控制台
            tracing_subscriber::fmt::layer()
                .with_timer(ChronoLocal::new("%Y-%m-%d %H:%M:%S".to_string()))
                .with_filter(filter::LevelFilter::INFO)
                .with_filter(filter::filter_fn(|metadata| metadata.target().starts_with("soda::info"))),
        )
        .with(
            // 收集DEBUG级别以上的日志到debug.log文件
            tracing_subscriber::fmt::layer()
                .with_timer(ChronoLocal::new("%Y-%m-%d %H:%M:%S".to_string()))
                .with_writer(all_log)
                .with_ansi(false)
                .with_filter(filter),
        )
        .with(
            // 收集INFO级别以上的日志到metadata_log文件
            tracing_subscriber::fmt::layer()
                .with_timer(ChronoLocal::new("%Y-%m-%d %H:%M:%S".to_string()))
                .with_writer(metadata_log)
                .with_ansi(false)
                .with_filter(filter::LevelFilter::INFO)
                .with_filter(filter::filter_fn(|metadata| metadata.target().starts_with("soda::metadata"))),
        )
        .init();
}

fn scrape_mt(resource_type: ResourceType, transfer_type: TransferType, src_dir: String, target_dir: String, scrape_image: bool) {
    tracing::info!(target:"soda::info", "刮削开始: 媒体类型: {:?}, 媒体从源目录转移到输出目录的方式: {:?}, 媒体源目录: {:?}, 媒体刮削输出目录: {:?}",resource_type, transfer_type,   src_dir,  target_dir);

    let mut scrape_config = ScrapeConfig::new();
    scrape_config.enable_scrape_image = scrape_image;
    scrape_config.enable_recognize = true;

    soda::scrape(resource_type, transfer_type, scrape_config, src_dir, target_dir);

    tracing::info!(target:"soda::info", "刮削结束");
}

fn clean_dir(dir: &PathBuf) {
    if dir.exists() {
        std::fs::remove_dir_all(dir).unwrap();
    }
}
