use std::collections::HashMap;
use std::error::Error;
use std::fs::{self, File};
use std::io::Read;
use std::sync::Mutex;

use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;

use crate::soda::extension_option::OptionExtensions;
use crate::soda::LIB_CONFIG;

use super::token::Tokens;

#[derive(Deserialize, Debug, Clone)]
struct Rule {
    rule: String,
    replaces: Option<Vec<RuleReplaces>>,
}

impl Rule {
    fn update(&mut self, rule: Rule) {
        self.rule = rule.rule;
        self.replaces = rule.replaces;
    }
}

#[derive(Deserialize, Debug, Clone)]
struct RuleReplaces {
    src: String,
    target: String,
}

#[derive(Deserialize, Debug, Clone)]
struct MatchRule {
    before_replaces: Option<Vec<RuleReplaces>>,
    rules: Vec<Rule>,
}

#[derive(Deserialize, Debug, Clone)]
struct RegexRule {
    source: Vec<String>,
    company: Vec<String>,
    video_codec: Vec<String>,
    color: Vec<String>,
    audio_codec: Vec<String>,
    release_group: Vec<String>,
}

pub(crate) fn init() {
    let source = &REGEX_RULES;
    let before_replaces = &RULES;
    let last_rule = &LAST_RULE;
}

static LAST_RULE: Lazy<Mutex<Rule>> = Lazy::new(|| Mutex::new(RULES.rules[0].clone()));

static REGEX_RULES: Lazy<RegexRule> = Lazy::new(|| {
    let config = LIB_CONFIG.lock().unwrap();

    let content = fs::read_to_string(config.strong_match_regex_rules_path.as_str()).unwrap();

    serde_json::from_str(&content).unwrap()
});

static RULES: Lazy<MatchRule> = Lazy::new(|| {
    let config = LIB_CONFIG.lock().unwrap();

    let content = fs::read_to_string(config.strong_match_rules_path.as_str()).unwrap();

    serde_json::from_str(&content).unwrap()
});

pub(crate) fn build_mt_file_tokens(title: &str) -> Option<Tokens> {
    let enable_cache = LIB_CONFIG.lock().unwrap().strong_match_regex_enable_cache;

    if enable_cache {
        let mut last_rule = LAST_RULE.lock().unwrap();
        if let Some(token) = build_tokens(&last_rule, title) {
            return Some(token);
        }
    }

    let rules = &RULES.rules;
    for rule in rules {
        if let Some(token) = build_tokens(rule, title) {
            if enable_cache {
                let mut last_rule = LAST_RULE.lock().unwrap();
                last_rule.update(rule.clone());
            }
            return Some(token);
        }
    }
    return None;
}

pub(crate) fn build_mt_dir_tokens(title: &str) -> Option<Tokens> {
    // let mut last_rule = LAST_RULE.lock().unwrap();
    // if let Some(token) = build_tokens(&last_rule, title) {
    //     return Some(token);
    // }

    for rule in &RULES.rules {
        let mut new_rule = rule.clone();
        new_rule.rule = new_rule.rule.replace(".$extension$", "");
        new_rule.rule = new_rule.rule.replace("$season_episode$", "$season$");
        if let Some(token) = build_tokens(&new_rule, title) {
            // last_rule.update(rule.clone());
            return Some(token);
        }
    }
    return None;
}

/// 匹配中文
/// (?P<title_cn>...)：这是一个命名捕获组，名为 "title_cn"。捕获组可以用来从匹配的文本中提取出特定部分。
/// [：\p{Script=Han}\p{Script=Hiragana}]：这是一个字符集，匹配任何在集合中的字符。这个集合包括冒号（：），以及任何属于汉字（Han）或平假名（Hiragana）的字符。
/// +：这表示前面的字符集可以在字符串中出现一次或多次。
/// 所以，整个表达式的意思是：匹配一次或多次的冒号，汉字或平假名字符，并将匹配的部分作为名为 "title_cn" 的捕获组。
fn regex_item_title_cn() -> &'static str {
    r"(?P<title_cn>[：\p{Script=Han}\p{Script=Hiragana}]+)"
}

