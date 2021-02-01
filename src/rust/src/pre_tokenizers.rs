use extendr_api::*;

pub struct RPreTokenizer {
    pre_tokenizer: tokenizers::PreTokenizerWrapper
}

#[extendr]
impl RPreTokenizer {}

impl tokenizers::PreTokenizer for RPreTokenizer {
    fn pre_tokenize(&self, normalized: &mut tokenizers::PreTokenizedString) -> tokenizers::Result<()> {
        self.pre_tokenizer.pre_tokenize(normalized)
    }
}

extendr_module! {
    mod pre_tokenizers;
    impl RPreTokenizer;
}