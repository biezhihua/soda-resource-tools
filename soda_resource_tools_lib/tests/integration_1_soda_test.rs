#[cfg(test)]
mod soda_tests {
    use std::path::PathBuf;

    use soda_resource_tools_lib::soda;
    use soda_resource_tools_lib::soda::entity::ScrapeConfig;
    use tracing::level_filters::LevelFilter;
    use tracing_subscriber::fmt::format::FmtSpan;
    use tracing_subscriber::EnvFilter;

    fn tests_dir() -> std::path::PathBuf {
        std::env::current_dir().unwrap().join("tests")
    }

    #[test]
    fn test_scrape_tv_1() {
        test_scrape_tv(
            "test_scrape_tv_1",
            "Moving.S01E01.2023.1080p.WEB-DL.x265.10bit.AC3￡cXcY@FRDS.mkv",
            "超异能族.Moving.2023",
            "超异能族.Moving.2023.S01.1080p.WEB-DL.H.265.AC3-cXcY@FRDS",
            "超异能族.Moving.2023.S01E01.1080p.WEB-DL.H.265.AC3-cXcY@FRDS.mkv",
        )
    }

    #[test]
    fn test_scrape_tv_2() {
        // 测试 - 仅在第一季按照发布年份查找
        test_scrape_tv(
            "test_scrape_tv_2",
            "The.It.Crowd.S02E01.2007.1080p.WEBrip.x265.10bit.AC3￡cXcY@FRDS.mkv",
            "IT狂人.The.IT.Crowd.2006",
            "IT狂人.The.IT.Crowd.2007.S02.1080p.WEBrip.H.265.AC3-cXcY@FRDS",
            "IT狂人.The.IT.Crowd.2007.S02E01.1080p.WEBrip.H.265.AC3-cXcY@FRDS.mkv",
        )
    }

    #[test]
    fn test_scrape_tv_3() {
        // 测试 - 移除名字中的特殊字符 - ,
        test_scrape_tv(
            "test_scrape_tv_3",
            "Yes,Minister.S01E07.1980.1080p.WEBrip.x265.10bit.AC3￡cXcY@FRDS.mkv",
            "是，大臣.Yes,Minister.1980",
            "是，大臣.Yes,Minister.1980.S01.1080p.WEBrip.H.265.AC3-cXcY@FRDS",
            "是，大臣.Yes,Minister.1980.S01E07.1080p.WEBrip.H.265.AC3-cXcY@FRDS.mkv",
        )
    }

    #[test]
    fn test_scrape_tv_4() {
        // 测试 - 特殊符号移除
        test_scrape_tv(
            "test_scrape_tv_4",
            "[壹高清]你喜欢勃拉姆斯吗.Do You Like Brahms.Ep01.HDTV.720p.H264-OneHD.mkv",
            "你喜欢勃拉姆斯吗.Do.You.Like.Brahms.2020",
            "你喜欢勃拉姆斯吗.Do.You.Like.Brahms.2020.S01.720p.HDTV.H.264-OneHD",
            "你喜欢勃拉姆斯吗.Do.You.Like.Brahms.2020.S01E01.720p.HDTV.H.264-OneHD.mkv",
        )
    }

    #[test]
    fn test_scrape_tv_5() {
        // 测试 - 特殊符号移除
        test_scrape_tv(
            "test_scrape_tv_5",
            "The.Witcher.Blood.Origin.S01E01.2022.Netflix.WEB-DL.1080p.x264.DDP-HDCTV.mkv",
            "猎魔人：血源.The.Witcher.Blood.Origin.2022",
            "猎魔人：血源.The.Witcher.Blood.Origin.2022.S01.1080p.WEB-DL.H.264.DDP-HDCTV",
            "猎魔人：血源.The.Witcher.Blood.Origin.2022.S01E01.1080p.WEB-DL.H.264.DDP-HDCTV.mkv",
        )
    }

