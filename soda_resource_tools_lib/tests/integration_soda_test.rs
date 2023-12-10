#[cfg(test)]
mod soda_tests {
    use std::fs;
    use std::path::Path;

    use soda_resource_tools_lib::soda::entity::ScrapeConfig;
    use tracing::level_filters::LevelFilter;
    use tracing_subscriber::fmt::format::FmtSpan;
    use tracing_subscriber::EnvFilter;

    #[test]
    fn test_log() {
        init_tracing();
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn test_run_scrape() {
        use soda_resource_tools_lib::soda::{
            self,
            entity::{ResourceType, TransferType},
        };

        tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env().add_directive(LevelFilter::INFO.into())).with_span_events(FmtSpan::FULL).init();

        let mut config = soda::get_lib_config();
        config.strong_match_regex_enable_cache = true;
        config.metadata_skip_special = true;
        soda::update_lib_config(config);

        let mut scrape_config = ScrapeConfig::new();
        scrape_config.enable_scrape_image = false;
        scrape_config.enable_recognize = true;

        // \\NAS-TANK\downloads_disk8\电视剧
        // let src_directory = "D:/Downloads/Src".to_string();
        let src_directory = "\\\\NAS-TANK\\downloads_disk8\\电视剧".to_string();
        // let src_directory = "\\\\NAS-TANK\\downloads_disk8\\电视剧\\北海鲸梦.The.North.Water.E01-E06.2021.1080p.Blu-ray.x265.DTS￡cXcY@FRDS".to_string();
        // let src_directory = "\\\\NAS-TANK\\downloads_disk8\\电视剧\\The Big Bang Theory S01-S12 2007-2018 BluRay 1080p Remux DTS-HD MA5.1 AVC-Gamma".to_string();
        // let src_directory = "\\\\NAS-TANK\\downloads_disk8\\电视剧\\The Big Bang Theory S01-S12 2007-2018 BluRay 1080p Remux DTS-HD MA5.1 AVC-Gamma\\S10 2016\\The Big Bang Theory S10E17 The Comic-Con Conundrum 1080p BluRay Remux AVC DTS-HD MA-Gamma.mkv".to_string();
        // let src_directory = "\\\\NAS-TANK\\downloads_disk8\\电视剧\\V世代.Gen.V.S01.2023.1080p.AMZN.WEB-DL.H264.DDP5.1-OurTV\\V世代.Gen.V.S01E07.2023.1080p.AMZN.WEB-DL.H264.DDP5.1-OurTV.mkv".to_string();
        let src_directory = "\\\\NAS-TANK\\downloads_disk8\\电视剧\\消失的十一层.The.Lost.11th.Floor.S01.2023.2160p.WEB-DL.H265.DDP5.1-OurTV".to_string();
        let target_directory = "D:/Downloads/Target".to_string();
        soda::scrape(ResourceType::MT, TransferType::SymbolLink, scrape_config, src_directory, target_directory);
        assert_eq!(1, 1);
    }

    #[test]
    fn test_scrape_mt_hardlink_src_to_target() {
        use soda_resource_tools_lib::soda::{
            self,
            entity::{ResourceType, TransferType},
        };

        // init_tracing();

        let current_path = std::env::current_dir().unwrap();
        tracing::info!("current_path = {:?}", current_path);

        let test_dir = current_path.join("tests").join("test_scrape_mt_hardlink_src_to_target");
        let src_directory = test_dir.join("src").to_str().unwrap().to_string();
        let target_directory = test_dir.join("target").to_str().unwrap().to_string();
        clean_target_directory(&target_directory);

        soda::scrape(ResourceType::MT, TransferType::HardLink, ScrapeConfig::new(), src_directory, target_directory.clone());

        let target_root = Path::new(&target_directory).join("凡人修仙传.The.Mortal.Ascention");
        let target_season = target_root.join("凡人修仙传.The.Mortal.Ascention.2020.S01.2160p.WEB-DL.H.264.AAC-OurTV");
        let target_episode_nfo = target_season.join("凡人修仙传.The.Mortal.Ascention.2020.S01E01.2160p.WEB-DL.H.264.AAC-OurTV.nfo");

        assert_eq!(true, Path::new(&target_root.join("tvshow.nfo")).exists());
        assert_eq!(true, Path::new(&target_season.join("season.nfo")).exists());
        assert_eq!(true, Path::new(&target_episode_nfo).exists());

        let target_episode_jpg = target_season.join("凡人修仙传.The.Mortal.Ascention.2020.S01E01.2160p.WEB-DL.H.264.AAC-OurTV.jpg");
        assert_eq!(true, Path::new(&target_root.join("backdrop.jpg")).exists());
        assert_eq!(true, Path::new(&target_root.join("background.jpg")).exists());
        assert_eq!(true, Path::new(&target_root.join("banner.jpg")).exists());
        assert_eq!(true, Path::new(&target_root.join("characterart.png")).exists());
        assert_eq!(true, Path::new(&target_root.join("clearart.png")).exists());
        assert_eq!(true, Path::new(&target_root.join("logo.png")).exists());
        assert_eq!(true, Path::new(&target_root.join("poster.jpg")).exists());
        assert_eq!(true, Path::new(&target_root.join("season01-poster.jpg")).exists());
        assert_eq!(true, Path::new(&target_root.join("thumb.jpg")).exists());
        assert_eq!(true, Path::new(&target_episode_jpg).exists());
    }

