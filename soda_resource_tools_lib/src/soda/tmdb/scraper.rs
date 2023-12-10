use std::{ops::Index, path::Path};

use tracing_subscriber::fmt::format;
use xml::{writer::XmlEvent, EmitterConfig, EventWriter};

use crate::soda::{
    dom,
    entity::*,
    extension_option::OptionExtensions,
    fanart::entity::FanartTV,
    request,
    utils::{self, str_replace_extension, time},
};

use super::entity::{TmdbEpisode, TmdbSeason, TmdbTV};

pub(crate) fn gen_scrape_files(scrape_config: &ScrapeConfig, mt_meta: &MTMetadata, mt_info: &MTInfo, path: &str) {
    match mt_info {
        MTInfo::MOVIE(_) => todo!(),
        MTInfo::TV(TVType::TMDB(tmdb_tv_info)) => {
            gen_scrape_tv_files(scrape_config, path, tmdb_tv_info, mt_meta);
        }
    }
}

fn gen_scrape_tv_files(scrape_config: &ScrapeConfig, path: &str, tmdb_tv_info: &super::entity::TmdbTVInfo, meta: &MTMetadata) {
    // root
    let tv_root_path = Path::new(path).parent().unwrap().parent().unwrap();

    // gen tvshow.nfo file
    if !tv_root_path.join("tvshow.nfo").exists() {
        gen_tvshow_nfo_file(&tmdb_tv_info.tv, tv_root_path);
    } else {
        tracing::info!("tvshow.nfo file exist, skip gen tvshow.nfo file");
    }

    if scrape_config.enable_scrape_image {
        // save tv images
        tmdb_tv_info.fanart_tv.is_some_then(|fanart_tv| {
            save_tv_show_images(&tmdb_tv_info.tv, fanart_tv, tv_root_path);
        });
    }

    // season
    meta.season_number().is_some_then(|season_number| {
        tmdb_tv_info.tv_seasons.get(season_number).is_some_then(|season_info| {
            let tv_season_path = Path::new(path).parent().unwrap();

            // gen season.nfo file
            if !tv_season_path.join("season.nfo").exists() {
                gen_season_nfo_file(meta, &season_info.tv_season, tv_season_path);
            } else {
                tracing::info!("season.nfo file exist, skip gen season.nfo file");
            }

            if scrape_config.enable_scrape_image {
                // save season images
                save_season_images(&season_info.tv_season, tv_root_path, tv_season_path);
            }
        });
    });

    // episode
    meta.season_number().is_some_then(|season_number| {
        meta.episode_number().is_some_then(|episode_number| {
            tmdb_tv_info.tv_seasons.get(season_number).is_some_then(|season_info| {
                season_info.tv_episodes.get(episode_number).is_some_then(|episode| {
                    let tv_episode_path = Path::new(path);

                    // gen episode.nfo file
                    if tv_episode_path.exists() {
                        gen_episode_nfo_file(meta, season_number, episode_number, episode, tv_episode_path);

                        if scrape_config.enable_scrape_image {
                            // save episode images
                            save_episode_images(episode, tv_episode_path);
                        }
                    } else {
                        tracing::info!("episode file not exist, skip gen episode.nfo file");
                    }
                });
            });
        });
    });
}

fn save_episode_images(episode: &TmdbEpisode, tv_episode_path: &Path) {
    tracing::info!("save episode images, tv_episode_path = {:?}", tv_episode_path);

    episode.still_path.is_some_then(|still_path| {
        let suffix = still_path.split(".").last().unwrap();

        let image_path = tv_episode_path.clone().with_extension(suffix);
        if !image_path.exists() {
            tracing::info!("save still image, image_path = {:?}", image_path);
            request::blocking_get_request_and_download_file(&format!("https://image.tmdb.org/t/p/original{}", still_path), &image_path);
        } else {
            tracing::info!("still image exist, skip save still image, image_path = {:?}", image_path);
        }
    });
}