    #[test]
    fn test_scrape_tv_6() {
        // 测试 - 特殊符号移除
        test_scrape_tv(
            "test_scrape_tv_6",
            "SAS.Rogue.Heroes.S01E02.2022.1080p.Blu-ray.x265.10bit.AC3￡cXcY@FRDS.mkv",
            "SAS：叛逆勇士.SAS.Rogue.Heroes.2022",
            "SAS：叛逆勇士.SAS.Rogue.Heroes.2022.S01.1080p.BluRay.H.265.AC3-cXcY@FRDS",
            "SAS：叛逆勇士.SAS.Rogue.Heroes.2022.S01E02.1080p.BluRay.H.265.AC3-cXcY@FRDS.mkv",
        )
    }

    #[test]
    fn test_scrape_tv_7() {
        // 测试 - 第一季的年份不准确导致查询失败
        // 测试 - 第一季的年份不准确则查询成功后合并季年份
        test_scrape_tv(
            "test_scrape_tv_7",
            "Sneaky.Pete.S01E01.2017.1080p.Blu-ray.x265.10bit.AC3￡cXcY@FRDS.mkv",
            "诈欺担保人.Sneaky.Pete.2015",
            "诈欺担保人.Sneaky.Pete.2015.S01.1080p.BluRay.H.265.AC3-cXcY@FRDS",
            "诈欺担保人.Sneaky.Pete.2015.S01E01.1080p.BluRay.H.265.AC3-cXcY@FRDS.mkv",
        )
    }

    #[test]
    fn test_scrape_tv_8() {
        test_scrape_tv(
            "test_scrape_tv_8",
            "Friends.S01E01.1080p.BluRay.Remux.AVC.AC3-WhaleHu.mkv",
            "老友记.Friends.1994",
            "老友记.Friends.1994.S01.1080p.BluRay.Remux.H.264.AC3-WhaleHu",
            "老友记.Friends.1994.S01E01.1080p.BluRay.Remux.H.264.AC3-WhaleHu.mkv",
        )
    }

    #[test]
    fn test_scrape_tv_9() {
        test_scrape_tv(
            "test_scrape_tv_9",
            "Friends.S02E01.1080p.BluRay.Remux.AVC.AC3-WhaleHu.mkv",
            "老友记.Friends.1994",
            "老友记.Friends.1995.S02.1080p.BluRay.Remux.H.264.AC3-WhaleHu",
            "老友记.Friends.1995.S02E01.1080p.BluRay.Remux.H.264.AC3-WhaleHu.mkv",
        )
    }

    #[test]
    fn test_scrape_tv_10() {
        test_scrape_tv(
            "test_scrape_tv_10",
            "JSTV.The.Guardian.E01.2022.HDTV.1080i.H264-HDCTV.mkv",
            "护卫者.The.Guardian.2022",
            "护卫者.The.Guardian.2022.S01.1080i.HDTV.H.264-HDCTV",
            "护卫者.The.Guardian.2022.S01E01.1080i.HDTV.H.264-HDCTV.mkv",
        )
    }

    #[test]
    fn test_scrape_tv_11() {
        test_scrape_tv(
            "test_scrape_tv_11",
            "大侠霍元甲.Fearless.2020.E01.2160P.WEB-DL.H265.AAC-HDHWEB.mp4",
            "大侠霍元甲.Fearless.2020",
            "大侠霍元甲.Fearless.2020.S01.2160p.WEB-DL.H.265.AAC-HDHWEB",
            "大侠霍元甲.Fearless.2020.S01E01.2160p.WEB-DL.H.265.AAC-HDHWEB.mp4",
        )
    }

    #[test]
    fn test_scrape_tv_12() {
        test_scrape_tv(
            "test_scrape_tv_12",
            "宇宙时空之旅.Cosmos.A.SpaceTime.Odyssey.S01E01.Standing.Up.in.the.Milky.Way.2014.BluRay.1080p.x265.10Bit.DTS.5.1.内封中字简繁双语特效-FFans@ws林小凡.mkv",
            "宇宙时空之旅.Cosmos.A.SpaceTime.Odyssey.2014",
            "宇宙时空之旅.Cosmos.A.SpaceTime.Odyssey.2014.S01.1080p.BluRay.H.265.DTS.5.1-FFans@ws林小凡",
            "宇宙时空之旅.Cosmos.A.SpaceTime.Odyssey.2014.S01E01.1080p.BluRay.H.265.DTS.5.1-FFans@ws林小凡.mkv"
        )
    }

