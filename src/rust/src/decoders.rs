use extendr_api::*;

pub struct RDecoder {
    pub decoder: tokenizers::DecoderWrapper
}

#[extendr]
impl RDecoder {}

extendr_module! {
    mod decoders;
    impl RDecoder;
}