fn save_season_images(tv_season: &TmdbSeason, tv_root_path: &Path, tv_season_path: &Path) {
    tracing::info!("save season images, tv_season_path = {:?}", tv_season_path);
    tv_season.season_number.is_some_then(|season_number| {
        tv_season.poster_path.is_some_then(|poster_path| {
            let suffix = poster_path.split(".").last().unwrap();
            let url = format!("https://image.tmdb.org/t/p/original{}", poster_path);
            let image_path = tv_root_path.join(format!("season{:02}-poster.{}", season_number, suffix));
            if !image_path.exists() {
                tracing::info!("save poster image, image_path = {:?}", image_path);
                request::blocking_get_request_and_download_file(&url, &image_path);
            } else {
                tracing::info!("poster image exist, skip save poster image, image_path = {:?}", image_path);
            }
        });
    });
}

fn save_tv_show_images(tv: &TmdbTV, fanart_tv: &FanartTV, tv_root_path: &Path) {
    tracing::info!("save tv show images, tv_root_path = {:?}", tv_root_path);

    // banner
    if let Some(banner) = &fanart_tv.tvbanner {
        if let Some(image) = banner.first() {
            let url = image.url();
            if !url.is_empty() {
                let suffix = url.split(".").last().unwrap();
                let image_path = tv_root_path.join(format!("banner.{}", suffix));
                if !image_path.exists() {
                    tracing::info!("save banner image, image_path = {:?}", image_path);
                    request::blocking_get_request_and_download_file(url, &image_path);
                } else {
                    tracing::info!("banner image exist, skip save banner image, image_path = {:?}", image_path);
                }
            }
        }
    }

    // characterart
    if let Some(characterart) = &fanart_tv.characterart {
        if let Some(image) = characterart.first() {
            let url = image.url();
            if !url.is_empty() {
                let suffix = url.split(".").last().unwrap();
                let image_path = tv_root_path.join(format!("characterart.{}", suffix));
                if !image_path.exists() {
                    tracing::info!("save characterart image, image_path = {:?}", image_path);
                    request::blocking_get_request_and_download_file(url, &image_path);
                } else {
                    tracing::info!("characterart image exist, skip save characterart image, image_path = {:?}", image_path);
                }
            }
        }
    }

    // logo
    if let Some(logo) = &fanart_tv.hdtvlogo {
        if let Some(image) = logo.first() {
            let url = image.url();
            if !url.is_empty() {
                let suffix = url.split(".").last().unwrap();
                let image_path = tv_root_path.join(format!("logo.{}", suffix));
                if !image_path.exists() {
                    tracing::info!("save logo image, image_path = {:?}", image_path);
                    request::blocking_get_request_and_download_file(url, &image_path);
                } else {
                    tracing::info!("logo image exist, skip save logo image, image_path = {:?}", image_path);
                }
            }
        }
    }

    // thumb
    if let Some(thumb) = &fanart_tv.tvthumb {
        if let Some(image) = thumb.first() {
            let url = image.url();
            if !url.is_empty() {
                let suffix = url.split(".").last().unwrap();
                let image_path = tv_root_path.join(format!("thumb.{}", suffix));
                if !image_path.exists() {
                    tracing::info!("save thumb image, image_path = {:?}", image_path);
                    request::blocking_get_request_and_download_file(url, &image_path);
                } else {
                    tracing::info!("thumb image exist, skip save thumb image, image_path = {:?}", image_path);
                }
            }
        }
    }

    // background
    if let Some(background) = &fanart_tv.showbackground {
        if let Some(image) = background.first() {
            let url = image.url();
            if !url.is_empty() {
                let suffix = url.split(".").last().unwrap();
                let image_path = tv_root_path.join(format!("background.{}", suffix));
                if !image_path.exists() {
                    tracing::info!("save background image, image_path = {:?}", image_path);
                    request::blocking_get_request_and_download_file(url, &image_path);
                } else {
                    tracing::info!("background image exist, skip save background image, image_path = {:?}", image_path);
                }
            }
        }
    }

    // clearart
    if let Some(clearart) = &fanart_tv.hdclearart {
        if let Some(image) = clearart.first() {
            let url = image.url();
            if !url.is_empty() {
                let suffix = url.split(".").last().unwrap();
                let image_path = tv_root_path.join(format!("clearart.{}", suffix));
                if !image_path.exists() {
                    tracing::info!("save clearart image, image_path = {:?}", image_path);
                    request::blocking_get_request_and_download_file(url, &image_path);
                } else {
                    tracing::info!("clearart image exist, skip save clearart image, image_path = {:?}", image_path);
                }
            }
        }
    }

    // poster
    if !tv.poster_path().is_empty() {
        let poster_path = tv.poster_path();
        let suffix = poster_path.split(".").last().unwrap();
        let image_path = tv_root_path.join(format!("poster.{}", suffix));
        if !image_path.exists() {
            tracing::info!("save poster image, image_path = {:?}", image_path);
            request::blocking_get_request_and_download_file(&format!("https://image.tmdb.org/t/p/original{}", poster_path), &image_path);
        } else {
            tracing::info!("poster image exist, skip save poster image, image_path = {:?}", image_path);
        }
    }

    // backdrop
    if !tv.backdrop_path().is_empty() {
        let backdrop_path = tv.backdrop_path();
        let suffix = backdrop_path.split(".").last().unwrap();
        let image_path = tv_root_path.join(format!("backdrop.{}", suffix));
        if !image_path.exists() {
            tracing::info!("save backdrop image, image_path = {:?}", image_path);
            request::blocking_get_request_and_download_file(&format!("https://image.tmdb.org/t/p/original{}", backdrop_path), &image_path);
        } else {
            tracing::info!("backdrop image exist, skip save backdrop image, image_path = {:?}", image_path);
        }
    }

    // poster
    // let poster = &fanart_tv.tvposter;
    // poster.first().is_some_then(|image| {
    //     let suffix = image.url.split(".").last().unwrap();
    //     let image_path = tv_root_path.join(format!("poster.{}", suffix));
    //     if !image_path.exists() {
    //         tracing::info!("save poster image, image_path = {:?}", image_path);
    //         utils::download_file(&image.url, &image_path);
    //     } else {
    //         tracing::info!("poster image exist, skip save poster image, image_path = {:?}", image_path);
    //     }
    // });

    // seasons
    // let seasons = &fanart_tv.seasonposter;
    // seasons.iter().for_each(|image| {
    //     let suffix = image.url.split(".").last().unwrap();
    //     image.season.is_some_then(|season| match season.parse::<i32>() {
    //         Ok(season_number) => {
    //             let image_path = tv_root_path.join(format!("season{:02}-poster.{}", season_number, suffix));
    //             if !image_path.exists() {
    //                 tracing::info!("save season image, image_path = {:?}", image_path);
    //                 utils::download_file(&image.url, &image_path);
    //             } else {
    //                 tracing::info!("season image exist, skip save season image, image_path = {:?}", image_path);
    //             }
    //         }
    //         Err(e) => {
    //             tracing::error!("parse season number error, season = {:?} error = {:?}", image.season, e);
    //         }
    //     });
    // });
}

