extern crate tokenizers as tk;

use crate::container::Container;

#[repr(C)]
pub struct Normalizer {
    pub normalizer: Container<dyn tk::tokenizer::Normalizer + Sync>,
}

#[no_mangle]
pub extern "C" fn bert_normalizer(
    clean_text: bool,
    handle_chinese_chars: bool,
    strip_accents: bool,
    lowercase: bool,
) -> *mut Normalizer {
    let mut normalizer = Normalizer {
        normalizer: Container::Empty,
    };

    normalizer
        .normalizer
        .to_owned(Box::new(tk::normalizers::bert::BertNormalizer::new(
            clean_text,
            handle_chinese_chars,
            strip_accents,
            lowercase,
        )));

    Box::into_raw(Box::new(normalizer))
}

#[no_mangle]
pub extern "C" fn nfd_normalizer() -> *mut Normalizer {
    let mut normalizer = Normalizer {
        normalizer: Container::Empty,
    };

    normalizer
        .normalizer
        .to_owned(Box::new(tk::normalizers::unicode::NFD));

    Box::into_raw(Box::new(normalizer))
}

#[no_mangle]
pub extern "C" fn nfkd_normalizer() -> *mut Normalizer {
    let mut normalizer = Normalizer {
        normalizer: Container::Empty,
    };

    normalizer
        .normalizer
        .to_owned(Box::new(tk::normalizers::unicode::NFKD));

    Box::into_raw(Box::new(normalizer))
}

#[no_mangle]
pub extern "C" fn nfc_normalizer() -> *mut Normalizer {
    let mut normalizer = Normalizer {
        normalizer: Container::Empty,
    };

    normalizer
        .normalizer
        .to_owned(Box::new(tk::normalizers::unicode::NFC));

    Box::into_raw(Box::new(normalizer))
}

#[no_mangle]
pub extern "C" fn nfkc_normalizer() -> *mut Normalizer {
    let mut normalizer = Normalizer {
        normalizer: Container::Empty,
    };

    normalizer
        .normalizer
        .to_owned(Box::new(tk::normalizers::unicode::NFKC));

    Box::into_raw(Box::new(normalizer))
}

#[no_mangle]
pub extern "C" fn strip_normalizer(left: bool, right: bool) -> *mut Normalizer {
    let mut normalizer = Normalizer {
        normalizer: Container::Empty,
    };

    normalizer
        .normalizer
        .to_owned(Box::new(tk::normalizers::strip::Strip::new(left, right)));

    Box::into_raw(Box::new(normalizer))
}

#[no_mangle]
pub extern "C" fn lowercase_normalizer() -> *mut Normalizer {
    let mut normalizer = Normalizer {
        normalizer: Container::Empty,
    };

    normalizer
        .normalizer
        .to_owned(Box::new(tk::normalizers::utils::Lowercase));

    Box::into_raw(Box::new(normalizer))
}
