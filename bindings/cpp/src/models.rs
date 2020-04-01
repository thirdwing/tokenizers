extern crate tokenizers as tk;

use crate::container::Container;

use std::path::Path;

pub struct Model {
    pub model: Container<dyn tk::tokenizer::Model + Sync>,
}