/// 匹配中文
/// 这个正则表达式似乎有一些错误，它应该是 r"(?P<name>[0-9A-Za-z]+[：\p{Script=Han}\p{Script=Hiragana}]+)"。下面是各部分的解释：
/// (?P<name>...)：这是一个命名捕获组，名为 "name"。捕获组可以用来从匹配的文本中提取出特定部分。
/// [0-9A-Za-z]：这是一个字符集，匹配任何在集合中的字符。这个集合包括所有的数字（0-9），大写字母（A-Z）和小写字母（a-z）。
/// +：这表示前面的字符集可以在字符串中出现一次或多次。
/// [：\p{Script=Han}\p{Script=Hiragana}]：这是另一个字符集，匹配任何在集合中的字符。这个集合包括冒号（：），以及任何属于汉字（Han）或平假名（Hiragana）的字符。
/// +：这表示前面的字符集可以在字符串中出现一次或多次。
/// 所以，整个表达式的意思是：匹配一次或多次的数字、字母，后面跟着一次或多次的冒号，汉字或平假名字符，并将匹配的部分作为名为 "name" 的捕获组。
fn regex_item_title_number_cn() -> &'static str {
    r"(?P<title_number_cn>[0-9A-Za-z]+[：\p{Script=Han}\p{Script=Hiragana}]+)"
}

/// 匹配英文名称
fn regex_item_title_en() -> &'static str {
    r"(?P<title_en>[I&'-. A-Za-z]+)"
}

/// 匹配英文名称
/// 这是一个正则表达式，用于匹配一种特定的字符串模式。下面是各部分的解释：
/// (?P<title_number_en>...)：这是一个命名捕获组，名为 "title_number_en"。捕获组可以用来从匹配的文本中提取出特定部分。
/// [\\()&'-. 0-9A-Za-z]：这是一个字符集，匹配任何在集合中的字符。这个集合包括反斜杠（\），括号（()），和号（&），单引号（'），破折号（-），点（.），空格（ ），所有的数字（0-9），大写字母（A-Z）和小写字母（a-z）。
/// +：这表示前面的字符集可以在字符串中出现一次或多次。
/// 所以，整个表达式的意思是：匹配一次或多次的上述字符集中的字符，并将匹配的部分作为名为 "title_number_en" 的捕获组
fn regex_item_title_number_en() -> &'static str {
    r"(?P<title_number_en>[\\()&'-. 0-9A-Za-z]+)"
}

/// 匹配标题年
fn regex_item_title_year() -> &'static str {
    r"(?P<title_year>\d{4})"
}

/// 匹配英文季名
/// https://docs.rs/regex/latest/regex/
/// Strange.Love
/// 1+1
/// Making.of
/// Tape.1.Side.A
/// Tape.Side.A.1
/// 这是一个正则表达式，用于匹配一种特定的字符串模式。下面是各部分的解释：
/// (?P<episode_title_en>...)：这是一个命名捕获组，名为 "episode_title_en"。捕获组可以用来从匹配的文本中提取出特定部分。
/// [&-. A-Za-z]+[0-9]{1,4}[&-. A-Za-z]+：这个模式匹配一次或多次的字符集中的字符（包括&，-，.，空格，以及所有的大写和小写字母），后面跟着1到4个数字，再后面跟着一次或多次的字符集中的字符。
/// [&-. A-Za-z]+[0-9]{1}：这个模式匹配一次或多次的字符集中的字符，后面跟着一个数字。
//// [&-. A-Za-z]+：这个模式匹配一次或多次的字符集中的字符。
/// [0-9]+\+[0-9]+：这个模式匹配一次或多次的数字，后面跟着一个加号，再后面跟着一次或多次的数字。
/// |：这是或（or）操作符，表示匹配前面或后面的模式。
/// 所以，整个表达式的意思是：匹配四种可能的模式之一，并将匹配的部分作为名为 "episode_title_en" 的捕获组
fn regex_item_episode_title_en() -> &'static str {
    r"(?P<episode_title_en>[&-. A-Za-z]+[0-9]{1,4}[&-. A-Za-z]+|[&-. A-Za-z]+[0-9]{1}|[&-. A-Za-z]+|[0-9]+\+[0-9]+)"
}

