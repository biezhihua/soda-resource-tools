use std::collections::HashMap;
use std::error::Error;
use std::f32::consts::E;
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;
use std::sync::Mutex;

use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;

use crate::soda::entity::{MatchRule, MetaContext, RegexRule, Rule, Token};
use crate::soda::extension_option::OptionExtensions;
use crate::soda::LIB_CONFIG;

use crate::soda::entity::*;

pub(crate) fn init() {
    let regex_rule = &REGEX_RULES;
    let tv_rules = &TV_RULES;
    let movie_rules = &MOVIE_RULES;
}

static LAST_PATH: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new("".to_string()));

static REGEX_RULES: Lazy<RegexRule> = Lazy::new(|| {
    let config = LIB_CONFIG.lock().unwrap();

    let content = if !config.strong_match_regex_rules.is_empty() {
        config.strong_match_regex_rules.clone()
    } else if !config.strong_match_regex_rules_path.is_empty() {
        fs::read_to_string(config.strong_match_regex_rules_path.as_str()).unwrap()
    } else {
        unreachable!("strong_match_regex_rules and strong_match_regex_rules_path is empty")
    };

    serde_json::from_str(&content).unwrap()
});

static TV_RULES: Lazy<MatchRule> = Lazy::new(|| {
    let config = LIB_CONFIG.lock().unwrap();

    let content = if !config.strong_match_rules_tv.is_empty() {
        config.strong_match_rules_tv.clone()
    } else if !config.strong_match_rules_tv_path.is_empty() {
        fs::read_to_string(config.strong_match_rules_tv_path.as_str()).unwrap()
    } else {
        unreachable!("strong_match_rules_tv and strong_match_rules_tv_path is empty")
    };

    serde_json::from_str(&content).unwrap()
});

static MOVIE_RULES: Lazy<MatchRule> = Lazy::new(|| {
    let config = LIB_CONFIG.lock().unwrap();

    let content = if !config.strong_match_rules_movie.is_empty() {
        config.strong_match_rules_movie.clone()
    } else if !config.strong_match_rules_movie_path.is_empty() {
        fs::read_to_string(config.strong_match_rules_movie_path.as_str()).unwrap()
    } else {
        unreachable!("strong_match_rules_movie and strong_match_rules_movie_path is empty")
    };

    serde_json::from_str(&content).unwrap()
});

pub(crate) fn build_mt_file_tokens(meta_context: &mut MetaContext, before_token: &mut Token) -> Option<Token> {
    // TV
    if let Some(value) = build_tv_tokens(meta_context, before_token) {
        return Some(value);
    }

    // MOVIE
    if let Some(value) = build_movie_tokens(meta_context, before_token) {
        return Some(value);
    }

    return None;
}

fn build_movie_tokens(meta_context: &mut MetaContext, before_token: &mut Token) -> Option<Token> {
    let input = meta_context.input.to_string();
    tracing::debug!("build_movie_tokens input = {}", input);
    meta_context.last_rule = None;
    let rules = &MOVIE_RULES.rules;
    for rule in rules {
        if let Some(mut token) = build_tokens(rule, &input) {
            // merge tokens to token
            for ele in &before_token.tokens {
                token.tokens.insert(ele.0.to_string(), ele.1.to_string());
            }
            return Some(token);
        }
    }
    return None;
}

fn build_tv_tokens(meta_context: &mut MetaContext, before_token: &mut Token) -> Option<Token> {
    let input = meta_context.input.to_string();
    let enable_cache = meta_context.enable_cache();
    tracing::debug!("build_tv_tokens enable_cache = {}", enable_cache);
    meta_context.last_rule = None;
    if enable_cache {
        if let Some(last_rule) = &meta_context.last_rule {
            if let Some(mut token) = build_tokens(&last_rule, &input) {
                // merge tokens to token
                for ele in &before_token.tokens {
                    token.tokens.insert(ele.0.to_string(), ele.1.to_string());
                }
                return Some(token);
            }
        }
    }
    let rules = &TV_RULES.rules;
    for rule in rules {
        if let Some(mut token) = build_tokens(rule, &input) {
            if enable_cache {
                meta_context.last_rule = Some(rule.clone());
            }
            // merge tokens to token
            for ele in &before_token.tokens {
                token.tokens.insert(ele.0.to_string(), ele.1.to_string());
            }
            return Some(token);
        }
    }
    return None;

    // 先使用TV规则处理

    None
}

pub(crate) fn build_mt_dir_tokens(title: &str, before_token: &mut Token) -> Option<Token> {
    let mut new_title = title.to_string();

    tracing::debug!("build_mt_file_tokens new_title = {}", new_title);

    for rule in &TV_RULES.rules {
        let mut new_rule = rule.clone();
        new_rule.rule = new_rule.rule.replace(".$extension$", "");
        new_rule.rule = new_rule.rule.replace("$season_episode$", "$season$");
        if let Some(mut token) = build_tokens(&new_rule, &new_title) {
            // merge tokens to token
            for ele in &before_token.tokens {
                token.tokens.insert(ele.0.to_string(), ele.1.to_string());
            }
            return Some(token);
        }
    }
    return None;
}

/// 匹配中文
/// (?P<title_cn>...)：这是一个命名捕获组，名为 "title_cn"。捕获组可以用来从匹配的文本中提取出特定部分。
/// [：\p{Script=Han}\p{Script=Hiragana}]：这是一个字符集，匹配任何在集合中的字符。这个集合包括冒号（：），以及任何属于汉字（Han）或平假名（Hiragana）的字符。
/// [A-Z]?: 后面可能跟着一个大写字母（[A-Z]? 表示匹配0次或1次大写字母）
/// +：这表示前面的字符集可以在字符串中出现一次或多次。
/// 所以，整个表达式的意思是：匹配一次或多次的冒号，汉字或平假名字符，并将匹配的部分作为名为 "title_cn" 的捕获组。
fn regex_item_title_cn() -> String {
    return regex_item_multi(KEY_TITLE_CN, 0, &REGEX_RULES.title_cn);
}

fn regex_item_season_title_cn() -> String {
    return regex_item_multi(KEY_SEASON_TITLE_CN, 0, &REGEX_RULES.season_title_cn);
}

/// 匹配中文
/// 这个正则表达式似乎有一些错误，它应该是 r"(?P<name>[0-9A-Za-z]+[：\p{Script=Han}\p{Script=Hiragana}]+)"。下面是各部分的解释：
/// (?P<name>...)：这是一个命名捕获组，名为 "name"。捕获组可以用来从匹配的文本中提取出特定部分。
/// [0-9A-Za-z]：这是一个字符集，匹配任何在集合中的字符。这个集合包括所有的数字（0-9），大写字母（A-Z）和小写字母（a-z）。
/// +：这表示前面的字符集可以在字符串中出现一次或多次。
/// [：\p{Script=Han}\p{Script=Hiragana}]：这是另一个字符集，匹配任何在集合中的字符。这个集合包括冒号（：），以及任何属于汉字（Han）或平假名（Hiragana）的字符。
/// +：这表示前面的字符集可以在字符串中出现一次或多次。
/// 所以，整个表达式的意思是：匹配一次或多次的数字、字母，后面跟着一次或多次的冒号，汉字或平假名字符，并将匹配的部分作为名为 "name" 的捕获组。
fn regex_item_title_number_cn() -> String {
    return regex_item_multi(KEY_TITLE_NUMBER_CN, 0, &REGEX_RULES.title_number_cn);
}

/// 匹配英文名称
fn regex_item_title_en() -> String {
    return regex_item_multi(KEY_TITLE_EN, 0, &REGEX_RULES.title_en);
}

/// 匹配AKA原始名称
///   "original_title_en": [
/// "AKA\\.[-A-Za-z]+",
/// "[-A-Za-z]+\\.AKA"
/// ],
// fn regex_item_original_title_en() -> String {
//     return regex_item_multi("original_title_en", 0, &REGEX_RULES.original_title_en);
// }

