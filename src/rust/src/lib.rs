use extendr_api::*;
use tokenizers::models::bpe::BPE;
use tokenizers::{DecoderWrapper, NormalizerWrapper, PostProcessorWrapper, PreTokenizerWrapper};
use tokenizers::{Model, TokenizerBuilder, ModelWrapper};

#[extendr]
fn hello() -> &'static str {
    return "hello";
}

#[extendr]
fn token() -> &'static str {
    
    let mut tokenizer = TokenizerBuilder::<
        BPE,
        NormalizerWrapper,
        PreTokenizerWrapper,
        PostProcessorWrapper,
        DecoderWrapper,
    >::default()
    .with_model(
        BPE::builder()
            .unk_token("[UNK]".to_string())
            .dropout(0.1)
            .build()
            .unwrap(),
    )
    .build()
    .unwrap();
    let mut trainer = tokenizer.get_model().get_trainer();
    tokenizer
        .train_from_files(&mut trainer, vec!["./small.txt".to_string()])
        .unwrap();

    "yeah done!"
}

// Models ---------------------

pub struct RModelWrapper {
    pub model: ModelWrapper
}

#[extendr]
impl RModelWrapper {
    fn new () -> Self {
        Self {model: ModelWrapper::BPE(BPE::default())}
    }
    fn set_bpe (&mut self, object: &RBPE) {
        self.model = ModelWrapper::BPE(object.bpe.clone());
    }
}

pub struct RBPE {
    pub bpe: tokenizers::models::bpe::BPE
}

#[extendr]
impl RBPE {
    fn new () -> Self {
        Self {bpe: tokenizers::models::bpe::BPE::default()}
    }
}

struct RTokenizerBuilder {
    pub tokenizer: TokenizerBuilder::<
    tokenizers::ModelWrapper,
    NormalizerWrapper,
    PreTokenizerWrapper,
    PostProcessorWrapper,
    DecoderWrapper
>
}

// Tokenizers ----------------

#[extendr]
impl RTokenizerBuilder {
    fn new () -> Self {
        Self {tokenizer: TokenizerBuilder::<
            tokenizers::ModelWrapper,
            NormalizerWrapper,
            PreTokenizerWrapper,
            PostProcessorWrapper,
            DecoderWrapper,
        >::default()}
    }
}


extendr_module! {
    mod helloextendr;
    fn hello;
    fn token;
    // Tokenizers
    impl RTokenizerBuilder;
    // Models ------
    impl RModelWrapper;
    impl RBPE;
}




