use extendr_api::*;
use extendr_api::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct RPreTokenizer {
    pub pre_tokenizer: tokenizers::PreTokenizerWrapper
}

#[extendr]
impl RPreTokenizer {}

impl tokenizers::PreTokenizer for RPreTokenizer {
    fn pre_tokenize(&self, normalized: &mut tokenizers::PreTokenizedString) -> tokenizers::Result<()> {
        self.pre_tokenizer.pre_tokenize(normalized)
    }
}

pub struct RWhitespace {}

#[extendr]
impl RWhitespace {
    fn new () -> RPreTokenizer {
        RPreTokenizer{
            pre_tokenizer: tokenizers::pre_tokenizers::PreTokenizerWrapper::Whitespace(
                tokenizers::pre_tokenizers::whitespace::Whitespace::default().into()
            )
        }
    }
}

extendr_module! {
    mod pre_tokenizers;
    impl RPreTokenizer;
    impl RWhitespace;
}

