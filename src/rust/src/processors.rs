use extendr_api::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct RPostProcessor {
    post_processor: tokenizers::PostProcessorWrapper
}

#[extendr]
impl RPostProcessor {}

impl tokenizers::PostProcessor for RPostProcessor {
    fn added_tokens(&self, is_pair: bool) -> usize {
        self.post_processor.added_tokens(is_pair)
    }

    fn process(
        &self,
        encoding: tokenizers::Encoding,
        pair_encoding: Option<tokenizers::Encoding>,
        add_special_tokens: bool,
    ) -> tokenizers::Result<tokenizers::Encoding> {
        self.post_processor
            .process(encoding, pair_encoding, add_special_tokens)
    }
}

extendr_module! {
    mod processors;
    impl RPostProcessor;
}