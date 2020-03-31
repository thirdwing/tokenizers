extern crate tokenizers as tk;

use crate::container::Container;

#[repr(C)]
pub struct Decoder {
    pub decoder: Container<dyn tk::tokenizer::Decoder + Sync>,
}

#[no_mangle]
pub extern "C" fn byte_level_decoder() -> *mut Decoder {
    let mut decoder = Decoder {
        decoder: Container::Empty,
    };

    decoder
        .decoder
        .to_owned(Box::new(tk::decoders::byte_level::ByteLevel::default()));

    Box::into_raw(Box::new(decoder))
}

#[no_mangle]
pub extern "C" fn wordpiece_decoder(prefix: *mut u8, length: usize, cleanup: bool) -> *mut Decoder {
    let mut decoder = Decoder {
        decoder: Container::Empty,
    };

    let s = unsafe { String::from_raw_parts(prefix, length, length) };

    decoder
        .decoder
        .to_owned(Box::new(tk::decoders::wordpiece::WordPiece::new(
            s, cleanup,
        )));

    Box::into_raw(Box::new(decoder))
}

#[no_mangle]
pub extern "C" fn metaspace_decoder(replacement: char, add_prefix_space: bool) -> *mut Decoder {
    let mut decoder = Decoder {
        decoder: Container::Empty,
    };

    decoder
        .decoder
        .to_owned(Box::new(tk::decoders::metaspace::Metaspace::new(
            replacement,
            add_prefix_space,
        )));

    Box::into_raw(Box::new(decoder))
}

#[no_mangle]
pub extern "C" fn bpe_decoder(suffix: *mut u8, length: usize) -> *mut Decoder {
    let mut decoder = Decoder {
        decoder: Container::Empty,
    };

    let s = unsafe { String::from_raw_parts(suffix, length, length) };

    decoder
        .decoder
        .to_owned(Box::new(tk::decoders::bpe::BPEDecoder::new(s)));

    Box::into_raw(Box::new(decoder))
}