    #[test]
    fn test_scrape_tv_13() {
        test_scrape_tv(
            "test_scrape_tv_13",
            "宇宙时空之旅.Cosmos.A.SpaceTime.Odyssey.S01E01.Standing.Up.in.the.Milky.Way.2014.BluRay.1080p.x265.10Bit.DTS.5.1.内封中字简繁双语特效-FFans@ws林小凡.mkv",
            "宇宙时空之旅.Cosmos.A.SpaceTime.Odyssey.2014",
            "宇宙时空之旅.Cosmos.A.SpaceTime.Odyssey.2014.S01.1080p.BluRay.H.265.DTS.5.1-FFans@ws林小凡",
            "宇宙时空之旅.Cosmos.A.SpaceTime.Odyssey.2014.S01E01.1080p.BluRay.H.265.DTS.5.1-FFans@ws林小凡.mkv"
        )
    }

    #[test]
    fn test_scrape_tv_14() {
        test_scrape_tv(
            "test_scrape_tv_14",
            "City.of.Angels.City.of.Death.S01E02.2021.Disney+.WEB-DL.1080p.H264.DDP-HDCTV.mkv",
            "City.of.Angels.City.of.Death.2021",
            "City.of.Angels.City.of.Death.2021.S01.1080p.WEB-DL.H.264.DDP-HDCTV",
            "City.of.Angels.City.of.Death.2021.S01E02.1080p.WEB-DL.H.264.DDP-HDCTV.mkv",
        )
    }

    #[test]
    fn test_scrape_tv_15() {
        test_scrape_tv(
            "test_scrape_tv_15",
            "Skam.S1E01.你看起来像个荡女彐.中挪字幕.WEBRip.1080P.不着调字幕组.mkv",
            "羞耻.SKAM.2015",
            "羞耻.SKAM.2015.S01.1080p.WEBRip-不着调字幕组",
            "羞耻.SKAM.2015.S01E01.1080p.WEBRip-不着调字幕组.mkv",
        );

        test_scrape_tv(
            "test_scrape_tv_15",
            "Skam.S2E02.你对一个朋友撒谎却怪罪于我.SweSub.1080p.WEB-DL.H264.mp4",
            "羞耻.SKAM.2015",
            "羞耻.SKAM.2016.S02.1080p.WEB-DL.H.264",
            "羞耻.SKAM.2016.S02E02.1080p.WEB-DL.H.264.mp4",
        );
    }

    #[test]
    fn test_scrape_tv_16() {
        // 电视剧多季解析成了不同的电视剧
        test_scrape_tv(
            "test_scrape_tv_16",
            "Suits.S01E01.2018.1080p.WEBrip.x265.10bit.AC3￡cXcY@FRDS.mkv",
            "金装律师（JP）.Suits.2018",
            "金装律师（JP）.Suits.2018.S01.1080p.WEBrip.H.265.AC3-cXcY@FRDS",
            "金装律师（JP）.Suits.2018.S01E01.1080p.WEBrip.H.265.AC3-cXcY@FRDS.mkv",
        );

        // 电视剧多季解析成了不同的电视剧
        // test_scrape_tv(
        //     "test_scrape_tv_16",
        //     "Suits.S02E01.2020.1080p.WEBrip.x265.10bit.AC3￡cXcY@FRDS.mkv",
        //     "金装律师（JP）.Suits.2018",
        //     "金装律师（JP）.Suits.2020.S02.1080p.WEBrip.H.265.AC3-cXcY@FRDS",
        //     "金装律师（JP）.Suits.2020.S02E01.1080p.WEBrip.H.265.AC3-cXcY@FRDS.mkv",
        // )
    }