/// 匹配英文名称
/// 这是一个正则表达式，用于匹配一种特定的字符串模式。下面是各部分的解释：
/// (?P<title_number_en>...)：这是一个命名捕获组，名为 "title_number_en"。捕获组可以用来从匹配的文本中提取出特定部分。
/// [\\()&'-. 0-9A-Za-z]：这是一个字符集，匹配任何在集合中的字符。这个集合包括反斜杠（\），括号（()），和号（&），单引号（'），破折号（-），点（.），空格（ ），所有的数字（0-9），大写字母（A-Z）和小写字母（a-z）。
/// +：这表示前面的字符集可以在字符串中出现一次或多次。
/// 所以，整个表达式的意思是：匹配一次或多次的上述字符集中的字符，并将匹配的部分作为名为 "title_number_en" 的捕获组
fn regex_item_title_number_en() -> String {
    return regex_item_multi(KEY_TITLE_NUMBER_EN, 0, &REGEX_RULES.title_number_en);
}

/// 匹配标题年
fn regex_item_title_year() -> String {
    return regex_item_multi_str(KEY_TITLE_YEAR, 0, &vec!["\\d{4}"]);
}

/// 匹配英文季名
/// https://docs.rs/regex/latest/regex/
// 这是一个正则表达式，用于匹配一种特定的字符串模式。下面是各部分的解释：
// (?P<episode_title_en>...)：这是一个命名捕获组，名为 "episode_title_en"。捕获组可以用来从匹配的文本中提取出特定部分。
// [&-. A-Za-z]+[0-9]{1,4}[&-. A-Za-z]+：这个模式匹配一次或多次的字符集中的字符（包括&，-，.，空格，以及所有的大写和小写字母），后面跟着1到4个数字，再后面跟着一次或多次的字符集中的字符。
// [&-. A-Za-z]+[0-9]{1,3}：这个模式匹配一次或多次的字符集中的字符，后面跟着1到3个数字。
// [&-. A-Za-z]+：这个模式匹配一次或多次的字符集中的字符。
// [0-9]+\+[0-9]+：这个模式匹配一次或多次的数字，后面跟着一个加号，再后面跟着一次或多次的数字。
// [0-9]{1,3}[&-. A-Za-z]+：这个模式匹配1到3个数字，后面跟着一次或多次的字符集中的字符。
// |：这是或（or）操作符，表示匹配前面或后面的模式。
// 所以，整个表达式的意思是：匹配五种可能的模式之一，并将匹配的部分作为名为 "episode_title_en" 的捕获组。
fn regex_item_episode_title_en() -> String {
    return regex_item_multi(KEY_EPISODE_TITLE_EN, 0, &REGEX_RULES.episode_title_en);
}

/// 匹配中文名
fn regex_item_episode_title_cn() -> String {
    return regex_item_multi(KEY_EPISODE_TITLE_CN, 0, &REGEX_RULES.episode_title_cn);
}

/// 匹配日文
fn regex_item_episode_title_jp() -> String {
    return regex_item_multi(KEY_EPISODE_TITLE_JP, 0, &REGEX_RULES.episode_title_jp);
}

/// 匹配中文名
fn regex_item_text_cn() -> String {
    return regex_item_multi_str(KEY_TEXT_CN, 0, &vec!["\\p{Script=Han}+"]);
}

/// 匹配年 2023
fn regex_item_year() -> String {
    return regex_item_multi_str(KEY_YEAR, 0, &vec!["\\d{4}"]);
}

/// 匹配年 2021-2023
fn regex_item_year_start_to_end() -> String {
    return regex_item_multi_str(KEY_YEAR_START_TO_END, 0, &vec!["\\d{4}-\\d{4}"]);
}

/// 匹配 2023-12-01
fn regex_item_year_month_day() -> String {
    return regex_item_multi_str(KEY_YEAR_MONTH_DAY, 0, &vec!["\\d{4}\\.\\d{2}\\.\\d{2}"]);
}

/// 匹配季和集 S01E01
fn regex_item_season_episode() -> String {
    return regex_item_multi_str(KEY_SEASON_EPISODE, 0, &vec!["[Ss]\\d{1,2}\\.?[Ee]\\d{2,3}"]);
}

/// 匹配季和集 S01E01E02
fn regex_item_season_episode_episode() -> String {
    return regex_item_multi_str(KEY_SEASON_EPISODE_EPISODE, 0, &vec!["S\\d{2}E\\d{2,3}E\\d{2,3}"]);
}

/// 匹配季和集 S01E01
fn regex_item_season_episode_split() -> &'static str {
    r"(?P<season>S\d{1,2})\.?(?P<episode>E[Pp]?\d{2,3})"
}

fn regex_item_season_and_episode() -> &'static str {
    r"(?P<season>S\d{1,2})\.(?P<episode>E[Pp]?\d{2,3})"
}

/// 匹配季和集 S01E01E02
fn regex_item_season_episode_episode_split() -> &'static str {
    r"(?P<season>S\d{2})(?P<episode1>E[Pp]?\d{2,3})(?P<episode2>E[Pp]?\d{2,3})"
}

/// 匹配单个数字
fn regex_item_season_number() -> String {
    return regex_item_multi_str(KEY_SEASON_NUMBER, 0, &vec!["[0-9]{1}"]);
}

/// match episode E01 EP01 Ep01 E100 E1024
/// 也可以匹配 S01E01中的E01，所以解析逻辑需要放在regex_item_season_episode之后
fn regex_item_episode() -> String {
    return regex_item_multi_str(KEY_EPISODE, 0, &vec!["E[Pp]?\\d{2,4}"]);
}

/// 匹配两位数字
fn regex_item_episode_number() -> String {
    return regex_item_multi_str(KEY_EPISODE_NUMBER, 0, &vec!["\\d{2,3}"]);
}

/// match season S01
fn regex_item_season() -> String {
    return regex_item_multi_str(KEY_SEASON, 0, &vec!["S\\d{2}"]);
}

/// match 第01季
fn regex_item_season_cn() -> String {
    return regex_item_multi_str(KEY_SEASON_CN, 0, &vec!["第\\d{1,2}季"]);
}

/// match 第01集
fn regex_item_episode_cn() -> String {
    return regex_item_multi_str(KEY_EPISODE_CN, 0, &vec!["第\\d{1,2}[集|话]"]);
}

/// 全十一季
fn regex_item_season_all_cn() -> String {
    return regex_item_multi_str(KEY_SEASON_ALL_CN, 0, &vec!["全[一二三四五六七八九十]+季"]);
}

/// 第01-08季
fn regex_item_season_start_to_end_cn() -> String {
    return regex_item_multi_str(KEY_SEASON_START_TO_END_CN, 0, &vec!["第\\d{2}-\\d{2}季"]);
}

/// S01-S03
fn regex_item_season_start_to_end_en() -> String {
    return regex_item_multi_str(KEY_SEASON_START_TO_END_EN, 0, &vec!["S\\d{2}-S\\d{2}"]);
}

/// 匹配分辨率 1080P/1080p/1080i
fn regex_item_resolution_cn() -> String {
    return regex_item_multi(KEY_RESOLUTION_CN, 0, &REGEX_RULES.resolution_cn);
}

/// 版本
/// [三体].Three-Body.2023.S01E01.2160p.V3.WEB-DL.HEVC.DDP5.1.Atmos.&.AAC-QHstudIo.mp4
fn regex_item_version() -> String {
    return regex_item_multi(KEY_VERSION, 0, &REGEX_RULES.version);
}

/// 匹配字幕语言
fn regex_item_subtitle_en() -> String {
    return regex_item_subtitle_en_multi(0);
}

