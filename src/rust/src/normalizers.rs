use extendr_api::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RNormalizer {
    normalizer: tokenizers::NormalizerWrapper
}

#[extendr]
impl RNormalizer {}

impl tokenizers::Normalizer for RNormalizer {
    fn normalize(&self, normalized: &mut tokenizers::NormalizedString) -> tokenizers::Result<()> {
        self.normalizer.normalize(normalized)
    }
}

extendr_module! {
    mod normalizers;
    impl RNormalizer;
}