    #[test]
    fn test_scrape_tv_17() {
        test_scrape_tv(
            "test_scrape_tv_17",
            "AOD.百万同居计划.Million Dollar Family.Ep01.HDTV.1080p.H264-OneHD.mkv",
            "百万同居计划.Million.Dollar.Family.2022",
            "百万同居计划.Million.Dollar.Family.2022.S01.1080p.HDTV.H.264-OneHD",
            "百万同居计划.Million.Dollar.Family.2022.S01E01.1080p.HDTV.H.264-OneHD.mkv",
        );
    }

    #[test]
    fn test_scrape_tv_18() {
        test_scrape_tv(
            "test_scrape_tv_18",
            "A.Perfect.Planet.S01E01.BluRay.2160p.Atmos.TrueHD.7.1.HDR.x265.10bit-CHD.mkv",
            "完美星球.A.Perfect.Planet.2021",
            "完美星球.A.Perfect.Planet.2021.S01.2160p.BluRay.H.265.Atmos.TrueHD.7.1-CHD",
            "完美星球.A.Perfect.Planet.2021.S01E01.2160p.BluRay.H.265.Atmos.TrueHD.7.1-CHD.mkv",
        );
    }

    #[test]
    fn test_scrape_tv_19() {
        test_scrape_tv(
            "test_scrape_tv_19",
            "Taiwan.Taste.S01E07.2012.1080p.KKTV.WEB-DL.H264.AAC-HHWEB.mkv",
            "Taiwan.Taste",
            "Taiwan.Taste.2012.S01.1080p.WEB-DL.H.264.AAC-HHWEB",
            "Taiwan.Taste.2012.S01E07.1080p.WEB-DL.H.264.AAC-HHWEB.mkv",
        );
    }

    #[test]
    fn test_scrape_tv_20() {
        let target = PathBuf::new()
            .join("Lost.S01-S06.COMPLETE.2004-2010.1080p.Blu-ray.x265.10bits.DTS.5.1-PTer")
            .join("S01-PTer")
            .join("S01E02-PTer.mkv");
        test_scrape_tv2(
            "test_scrape_tv_20",
            &target,
            "迷失.Lost.2004",
            "迷失.Lost.2004.S01-PTer",
            "迷失.Lost.2004.S01E02-PTer.mkv",
        );
    }

    #[test]
    fn test_scrape_tv_21() {
        test_scrape_tv(
            "test_scrape_tv_21",
            "Yan.Huo.Shi.Wei.2021.E01.1080p.WEB-DL.H264.AAC-LeagueWEB.mp4",
            "烟火拾味.Yan.Huo.Shi.Wei.2021",
            "烟火拾味.Yan.Huo.Shi.Wei.2021.S01.1080p.WEB-DL.H.264.AAC-LeagueWEB",
            "烟火拾味.Yan.Huo.Shi.Wei.2021.S01E01.1080p.WEB-DL.H.264.AAC-LeagueWEB.mp4",
        );
    }

    #[test]
    fn test_scrape_tv_22() {
        test_scrape_tv(
            "test_scrape_tv_22",
            "The.College.Entrance.Exam.2015.E01.WEB-DL.1080p.H264.AAC-PTerWEB.mp4",
            "高考.The.College.Entrance.Exam.2015",
            "高考.The.College.Entrance.Exam.2015.S01.1080p.WEB-DL.H.264.AAC-PTerWEB",
            "高考.The.College.Entrance.Exam.2015.S01E01.1080p.WEB-DL.H.264.AAC-PTerWEB.mp4",
        );
    }

    #[test]
    fn test_scrape_tv_23() {
        test_scrape_tv(
            "test_scrape_tv_23",
            "[易中天品三国01大江东去].Yi.Zhong.Tian.Pin.San.Guo.E01.2006.DVDRip.576p.x264.AC3-CMCT.mkv",
            "易中天品三国.Yi.Zhong.Tian.Pin.San.Guo.2006",
            "易中天品三国.Yi.Zhong.Tian.Pin.San.Guo.2006.S01.576p.DVDRip.H.264.AC3-CMCT",
            "易中天品三国.Yi.Zhong.Tian.Pin.San.Guo.2006.S01E01.576p.DVDRip.H.264.AC3-CMCT.mkv",
        );
    }

