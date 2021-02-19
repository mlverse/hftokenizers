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

/// @export
#[extendr]
impl RBpeTrainer {

    /// Trainer capable of training a BPE model
    /// 
    /// @param vocab_size (int, optional) – The size of the final vocabulary, including all tokens and alphabet.
    /// @param min_frequency (int, optional) – The minimum frequency a pair should have in order to be merged.
    /// @param show_progress (bool, optional) – Whether to show progress bars while training.
    /// @param special_tokens (List[Union[str, AddedToken]], optional) – A list of special tokens the model should know of.
    /// @param limit_alphabet (int, optional) – The maximum different characters to keep in the alphabet.
    /// @param initial_alphabet (List[str], optional) – A list of characters to include in the initial alphabet, even if not seen in the training dataset. If the strings contain more than one character, only the first one is kept.
    /// @param continuing_subword_prefix (str, optional) – A prefix to be used for every subword that is not a beginning-of-word.
    /// @param end_of_word_suffix (str, optional) – A suffix to be used for every subword that is a end-of-word.
    /// 
    /// 
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

