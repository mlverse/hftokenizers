use extendr_api::*;
use crate::models::*;
use crate::pre_tokenizers::*;
use crate::normalizers::*;
use crate::decoders::*;
use crate::processors::*;
use tokenizers::Model;

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

    fn train(&mut self, files: Vec<String>) {
        let mut trainer = self.tokenizer.get_model().get_trainer();
        match self.tokenizer.train_from_files(&mut trainer, files) {
            Err(e) => panic!("Error: {}", e),
            _ => {}
        }        
    }

    fn encode (&self, sequence : Vec<String>, add_special_tokens: bool) -> Vec<u32> {
        
        let input_sequence = tokenizers::InputSequence::from(sequence);
        let input = tokenizers::EncodeInput::Single(input_sequence);
        
        match self.tokenizer.encode_char_offsets(input, add_special_tokens) {
            Err(e) => panic!("Error while encoding: {}", e),
            Ok(v) => v.get_ids().to_vec()
        }
    }

    fn get_vocab(&self, with_added_tokens: bool) -> Robj {
        
        // TODO change when we support auto convertion from HashMap<String, Robj>
        let res : Robj= self.tokenizer
            .get_vocab(with_added_tokens)
            .iter()
            .map(|(_, v)| v)
            .collect_robj();
        
        let names = self.tokenizer
            .get_vocab(with_added_tokens)
            .iter()
            .map(|(k, _)| k)
            .collect_robj();
        
        res.set_attrib(names_symbol(), names);
        res
    }
}

extendr_module! {
    mod tokenizer;
    impl RTokenizer;
}