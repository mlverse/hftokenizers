use extendr_api::*;
use crate::models::*;

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

pub struct RBpeTrainer {}

#[extendr]
impl RBpeTrainer {

    fn new (vocab_size: Nullable<u64>, min_frequency: Nullable<u32>, show_progress: Nullable<bool>,
        special_tokens: Nullable<Vec<String>>, limit_alphabet: Nullable<u32>, 
        initial_alphabet: Nullable<Vec<String>>,
        continuing_subword_prefix: Nullable<String>,
        end_of_word_suffix: Nullable<String>) -> RTrainer {
        let mut builder = tokenizers::models::bpe::BpeTrainer::builder();

        match vocab_size {
            Nullable::NotNull(v) => {builder = builder.vocab_size(v as usize);},
            _ => {}
        }

        match min_frequency {
            Nullable::NotNull(v) => {builder = builder.min_frequency(v);},
            _ => {}
        }

        match show_progress {
            Nullable::NotNull(v) => {builder = builder.show_progress(v);},
            _ => {}
        }

        match special_tokens {
            Nullable::NotNull(v) => {
                builder = builder
                    .special_tokens(v
                        .iter()
                        .map(|tk| tokenizers::AddedToken::from(tk, true)).collect()
                    );
            },
            _ => {}
        }

        match limit_alphabet {
            Nullable::NotNull(v) => {
                builder = builder.limit_alphabet(v as usize);
            },
            _ => {}
        }

        match initial_alphabet {
            Nullable::NotNull(v) => {
                let mut hash_set = std::collections::HashSet::new();                
                
                for a in v.into_iter() {
                    match a.chars().next() {
                        Some(c) => {hash_set.insert(c);}
                        _ => {}
                    }
                }
                
                builder = builder.initial_alphabet(hash_set);
            },
            _ => {}
        }

        match continuing_subword_prefix {
            Nullable::NotNull(v) => {builder = builder.continuing_subword_prefix(v);}
            _ => {}
        }

        match end_of_word_suffix {
            Nullable::NotNull(v) => {builder = builder.end_of_word_suffix(v);},
            _ => {}
        }

        builder.build().into()
    }
}

extendr_module! {
    mod trainers;
    impl RBpeTrainer;
}

