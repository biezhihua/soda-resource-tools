use once_cell::sync::Lazy;
use regex::Regex;

pub(crate) static REGEX_MT_EXT: Lazy<Regex> = Lazy::new(|| Regex::new(r"\.(mp4|mkv|rmvb|avi|mov|mepg|mpg|wmv|3gp|asf|m4v|flv)$").unwrap());

#[cfg(test)]
mod global_tests {
    use super::REGEX_MT_EXT;

    #[test]
    fn test_regex_mt_ext() {
        assert_eq!(true, REGEX_MT_EXT.is_match("test.mp4"));
        assert_eq!(false, REGEX_MT_EXT.is_match("test.mp4.jpg"));
    }
}