fn regex_item_subtitle_en_multi(index: i32) -> String {
    return regex_item_multi(KEY_SUBTITLE_EN, index, &REGEX_RULES.subtitle_en);
}

/// 匹配中文名
fn regex_item_subtitle_cn() -> String {
    return regex_item_multi_str(KEY_SUBTITLE_CN, 0, &vec!["中[英|美|日|挪]?字[幕]?(简繁双语特效)?"]);
}

/// 匹配中文名
fn regex_item_audio_cn() -> String {
    return regex_item_multi_str(KEY_AUDIO_CN, 0, &vec!["中[英|美|日|挪]双语"]);
}

/// 匹配特典
fn regex_item_special() -> String {
    return regex_item_multi_str(KEY_SPECIAL, 0, &vec!["Manual"]);
}

///
fn regex_item_anything() -> String {
    return regex_item_multi_str(KEY_ANYTHING, 0, &vec![".*"]);
}

/// 匹配国家
fn regex_item_country() -> String {
    return regex_item_country_multi(0);
}

fn regex_item_country_multi(index: i32) -> String {
    return regex_item_multi(KEY_COUNTRY, index, &REGEX_RULES.country);
}

/// 匹配分辨率 1080P/1080p/1080i
fn regex_item_resolution() -> String {
    return regex_item_resolution_multi(0);
}

fn regex_item_resolution_multi(index: i32) -> String {
    return regex_item_multi(KEY_RESOLUTION, index, &REGEX_RULES.resolution);
}

/// 匹配视频源
/// "BluRay REMUX" 是与蓝光光盘相关的术语，其中包含了两个部分的含义：
/// 1. **BluRay（蓝光）：** 这指的是蓝光光盘，是一种高清晰度的光盘格式，通常用于存储高质量的视频和音频内容。
/// 2. **REMUX：** 这是“remultiplex”的缩写，指的是重新多路复用。在这个上下文中，REMUX 是指从蓝光光盘中提取视频、音频和字幕等元素，然后重新组合它们，但没有进行重新编码。这样可以保留原始视频和音频的高质量，而不会降低其编码质量。因此，BluRay REMUX 文件通常具有与原始蓝光光盘相同的视觉和音频质量，但文件大小相对较小。
/// 总体来说，BluRay REMUX 是指从蓝光光盘中提取内容并重新组合，以保留高质量的视频和音频，而不经过再次编码。这通常用于要求高质量的家庭影院系统和视频爱好者。
///
/// "AVC.REMUX" 是指使用高级视频编解码器（Advanced Video Codec，AVC）进行重新复制（Remux）的视频文件。在这个上下文中：
/// - **AVC：** 是 H.264（也称为 MPEG-4 Part 10）视频编解码器的一种实现，被广泛用于视频压缩。它是一种高效的视频编解码标准，常见于蓝光光盘和在线视频流。
/// - **REMUX：** 是重新复制的缩写，表示视频和音频流从一个容器格式复制到另一个容器格式，而不进行重新编码。在这里，它指的是将视频从一个源中提取并重新封装，而不对视频进行再压缩，以保持最佳的视觉和听觉质量。
/// 因此，AVC.REMUX 文件通常是高质量的视频文件，因为它们没有经历再压缩，保留了原始来源的视频和音频质量。
fn regex_item_source() -> String {
    return regex_item_source_multi(0);
}

fn regex_item_source_multi(index: i32) -> String {
    return regex_item_multi(KEY_SOURCE, index, &REGEX_RULES.source);
}

/// 匹配视频源厂商
/// netflix = nf
fn regex_item_company() -> String {
    return regex_item_multi(KEY_COMPANY, 0, &REGEX_RULES.company);
}

/// 匹配视频编码器
fn regex_item_video_codec() -> String {
    return regex_item_video_codec_multi(0);
}

fn regex_item_video_codec_multi(index: i32) -> String {
    return regex_item_multi(KEY_VIDEO_CODEC, index, &REGEX_RULES.video_codec);
}

/// 视频编码器扩展信息
/// https://www.reddit.com/r/VideoEditing/comments/b45b85/10_bit_12_bit_or_h265_which_ones_the_best_and_has/
/// color depth
/// FPS
fn regex_item_color() -> String {
    return regex_item_color_multi(0);
}

fn regex_item_color_multi(index: i32) -> String {
    return regex_item_multi(KEY_COLOR, index, &REGEX_RULES.color);
}

fn regex_item_audio_codec_multi(index: i32) -> String {
    return regex_item_multi(KEY_AUDIO_CODEC, index, &REGEX_RULES.audio_codec);
}

fn regex_item_edition() -> String {
    return regex_item_multi(KEY_EDITION, 0, &REGEX_RULES.edition);
}

/// 匹配音频编码器
fn regex_item_audio_codec() -> String {
    return regex_item_audio_codec_multi(0);
}

/// 匹配发布组
fn regex_item_release_group() -> String {
    return regex_item_multi(KEY_RELEASE_GROUP, 0, &REGEX_RULES.release_group);
}

/// 视频格式
fn regex_item_extension() -> String {
    return regex_item_multi_str(KEY_EXTENSION, 0, &vec!["mp4|mkv|iso|rmvb|avi|mov|mepg|mpg|wmv|3gp|asf|m4v|flv"]);
}

/// 匹配数字字母混合
/// 例如：4ABDCA70
fn regex_item_mix_numbers_letters() -> String {
    return regex_item_multi_str(KEY_MIX_NUMBERS_LETTERS, 0, &vec!["[\\.0-9A-Za-z]+"]);
}

/// 匹配单个数字
fn regex_item_number() -> String {
    return regex_item_multi_str(KEY_NUMBER, 0, &vec!["[0-9]{1}"]);
}

