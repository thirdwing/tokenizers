extern crate tokenizers as tk;

use tk::tokenizer::{
    PaddingDirection, PaddingParams, PaddingStrategy, TruncationParams, TruncationStrategy,
};

#[repr(transparent)]
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
) -> AddedToken {
    let s = unsafe { String::from_raw_parts(content, length, length) };
    let mut token = tk::tokenizer::AddedToken::from(s);
    token = token.single_word(single_word);
    token = token.lstrip(lstrip);
    token = token.rstrip(rstrip);

    AddedToken { token }
}
