extern crate tokenizers as tk;

use tk::tokenizer::{
    PaddingDirection, PaddingParams, PaddingStrategy, TruncationParams, TruncationStrategy,
};

#[repr(C)]
pub struct AddedToken {
    pub token: tk::tokenizer::AddedToken,
}

#[no_mangle]
pub extern "C" fn added_token_init(
    content: *mut u8,
    length: usize,
    single_word: bool,
    lstrip: bool,
    rstrip: bool,
) -> *mut AddedToken {
    let s = unsafe { String::from_raw_parts(content, length, length) };
    let mut token = tk::tokenizer::AddedToken::from(s);
    token = token.single_word(single_word);
    token = token.lstrip(lstrip);
    token = token.rstrip(rstrip);

    let added_token = AddedToken { token };
    Box::into_raw(Box::new(added_token))
}

/// Tokenizer
#[repr(C)]
pub struct Tokenizer {
    tokenizer: tk::tokenizer::Tokenizer,
}