///
/// $title_cn.$title_en.$year.$season_episode.$resolution.$source.$video_codec.$audio_codec-$release_group.$extension$
fn regex_build(rule: &Rule, regex_index: &mut HashMap<String, i32>) -> String {
    let rule_copy = rule.rule.to_string();

    // build pre
    let pre = regex_builder_pre(rule_copy);

    // build before
    let before = regex_builder_before(rule, pre);

    // build core
    let mut format = before;
    format = format.replace(&wrap(KEY_TITLE_CN), &regex_item_title_cn());
    format = format.replace(&wrap(KEY_TITLE_NUMBER_CN), &regex_item_title_number_cn());
    format = format.replace(&wrap(KEY_TITLE_EN), &regex_item_title_en());
    format = format.replace(&wrap(KEY_TITLE_NUMBER_EN), &regex_item_title_number_en());
    format = format.replace(&wrap(KEY_TITLE_YEAR), &regex_item_title_year());
    format = format.replace(&wrap(KEY_YEAR_START_TO_END), &regex_item_year_start_to_end());
    format = format.replace(&wrap(KEY_YEAR_MONTH_DAY), &regex_item_year_month_day());
    format = format.replace(&wrap(KEY_YEAR), &regex_item_year());
    format = format.replace(&wrap(KEY_SEASON_EPISODE), &regex_item_season_episode());
    format = format.replace(&wrap(KEY_SEASON_EPISODE_EPISODE), &regex_item_season_episode_episode());
    format = format.replace(&wrap(KEY_SEASON), &regex_item_season());
    format = format.replace(&wrap(KEY_SEASON_START_TO_END_CN), &regex_item_season_start_to_end_cn());
    format = format.replace(&wrap(KEY_SEASON_ALL_CN), &regex_item_season_all_cn());
    format = format.replace(&wrap(KEY_SEASON_CN), &regex_item_season_cn());
    format = format.replace(&wrap(KEY_SEASON_START_TO_END_EN), &regex_item_season_start_to_end_en());
    format = format.replace(&wrap(KEY_EPISODE), &regex_item_episode());
    format = format.replace(&wrap(KEY_EPISODE_CN), &regex_item_episode_cn());
    format = format.replace(&wrap(KEY_EPISODE_NUMBER), &regex_item_episode_number());
    format = format.replace(&wrap(KEY_EPISODE_TITLE_EN), &regex_item_episode_title_en());
    format = format.replace(&wrap(KEY_EPISODE_TITLE_CN), &regex_item_episode_title_cn());
    format = format.replace(&wrap(KEY_EPISODE_TITLE_JP), &regex_item_episode_title_jp());
    format = format.replace(&wrap(KEY_RESOLUTION_CN), &regex_item_resolution_cn());
    format = format.replace(&wrap(KEY_VERSION), &regex_item_version());
    format = format.replace(&wrap(KEY_COMPANY), &regex_item_company());
    format = format.replace(&wrap(KEY_SEASON_TITLE_CN), &regex_item_season_title_cn());
    format = format.replace(&wrap(KEY_RELEASE_GROUP), &regex_item_release_group());
    format = format.replace(&wrap(KEY_SUBTITLE_CN), &regex_item_subtitle_cn());
    format = format.replace(&wrap(KEY_AUDIO_CN), &regex_item_audio_cn());
    format = format.replace(&wrap(KEY_SPECIAL), &regex_item_special());
    format = format.replace(&wrap(KEY_EXTENSION), &regex_item_extension());
    format = format.replace(&wrap(KEY_MIX_NUMBERS_LETTERS), &regex_item_mix_numbers_letters());
    format = format.replace(&wrap(KEY_NUMBER), &regex_item_number());
    format = format.replace(&wrap(KEY_SEASON_NUMBER), &regex_item_season_number());
    format = format.replace(&wrap(KEY_TEXT_CN), &regex_item_text_cn());
    format = format.replace(&wrap(KEY_ANYTHING), &regex_item_anything());

    format = replace_multi(format, &wrap(KEY_SUBTITLE_EN), |index: i32| regex_item_subtitle_en_multi(index));
    format = replace_multi(format, &wrap(KEY_COUNTRY), |index: i32| regex_item_country_multi(index));
    format = replace_multi(format, &wrap(KEY_SOURCE), |index: i32| regex_item_source_multi(index));
    format = replace_multi(format, &wrap(KEY_VIDEO_CODEC), |index: i32| regex_item_video_codec_multi(index));
    format = replace_multi(format, &wrap(KEY_AUDIO_CODEC), |index: i32| regex_item_audio_codec_multi(index));
    format = replace_multi(format, &wrap(KEY_COLOR), |index: i32| regex_item_color_multi(index));
    format = replace_multi(format, &wrap(KEY_RESOLUTION), |index: i32| regex_item_resolution_multi(index));

    // build after
    let after = regex_builder_after(format);

    let mut ret = String::new();
    ret.push_str("^");
    ret.push_str(&after);
    ret.push_str("$");
    return ret;
}

fn replace_multi(format: String, arg: &str, exec: impl Fn(i32) -> String) -> String {
    let mut index = 0;
    let mut ret = format.clone();
    while let Some(_index) = ret.find(arg) {
        let target = exec(index);
        ret = ret.replacen(arg, &target, 1);
        index += 1;
    }
    return ret;
}

fn regex_builder_pre(config: String) -> String {
    // replace special chars
    // 对于强匹配的规则，需要将一些特殊字符转义，例如.()[]等，他们并不是正则表达式的一部分
    // {
    //   "example": "[三国].Three.Kingdoms.EP01.2010.BluRay.720p.x264.AC3-CMCT.mkv",
    //   "rule": "[$title_cn$].$title_en$.$episode$.$year$.$source$.$resolution$.$video_codec$.$audio_codec$.$extension$"
    // }
    // 例如这里面的[$title_cn$]的[和]需要转义，否则会被当做正则表达式的一部分
    let config = config.replace("[", "\\[");
    let config = config.replace("]", "\\]");
    let config = config.replace("(", "\\(");
    let config = config.replace(")", "\\)");

    // tracing::debug!("regex_builder_pre config = {}", config);
    return config;
}

fn regex_builder_after(input: String) -> String {
    // 最后拼接出来的正则表达式是有规律的，需要将.转义
    let input = input.replace(").(", ")\\.(");
    // tracing::debug!("regex_builder_after input = {}", input);
    return input;
}

fn regex_builder_before(_rule: &Rule, pre: String) -> String {
    return pre;
}

