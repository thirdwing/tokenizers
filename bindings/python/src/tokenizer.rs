extern crate tokenizers as tk;

use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::types::*;
use pyo3::PyObjectProtocol;
use std::collections::HashMap;

use super::decoders::Decoder;
use super::encoding::Encoding;
use super::error::{PyError, ToPyResult};
use super::models::Model;
use super::normalizers::Normalizer;
use super::pre_tokenizers::PreTokenizer;
use super::processors::PostProcessor;
use super::trainers::Trainer;
use super::utils::Container;

use tk::tokenizer::{
    PaddingDirection, PaddingParams, PaddingStrategy, TruncationParams, TruncationStrategy,
};

#[pyclass(dict)]
pub struct AddedToken {
    pub token: tk::tokenizer::AddedToken,
}
#[pymethods]
impl AddedToken {
    #[new]
    #[args(kwargs = "**")]
    fn new(obj: &PyRawObject, content: &str, kwargs: Option<&PyDict>) -> PyResult<()> {
        let mut token = tk::tokenizer::AddedToken::from(content.to_owned());

        if let Some(kwargs) = kwargs {
            for (key, value) in kwargs {
                let key: &str = key.extract()?;
                match key {
                    "single_word" => token = token.single_word(value.extract()?),
                    "lstrip" => token = token.lstrip(value.extract()?),
                    "rstrip" => token = token.rstrip(value.extract()?),
                    _ => println!("Ignored unknown kwarg option {}", key),
                }
            }
        }

        obj.init({ AddedToken { token } });
        Ok(())
    }
}
#[pyproto]
impl PyObjectProtocol for AddedToken {
    fn __str__(&'p self) -> PyResult<&'p str> {
        Ok(&self.token.content)
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "AddedToken(\"{}\", rstrip={}, lstrip={}, single_word={})",
            self.token.content, self.token.rstrip, self.token.lstrip, self.token.single_word
        ))
    }
}

#[pyclass(dict)]
pub struct Tokenizer {
    tokenizer: tk::tokenizer::Tokenizer,
}

#[pymethods]
impl Tokenizer {
    #[new]
    fn new(obj: &PyRawObject, model: &mut Model) -> PyResult<()> {
        if let Some(model) = model.model.to_pointer() {
            let tokenizer = tk::tokenizer::Tokenizer::new(model);
            obj.init({ Tokenizer { tokenizer } });
            Ok(())
        } else {
            Err(exceptions::Exception::py_err(
                "The Model is already being used in another Tokenizer",
            ))
        }
    }

    fn num_special_tokens_to_add(&self, is_pair: bool) -> PyResult<usize> {
        Ok(self
            .tokenizer
            .get_post_processor()
            .map_or(0, |p| p.as_ref().added_tokens(is_pair)))
    }

    #[args(with_added_tokens = true)]
    fn get_vocab(&self, with_added_tokens: bool) -> PyResult<HashMap<String, u32>> {
        Ok(self.tokenizer.get_vocab(with_added_tokens))
    }

    #[args(with_added_tokens = true)]
    fn get_vocab_size(&self, with_added_tokens: bool) -> PyResult<usize> {
        Ok(self.tokenizer.get_vocab_size(with_added_tokens))
    }

    #[args(kwargs = "**")]
    fn enable_truncation(&mut self, max_length: usize, kwargs: Option<&PyDict>) -> PyResult<()> {
        let mut stride = 0;
        let mut strategy = TruncationStrategy::LongestFirst;

        if let Some(kwargs) = kwargs {
            for (key, value) in kwargs {
                let key: &str = key.extract()?;
                match key {
                    "stride" => stride = value.extract()?,
                    "strategy" => {
                        let value: &str = value.extract()?;
                        strategy = match value {
                            "longest_first" => Ok(TruncationStrategy::LongestFirst),
                            "only_first" => Ok(TruncationStrategy::OnlyFirst),
                            "only_second" => Ok(TruncationStrategy::OnlySecond),
                            _ => Err(PyError(format!(
                                "Unknown `strategy`: `{}`. Use \
                                 one of `longest_first`, `only_first`, or `only_second`",
                                value
                            ))
                            .into_pyerr()),
                        }?
                    }
                    _ => println!("Ignored unknown kwarg option {}", key),
                }
            }
        }

        self.tokenizer.with_truncation(Some(TruncationParams {
            max_length,
            stride,
            strategy,
        }));

        Ok(())
    }

    fn no_truncation(&mut self) {
        self.tokenizer.with_truncation(None);
    }

