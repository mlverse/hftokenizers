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