/// 匹配中文名
fn regex_item_episode_title_cn() -> &'static str {
    r"(?P<episode_title_cn>[：\p{Script=Han}\p{Script=Hiragana}]+)"
}

/// 匹配年 2023
fn regex_item_year() -> &'static str {
    r"(?P<year>\d{4})"
}

/// 匹配年 2021-2023
fn regex_item_year_start_to_end() -> &'static str {
    r"(?P<year_start_to_end>\d{4}-\d{4})"
}

/// 匹配 2023-12-01
fn regex_item_year_month_day() -> &'static str {
    r"(?P<year_month_day>\d{4}\.\d{2}\.\d{2})"
}

/// 匹配季和集 S01E01
fn regex_item_season_episode() -> &'static str {
    r"(?P<season_episode>S\d{1,2}E\d{2})"
}

/// 匹配季和集 S01E01E02
fn regex_item_season_episode_episode() -> &'static str {
    r"(?P<season_episode_episode>S\d{2}E\d{2}E\d{2})"
}

/// 匹配季和集 S01E01
fn regex_item_season_episode_split() -> &'static str {
    r"(?P<season>S\d{1,2})(?P<episode>E[Pp]?\d{2})"
}

/// 匹配季和集 S01E01E02
fn regex_item_season_episode_episode_split() -> &'static str {
    r"(?P<season>S\d{2})(?P<episode1>E[Pp]?\d{2})(?P<episode2>E[Pp]?\d{2})"
}

/// match episode E01 EP01 Ep01
/// 也可以匹配 S01E01中的E01，所以解析逻辑需要放在regex_item_season_episode之后
fn regex_item_episode() -> &'static str {
    r"(?P<episode>E[Pp]?\d{2})"
}

/// 匹配两位数字
fn regex_item_episode_number() -> &'static str {
    r"(?P<episode_number>\d{2})"
}

/// match season S01
fn regex_item_season() -> &'static str {
    r"(?P<season>S\d{2})"
}

/// match 第01季
fn regex_item_season_cn() -> &'static str {
    r"(?P<season_cn>第\d{2}季)"
}

/// 全十一季
fn regex_item_season_all_cn() -> &'static str {
    r"(?P<season_all_cn>全[一二三四五六七八九十]+季)"
}

/// 第01-08季
fn regex_item_season_start_to_end_cn() -> &'static str {
    r"(?P<season_start_to_end_cn>第\d{2}-\d{2}季)"
}

/// S01-S03
fn regex_item_season_start_to_end_en() -> &'static str {
    r"(?P<season_start_to_end_en>S\d{2}-S\d{2})"
}

/// 匹配分辨率 1080P/1080p/1080i
fn regex_item_resolution() -> &'static str {
    r"(?P<resolution>240[Ppi]|360[Ppi]|480[Ppi]|720[Ppi]|1080[Ppi]|1440[Ppi]|2160[Ppi]|4320[Ppi]|4[Kk])"
}

/// 版本
/// [三体].Three-Body.2023.S01E01.2160p.V3.WEB-DL.HEVC.DDP5.1.Atmos.&.AAC-QHstudIo.mp4
fn regex_item_version() -> &'static str {
    r"(?P<version>V\d{1})"
}

/// 匹配国家
fn regex_item_country() -> &'static str {
    r"(?P<country>US|JP|Ger|GER|USA)"
}

/// 匹配字幕语言
fn regex_item_subtitle_en() -> &'static str {
    r"(?P<subtitle_en>SweSub)"
}

/// 匹配中文名
fn regex_item_subtitle_cn() -> &'static str {
    r"(?P<subtitle_cn>中[英|美|日|挪]字幕)"
}

/// 匹配特典
fn regex_item_special() -> &'static str {
    r"(?P<special>Manual)"
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
    let mut combined = String::from("(?P<source>(");
    let last_index = REGEX_RULES.source.len() - 1;
    for (index, item) in REGEX_RULES.source.iter().enumerate() {
        combined.push_str(item);
        if index != (last_index) {
            combined.push_str("|")
        }
    }
    combined.push_str("))");
    return combined;
}