// <?xml version="1.0" encoding="utf-8"?>
// <episodedetails>
//   <dateadded>2023-11-25 23:33:19</dateadded>
//   <uniqueid type="tmdb" default="true">1575221</uniqueid>
//   <tmdbid>1575221</tmdbid>
//   <title>孩子之争</title>
//   <plot><![CDATA[拉杰什通过印度婚姻调查问卷，找到一位潜在的对象。佩妮挑明她的想法，莱纳德感到出乎意料。]]></plot>
//   <outline><![CDATA[拉杰什通过印度婚姻调查问卷，找到一位潜在的对象。佩妮挑明她的想法，莱纳德感到出乎意料。]]></outline>
//   <aired>2018-10-04</aired>
//   <year>2018</year>
//   <season>12</season>
//   <episode>3</episode>
//   <rating>6.9</rating>
//   <director tmdbid="2155176">T. Ryan Brennan</director>
//   <actor>
//     <name>Rati Gupta</name>
//     <type>Actor</type>
//     <tmdbid>932790</tmdbid>
//   </actor>
// </episodedetails>
fn gen_episode_nfo_file(meta: &MTMetadata, season_number: &i64, episode_number: &i64, tmdb_episode: &TmdbEpisode, tv_episode_path: &Path) {
    let tv_episode_path = tv_episode_path.clone().with_extension("nfo");

    tracing::info!("gen episode.nfo file, tv_episode_path = {:?}", tv_episode_path);

    let mut xml: Vec<u8> = Vec::new();

    //
    let mut w = EmitterConfig::new().write_document_declaration(true).pad_self_closing(true).perform_indent(true).create_writer(&mut xml);

    // season
    // root
    w.write(XmlEvent::start_element("episodedetails")).unwrap();

    // dateadded
    // get now time and format to 2021-01-01 00:00:00
    dom::write_text_element(&mut w, "dateadded", &time::now_time_format());

    // uniqueid
    w.write(XmlEvent::start_element("uniqueid").attr("type", "tmdb").attr("default", "true")).unwrap();
    w.write(XmlEvent::characters(&tmdb_episode.tmdb_id_str())).unwrap();
    w.write(XmlEvent::end_element()).unwrap();

    // tmdbid
    dom::write_text_element(&mut w, "tmdbid", &tmdb_episode.tmdb_id_str());
    // title
    dom::write_text_element(&mut w, "title", tmdb_episode.title());
    // plot
    dom::write_cdata_text_element(&mut w, "plot", tmdb_episode.overview());
    // outline
    dom::write_cdata_text_element(&mut w, "outline", tmdb_episode.overview());
    // aired
    dom::write_text_element(&mut w, "aired", tmdb_episode.air_date());
    // year
    dom::write_text_element(&mut w, "year", tmdb_episode.year());
    // season
    dom::write_text_element(&mut w, "season", &season_number.to_string());
    // episode
    dom::write_text_element(&mut w, "episode", &episode_number.to_string());
    // rating
    dom::write_text_element(&mut w, "rating", &tmdb_episode.vote_average().to_string());

    // directors
    // tmdb_episode.directors().is_some_then(|director| {
    //     dom::write_text_element(&mut w, "director", director);
    // });
    // actor
    // episode_detail.actor().is_some_then(|actor| {
    //     dom::write_text_element(&mut w, "actor", actor);
    // });

    // end root
    w.write(XmlEvent::end_element()).unwrap();

    // save nfo

    dom::save_nfo(&xml, tv_episode_path.to_str().unwrap());

    tracing::info!("gen episode.nfo file, tv_episode_path = {:?} success", tv_episode_path);
}

