use std::collections::HashMap;

use crate::soda::{
    entity::{MetaContext, Token},
    global::REGEX_MT_EXT,
};

use super::strong_match_token;

pub(crate) fn tokens(meta_context: &mut MetaContext) -> Option<Token> {
    let mut before_token = Token::new();
    meta_context.input = meta_context.cur_file_name.to_string();

    tracing::debug!("meta_context.input 1 = {:?}", meta_context.input);

    meta_context.input = strong_match_token::input_before_replaces(&meta_context.input);
    meta_context.input = strong_match_token::input_before_process(&mut before_token, &meta_context.input);

    tracing::debug!("meta_context.input 2 = {:?}", meta_context.input);

    if REGEX_MT_EXT.is_match(&meta_context.input) {
        strong_match_token::build_mt_file_tokens(meta_context, &mut before_token)
    } else {
        strong_match_token::build_mt_dir_tokens(&meta_context.input, &mut before_token)
    }
}