fn regex_item_source_multi(index: i32) -> String {
    let mut combined = if index == 0 { String::from("(?P<source>(") } else { String::from(format!("(?P<source{}>(", index)) };
    let last_index = REGEX_RULES.source.len() - 1;
    for (index, item) in REGEX_RULES.source.iter().enumerate() {
        combined.push_str(item);
        if index != (last_index) {
            combined.push_str("|")
        }
    }
    combined.push_str("))");
    return combined;
}

/// 匹配视频源厂商
/// netflix = nf
fn regex_item_company() -> String {
    let mut combined = String::from("(?P<company>(");
    let last_index = REGEX_RULES.company.len() - 1;
    for (index, item) in REGEX_RULES.company.iter().enumerate() {
        combined.push_str(item);
        if index != (last_index) {
            combined.push_str("|")
        }
    }
    combined.push_str("))");
    return combined;
}

/// 匹配视频编码器
fn regex_item_video_codec() -> String {
    let mut combined = String::from("(?P<video_codec>(");
    let last_index = REGEX_RULES.video_codec.len() - 1;
    for (index, item) in REGEX_RULES.video_codec.iter().enumerate() {
        combined.push_str(item);
        if index != (last_index) {
            combined.push_str("|")
        }
    }
    combined.push_str("))");
    return combined;
}

/// 视频编码器扩展信息
/// https://www.reddit.com/r/VideoEditing/comments/b45b85/10_bit_12_bit_or_h265_which_ones_the_best_and_has/
/// color depth
/// FPS
fn regex_item_color() -> String {
    let mut combined = String::from("(?P<color>(");
    let last_index = REGEX_RULES.color.len() - 1;
    for (index, item) in REGEX_RULES.color.iter().enumerate() {
        combined.push_str(item);
        if index != (last_index) {
            combined.push_str("|")
        }
    }
    combined.push_str("))");
    return combined;
}

fn regex_item_color_multi(index: i32) -> String {
    let mut combined = if index == 0 { String::from("(?P<color>(") } else { String::from(format!("(?P<color{}>(", index)) };
    let last_index = REGEX_RULES.color.len() - 1;
    for (index, item) in REGEX_RULES.color.iter().enumerate() {
        combined.push_str(item);
        if index != (last_index) {
            combined.push_str("|")
        }
    }
    combined.push_str("))");
    return combined;
}

/// 匹配音频编码器
fn regex_item_audio_codec() -> String {
    let mut combined = String::from("(?P<audio_codec>(");
    let last_index = REGEX_RULES.audio_codec.len() - 1;
    for (index, item) in REGEX_RULES.audio_codec.iter().enumerate() {
        combined.push_str(item);
        if index != (last_index) {
            combined.push_str("|")
        }
    }
    combined.push_str("))");
    return combined;
}

/// 匹配发布组
fn regex_item_release_group() -> String {
    let mut combined = String::from("(?P<release_group>(");
    let last_index = REGEX_RULES.release_group.len() - 1;
    for (index, item) in REGEX_RULES.release_group.iter().enumerate() {
        combined.push_str(item);
        if index != (last_index) {
            combined.push_str("|")
        }
    }
    combined.push_str("))");
    return combined;
}

/// 视频格式
fn regex_item_format() -> &'static str {
    r"(?P<format>(mp4|mkv|ts|iso|rmvb|avi|mov|mepg|mpg|wmv|3gp|asf|m4v|flv|m2ts))"
}