// <?xml version="1.0" encoding="utf-8"?>
// <season>
//   <dateadded>2023-11-25 23:30:50</dateadded>
//   <plot><![CDATA[最后一季中，这群朋友持续追寻诺贝尔奖的梦想，彼此的关系也不断变化，看来他们唯一不变的就是改变。]]></plot>
//   <outline><![CDATA[最后一季中，这群朋友持续追寻诺贝尔奖的梦想，彼此的关系也不断变化，看来他们唯一不变的就是改变。]]></outline>
//   <title>季 12</title>
//   <premiered>2018-09-24</premiered>
//   <releasedate>2018-09-24</releasedate>
//   <year>2018</year>
//   <seasonnumber>12</seasonnumber>
// </season>
fn gen_season_nfo_file(meta: &MTMetadata, season: &TmdbSeason, tv_season_path: &Path) {
    tracing::info!("gen season.nfo file, title = {:?} tv_season_path = {:?}", season.title(), tv_season_path);

    let mut xml: Vec<u8> = Vec::new();

    //
    let mut w = EmitterConfig::new().write_document_declaration(true).pad_self_closing(true).perform_indent(true).create_writer(&mut xml);

    // season
    // root
    w.write(XmlEvent::start_element("season")).unwrap();

    // dateadded
    // get now time and format to 2021-01-01 00:00:00
    dom::write_text_element(&mut w, "dateadded", &time::now_time_format());

    // plot
    dom::write_cdata_text_element(&mut w, "plot", season.overview());
    // outline
    dom::write_cdata_text_element(&mut w, "outline", season.overview());
    // title
    dom::write_text_element(&mut w, "title", &season.title());
    // premiered
    dom::write_text_element(&mut w, "premiered", season.air_date());
    // releasedate
    dom::write_text_element(&mut w, "releasedate", season.air_date());
    // year
    dom::write_text_element(&mut w, "year", season.year());
    // seasonnumber
    dom::write_text_element(&mut w, "seasonnumber", &season.season_number());

    // end root
    w.write(XmlEvent::end_element()).unwrap();

    // save nfo
    dom::save_nfo(&xml, tv_season_path.join("season.nfo").to_str().unwrap());

    tracing::info!("gen season.nfo file, title = {:?} tv_season_path = {:?} success", season.title(), tv_season_path);
}

