use extendr_api::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct RDecoder {
    pub decoder: tokenizers::DecoderWrapper
}

#[extendr]
impl RDecoder {}

impl tokenizers::Decoder for RDecoder {
    fn decode(&self, tokens: Vec<String>) -> tokenizers::Result<String> {
        self.decoder.decode(tokens)
    }
}

extendr_module! {
    mod decoders;
    impl RDecoder;
}