/// 匹配数字字母混合
/// 例如：4ABDCA70
fn regex_item_mix_numbers_letters() -> &'static str {
    r"(?P<mix_numbers_letters>[0-9A-Za-z]+)"
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
    format = format.replace("$title_cn$", regex_item_title_cn());
    format = format.replace("$title_number_cn$", regex_item_title_number_cn());
    format = format.replace("$title_en$", regex_item_title_en());
    format = format.replace("$title_number_en$", regex_item_title_number_en());
    format = format.replace("$title_year$", regex_item_title_year());
    format = format.replace("$year_start_to_end$", regex_item_year_start_to_end());
    format = format.replace("$year_month_day$", regex_item_year_month_day());
    format = format.replace("$year$", regex_item_year());
    format = format.replace("$season_episode$", regex_item_season_episode());
    format = format.replace("$season_episode_episode$", regex_item_season_episode_episode());
    format = format.replace("$season$", regex_item_season());
    format = format.replace("$season_start_to_end_cn$", regex_item_season_start_to_end_cn());
    format = format.replace("$season_all_cn$", regex_item_season_all_cn());
    format = format.replace("$season_cn$", regex_item_season_cn());
    format = format.replace("$season_start_to_end_en$", regex_item_season_start_to_end_en());
    format = format.replace("$episode$", regex_item_episode());
    format = format.replace("$episode_number$", regex_item_episode_number());
    format = format.replace("$episode_title_en$", regex_item_episode_title_en());
    format = format.replace("$episode_title_cn$", regex_item_episode_title_cn());
    format = format.replace("$resolution$", regex_item_resolution());
    format = format.replace("$version$", regex_item_version());

    //
    let mut source_index = 0;
    while let Some(_index) = format.find("$source$") {
        format = format.replacen("$source$", &regex_item_source_multi(source_index), 1);
        source_index += 1;
    }
    regex_index.insert("$source$".to_string(), source_index);

    format = format.replace("$company$", &regex_item_company());
    format = format.replace("$video_codec$", &regex_item_video_codec());

    //
    let mut color_index = 0;
    while let Some(_index) = format.find("$color$") {
        format = format.replacen("$color$", &regex_item_color_multi(color_index), 1);
        color_index += 1;
    }
    regex_index.insert("$color$".to_string(), color_index);

    //
    format = format.replace("$audio_codec$", &regex_item_audio_codec());
    format = format.replace("$release_group$", &regex_item_release_group());
    format = format.replace("$subtitle_en$", regex_item_subtitle_en());
    format = format.replace("$subtitle_cn$", regex_item_subtitle_cn());
    format = format.replace("$special$", regex_item_special());
    format = format.replace("$country$", regex_item_country());
    format = format.replace("$extension$", &regex_item_format());
    format = format.replace("$mix_numbers_letters$", &regex_item_mix_numbers_letters());

    // build after
    let after = regex_builder_after(format);

    let mut ret = String::new();
    ret.push_str("^");
    ret.push_str(&after);
    ret.push_str("$");
    return ret;
}

fn regex_builder_pre(regex_config: String) -> String {
    // replace special chars
    let config = regex_config.replace("[", "\\[");
    let config = config.replace("]", "\\]");
    let config = config.replace("(", "\\(");
    let config = config.replace(")", "\\)");

    return config;
}

fn regex_builder_after(config: String) -> String {
    return config;
}

fn regex_builder_before(_rule: &Rule, pre: String) -> String {
    return pre;
}