/// build token from input by config
fn build_tokens(rule: &Rule, input: &str) -> Option<Token> {
    // tracing::debug!("build_tokens input = {}", input);

    let new_input = input_before(rule, input);

    // tracing::debug!("build_tokens new_input = {}", new_input);

    let mut regex_index: HashMap<String, i32> = HashMap::new();
    let final_regex = regex_build(rule, &mut regex_index);

    let regex = Regex::new(final_regex.as_str()).unwrap();

    if let Some(captures) = regex.captures(&new_input) {
        let mut token = Token { tokens: HashMap::new() };

        tracing::debug!("build_tokens new_input = {}", new_input);
        tracing::debug!("build_tokens regex = {}", rule.rule);
        tracing::debug!("build_tokens final_regex = {}", final_regex);

        token.tokens.insert(KEY_ORIGIN_TITLE.to_string(), input.to_string());

        if let Some(value) = captures.name(KEY_TITLE_CN) {
            token.tokens.insert(KEY_TITLE_CN.to_string(), value.as_str().to_string());
        }

        if let Some(value) = captures.name(KEY_TITLE_NUMBER_CN) {
            token.tokens.insert(KEY_TITLE_CN.to_string(), value.as_str().to_string());
        }

        if let Some(value) = captures.name(KEY_TITLE_EN) {
            let value = value.as_str().to_string();
            let value = value.replace("∙", " ").trim_end().to_string();
            let value = value.replace(".", " ").trim_end().to_string();
            let value = value.replace("-", " ").trim_end().to_string();
            let value = value.trim_start().to_string();
            let aka: Vec<&str> = value.split(KEY_AKA).collect();
            if aka.len() == 2 {
                token.tokens.insert(KEY_AKA_TITLE_EN.to_string(), value.to_string());
                token.tokens.insert(KEY_AKA_TITLE_EN_FIRST.to_string(), aka[0].trim_end().to_string());
                token.tokens.insert(KEY_AKA_TITLE_EN_SECOND.to_string(), aka[1].trim_start().to_string());

                // 使用aka的第一部分作为默认名字
                token.tokens.insert(KEY_TITLE_EN.to_string(), aka[0].trim_end().to_string());
            } else {
                // 原始名字
                token.tokens.insert(KEY_TITLE_EN.to_string(), value);
            }
        }

        if let Some(value) = captures.name(KEY_TITLE_YEAR) {
            token.tokens.insert(KEY_TITLE_YEAR.to_string(), value.as_str().to_string());
            token.tokens.insert(KEY_TITLE_EN.to_string(), value.as_str().to_string());
        }

        if let Some(value) = captures.name(KEY_TITLE_NUMBER_EN) {
            let value = value.as_str().to_string();
            let value = value.replace(".", " ").trim_end().to_string();
            let value = value.replace("-", " ").trim_end().to_string();
            token.tokens.insert(KEY_TITLE_EN.to_string(), value);
        }

        if let Some(value) = captures.name(KEY_YEAR) {
            token.tokens.insert(KEY_YEAR.to_string(), value.as_str().to_string());
        }

        if let Some(value) = captures.name(KEY_SEASON_EPISODE) {
            let value = value.as_str().to_string().to_uppercase();
            token.tokens.insert(KEY_SEASON_EPISODE.to_string(), value.clone());

            if let Ok(regex) = Regex::new(&regex_item_season_episode_split()) {
                if let Some(captures) = regex.captures(&value) {
                    if let Some(value) = captures.name(KEY_SEASON) {
                        token.tokens.insert(KEY_SEASON.to_string(), value.as_str().to_string());
                    }

                    if let Some(value) = captures.name(KEY_EPISODE) {
                        token.tokens.insert(KEY_EPISODE.to_string(), value.as_str().to_string());
                    }
                }
            }
        }

        if let Some(value) = captures.name(KEY_SEASON_EPISODE_EPISODE) {
            let value = value.as_str().to_string();
            token.tokens.insert(KEY_SEASON_EPISODE_EPISODE.to_string(), value.clone());

            if let Ok(regex) = Regex::new(&regex_item_season_episode_episode_split()) {
                if let Some(captures) = regex.captures(&value) {
                    if let Some(value) = captures.name(KEY_SEASON) {
                        token.tokens.insert(KEY_SEASON.to_string(), value.as_str().to_string());
                    }

                    if let Some(value) = captures.name(KEY_EPISODE_1) {
                        token.tokens.insert(KEY_EPISODE.to_string(), value.as_str().to_string());
                        token.tokens.insert(KEY_EPISODE_1.to_string(), value.as_str().to_string());
                    }

                    if let Some(value) = captures.name(KEY_EPISODE_2) {
                        token.tokens.insert(KEY_EPISODE_2.to_string(), value.as_str().to_string());
                    }
                }
            }
        }

        if let Some(value) = captures.name(KEY_SEASON) {
            token.tokens.insert(KEY_SEASON.to_string(), value.as_str().to_string());
        }

        if let Some(value) = captures.name(KEY_SEASON_ALL_CN) {
            token.tokens.insert(KEY_SEASON_ALL_CN.to_string(), value.as_str().to_string());
        }

        if let Some(value) = captures.name(KEY_SEASON_START_TO_END_CN) {
            token.tokens.insert(KEY_SEASON_START_TO_END_CN.to_string(), value.as_str().to_string());
        }

        if let Some(value) = captures.name(KEY_SEASON_START_TO_END_EN) {
            token.tokens.insert(KEY_SEASON_START_TO_END_EN.to_string(), value.as_str().to_string());
        }

        if let Some(value) = captures.name(KEY_EPISODE) {
            let value = value.as_str().to_string();
            let value = value.replace("Ep", "E");
            let value = value.replace("EP", "E");
            token.tokens.insert(KEY_EPISODE.to_string(), value);
        }

        if let Some(value) = captures.name(KEY_EPISODE_CN) {
            let value = value.as_str().to_string();
            let value = value.replace("第", "");
            let value = value.replace("集", "");
            let value = value.replace("话", "");
            token.tokens.insert(KEY_EPISODE.to_string(), format!("E{}", value));
        }

        if let Some(value) = captures.name(KEY_EPISODE_NUMBER) {
            tracing::debug!("episode_number = {}", value.as_str().to_string());
            // value to i32
            let value = value.as_str().to_string().parse::<i32>().unwrap();
            token.tokens.insert(KEY_EPISODE.to_string(), format!("E{:02}", value));
        }

        if let Some(value) = captures.name(KEY_RESOLUTION) {
            let value = value.as_str().to_lowercase().to_string();
            let value = value.replace("4k", "2160p");
            let value = value.replace("2k", "2160p");
            let value = value.replace("1920x1080", "1080p");
            let value = value.replace("1920X1080", "1080p");
            let value = value.replace("3840x2160", "2160p");
            let value = value.replace("3840X2160", "2160p");
            token.tokens.insert(KEY_RESOLUTION.to_string(), value);
        }

        if let Some(value) = captures.name(KEY_SOURCE) {
            let mut source = value.as_str().to_string();

            // 处理BluRay的各种样式
            let bluray_regex = Regex::new(r"[Bb]lu[-]?[Rr]ay").unwrap();
            source = bluray_regex.replace(&source, "BluRay").to_string();

            // 处理Remux的各种样式
            source = source.replace("REMUX", "Remux").to_string();
            source = source.replace("ReMuX", "Remux").to_string();

            //
            source = source.replace("Remux BluRay", "BluRay.Remux").to_string();

            // 处理BluRay[.- ]Remux的各种样式
            source = Regex::new(r"BluRay[ -]Remux").unwrap().replace(&source, "BluRay.Remux").to_string();

            //
            if source == "WEB" {
                source = source.replace("WEB", "WEB-DL").to_string();
            }
            source = source.replace("web", "WEB-DL").to_string();
            source = source.replace("WEB DL", "WEB-DL").to_string();

            // BDRip
            source = source.replace("BDRIP", "BDRip").to_string();
            source = source.replace("BDrip", "BDRip").to_string();

            //
            token.tokens.insert(KEY_SOURCE.to_string(), source);
        }

        if let Some(value) = captures.name(KEY_COMPANY) {
            token.tokens.insert(KEY_COMPANY.to_string(), value.as_str().to_string());
        }

        if let Some(value) = captures.name(KEY_VIDEO_CODEC) {
            let mut video_codec = value.as_str().to_string();

            //
            let regex = Regex::new(r"(HEVC|Hevc)").unwrap();
            video_codec = regex.replace(&video_codec, "HEVC").to_string();

            //
            let regex = Regex::new(r"[XxHh]\.?264").unwrap();
            video_codec = regex.replace(&video_codec, "H.264").to_string();

            //
            let regex = Regex::new(r"[XxHh]\.?265").unwrap();
            video_codec = regex.replace(&video_codec, "H.265").to_string();

            //
            video_codec = video_codec.replace("AVC.REMUX", "H.264").to_string();
            video_codec = video_codec.replace("AVC", "H.264").to_string();

            //
            video_codec = video_codec.replace("HEVC", "H.265").to_string();

            token.tokens.insert(KEY_VIDEO_CODEC.to_string(), video_codec);
        }

        if let Some(value) = captures.name(KEY_COLOR) {
            token.tokens.insert(KEY_COLOR.to_string(), value.as_str().to_string());
        }

        if let Some(value) = captures.name(KEY_AUDIO_CODEC) {
            let mut value = value.as_str().to_string();

            // 处理audio和atmos大小写
            value = value.replace("audio", "Audio").to_string();
            value = value.replace("atmos", "Atmos").to_string();

            // 处理DTS-HD的各种组合
            value = value.replace(" MA ", ".MA.").to_string();
            value = value.replace(".MA5", ".MA.5").to_string();
            value = value.replace(".MA2", ".MA.2").to_string();
            value = value.replace(".MA7", ".MA.7").to_string();
            value = value.replace("DMa5", "D.MA.5").to_string();
            value = value.replace("DMa7", "D.MA.7").to_string();
            value = value.replace("DMa2", "D.MA.2").to_string();
            value = value.replace("Dts", "DTS").to_string();
            value = value.replace("HD ", "HD.").to_string();

            // DDP
            value = value.replace("DDP2.0", "DDP.2.0").to_string();
            value = value.replace("DDP5.1", "DDP.5.1").to_string();
            value = value.replace("DDP7.1", "DDP.7.1").to_string();

            //
            value = value.replace("flac_aac", "FLAC.AAC").to_string();
            value = value.replace("flac", "FLAC").to_string();
            value = value.replace("FLAC.H", "FLAC").to_string();

            //
            value = value.replace(".&.", ".").to_string();

            //
            token.tokens.insert(KEY_AUDIO_CODEC.to_string(), value);
        }

        if let Some(value) = captures.name(KEY_RELEASE_GROUP) {
            let release_group = value.as_str().to_string();
            token.tokens.insert(KEY_RELEASE_GROUP.to_string(), release_group);
        }

        if let Some(value) = captures.name(KEY_COUNTRY) {
            token.tokens.insert(KEY_COUNTRY.to_string(), value.as_str().to_string());
        }

        if let Some(value) = captures.name(KEY_SUBTITLE_EN) {
            token.tokens.insert(KEY_SUBTITLE.to_string(), value.as_str().to_string());
        }

        if let Some(value) = captures.name(KEY_SUBTITLE_CN) {
            token.tokens.insert(KEY_SUBTITLE.to_string(), value.as_str().to_string());
        }

        if let Some(value) = captures.name(KEY_SPECIAL) {
            token.tokens.insert(KEY_SPECIAL.to_string(), value.as_str().to_string());
        }

        if let Some(value) = captures.name(KEY_EXTENSION) {
            token.tokens.insert(KEY_EXTENSION.to_string(), value.as_str().to_string());
        }

        // tracing::debug!("tokenizer = {:?}",tokenizer);

        return Some(token);
    }

    return None;
}

