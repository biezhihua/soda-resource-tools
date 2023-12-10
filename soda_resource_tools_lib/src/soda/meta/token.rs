use std::collections::HashMap;

use crate::soda::global::REGEX_MT_EXT;

use super::strong_match_token::{build_mt_dir_tokens, build_mt_file_tokens};

pub(crate) enum Token {
    FastToken(String),
    StrongMatchToken(String),
}

impl Token {
    pub(crate) fn tokens(&self) -> Option<Tokens> {
        match self {
            Token::FastToken(title) => None,
            Token::StrongMatchToken(title) => {
                if REGEX_MT_EXT.is_match(title) {
                    build_mt_file_tokens(title)
                } else {
                    build_mt_dir_tokens(title)
                }
            }
        }
    }
}

pub(crate) fn build_tokens(title: &str) -> Option<Tokens> {
    if let Some(ret) = Token::FastToken(title.to_string()).tokens() {
        return Some(ret);
    }
    if let Some(ret) = Token::StrongMatchToken(title.to_string()).tokens() {
        return Some(ret);
    }
    return None;
}

#[derive(Debug, Clone)]
pub(crate) struct Tokens {
    pub(crate) tokens: HashMap<String, String>,
}
