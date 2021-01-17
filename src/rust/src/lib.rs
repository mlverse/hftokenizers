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

#[extendr]
impl RTrainer {

}

// Models ---------------------

#[extendr]
#[derive(Clone)]
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
            model: model.into()
        }
    }
}

#[extendr]
impl RModel {
    fn get_trainer (&self) -> RTrainer {
        RTrainer {trainer: self.model.get_trainer()}
    }
}

// Parameters ----

enum RVocab {
    Vocab(tokenizers::models::bpe::Vocab),
    None
}

impl<'a> FromRobj<'a> for RVocab {
    fn from_robj (robj: &'a Robj) -> std::result::Result<Self, &'static str> {
        if let Some(iter) = robj.as_named_list_iter() {
            let hash_map = iter
                .map(|(k, v)| (k.to_string(), v.as_integer().unwrap() as u32))
                .collect::<HashMap<String, u32>>();
            std::result::Result::Ok(RVocab::Vocab(hash_map))
        } else if robj.is_null() {
            std::result::Result::Ok(RVocab::None)
        } else {
            Err("expected a list")
        }
    }
}

impl<'a> FromRobj<'a> for RMerges {
    fn from_robj (robj: &'a Robj) -> std::result::Result<Self, &'static str> {
        if let Some(iter) = robj.as_list_iter() {
            let vector = iter
                .map(|k| {
                    let v = k.as_str_iter().unwrap().collect::<Vec<_>>();
                    (String::from(v[0]), String::from(v[1]))
                })
                .collect::<Vec<(String, String)>>();
            std::result::Result::Ok(RMerges::Merges(vector))
        } else if robj.is_null() {
            std::result::Result::Ok(RMerges::None)
        } else {
            Err("expected a list")
        }
    }
}

enum RMerges {
    Merges(tokenizers::models::bpe::Merges),
    None
}

// Models --------

#[extendr]
pub struct RModelsBpe {}

#[extendr]
impl RModelsBpe {
    fn new (vocab: RVocab, merges: RMerges, dropout: Option<f64>, unk_token: Option<String>) -> RModel {
        let mut builder = tokenizers::models::bpe::BPE::builder();

        match (vocab, merges) {
            (RVocab::Vocab(vocab), RMerges::Merges(merges)) => {
                builder = builder.vocab_and_merges(vocab, merges);
            }
            _ => {}
        }

        match unk_token {
            Some(v) => builder = builder.unk_token(v),
            None => {}
        }

        match dropout {
            Some(v) => builder = builder.dropout(v as f32),
            None => {}
        }

        builder.build().unwrap().into()
    }
}

// Normalizers ---------------

struct RNormalizer {
    normalizer: tokenizers::NormalizerWrapper
}

#[extendr]
impl RNormalizer {}

impl tokenizers::Normalizer for RNormalizer {
    fn normalize(&self, normalized: &mut tokenizers::NormalizedString) -> tokenizers::Result<()> {
        self.normalizer.normalize(normalized)
    }
}

// Pre-tokenizers ---------------

struct RPreTokenizer {
    pre_tokenizer: tokenizers::PreTokenizerWrapper
}

#[extendr]
impl RPreTokenizer {}

impl tokenizers::PreTokenizer for RPreTokenizer {
    fn pre_tokenize(&self, normalized: &mut tokenizers::PreTokenizedString) -> tokenizers::Result<()> {
        self.pre_tokenizer.pre_tokenize(normalized)
    }
}

// Post processor wrapper --------

struct RPostProcessor {
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

// Decoders ----------------------

struct RDecoder {
    decoder: tokenizers::DecoderWrapper
}

#[extendr]
impl RDecoder {}

// Tokenizers ----------------

type Tokenizer = tokenizers::TokenizerImpl::<RModel, RNormalizer,RPreTokenizer,RPostProcessor,RDecoder>;

struct RTokenizer {
    pub tokenizer: Tokenizer
}

impl tokenizers::Decoder for RDecoder {
    fn decode(&self, tokens: Vec<String>) -> tokenizers::Result<String> {
        self.decoder.decode(tokens)
    }
}

#[extendr]
impl RTokenizer {
    fn from_model (model: &RModel) -> Self {
        RTokenizer {tokenizer: tokenizers::TokenizerImpl::new(model.clone())}
    }
}

extendr_module! {
    mod helloextendr;
    fn hello;
    fn token;
    // Tokenizers
    impl RTokenizer;
    // Models ------
    impl RModel;
    impl RModelsBpe;
    // Other stuff -----
    impl RNormalizer;
    impl RPreTokenizer;
    impl RPostProcessor;
    impl RDecoder;
}