/// build token from input by config
fn build_tokens(rule: &Rule, input: &str) -> Option<Tokens> {
    let mut regex_index: HashMap<String, i32> = HashMap::new();
    let final_regex = regex_build(rule, &mut regex_index);

    let regex = Regex::new(final_regex.as_str()).unwrap();

    let new_input = input_before(rule, input);

    if let Some(captures) = regex.captures(&new_input) {
        let mut tokenizer = Tokens { tokens: HashMap::new() };

        tokenizer.tokens.insert("origin_title".to_string(), input.to_string());

        if let Some(value) = captures.name("title_cn") {
            tokenizer.tokens.insert("title_cn".to_string(), value.as_str().to_string());
        }

        if let Some(value) = captures.name("title_number_cn") {
            tokenizer.tokens.insert("title_cn".to_string(), value.as_str().to_string());
        }

        if let Some(value) = captures.name("title_en") {
            let value = value.as_str().to_string();
            // 替换掉英文中的连接字符
            let value = value.replace(".", " ").trim_end().to_string();
            let value = value.replace("-", " ").trim_end().to_string();
            tokenizer.tokens.insert("title_en".to_string(), value);
        }

        if let Some(value) = captures.name("title_year") {
            tokenizer.tokens.insert("title_year".to_string(), value.as_str().to_string());
            tokenizer.tokens.insert("title_en".to_string(), value.as_str().to_string());
        }

        if let Some(value) = captures.name("title_number_en") {
            let value = value.as_str().to_string();
            tokenizer.tokens.insert("title_en".to_string(), value);
        }

        if let Some(value) = captures.name("year") {
            tokenizer.tokens.insert("year".to_string(), value.as_str().to_string());
        }

        if let Some(value) = captures.name("year_start_to_end") {
            tokenizer.tokens.insert("year_start_to_end".to_string(), value.as_str().to_string());
        }

        if let Some(value) = captures.name("year_month_day") {
            tokenizer.tokens.insert("year_month_day".to_string(), value.as_str().to_string());
        }

        if let Some(value) = captures.name("season_episode") {
            let value = value.as_str().to_string();
            tokenizer.tokens.insert("season_episode".to_string(), value.clone());

            if let Ok(regex) = Regex::new(&regex_item_season_episode_split()) {
                if let Some(captures) = regex.captures(&value) {
                    if let Some(value) = captures.name("season") {
                        tokenizer.tokens.insert("season".to_string(), value.as_str().to_string());
                    }

                    if let Some(value) = captures.name("episode") {
                        tokenizer.tokens.insert("episode".to_string(), value.as_str().to_string());
                    }
                }
            }
        }

        if let Some(value) = captures.name("season_episode_episode") {
            let value = value.as_str().to_string();
            tokenizer.tokens.insert("season_episode_episode".to_string(), value.clone());

            if let Ok(regex) = Regex::new(&regex_item_season_episode_episode_split()) {
                if let Some(captures) = regex.captures(&value) {
                    if let Some(value) = captures.name("season") {
                        tokenizer.tokens.insert("season".to_string(), value.as_str().to_string());
                    }

                    if let Some(value) = captures.name("episode1") {
                        tokenizer.tokens.insert("episode".to_string(), value.as_str().to_string());
                    }

                    if let Some(value) = captures.name("episode2") {
                        tokenizer.tokens.insert("episode2".to_string(), value.as_str().to_string());
                    }
                }
            }
        }

        if let Some(value) = captures.name("season") {
            tokenizer.tokens.insert("season".to_string(), value.as_str().to_string());
        }

        if let Some(value) = captures.name("season_cn") {
            tokenizer.tokens.insert("season_cn".to_string(), value.as_str().to_string());
        }

        if let Some(value) = captures.name("season_all_cn") {
            tokenizer.tokens.insert("season_all_cn".to_string(), value.as_str().to_string());
        }

        if let Some(value) = captures.name("season_start_to_end_cn") {
            tokenizer.tokens.insert("season_start_to_end_cn".to_string(), value.as_str().to_string());
        }

        if let Some(value) = captures.name("season_start_to_end_en") {
            tokenizer.tokens.insert("season_start_to_end_en".to_string(), value.as_str().to_string());
        }

        if let Some(value) = captures.name("episode") {
            tokenizer.tokens.insert("episode".to_string(), value.as_str().to_string());
        }

        if let Some(value) = captures.name("episode_number") {
            tokenizer.tokens.insert("episode".to_string(), value.as_str().to_string());
        }

        if let Some(value) = captures.name("resolution") {
            let value = value.as_str().to_lowercase().to_string();
            let value = value.replace("4k", "2160p");
            let value = value.replace("2k", "2160p");
            tokenizer.tokens.insert("resolution".to_string(), value);
        }

        if let Some(value) = captures.name("source") {
            let mut source = value.as_str().to_string();

            if let Some(index) = regex_index.get("$source$") {
                if *index > 0 {
                    // 处理扩展源
                    if let Some(value) = captures.name("source1") {
                        let source1 = value.as_str().to_string();
                        // 合并源
                        source = format!("{}.{}", source, source1);
                        tokenizer.tokens.insert("source1".to_string(), source1);
                    }
                }
            }

            // 处理BluRay的各种样式
            let bluray_regex = Regex::new(r"Blu[-]?[Rr]ay").unwrap();
            source = bluray_regex.replace(&source, "BluRay").to_string();

            // 处理Remux的各种样式
            source = source.replace("REMUX", "Remux").to_string();

            // 处理BluRay[.- ]Remux的各种样式
            let all_regex = Regex::new(r"BluRay[ -]Remux").unwrap();
            source = all_regex.replace(&source, "BluRay.Remux").to_string();

            //
            tokenizer.tokens.insert("source".to_string(), source);
        }

        if let Some(value) = captures.name("company") {
            tokenizer.tokens.insert("company".to_string(), value.as_str().to_string());
        }

        if let Some(value) = captures.name("video_codec") {
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
            video_codec = video_codec.replace("AVC", "H.264").to_string();

            //
            video_codec = video_codec.replace("HEVC", "H.265").to_string();

            tokenizer.tokens.insert("video_codec".to_string(), video_codec);
        }

        if let Some(index) = regex_index.get("$color$") {
            for i in 0..(*index) {
                if i == 0 {
                    if let Some(value) = captures.name("color") {
                        tokenizer.tokens.insert("color".to_string(), value.as_str().to_string());
                    }
                } else {
                    let target = format!("color{}", index);
                    if let Some(value) = captures.name(&target) {
                        tokenizer.tokens.insert(target, value.as_str().to_string());
                    }
                }
            }
        }

        if let Some(value) = captures.name("audio_codec") {
            let mut audio_codec = value.as_str().to_string();

            // 处理audio和atmos大小写
            audio_codec = audio_codec.replace("audio", "Audio").to_string();
            audio_codec = audio_codec.replace("atmos", "Atmos").to_string();

            // 处理DTS-HD的各种组合
            audio_codec = audio_codec.replace(" MA ", ".MA.").to_string();
            audio_codec = audio_codec.replace(".MA5", ".MA.5").to_string();
            audio_codec = audio_codec.replace(".MA2", ".MA.2").to_string();
            audio_codec = audio_codec.replace(".MA7", ".MA.7").to_string();
            audio_codec = audio_codec.replace("DMa5", "D.MA.5").to_string();
            audio_codec = audio_codec.replace("DMa7", "D.MA.7").to_string();
            audio_codec = audio_codec.replace("DMa2", "D.MA.2").to_string();
            audio_codec = audio_codec.replace("Dts", "DTS").to_string();
            audio_codec = audio_codec.replace("HD ", "HD.").to_string();

            // DDP
            audio_codec = audio_codec.replace("DDP2.0", "DDP.2.0").to_string();
            audio_codec = audio_codec.replace("DDP5.1", "DDP.5.1").to_string();
            audio_codec = audio_codec.replace("DDP7.1", "DDP.7.1").to_string();

            //
            audio_codec = audio_codec.replace(".&.", ".").to_string();

            //
            tokenizer.tokens.insert("audio_codec".to_string(), audio_codec);
        }

        if let Some(value) = captures.name("release_group") {
            let release_group = value.as_str().to_string();
            tokenizer.tokens.insert("release_group".to_string(), release_group);
        }

        if let Some(value) = captures.name("country") {
            tokenizer.tokens.insert("country".to_string(), value.as_str().to_string());
        }

        if let Some(value) = captures.name("subtitle_en") {
            tokenizer.tokens.insert("subtitle".to_string(), value.as_str().to_string());
        }

        if let Some(value) = captures.name("subtitle_cn") {
            tokenizer.tokens.insert("subtitle".to_string(), value.as_str().to_string());
        }

        if let Some(value) = captures.name("special") {
            tokenizer.tokens.insert("special".to_string(), value.as_str().to_string());
        }

        if let Some(value) = captures.name("format") {
            tokenizer.tokens.insert("format".to_string(), value.as_str().to_string());
        }

        // tracing::info!("tokenizer = {:?}",tokenizer);

        return Some(tokenizer);
    }

    return None;
}