    #[test]
    fn test_scrape_tv_24() {
        test_scrape_tv(
            "test_scrape_tv_24",
            "The.Greed.of.Man.E01.1992.1080p.WEBrip.x265.10bit.AC3￡cXcY@FRDS.mkv",
            "大时代.The.Greed.of.Man.1992",
            "大时代.The.Greed.of.Man.1992.S01.1080p.WEBrip.H.265.AC3-cXcY@FRDS",
            "大时代.The.Greed.of.Man.1992.S01E01.1080p.WEBrip.H.265.AC3-cXcY@FRDS.mkv",
        );
    }

    #[test]
    fn test_scrape_tv_25() {
        test_scrape_tv(
            "test_scrape_tv_25",
            "The.Ivory.Tower.S01E01.2003.2160p.WEB-DL.H264.60fps.AAC-HHWEB.mp4",
            "白色巨塔.The.Great.White.Tower.2003",
            "白色巨塔.The.Great.White.Tower.2003.S01.2160p.WEB-DL.H.264.AAC-HHWEB",
            "白色巨塔.The.Great.White.Tower.2003.S01E01.2160p.WEB-DL.H.264.AAC-HHWEB.mp4",
        )
    }

    #[test]
    fn test_scrape_tv_26() {
        test_scrape_tv(
            "test_scrape_tv_26",
            "24小时.S01E01.2001.1080p.Blu-ray.x265.10bit.AC3￡cXcY@FRDS.mkv",
            "24.2001",
            "24.2001.S01.1080p.BluRay.H.265.AC3-cXcY@FRDS",
            "24.2001.S01E01.1080p.BluRay.H.265.AC3-cXcY@FRDS.mkv",
        )
    }

    #[test]
    fn test_scrape_tv_27() {
        test_scrape_tv(
            "test_scrape_tv_27",
            "与摩根·弗里曼一起穿越虫洞.Through.The.Wormhole.With.Morgan.Freeman.2010.S01E05.4K.WEB-DL.H265.AAC-PTerWEB.mp4",
            "与摩根·弗里曼一起穿越虫洞.Through.the.Wormhole.2010",
            "与摩根·弗里曼一起穿越虫洞.Through.the.Wormhole.2010.S01.2160p.WEB-DL.H.265.AAC-PTerWEB",
            "与摩根·弗里曼一起穿越虫洞.Through.the.Wormhole.2010.S01E05.2160p.WEB-DL.H.265.AAC-PTerWEB.mp4",
        )
    }

    #[test]
    fn test_scrape_tv_28() {
        test_scrape_tv(
            "test_scrape_tv_28",
            "History.of.the.World.2008.S01E02.2160p.WEB-DL.H265.AAC-LeagueWEB.mp4",
            "世界历史.History.of.the.World.2008",
            "世界历史.History.of.the.World.2008.S01.2160p.WEB-DL.H.265.AAC-LeagueWEB",
            "世界历史.History.of.the.World.2008.S01E02.2160p.WEB-DL.H.265.AAC-LeagueWEB.mp4",
        )
    }

    #[test]
    fn test_scrape_movie_1() {
        test_scrape_movie(
            "test_scrape_movie_1",
            "Spider-Man.Across.the.Spider-Verse.2023.2160p.MA.WEB-DL.DDP5.1.Atmos.DV.HDR.H.265-FLUX.mkv",
            "蜘蛛侠：纵横宇宙.Spider.Man.Across.the.Spider.Verse.2023.2160p.WEB-DL.H.265.DDP.5.1.Atmos",
            "蜘蛛侠：纵横宇宙.Spider.Man.Across.the.Spider.Verse.2023.2160p.WEB-DL.H.265.DDP.5.1.Atmos.mkv",
        );
    }

    #[test]
    fn test_scrape_movie_2() {
        test_scrape_movie(
            "test_scrape_movie_2",
            "To.the.Wonder.2012.BluRay.1080p.DTS-HDMA.7.1.AVC.REMUX-FraMeSToR-4P.mkv",
            "通往仙境.To.the.Wonder.2012.1080p.BluRay.H.264.DTS-HDMA.7.1",
            "通往仙境.To.the.Wonder.2012.1080p.BluRay.H.264.DTS-HDMA.7.1.mkv",
        );
    }

