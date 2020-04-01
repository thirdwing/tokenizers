extern crate tokenizers as tk;

use crate::container::Container;

use std::path::Path;

pub struct Model {
    pub model: Container<dyn tk::tokenizer::Model + Sync>,
}

#[no_mangle]
pub extern "C" fn save_model(
    model: *const Model,
    folder: *mut u8,
    folder_len: usize,
    name: *mut u8,
    name_len: usize,
) -> usize {
    let f = unsafe { String::from_raw_parts(folder, folder_len, folder_len) };

    let n = unsafe { String::from_raw_parts(name, name_len, name_len) };

    let result = unsafe {
        (*model)
            .model
            .execute(|model| model.unwrap().save(Path::new(&f), Some(&n)))
    };
    match result {
        Ok(r) => 0,
        Err(e) => 1,
    }
}

#[no_mangle]
pub extern "C" fn empty_bpe() -> *mut Model {
    let m = Model {
        model: Container::Owned(Box::new(tk::models::bpe::BPE::default())),
    };
    Box::into_raw(Box::new(m))
}

#[no_mangle]
pub extern "C" fn bpe_from_files(
    vocab: *mut u8,
    vocab_len: usize,
    merges: *mut u8,
    merges_len: usize,
    cacheCapacity: usize,
    dropout: f32,
    unkToken: *mut u8,
    unkToken_len: usize,
    continuingSubwordPrefix: *mut u8,
    continuingSubwordPrefix_len: usize,
    endOfWordSuffix: *mut u8,
    endOfWordSuffix_len: usize,
) -> *mut Model {
    let v = unsafe { String::from_raw_parts(vocab, vocab_len, vocab_len) };
    let m = unsafe { String::from_raw_parts(merges, merges_len, merges_len) };
    let mut builder = tk::models::bpe::BPE::from_files(&v, &m);

    builder = builder.cache_capacity(cacheCapacity);
    builder = builder.dropout(dropout);

    let unk = unsafe { String::from_raw_parts(unkToken, unkToken_len, unkToken_len) };
    builder = builder.unk_token(unk);

    let continuing_subword_prefix = unsafe {
        String::from_raw_parts(
            continuingSubwordPrefix,
            continuingSubwordPrefix_len,
            continuingSubwordPrefix_len,
        )
    };
    builder = builder.continuing_subword_prefix(continuing_subword_prefix);

    let end_of_word_suffix = unsafe {
        String::from_raw_parts(endOfWordSuffix, endOfWordSuffix_len, endOfWordSuffix_len)
    };
    builder = builder.end_of_word_suffix(end_of_word_suffix);

    match builder.build() {
        Err(e) => {
            print!("Error while initializing BPE: {}", e);
            let model = Model {
                model: Container::Empty,
            };
            Box::into_raw(Box::new(model))
        }
        Ok(bpe) => {
            let model = Model {
                model: Container::Owned(Box::new(bpe)),
            };
            Box::into_raw(Box::new(model))
        }
    }
}