pub(crate) fn input_before_replaces(input: &str) -> String {
    let mut new_input = input.to_string();

    if let Some(replaces) = &TV_RULES.before_replaces {
        replaces.iter().for_each(|replace| {
            if new_input.contains(&replace.src) {
                tracing::debug!("replace.src = {:?} replace.target = {:?}", replace.src, replace.target);
                new_input = new_input.replace(&replace.src, &replace.target);
            }
        })
    }

    if let Some(replaces) = &MOVIE_RULES.before_replaces {
        replaces.iter().for_each(|replace| {
            if new_input.contains(&replace.src) {
                tracing::debug!("replace.src = {:?} replace.target = {:?}", replace.src, replace.target);
                new_input = new_input.replace(&replace.src, &replace.target);
            }
        })
    }

    return new_input;
}

pub fn input_before_process(before_token: &mut Token, input: &str) -> String {
    let mut new_input = input.to_string();

    // 使用正则提前识别发布方和视频格式
    let mut release_group_format: String = format!("{}.{}", wrap(KEY_RELEASE_GROUP), wrap(KEY_EXTENSION));
    release_group_format = release_group_format.replace(&wrap(KEY_RELEASE_GROUP), &regex_item_release_group());
    release_group_format = release_group_format.replace(&wrap(KEY_EXTENSION), &regex_item_extension());
    let release_group_format_regex = Regex::new(&release_group_format).unwrap();
    if let Some(capture) = release_group_format_regex.captures(&new_input) {
        if let Some(value) = capture.name(KEY_RELEASE_GROUP) {
            let mut value = value.as_str().to_string();
            tracing::debug!("release_group_format = {}", value);
            new_input = new_input.replace(&format!(" {}.", value), ".").to_string();
            new_input = new_input.replace(&format!(".{}.", value), ".").to_string();
            new_input = new_input.replace(&format!("-{}.", value), ".").to_string();
            before_token.tokens.insert(KEY_RELEASE_GROUP.to_string(), value);
        }
    }

    // 使用正则移除公司，简化逻辑
    Regex::new(&regex_item_company()).unwrap().find_iter(&new_input.clone()).for_each(|m| {
        let mut value = m.as_str().to_string();
        tracing::debug!("company = {}", value);
        new_input = new_input.replace(&format!(" {} ", value), " ").to_string();
        new_input = new_input.replace(&format!(".{}.", value), ".").to_string();
        new_input = new_input.replace(&format!("{}.", value), "").to_string();
        new_input = new_input.replace(&format!(".{}", value), ".").to_string();
        new_input = new_input.replace(&format!("[{}]", value), "").to_string();
    });

    // 使用正则移除国家，简化逻辑
    if let Some(capture) = Regex::new(&regex_item_country()).unwrap().captures(&new_input) {
        if let Some(value) = capture.name(KEY_COUNTRY) {
            let mut value = value.as_str().to_string();
            tracing::debug!("country = {}", value);
            new_input = new_input.replace(&format!(" {} ", value), " ").to_string();
            new_input = new_input.replace(&format!(".{}.", value), ".").to_string();
            new_input = new_input.replace(&format!("{}.", value), "").to_string();
            new_input = new_input.replace(&format!(".{}", value), ".").to_string();
        }
    }

    // 使用正则移除特典
    if let Some(capture) = Regex::new(&regex_item_special()).unwrap().captures(&new_input) {
        if let Some(value) = capture.name(KEY_SPECIAL) {
            let mut value = value.as_str().to_string();
            tracing::debug!("special = {}", value);
            new_input = new_input.replace(&format!(".{}.", value), ".").to_string();
            new_input = new_input.replace(&format!("{}.", value), "").to_string();
            new_input = new_input.replace(&format!(".{}", value), ".").to_string();
        }
    }

    // 使用正则移除字幕
    if let Some(capture) = Regex::new(&regex_item_subtitle_en()).unwrap().captures(&new_input) {
        if let Some(value) = capture.name(KEY_SUBTITLE_EN) {
            let mut value = value.as_str().to_string();
            tracing::debug!("subtitle_en = {}", value);
            new_input = new_input.replace(&format!(".{}.", value), ".").to_string();
            new_input = new_input.replace(&format!("{}.", value), "").to_string();
            new_input = new_input.replace(&format!(".{}", value), ".").to_string();
        }
    }

    // 使用正则移除字幕
    if let Some(capture) = Regex::new(&regex_item_subtitle_cn()).unwrap().captures(&new_input) {
        if let Some(value) = capture.name(KEY_SUBTITLE_CN) {
            let mut value = value.as_str().to_string();
            tracing::debug!("subtitle_cn = {}", value);
            new_input = new_input.replace(&format!(".{}.", value), ".").to_string();
            new_input = new_input.replace(&format!("{}.", value), "").to_string();
            new_input = new_input.replace(&format!(".{}]", value), "]").to_string();
            new_input = new_input.replace(&format!("{}]", value), "]").to_string();
        }
    }

    // 使用正则移除音频
    if let Some(capture) = Regex::new(&regex_item_audio_cn()).unwrap().captures(&new_input) {
        if let Some(value) = capture.name(KEY_AUDIO_CN) {
            let mut value = value.as_str().to_string();
            tracing::debug!("audio_cn = {}", value);
            new_input = new_input.replace(&format!(" {}]", value), "]").to_string();
        }
    }

    // 使用正则移除版本
    if let Some(capture) = Regex::new(&regex_item_version()).unwrap().captures(&new_input) {
        if let Some(value) = capture.name(KEY_VERSION) {
            let mut value = value.as_str().to_string();
            tracing::debug!("version = {}", value);
            new_input = new_input.replace(&format!(".{}.", value), ".").to_string();
        }
    }

    // 使用正则处理季集信息
    // $season$.$episode$
    if let Ok(regex) = Regex::new(&regex_item_season_and_episode()) {
        if let Some(captures) = regex.captures(&new_input) {
            let season = captures.name(KEY_SEASON).unwrap().as_str().to_string();
            let episode = captures.name(KEY_EPISODE).unwrap().as_str().to_string();
            let new_episode = episode.replace("EP", "E");
            let new_episode = new_episode.replace("Ep", "E");
            tracing::debug!("season = {}, episode = {}, new_episode = {}", season, episode, new_episode);
            new_input = new_input
                .replace(&format!("{}.{}", season, episode), &format!("{}{}", season, new_episode))
                .to_string();
        }
    }
    if let Ok(regex) = Regex::new(r"\.(?P<season>S\d{2})(?P<episode>E[Pp]?\d{1})\.") {
        if let Some(captures) = regex.captures(&new_input) {
            let season = captures.name(KEY_SEASON).unwrap().as_str().to_string();
            let episode = captures.name(KEY_EPISODE).unwrap().as_str().to_string();
            let new_episode = episode.replace("EP", "E");
            let new_episode = new_episode.replace("Ep", "E");
            let new_episode: i32 = new_episode.replace("E", "").parse().unwrap();
            tracing::debug!(
                "input = {}, season = {}, episode = {}, new_episode = {}",
                new_input,
                season,
                episode,
                new_episode
            );
            new_input = new_input
                .replace(&format!("{}{}", season, episode), &format!("{}E{:02}", season, new_episode))
                .to_string();
        }
    }
    if let Ok(regex) = Regex::new(r"\.(?P<episode>E[Pp]?\d{1})\.") {
        if let Some(captures) = regex.captures(&new_input) {
            let episode = captures.name(KEY_EPISODE).unwrap().as_str().to_string();
            let new_episode = episode.replace("EP", "E");
            let new_episode = new_episode.replace("Ep", "E");
            let new_episode: i32 = new_episode.replace("E", "").parse().unwrap();
            tracing::debug!("input = {}, episode = {}, new_episode = {}", new_input, episode, new_episode);
            new_input = new_input.replace(&format!("{}", episode), &format!("E{:02}", new_episode)).to_string();
        }
    }

    // 使用正则提前识别颜色
    Regex::new(&regex_item_color()).unwrap().find_iter(&new_input.clone()).for_each(|m| {
        let value = m.as_str().to_string();
        tracing::debug!("color = {}", value);
        // color的识别性不是很强，所以要做强制处理
        new_input = new_input.replace(&format!("{} ", value), "").to_string();
        new_input = new_input.replace(&format!("[{}_", value), "[").to_string();
        new_input = new_input.replace(&format!("_{}]", value), "]").to_string();
        new_input = new_input.replace(&format!(" {} ", value), " ").to_string();
        new_input = new_input.replace(&format!(".{}.", value), ".").to_string();
        new_input = new_input.replace(&format!(".{}-", value), "-").to_string();
        new_input = new_input.replace(&format!("_{}", value), "").to_string();
    });

    // 使用正则提前识别发布方
    let mut release_group = format!("{}$", wrap(KEY_RELEASE_GROUP)).to_string();
    release_group = release_group.replace(&wrap(KEY_RELEASE_GROUP), &regex_item_release_group());
    let release_group_regex = Regex::new(&release_group).unwrap();
    if let Some(capture) = release_group_regex.captures(&new_input) {
        if let Some(value) = capture.name(KEY_RELEASE_GROUP) {
            let mut value = value.as_str().to_string();
            new_input = new_input.replace(&format!(".{}", value), "").to_string();
            new_input = new_input.replace(&format!("-{}", value), "").to_string();
            tracing::debug!("release_group = {}", value);
            before_token.tokens.insert(KEY_RELEASE_GROUP.to_string(), value);
        }
    }

    let mut release_group = format!("\\[{}\\]", wrap(KEY_RELEASE_GROUP)).to_string();
    release_group = release_group.replace(&wrap(KEY_RELEASE_GROUP), &regex_item_release_group());
    let release_group_regex = Regex::new(&release_group).unwrap();
    if let Some(capture) = release_group_regex.captures(&new_input) {
        if let Some(value) = capture.name(KEY_RELEASE_GROUP) {
            let mut value = value.as_str().to_string();
            new_input = new_input.replace(&format!("[{}]", value), "").to_string().trim().to_string();
            tracing::debug!("release_group = {}", value);
            before_token.tokens.insert(KEY_RELEASE_GROUP.to_string(), value);
        }
    }

    // 使用正则移除edition
    Regex::new(&regex_item_edition()).unwrap().find_iter(&new_input.clone()).for_each(|m| {
        let value = m.as_str().to_string();
        tracing::debug!("edition = {}", value);
        new_input = new_input.replace(&format!("-{}", value), "").to_string();
        new_input = new_input.replace(&format!("{}-", value), "").to_string();
        new_input = new_input.replace(&format!(".{}.", value), ".").to_string();
        new_input = new_input.replace(&format!("{}.", value), "").to_string();
    });

    new_input
}