    #[test]
    fn test_scrape_movie_3() {
        test_scrape_movie(
            "test_scrape_movie_3",
            "A.Chinese.Odyssey.Part.2.1995.BluRay.1080p.x265.10bit.2Audio.MNHD-FRDS.mkv",
            "大话西游之仙履奇缘.A.Chinese.Odyssey.Part.Two.Cinderella.1995.1080p.BluRay.H.265.2Audio",
            "大话西游之仙履奇缘.A.Chinese.Odyssey.Part.Two.Cinderella.1995.1080p.BluRay.H.265.2Audio.mkv",
        );
    }

    #[test]
    fn test_scrape_movie_4() {
        test_scrape_movie(
            "test_scrape_movie_4",
            "A.Chinese.Odyssey.Part.1.1995.BluRay.1080p.x265.10bit.2Audio.MNHD-FRDS.mkv",
            "大话西游之月光宝盒.A.Chinese.Odyssey.Part.One.Pandoras.Box.1995.1080p.BluRay.H.265.2Audio",
            "大话西游之月光宝盒.A.Chinese.Odyssey.Part.One.Pandoras.Box.1995.1080p.BluRay.H.265.2Audio.mkv",
        );
    }

    #[test]
    fn test_scrape_movie_5() {
        test_scrape_movie(
            "test_scrape_movie_5",
            "Parasite.AKA.Gisaengchung.2019.BluRay.1080p.x265.10bit.MNHD-FRDS.mkv",
            "寄生虫.Parasite.2019.1080p.BluRay.H.265",
            "寄生虫.Parasite.2019.1080p.BluRay.H.265.mkv",
        );
    }

    #[test]
    fn test_scrape_movie_6() {
        test_scrape_movie(
            "test_scrape_movie_6",
            "Hachi.A.Dog'sTale.2009.BluRay.1080p.x265.10bit.MNHD-FRDS.mkv",
            "忠犬八公的故事.Hachi.A.Dog's.Tale.2009.1080p.BluRay.H.265",
            "忠犬八公的故事.Hachi.A.Dog's.Tale.2009.1080p.BluRay.H.265.mkv",
        );
    }

    // #[test]
    // fn test_scrape_tv_29() {
    //     test_scrape_emby_tv(
    //         "test_scrape_tv_29",
    //         "Rick.and.Morty.S02E01.A.Rickle.in.Time.1080p.BluRay.REMUX.VC-1.TrueHD.5.1-NOGRP.mkv",
    //         "瑞克和莫蒂 (2013)",
    //         "Season 2",
    //         "S02E01.mkv",
    //     )
    // }

    fn test_scrape_tv2(tag: &str, path: &PathBuf, root: &str, season: &str, episode: &str) {
        use soda_resource_tools_lib::soda::entity::{ResourceType, TransferType};

        init_tracing();

        default_lib_config();
        let scrape_config = default_scrape_config();

        let test_dir = tests_dir().join(tag);
        clean_dir(&test_dir);

        create_file(&test_dir.join(path));

        soda::scrape(
            ResourceType::MT,
            TransferType::Copy,
            scrape_config.clone(),
            test_dir.to_str().unwrap().to_string(),
            test_dir.to_str().unwrap().to_string(),
        );

        let target_file: std::path::PathBuf = test_dir.join(root).join(season).join(episode);

        tracing::debug!("test_scrape_tv target_file = {:?} exist = {}", target_file, target_file.exists());

        assert_eq!(true, target_file.exists());

        clean_dir(&test_dir);
    }

