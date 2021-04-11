use crate::trainers::*;
use extendr_api::prelude::*;
use std::collections::HashMap;
use tokenizers::Model; 
use serde::{Deserialize, Serialize};

// Models ---------------------

#[extendr]
#[derive(Clone, Serialize, Deserialize)]
pub struct RModel {
    pub model: tokenizers::ModelWrapper
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

    fn save (&self, folder: &str, prefix: &str) -> Vec<String> {
        let saved = self.model.save(std::path::Path::new(folder), Some(prefix));
        saved
            .unwrap()
            .into_iter()
            .map(|path| path.to_string_lossy().into_owned())
            .collect()
    }
    
}

#[extendr]
pub struct RModelsBpe {}

#[extendr]
impl RModelsBpe {
    fn new (vocab: RVocab, merges: RMerges, cache_capacity: Option<i32>, dropout: Option<f64>, unk_token: Option<String>, continuing_subword_prefix: Option<String>, end_of_word_suffix: Option<String>, fuse_unk: Option<bool>) -> RModel {
        let mut builder = tokenizers::models::bpe::BPE::builder();

        match (vocab, merges) {
            (RVocab::Vocab(vocab), RMerges::Merges(merges)) => {
                builder = builder.vocab_and_merges(vocab, merges);
            }
            _ => {}
        }

        match cache_capacity {
            Some(v) => builder = builder.cache_capacity(v as usize),
            None => {}
        }

        match dropout {
            Some(v) => builder = builder.dropout(v as f32),
            None => {}
        }

        match unk_token {
            Some(v) => builder = builder.unk_token(v),
            None => {}
        }

        match continuing_subword_prefix {
            Some(v) => builder = builder.continuing_subword_prefix(v),
            None => {}
        }

        match end_of_word_suffix {
            Some(v) => builder = builder.end_of_word_suffix(v),
            None => {}
        }

        match fuse_unk {
            Some(v) => builder = builder.fuse_unk(v),
            None => {}
        }

        builder.build().unwrap().into()
    }

    fn read_file (vocab: &str, merges: &str) -> RVocabAndMerges {
        let out = tokenizers::models::bpe::BPE::read_file(vocab, merges).unwrap();
        RVocabAndMerges{vocab: RVocab::Vocab(out.0), merges: RMerges::Merges(out.1)}
    }

    fn from_file (vocab: &str, merges: &str, cache_capacity: Option<i32>, dropout: Option<f64>, unk_token: Option<String>, continuing_subword_prefix: Option<String>, end_of_word_suffix: Option<String>, fuse_unk: Option<bool>) -> RModel {
        let vm = RModelsBpe::read_file(vocab, merges);
        RModelsBpe::new(vm.vocab, vm.merges, cache_capacity, dropout, unk_token, continuing_subword_prefix, end_of_word_suffix, fuse_unk)
    }
}

enum RVocab {
    Vocab(tokenizers::models::bpe::Vocab),
    None
}

impl<'a> FromRobj<'a> for RVocab {
    fn from_robj (robj: &'a Robj) -> std::result::Result<Self, &'static str> {
        if let Some(iter) = robj.as_list() {
            let hash_map = iter
                .iter()
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
        if let Some(iter) = robj.as_list() {
            let vector = iter
                .iter()
                .map(|(n, k)| {
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

struct RVocabAndMerges {
    pub vocab: RVocab,
    pub merges: RMerges
}

#[extendr]
impl RVocabAndMerges {}

enum RMerges {
    Merges(tokenizers::models::bpe::Merges),
    None
}

extendr_module! {
    mod models;
    impl RModel;
    impl RModelsBpe;
}