    #[test]
    fn test_scrape_mt_hardlink_src() {
        use soda_resource_tools_lib::soda::{
            self,
            entity::{ResourceType, TransferType},
        };

        let current_path = std::env::current_dir().unwrap();
        tracing::info!("current_path = {:?}", current_path);

        let test_dir = current_path.join("tests").join("test_scrape_mt_hardlink_src");
        let src_directory = test_dir.join("src").to_str().unwrap().to_string();
        let target_directory = test_dir.join("target").to_str().unwrap().to_string();
        clean_target_directory(&target_directory);

        soda::scrape(ResourceType::MT, TransferType::HardLink, ScrapeConfig::new(), src_directory.clone(), src_directory.clone());

        let target_root = Path::new(&src_directory).join("凡人修仙传.The.Mortal.Ascention");
        let target_season = target_root.join("凡人修仙传.The.Mortal.Ascention.2020.S01.2160p.WEB-DL.H.264.AAC-OurTV");
        let target_episode_nfo = target_season.join("凡人修仙传.The.Mortal.Ascention.2020.S01E01.2160p.WEB-DL.H.264.AAC-OurTV.nfo");

        assert_eq!(true, Path::new(&target_root.join("tvshow.nfo")).exists());
        assert_eq!(true, Path::new(&target_season.join("season.nfo")).exists());
        assert_eq!(true, Path::new(&target_episode_nfo).exists());
    }

    #[test]
    fn test_scrape_mt_move_src() {
        use soda_resource_tools_lib::soda::{
            self,
            entity::{ResourceType, TransferType},
        };

        let current_path = std::env::current_dir().unwrap();
        tracing::info!("current_path = {:?}", current_path);

        let test_dir = current_path.join("tests").join("test_scrape_mt_move_src");
        let src_directory = test_dir.join("src").to_str().unwrap().to_string();
        let target_directory = test_dir.join("target").to_str().unwrap().to_string();
        let target_root = Path::new(&src_directory).join("凡人修仙传.The.Mortal.Ascention");

        clean_target_directory(&target_directory);
        clean_target_directory(&target_root.to_str().unwrap().to_string());

        let mut scrape_config = ScrapeConfig::new();
        scrape_config.enable_scrape_image = false;
        soda::scrape(ResourceType::MT, TransferType::Move, scrape_config, src_directory.clone(), src_directory.clone());

        let src_file = Path::new(&src_directory).join("凡人修仙传.The.Mortal.Ascention.2020.S01.2160p.WEB-DL.H264.AAC-OurTV").join("凡人修仙传.The.Mortal.Ascention.2020.S01E01.2160p.WEB-DL.H264.AAC-OurTV.mp4");
        assert_eq!(true, !src_file.exists());

        let target_season = target_root.join("凡人修仙传.The.Mortal.Ascention.2020.S01.2160p.WEB-DL.H.264.AAC-OurTV");
        let target_episode_nfo = target_season.join("凡人修仙传.The.Mortal.Ascention.2020.S01E01.2160p.WEB-DL.H.264.AAC-OurTV.nfo");

        assert_eq!(true, Path::new(&target_root.join("tvshow.nfo")).exists());
        assert_eq!(true, Path::new(&target_season.join("season.nfo")).exists());
        assert_eq!(true, Path::new(&target_episode_nfo).exists());

        let target_episode = target_season.join("凡人修仙传.The.Mortal.Ascention.2020.S01E01.2160p.WEB-DL.H.264.AAC-OurTV.mp4");
        fs::rename(target_episode, src_file).unwrap();
    }

    /// 初始化日志配置
    fn init_tracing() {
        tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env().add_directive(LevelFilter::INFO.into())).with_span_events(FmtSpan::FULL).init();
    }

    fn clean_target_directory(target_directory: &str) {
        let target_directory = Path::new(target_directory);
        if target_directory.exists() {
            std::fs::remove_dir_all(target_directory).unwrap();
        }
    }
}