    fn test_scrape_emby_tv(tag: &str, path: &str, root: &str, season: &str, episode: &str) {
        use soda_resource_tools_lib::soda::entity::{ResourceType, TransferType};

        init_tracing();

        emby_lib_config();
        let scrape_config = default_scrape_config();

        let test_dir = tests_dir().join(tag);
        clean_dir(&test_dir);

        create_file(&test_dir.join(path));

        soda::scrape(
            ResourceType::MT,
            TransferType::Copy,
            scrape_config.clone(),
            test_dir.to_str().unwrap().to_string(),
            test_dir.to_str().unwrap().to_string(),
        );

        let target_file: std::path::PathBuf = test_dir.join(root).join(season).join(episode);

        tracing::debug!("test_scrape_tv target_file = {:?} exist = {}", target_file, target_file.exists());

        assert_eq!(true, target_file.exists());

        clean_dir(&test_dir);
    }

    fn test_scrape_tv(tag: &str, path: &str, root: &str, season: &str, episode: &str) {
        use soda_resource_tools_lib::soda::entity::{ResourceType, TransferType};

        init_tracing();

        default_lib_config();
        let scrape_config = default_scrape_config();

        let test_dir = tests_dir().join(tag);
        clean_dir(&test_dir);

        create_file(&test_dir.join(path));

        soda::scrape(
            ResourceType::MT,
            TransferType::Copy,
            scrape_config.clone(),
            test_dir.to_str().unwrap().to_string(),
            test_dir.to_str().unwrap().to_string(),
        );

        let target_file: std::path::PathBuf = test_dir.join(root).join(season).join(episode);

        tracing::debug!("test_scrape_tv target_file = {:?} exist = {}", target_file, target_file.exists());

        assert_eq!(true, target_file.exists());

        clean_dir(&test_dir);
    }

    fn test_scrape_movie(tag: &str, title: &str, root: &str, movie: &str) {
        use soda_resource_tools_lib::soda::entity::{ResourceType, TransferType};

        init_tracing();

        default_lib_config();
        let scrape_config = default_scrape_config();

        let test_dir = tests_dir().join(tag);
        clean_dir(&test_dir);

        create_file(&test_dir.join(title));

        soda::scrape(
            ResourceType::MT,
            TransferType::Copy,
            scrape_config.clone(),
            test_dir.to_str().unwrap().to_string(),
            test_dir.to_str().unwrap().to_string(),
        );

        let target_file: std::path::PathBuf = test_dir.join(root).join(movie);

        assert_eq!(true, target_file.exists());

        clean_dir(&test_dir);
    }

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

    fn clean_dir(dir: &PathBuf) {
        if dir.exists() {
            std::fs::remove_dir_all(dir).unwrap();
        }
    }

    fn create_file(file_path: &PathBuf) {
        if !file_path.exists() {
            let parent = file_path.parent().unwrap();
            if !parent.exists() {
                std::fs::create_dir_all(parent).unwrap();
            }
            std::fs::File::create(file_path).unwrap();
        }
    }

    fn default_scrape_config() -> ScrapeConfig {
        let mut config = ScrapeConfig::new();
        config.enable_scrape_image = false;
        config.enable_recognize = true;
        config
    }

    fn default_lib_config() {
        let mut config = soda::get_lib_config();
        config.cache_path = tests_dir().join("test_cache").to_string_lossy().to_string();
        config.metadata_skip_special = true;
        config.transfer_rename_format_tv = "$title_cn$.$title_en$.$release_year$/$title_cn$.$title_en$.$year$.$season$.$resolution$.$source$.$video_codec$.$audio_codec$-$release_group$/$title_cn$.$title_en$.$year$.$season$$episode$.$resolution$.$source$.$video_codec$.$audio_codec$-$release_group$.$extension$".to_string();
        config.transfer_rename_format_movie = "$title_cn$.$title_en$.$year$.$resolution$.$source$.$video_codec$.$audio_codec$/$title_cn$.$title_en$.$year$.$resolution$.$source$.$video_codec$.$audio_codec$.$extension$".to_string();
        soda::update_lib_config(config);
    }

    fn emby_lib_config() {
        let mut config = soda::get_lib_config();
        config.cache_path = tests_dir().join("test_cache").to_string_lossy().to_string();
        config.metadata_skip_special = true;
        config.rename_style = Some(soda::entity::RenameStyle::Emby);
        soda::update_lib_config(config);
    }
}