    #[args(kwargs = "**")]
    fn enable_padding(&mut self, kwargs: Option<&PyDict>) -> PyResult<()> {
        let mut direction = PaddingDirection::Right;
        let mut pad_id: u32 = 0;
        let mut pad_type_id: u32 = 0;
        let mut pad_token = String::from("[PAD]");
        let mut max_length: Option<usize> = None;

        if let Some(kwargs) = kwargs {
            for (key, value) in kwargs {
                let key: &str = key.extract()?;
                match key {
                    "direction" => {
                        let value: &str = value.extract()?;
                        direction = match value {
                            "left" => Ok(PaddingDirection::Left),
                            "right" => Ok(PaddingDirection::Right),
                            other => Err(PyError(format!(
                                "Unknown `direction`: `{}`. Use \
                                 one of `left` or `right`",
                                other
                            ))
                            .into_pyerr()),
                        }?;
                    }
                    "pad_id" => pad_id = value.extract()?,
                    "pad_type_id" => pad_type_id = value.extract()?,
                    "pad_token" => pad_token = value.extract()?,
                    "max_length" => max_length = value.extract()?,
                    _ => println!("Ignored unknown kwarg option {}", key),
                }
            }
        }

        let strategy = if let Some(max_length) = max_length {
            PaddingStrategy::Fixed(max_length)
        } else {
            PaddingStrategy::BatchLongest
        };

        self.tokenizer.with_padding(Some(PaddingParams {
            strategy,
            direction,
            pad_id,
            pad_type_id,
            pad_token: pad_token.to_owned(),
        }));

        Ok(())
    }

    fn no_padding(&mut self) {
        self.tokenizer.with_padding(None);
    }

    fn normalize(&self, sentence: &str) -> PyResult<String> {
        ToPyResult(
            self.tokenizer
                .normalize(sentence)
                .map(|s| s.get().to_owned()),
        )
        .into()
    }

    #[args(add_special_tokens = true)]
    fn encode(
        &self,
        sentence: &str,
        pair: Option<&str>,
        add_special_tokens: bool,
    ) -> PyResult<Encoding> {
        ToPyResult(
            self.tokenizer
                .encode(
                    if let Some(pair) = pair {
                        tk::tokenizer::EncodeInput::Dual(sentence.to_owned(), pair.to_owned())
                    } else {
                        tk::tokenizer::EncodeInput::Single(sentence.to_owned())
                    },
                    add_special_tokens,
                )
                .map(Encoding::new),
        )
        .into()
    }

    #[args(add_special_tokens = true)]
    fn encode_batch(
        &self,
        sentences: &PyList,
        add_special_tokens: bool,
    ) -> PyResult<Vec<Encoding>> {
        let inputs = sentences
            .into_iter()
            .map(|item| {
                if let Ok(s1) = item.extract::<String>() {
                    Ok(tk::tokenizer::EncodeInput::Single(s1))
                } else if let Ok((s1, s2)) = item.extract::<(String, String)>() {
                    Ok(tk::tokenizer::EncodeInput::Dual(s1, s2))
                } else {
                    Err(exceptions::Exception::py_err(
                        "Input must be a list[str] or list[(str, str)]",
                    ))
                }
            })
            .collect::<PyResult<Vec<_>>>()?;

        ToPyResult(
            self.tokenizer
                .encode_batch(inputs, add_special_tokens)
                .map(|encodings| encodings.into_iter().map(Encoding::new).collect()),
        )
        .into()
    }

    fn decode(&self, ids: Vec<u32>, skip_special_tokens: Option<bool>) -> PyResult<String> {
        ToPyResult(
            self.tokenizer
                .decode(ids, skip_special_tokens.unwrap_or(true)),
        )
        .into()
    }

    fn decode_batch(
        &self,
        sentences: Vec<Vec<u32>>,
        skip_special_tokens: Option<bool>,
    ) -> PyResult<Vec<String>> {
        ToPyResult(
            self.tokenizer
                .decode_batch(sentences, skip_special_tokens.unwrap_or(true)),
        )
        .into()
    }

    fn token_to_id(&self, token: &str) -> Option<u32> {
        self.tokenizer.token_to_id(token)
    }

    fn id_to_token(&self, id: u32) -> Option<String> {
        self.tokenizer.id_to_token(id)
    }

    fn add_tokens(&mut self, tokens: &PyList) -> PyResult<usize> {
        let tokens = tokens
            .into_iter()
            .map(|token| {
                if let Ok(content) = token.extract::<String>() {
                    Ok(tk::tokenizer::AddedToken {
                        content,
                        ..Default::default()
                    })
                } else if let Ok(token) = token.cast_as::<AddedToken>() {
                    Ok(token.token.clone())
                } else {
                    Err(exceptions::Exception::py_err(
                        "Input must be a List[Union[str, AddedToken]]",
                    ))
                }
            })
            .collect::<PyResult<Vec<_>>>()?;

        Ok(self.tokenizer.add_tokens(&tokens))
    }