fn input_before(rule: &Rule, input: &str) -> String {
    let mut new_input = input.to_string();

    if let Some(replaces) = &RULES.before_replaces {
        replaces.iter().for_each(|replace| {
            new_input = new_input.replace(&replace.src, &replace.target);
        })
    }

    if let Some(replaces) = &rule.replaces {
        replaces.iter().for_each(|replace| {
            new_input = new_input.replace(&replace.src, &replace.target);
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

#[cfg(test)]
mod tests {
    use regex::Regex;

    use crate::soda::meta::strong_match_token::*;

    #[test]
    fn test_regex_item_episode_title_en() {
        let regex = Regex::new(regex_item_episode_title_en()).unwrap();
        assert_eq!("1+1", regex.captures("1+1").unwrap().name("episode_title_en").unwrap().as_str());
        assert_eq!("2+2", regex.captures("2+2").unwrap().name("episode_title_en").unwrap().as_str());
        assert_eq!("Tape.Side.A", regex.captures("Tape.Side.A").unwrap().name("episode_title_en").unwrap().as_str());
        assert_eq!("Tape.1.Side.A", regex.captures("Tape.1.Side.A").unwrap().name("episode_title_en").unwrap().as_str());
        assert_eq!("Tape.Side.A.1", regex.captures("Tape.Side.A.1").unwrap().name("episode_title_en").unwrap().as_str());
    }

    #[test]
    fn test_regex() {
        Regex::new(regex_item_country()).unwrap();
        Regex::new(regex_item_title_cn()).unwrap();
        Regex::new(regex_item_title_en()).unwrap();
        Regex::new(regex_item_title_number_en()).unwrap();
        Regex::new(regex_item_year()).unwrap();
        Regex::new(regex_item_season_episode()).unwrap();
        Regex::new(regex_item_season_episode_split()).unwrap();
        Regex::new(regex_item_season_episode_episode()).unwrap();
        Regex::new(regex_item_season()).unwrap();
        Regex::new(regex_item_episode()).unwrap();
        Regex::new(regex_item_episode_title_en()).unwrap();
        Regex::new(regex_item_resolution()).unwrap();
        Regex::new(regex_item_version()).unwrap();
        Regex::new(&regex_item_source()).unwrap();
        Regex::new(&regex_item_company()).unwrap();
        Regex::new(&regex_item_video_codec()).unwrap();
        Regex::new(&regex_item_color()).unwrap();
        Regex::new(&regex_item_audio_codec()).unwrap();
        Regex::new(&regex_item_release_group()).unwrap();
        Regex::new(&regex_item_format()).unwrap();
        Regex::new(&regex_item_mix_numbers_letters()).unwrap();
    }

    #[test]
    fn test_parse4() {
        // $title_cn$.$title_en$.$year$.$season$.$resolution$.$source$.$video_codec$.$audio_codec$-$release_group$
        let input = "凡人修仙传.The.Mortal.Ascention.2020.S01.2160p.WEB-DL.H.264.AAC-OurTV";
        let regex = Regex::new(r"\((?P<year_month_day>\d{4}\.\d{2}\.\d{2})\)(?P<title_number_en>(([\\(\\)&'-. 0-9A-Za-z])+))-\[(?P<resolution>(240[Ppi]|360[Ppi]|720[Ppi]|1080[Ppi]|1440[Ppi]|2160[Ppi]|4320[Ppi]|4[Kk]))\]\[(?P<country>(Ger|GER|USA)).(?P<source>(AVC.REMUX|Blu-ray Remux|Blu-ray.REMUX|Blu-ray.Remux|BluRay.REMUX|BluRay.Remux|Blu-Ray|BluRay|Bluray|BDRIP|BDRip|BDrip|WEBRip|WEBrip|WEB-DL|WEB|Blu-?[Rr]ay|DVDRip|HDTVRip|BRRip|HDRip|CAM|HDTV|BD[.-]?Remux|REMUX|Remux))\].(?P<format>(mp4|mkv|ts|iso|rmvb|avi|mov|mepg|mpg|wmv|3gp|asf|m4v|flv|m2ts))").unwrap();
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