// <?xml version="1.0" encoding="utf-8"?>
// <tvshow>
//   <dateadded>2023-11-25 23:30:35</dateadded>
//   <tmdbid>1418</tmdbid>
//   <uniqueid type="tmdb" default="false">1418</uniqueid>
//   <tvdbid>80379</tvdbid>
//   <uniqueid type="tvdb">80379</uniqueid>
//   <imdbid>tt0898266</imdbid>
//   <uniqueid type="imdb" default="true">tt0898266</uniqueid>
//   <plot><![CDATA[]]></plot>
//   <outline><![CDATA[]]></outline>
//   <actor>
//     <name>约翰尼·盖尔克奇</name>
//     <type>Actor</type>
//     <role>Leonard Hofstadter  莱纳德·霍夫斯塔特</role>
//     <order>0</order>
//     <tmdbid>16478</tmdbid>
//     <thumb>https://image.tmdb.org/t/p/h632/kwMOVJvWDkiXEuKiyNJMaoFnhkj.jpg</thumb>
//     <profile>https://www.themoviedb.org/person/16478</profile>
//   </actor>
//   <actor>
//     <name>吉姆·帕森斯</name>
//     <type>Actor</type>
//     <role>Sheldon Cooper  谢尔顿·库珀</role>
//     <order>1</order>
//     <tmdbid>5374</tmdbid>
//     <thumb>https://image.tmdb.org/t/p/h632/sa05slVgacuXe94UFnQs4rfqZL4.jpg</thumb>
//     <profile>https://www.themoviedb.org/person/5374</profile>
//   </actor>
//   <actor>
//     <name>凯莉·库柯</name>
//     <type>Actor</type>
//     <role>Penny 佩妮</role>
//     <order>2</order>
//     <tmdbid>53862</tmdbid>
//     <thumb>https://image.tmdb.org/t/p/h632/hP5iqQ03ZUg7B4m6UEf6Gg0CuEX.jpg</thumb>
//     <profile>https://www.themoviedb.org/person/53862</profile>
//   </actor>
//   <actor>
//     <name>西蒙·赫尔伯格</name>
//     <type>Actor</type>
//     <role>Howard Wolowitz 霍华德·沃洛维茨</role>
//     <order>3</order>
//     <tmdbid>53863</tmdbid>
//     <thumb>https://image.tmdb.org/t/p/h632/9tLEc6N4lImJX4FohLWHRHFbFXD.jpg</thumb>
//     <profile>https://www.themoviedb.org/person/53863</profile>
//   </actor>
//   <actor>
//     <name>昆瑙·内亚</name>
//     <type>Actor</type>
//     <role>Raj 拉杰什·库斯拉帕里</role>
//     <order>4</order>
//     <tmdbid>208099</tmdbid>
//     <thumb>https://image.tmdb.org/t/p/h632/o58v195EJs4U4whNLqyoIh7Qzef.jpg</thumb>
//     <profile>https://www.themoviedb.org/person/208099</profile>
//   </actor>
//   <genre>喜剧</genre>
//   <rating>7.884</rating>
//   <title>生活大爆炸</title>
//   <originaltitle>The Big Bang Theory</originaltitle>
//   <premiered>2007-09-24</premiered>
//   <year>2007</year>
//   <season>-1</season>
//   <episode>-1</episode>
// </tvshow>
fn gen_tvshow_nfo_file(tmdb_tv: &TmdbTV, tv_root_path: &Path) {
    tracing::info!("gen tvshow.nfo file, title = {:?} tv_root_path = {:?}", tmdb_tv.name, tv_root_path);

    let mut xml: Vec<u8> = Vec::new();

    //
    let mut w = EmitterConfig::new().write_document_declaration(true).pad_self_closing(true).perform_indent(true).create_writer(&mut xml);

    // root
    w.write(XmlEvent::start_element("tvshow")).unwrap();

    // gen common nfo
    gen_common_nfo(&mut w, tmdb_tv);

    // title
    dom::write_text_element(&mut w, "title", tmdb_tv.name());

    // originaltitle
    dom::write_text_element(&mut w, "originaltitle", tmdb_tv.original_language());

    // premiered
    dom::write_text_element(&mut w, "premiered", tmdb_tv.first_air_date());

    // year
    dom::write_text_element(&mut w, "year", tmdb_tv.year());

    // season
    dom::write_text_element(&mut w, "season", "-1");

    // episode
    dom::write_text_element(&mut w, "episode", "-1");

    // end root
    w.write(XmlEvent::end_element()).unwrap();

    // save nfo
    dom::save_nfo(&xml, tv_root_path.join("tvshow.nfo").to_str().unwrap());

    tracing::info!("gen tvshow.nfo file, title = {:?} tv_root_path = {:?} success", tmdb_tv.name, tv_root_path);
}