    fn add_special_tokens(&mut self, tokens: &PyList) -> PyResult<usize> {
        let tokens = tokens
            .into_iter()
            .map(|token| {
                if let Ok(content) = token.extract::<String>() {
                    Ok(tk::tokenizer::AddedToken {
                        content,
                        ..Default::default()
                    })
                } else if let Ok(token) = token.cast_as::<AddedToken>() {
                    Ok(token.token.clone())
                } else {
                    Err(exceptions::Exception::py_err(
                        "Input must be a List[Union[str, AddedToken]]",
                    ))
                }
            })
            .collect::<PyResult<Vec<_>>>()?;

        Ok(self.tokenizer.add_special_tokens(&tokens))
    }

    fn train(&mut self, trainer: &Trainer, files: Vec<String>) -> PyResult<()> {
        trainer.trainer.execute(|trainer| {
            if let Err(e) = self.tokenizer.train(trainer, files) {
                Err(exceptions::Exception::py_err(format!("{}", e)))
            } else {
                Ok(())
            }
        })
    }

    #[args(pair = "None", add_special_tokens = true)]
    fn post_process(
        &self,
        encoding: &Encoding,
        pair: Option<&Encoding>,
        add_special_tokens: bool,
    ) -> PyResult<Encoding> {
        ToPyResult(
            self.tokenizer
                .post_process(
                    encoding.encoding.clone(),
                    pair.map(|p| p.encoding.clone()),
                    add_special_tokens,
                )
                .map(Encoding::new),
        )
        .into()
    }

    #[getter]
    fn get_model(&self) -> PyResult<Model> {
        Ok(Model {
            model: Container::from_ref(self.tokenizer.get_model()),
        })
    }

    #[setter]
    fn set_model(&mut self, model: &mut Model) -> PyResult<()> {
        if let Some(model) = model.model.to_pointer() {
            self.tokenizer.with_model(model);
            Ok(())
        } else {
            Err(exceptions::Exception::py_err(
                "The Model is already being used in another Tokenizer",
            ))
        }
    }

    #[getter]
    fn get_normalizer(&self) -> PyResult<Option<Normalizer>> {
        Ok(self
            .tokenizer
            .get_normalizer()
            .map(|normalizer| Normalizer {
                normalizer: Container::from_ref(normalizer),
            }))
    }

    #[setter]
    fn set_normalizer(&mut self, normalizer: &mut Normalizer) -> PyResult<()> {
        if let Some(normalizer) = normalizer.normalizer.to_pointer() {
            self.tokenizer.with_normalizer(normalizer);
            Ok(())
        } else {
            Err(exceptions::Exception::py_err(
                "The Normalizer is already being used in another Tokenizer",
            ))
        }
    }

    #[getter]
    fn get_pre_tokenizer(&self) -> PyResult<Option<PreTokenizer>> {
        Ok(self
            .tokenizer
            .get_pre_tokenizer()
            .map(|pretok| PreTokenizer {
                pretok: Container::from_ref(pretok),
            }))
    }

    #[setter]
    fn set_pre_tokenizer(&mut self, pretok: &mut PreTokenizer) -> PyResult<()> {
        if let Some(pretok) = pretok.pretok.to_pointer() {
            self.tokenizer.with_pre_tokenizer(pretok);
            Ok(())
        } else {
            Err(exceptions::Exception::py_err(
                "The PreTokenizer is already being used in another Tokenizer",
            ))
        }
    }

    #[getter]
    fn get_post_processor(&self) -> PyResult<Option<PostProcessor>> {
        Ok(self
            .tokenizer
            .get_post_processor()
            .map(|processor| PostProcessor {
                processor: Container::from_ref(processor),
            }))
    }

    #[setter]
    fn set_post_processor(&mut self, processor: &mut PostProcessor) -> PyResult<()> {
        if let Some(processor) = processor.processor.to_pointer() {
            self.tokenizer.with_post_processor(processor);
            Ok(())
        } else {
            Err(exceptions::Exception::py_err(
                "The Processor is already being used in another Tokenizer",
            ))
        }
    }

    #[getter]
    fn get_decoder(&self) -> PyResult<Option<Decoder>> {
        Ok(self.tokenizer.get_decoder().map(|decoder| Decoder {
            decoder: Container::from_ref(decoder),
        }))
    }

    #[setter]
    fn set_decoder(&mut self, decoder: &mut Decoder) -> PyResult<()> {
        if let Some(decoder) = decoder.decoder.to_pointer() {
            self.tokenizer.with_decoder(decoder);
            Ok(())
        } else {
            Err(exceptions::Exception::py_err(
                "The Decoder is already being used in another Tokenizer",
            ))
        }
    }
}