fn input_before(rule: &Rule, input: &str) -> String {
    let mut new_input = input.to_string();

    // 处理正则替换逻辑
    if let Some(replaces) = &rule.regex_replaces {
        replaces.iter().for_each(|replace| {
            let mut regex_str = replace.src.to_string();
            regex_str = regex_str.replace("$episode_cn$", &regex_item_episode_cn());

            tracing::debug!("regex_str = {}", regex_str);

            let regex = Regex::new(&regex_str).unwrap();
            if let Some(capture) = regex.captures(&new_input) {
                if let Some(value) = capture.name("episode_cn") {
                    let target = replace.target.replace("$episode_cn$", value.as_str());
                    let mut src = value.as_str().to_string();
                    new_input = new_input.replace(&src, &target).to_string().trim().to_string();
                    tracing::debug!("regex_replaces new_input = {}", new_input);
                }
            }
        })
    }

    // 处理普通的替换逻辑
    if let Some(replaces) = &rule.replaces {
        replaces.iter().for_each(|replace| {
            if new_input.contains(&replace.src) {
                tracing::debug!("replaces src = {}, target = {}", replace.src, replace.target);
                new_input = new_input.replace(&replace.src, &replace.target);
            }
        })
    }

    new_input
}

fn read_match_rules(file_path: &str) -> Result<MatchRule, Box<dyn Error>> {
    // 打开文件
    let mut file = File::open(file_path)?;

    // 读取文件内容到字符串
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;

    // 使用serde_json库将JSON字符串反序列化为Rust对象
    let data: MatchRule = serde_json::from_str(&file_contents)?;

    Ok(data)
}

fn read_rule_items(file_path: &str) -> Result<RegexRule, Box<dyn Error>> {
    // 打开文件
    let mut file = File::open(file_path)?;

    // 读取文件内容到字符串
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;

    // 使用serde_json库将JSON字符串反序列化为Rust对象
    let data: RegexRule = serde_json::from_str(&file_contents)?;

    Ok(data)
}

fn regex_item_multi(name: &str, index: i32, regex: &Vec<String>) -> String {
    let mut combined = if index == 0 {
        String::from(format!("(?P<{}>(", name))
    } else {
        String::from(format!("(?P<{}{}>(", name, index))
    };
    let last_index = regex.len() - 1;
    for (index, item) in regex.iter().enumerate() {
        combined.push_str(item);
        if index != (last_index) {
            combined.push_str("|")
        }
    }
    combined.push_str("))");
    return combined;
}