fn gen_common_nfo(w: &mut EventWriter<&mut Vec<u8>>, tmdb_tv: &TmdbTV) {
    // dateadded
    // get now time and format to 2021-01-01 00:00:00
    dom::write_text_element(w, "dateadded", &time::now_time_format());

    // TMDB
    if !tmdb_tv.tmdb_id().is_empty() {
        // <tmdbid>1418</tmdbid>
        dom::write_text_element(w, "tmdbid", &tmdb_tv.tmdb_id());

        // <uniqueid type="tmdb" default="false">1418</uniqueid>
        w.write(XmlEvent::start_element("uniqueid").attr("type", "tmdb").attr("default", "false")).unwrap();
        w.write(XmlEvent::characters(&tmdb_tv.tmdb_id())).unwrap();
        w.write(XmlEvent::end_element()).unwrap();
    }

    // TVDB
    tmdb_tv.tvdb_id().is_some_then(|tvdb_id| {
        // <tvdbid>80379</tvdbid>
        dom::write_text_element(w, "tvdbid", &tvdb_id.to_string());

        // <uniqueid type="tvdb">80379</uniqueid>
        w.write(XmlEvent::start_element("uniqueid").attr("type", "tvdb")).unwrap();
        w.write(XmlEvent::characters(&tvdb_id.to_string())).unwrap();
        w.write(XmlEvent::end_element()).unwrap();
    });

    // IMDB
    tmdb_tv.imdb_id().is_some_then(|imdb_id| {
        // <imdbid>tt0898266</imdbid>
        dom::write_text_element(w, "imdbid", imdb_id);

        // <uniqueid type="imdb" default="true">tt0898266</uniqueid>
        w.write(XmlEvent::start_element("uniqueid").attr("type", "imdb").attr("default", "true")).unwrap();
        w.write(XmlEvent::characters(imdb_id)).unwrap();
        w.write(XmlEvent::end_element()).unwrap();
    });

    // overview
    // plot
    dom::write_cdata_text_element(w, "plot", &tmdb_tv.overview());

    // outline
    dom::write_cdata_text_element(w, "outline", &tmdb_tv.overview());

    // director
    tmdb_tv.directors().is_some_then(|directors| {
        directors.iter().for_each(|director| {
            w.write(XmlEvent::start_element("director").attr("tmdbid", &director.id())).unwrap();
            w.write(XmlEvent::characters(director.name())).unwrap();
            w.write(XmlEvent::end_element()).unwrap();
        });
    });

    // actors
    tmdb_tv.actors().is_some_then(|actors| {
        actors.iter().for_each(|actor| {
            w.write(XmlEvent::start_element("actor")).unwrap();

            dom::write_text_element(w, "name", actor.name());
            dom::write_text_element(w, "type", "Actor");
            dom::write_text_element(w, "role", actor.character());
            dom::write_text_element(w, "tmdbid", &actor.id());
            dom::write_text_element(w, "thumb", &actor.thumb());
            dom::write_text_element(w, "profile", &actor.profile());

            w.write(XmlEvent::end_element()).unwrap();
        });
    });

    // genre
    tmdb_tv.genres.is_some_then(|genres| {
        genres.iter().for_each(|genre| dom::write_text_element(w, "genre", genre.name()));
    });

    // rating
    dom::write_text_element(w, "rating", &tmdb_tv.vote_average());
}
