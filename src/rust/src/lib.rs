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

// Trainers -------------------

pub struct RTrainer {
    pub trainer: tokenizers::models::TrainerWrapper
}

impl tokenizers::Trainer for RTrainer {
    type Model = RModel;

    fn should_show_progress(&self) -> bool {
        self.trainer.should_show_progress()
    }

    fn train(&self, model: &mut RModel) -> tokenizers::Result<Vec<tokenizers::AddedToken>> {
        self.trainer
            .train(&mut model.model)
    }

    fn feed<I, S, F>(&mut self, iterator: I, process: F) -> tokenizers::Result<()>
    where
        I: Iterator<Item = S> + Send,
        S: AsRef<str> + Send,
        F: Fn(&str) -> tokenizers::Result<Vec<String>> + Sync,
    {
        self.trainer.feed(iterator, process)
    }
}

impl<I> From<I> for RTrainer
where
    I: Into<tokenizers::models::TrainerWrapper>,
{
    fn from(trainer: I) -> Self {
        RTrainer {
            trainer: trainer.into()
        }
    }
}

// Models ---------------------

#[extendr]
pub struct RModel {
    pub model: ModelWrapper
}

impl tokenizers::Model for RModel {

    type Trainer = RTrainer;

    fn tokenize(&self, tokens: &str) -> tokenizers::Result<Vec<tokenizers::Token>> {
        self.model.tokenize(tokens)
    }

    fn token_to_id(&self, token: &str) -> Option<u32> {
        self.model.token_to_id(token)
    }

    fn id_to_token(&self, id: u32) -> Option<String> {
        self.model.id_to_token(id)
    }

    fn get_vocab(&self) -> HashMap<String, u32> {
        self.model.get_vocab()
    }

    fn get_vocab_size(&self) -> usize {
        self.model.get_vocab_size()
    }

    fn save(&self, folder: &std::path::Path, name: Option<&str>) -> tokenizers::Result<Vec<std::path::PathBuf>> {
        self.model.save(folder, name)
    }

    fn get_trainer(&self) -> Self::Trainer {
        self.model.get_trainer().into()
    }
}

impl<I> From<I> for RModel
where
    I: Into<tokenizers::ModelWrapper>,
{
    fn from(model: I) -> Self {
        Self {
            model: model.into(),
        }
    }
}

#[extendr]
impl RModel {

}

#[extendr]
pub struct RModelsBpe {}

#[extendr]
impl RModelsBpe {
    fn new () -> RModel {
        let builder = tokenizers::models::bpe::BPE::builder();
        builder.build().unwrap().into()
    }
}

// Tokenizers ----------------

struct RTokenizerImpl {
    pub tokenizer: tokenizers::TokenizerImpl<
    tokenizers::ModelWrapper,
    NormalizerWrapper,
    PreTokenizerWrapper,
    PostProcessorWrapper,
    DecoderWrapper
>
}

#[extendr]
impl RTokenizerImpl {
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
    //fn build (& self) {
    //    RTokenizerImpl{tokenizer: self.tokenizer.build().unwrap()};
    //}
}


extendr_module! {
    mod helloextendr;
    fn hello;
    fn token;
    // Tokenizers
    impl RTokenizerBuilder;
    // Models ------
    impl RModel;
    impl RModelsBpe;
    //impl RModelsBpeBuilder;
}




