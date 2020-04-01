extern crate tokenizers as tk;

use tk::tokenizer::PaddingDirection;

use crate::container::Container;

pub struct Encoding {
    pub encoding: Container<tk::tokenizer::Encoding>,
}

#[no_mangle]
pub extern "C" fn get_encoding_length(en: *const Encoding) -> usize {
    let ids = unsafe {
        (*en)
            .encoding
            .execute(|encoding| encoding.unwrap().get_ids().to_vec())
    };

    ids.len()
}
