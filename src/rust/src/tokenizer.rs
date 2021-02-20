use extendr_api::*;
use crate::models::*;
use crate::pre_tokenizers::*;
use crate::normalizers::*;
use crate::decoders::*;
use crate::processors::*;
use crate::encoding::*;
use crate::trainers;
use tokenizers::Model;

type Tokenizer = tokenizers::TokenizerImpl::<RModel, RNormalizer,RPreTokenizer,RPostProcessor,RDecoder>;

struct RTokenizer {
    pub tokenizer: Tokenizer
}

#[extendr]
impl RTokenizer {
    fn from_model (model: &RModel) -> Self {
        RTokenizer {tokenizer: tokenizers::TokenizerImpl::new(model.clone())}
    }

    fn train(&mut self, files: Vec<String>, trainer : Nullable<&mut trainers::RTrainer>) {
        match trainer {
            Nullable::NotNull(v) => {
                match self.tokenizer.train_from_files(v, files) {
                    Err(e) => panic!("Error: {}", e),
                    _ => {}
                };
            },
            Nullable::Null => {
                let mut trainer = self.tokenizer.get_model().get_trainer();
                match self.tokenizer.train_from_files(&mut trainer, files) {
                    Err(e) => panic!("Error: {}", e),
                    _ => {}
                };
            }
        };       
    }

    fn encode (&self, sequence : Robj, pair: Nullable<Robj>, is_pretokenized: bool, add_special_tokens: bool) -> REncoding {
        
        let sequence: tokenizers::InputSequence = if is_pretokenized {
            pre_tokenized_input_sequence(sequence).unwrap()
        } else {
            text_input_sequence(sequence).unwrap()
        };

        let input = match pair {
            extendr_api::wrapper::Nullable::NotNull(pair) => {
                let pair: tokenizers::InputSequence = if is_pretokenized {
                    pre_tokenized_input_sequence(pair).unwrap()
                } else {
                    text_input_sequence(pair).unwrap()
                };
                tokenizers::EncodeInput::Dual(sequence, pair)
            }
            extendr_api::wrapper::Nullable::Null => tokenizers::EncodeInput::Single(sequence),
        };
        
        match self.tokenizer.encode_char_offsets(input, add_special_tokens) {
            Err(e) => panic!("Error while encoding: {}", e),
            Ok(v) => REncoding{encoding: v}
        }
    }

    fn decode(&self, ids: Vec<i32>, skip_special_tokens: bool) -> String {

        let input_ids : Vec<u32> = ids
            .iter()
            .map(|v| *v as u32)
            .collect();

        match self.tokenizer.decode(input_ids, skip_special_tokens) {
            Err(e) => panic!("Error while decoding: {}", e),
            Ok(v) => v.as_str().to_owned()
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
        
        match res.set_attrib(names_symbol(), names) {
            Err(e) => panic!("Error: {}", e),
            _ => {}
        }

        res
    }

    fn get_pre_tokenizer (&self) -> &RPreTokenizer {
        self.tokenizer.get_pre_tokenizer().unwrap()
    }
 
    fn set_pre_tokenizer (&mut self, pre_tokenizer: &RPreTokenizer) {
        self.tokenizer.with_pre_tokenizer(pre_tokenizer.clone());
    }

    fn save(&self, path: &str, pretty: bool) {
        match self.tokenizer.save(path, pretty) {
            Err(e) => panic!("Error: {}", e),
            _ => {}
        }
    }

    fn from_file (path : &str) -> Self {
        RTokenizer{tokenizer: Tokenizer::from_file(path).unwrap()}
    }

}

fn pre_tokenized_input_sequence<'s> (obj: Robj) -> std::result::Result<tokenizers::InputSequence<'s>, &'static str> {
    if let Some(v) = obj.as_string_vector() {
        Ok(tokenizers::InputSequence::from(v.to_vec()))
    } else {
        Err("Expected a chracter vectors.")
    }
}
fn text_input_sequence<'s> (obj: Robj) -> std::result::Result<tokenizers::InputSequence<'s>, &'static str> {
    if let Some(v) = obj.as_str() {
        Ok(tokenizers::InputSequence::from(v))
    } else {
        Err("Expected a length 1 character vector.")
    }
}

extendr_module! {
    mod tokenizer;
    impl RTokenizer;
}