fn regex_item_multi_str(name: &str, index: i32, regex: &Vec<&str>) -> String {
    let mut combined = if index == 0 {
        String::from(format!("(?P<{}>(", name))
    } else {
        String::from(format!("(?P<{}{}>(", name, index))
    };
    let last_index = regex.len() - 1;
    for (index, item) in regex.iter().enumerate() {
        combined.push_str(item);
        if index != (last_index) {
            combined.push_str("|")
        }
    }
    combined.push_str("))");
    return combined;
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    use crate::soda::meta::strong_match_token::*;

    #[test]
    fn test_regex_item_title_number_en() {
        let regex = Regex::new(&regex_item_title_number_en()).unwrap();
        assert_eq!(
            "Attenborough 60 Years in the Wild",
            regex
                .captures("Attenborough 60 Years in the Wild")
                .unwrap()
                .name("title_number_en")
                .unwrap()
                .as_str()
        );
        assert_eq!(
            "Reply.1988",
            regex.captures("Reply.1988").unwrap().name("title_number_en").unwrap().as_str()
        );
        assert_eq!(
            "John Wick Chapter 4",
            regex.captures("John Wick Chapter 4").unwrap().name("title_number_en").unwrap().as_str()
        );
        assert_eq!(
            "13.Reasons.Why",
            regex.captures("13.Reasons.Why").unwrap().name("title_number_en").unwrap().as_str()
        );
        assert_eq!(
            "Evangelion 2.22 You Can (Not) Advance",
            regex
                .captures("Evangelion 2.22 You Can (Not) Advance")
                .unwrap()
                .name("title_number_en")
                .unwrap()
                .as_str()
        );
        assert_eq!(
            "Jujutsu Kaisen 0",
            regex.captures("Jujutsu Kaisen 0").unwrap().name("title_number_en").unwrap().as_str()
        );
    }

    #[test]
    fn test_regex_item_episode_number2() {
        let re = Regex::new(r"(?P<title_en>[!×I&'-. A-Za-z]+)\.(?P<episode_number>\d{2,3})\.(?P<resolution>240[Ppi]|360[Ppi]|480[Ppi]|576[Pp]|720[Ppi]|1080[Ppi]|1440[Ppi]|2160[Ppi]|4320[Ppi]|4[Kk])\.(?P<video_codec>([XxHh]\.?26[45]|(HEVC|Hevc)|MPEG-\d|VC-\d|MPEG2|VP9|AV1|VC1|AVC))\.(?P<extension>(mp4|mkv|iso|rmvb|avi|mov|mepg|mpg|wmv|3gp|asf|m4v|flv))$").unwrap();
        let text = "Dragon.Ball.150.480p.x264.mkv";

        if let Some(caps) = re.captures(text) {
            println!("Title (en): {}", &caps["title_en"]);
            println!("Episode Number: {}", &caps["episode_number"]);
            println!("Resolution: {}", &caps["resolution"]);
            println!("Video Codec: {}", &caps["video_codec"]);
            println!("Format: {}", &caps["extension"]);
        } else {
            println!("No match found");
        }
    }

    #[test]
    fn test_regex_item_episode() {
        let regex = Regex::new(&regex_item_episode()).unwrap();
        assert_eq!("E01", regex.captures("E01").unwrap().name("episode").unwrap().as_str());
        assert_eq!("E001", regex.captures("E001").unwrap().name("episode").unwrap().as_str());
    }

    #[test]
    fn test_regex_item_episode_number() {
        let regex = Regex::new(&regex_item_episode_number()).unwrap();
        assert_eq!("150", regex.captures("150").unwrap().name("episode_number").unwrap().as_str());
    }

    #[test]
    fn test_regex_item_episode_title_jp() {
        let regex = Regex::new(&regex_item_episode_title_jp()).unwrap();
        assert_eq!(
            "桜舞うソラに",
            regex.captures("桜舞うソラに").unwrap().name("episode_title_jp").unwrap().as_str()
        );
    }

    #[test]
    fn test_regex_item_title_number_cn() {
        let regex = Regex::new(&regex_item_title_number_cn()).unwrap();
        assert_eq!(
            "拜托了！8小时",
            regex.captures("拜托了！8小时").unwrap().name("title_number_cn").unwrap().as_str()
        );
        assert_eq!(
            "风味人间3·大海小鲜",
            regex.captures("风味人间3·大海小鲜").unwrap().name("title_number_cn").unwrap().as_str()
        );
        assert_eq!(
            "风味人间4",
            regex.captures("风味人间4").unwrap().name("title_number_cn").unwrap().as_str()
        );
    }

    #[test]
    fn test_regex_item_title_cn() {
        let regex = Regex::new(&regex_item_title_cn()).unwrap();
        assert_eq!(
            "与摩根·弗里曼一起穿越虫洞",
            regex.captures("与摩根·弗里曼一起穿越虫洞").unwrap().name("title_cn").unwrap().as_str()
        );
    }

    #[test]
    fn test_regex_item_episode_title_en() {
        let regex = Regex::new(&regex_item_episode_title_en()).unwrap();
        assert_eq!("1+1", regex.captures("1+1").unwrap().name("episode_title_en").unwrap().as_str());
        assert_eq!("2+2", regex.captures("2+2").unwrap().name("episode_title_en").unwrap().as_str());
        assert_eq!(
            "Tape.Side.A",
            regex.captures("Tape.Side.A").unwrap().name("episode_title_en").unwrap().as_str()
        );
        assert_eq!(
            "Tape.1.Side.A",
            regex.captures("Tape.1.Side.A").unwrap().name("episode_title_en").unwrap().as_str()
        );
        assert_eq!(
            "Tape.Side.A.1",
            regex.captures("Tape.Side.A.1").unwrap().name("episode_title_en").unwrap().as_str()
        );
        assert_eq!(
            "994.Cars.Long",
            regex.captures("994.Cars.Long").unwrap().name("episode_title_en").unwrap().as_str()
        );
        assert_eq!(
            "Route.666",
            regex.captures("Route.666").unwrap().name("episode_title_en").unwrap().as_str()
        );
        assert_eq!(
            "Docket.No.11-19-41-73",
            regex
                .captures("Docket.No.11-19-41-73")
                .unwrap()
                .name("episode_title_en")
                .unwrap()
                .as_str()
        );
    }

    #[test]
    fn test_parse4() {
        // $title_cn$.$title_en$.$year$.$season$.$resolution$.$source$.$video_codec$.$audio_codec$-$release_group$
        let input = "凡人修仙传.The.Mortal.Ascention.2020.S01.2160p.WEB-DL.H.264.AAC-OurTV";
        let regex = Regex::new(r"\((?P<year_month_day>\d{4}\.\d{2}\.\d{2})\)(?P<title_number_en>(([\\(\\)&'-. 0-9A-Za-z])+))-\[(?P<resolution>(240[Ppi]|360[Ppi]|720[Ppi]|1080[Ppi]|1440[Ppi]|2160[Ppi]|4320[Ppi]|4[Kk]))\]\[(?P<country>(Ger|GER|USA)).(?P<source>(AVC.REMUX|Blu-ray Remux|Blu-ray.REMUX|Blu-ray.Remux|BluRay.REMUX|BluRay.Remux|Blu-Ray|BluRay|Bluray|BDRIP|BDRip|BDrip|WEBRip|WEBrip|WEB-DL|WEB|Blu-?[Rr]ay|DVDRip|HDTVRip|BRRip|HDRip|CAM|HDTV|BD[.-]?Remux|REMUX|Remux))\].(?P<extension>(mp4|mkv|ts|iso|rmvb|avi|mov|mepg|mpg|wmv|3gp|asf|m4v|flv|m2ts))").unwrap();
        if regex.is_match(input) {
            println!("match");
        }
        if let Some(captures) = regex.captures(&input) {
            if let Some(value) = captures.name("year_month_day") {
                println!("匹配到: {}", value.as_str().to_string());
            }
            if let Some(value) = captures.name("title_cn") {
                println!("匹配到: {}", value.as_str().to_string());
            }
            if let Some(value) = captures.name("title_number_en") {
                println!("匹配到: {}", value.as_str().to_string());
            }
            if let Some(value) = captures.name("title_en") {
                println!("匹配到: {}", value.as_str().to_string());
            }
            if let Some(value) = captures.name("year") {
                println!("匹配到: {}", value.as_str().to_string());
            }
            if let Some(value) = captures.name("season_episode") {
                println!("匹配到: {}", value.as_str().to_string());
            }
            if let Some(value) = captures.name("season_cn") {
                println!("匹配到: {}", value.as_str().to_string());
            }
            if let Some(value) = captures.name("season_all_cn") {
                println!("匹配到: {}", value.as_str().to_string());
            }
            if let Some(value) = captures.name("resolution") {
                println!("匹配到: {}", value.as_str().to_string());
            }
            if let Some(value) = captures.name("source") {
                println!("匹配到: {}", value.as_str().to_string());
            }
            if let Some(value) = captures.name("video_codec") {
                println!("匹配到: {}", value.as_str().to_string());
            }
            if let Some(value) = captures.name("audio_codec") {
                println!("匹配到: {}", value.as_str().to_string());
            }
            if let Some(value) = captures.name("release_group") {
                println!("匹配到: {}", value.as_str().to_string());
            }
            if let Some(value) = captures.name("format") {
                println!("匹配到: {}", value.as_str().to_string());
            }
        }
    }
}
