use extendr_api::prelude::*;
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

fn make_encode_input<'a> (sequence : Robj, pair : Nullable<Robj>, is_pretokenized: bool) 
-> tokenizers::EncodeInput<'a> {

    let sequence: tokenizers::InputSequence = if is_pretokenized {
        pre_tokenized_input_sequence(sequence).unwrap()
    } else {
        text_input_sequence(sequence).unwrap()
    };

    
    match pair {
        extendr_api::wrapper::Nullable::NotNull(pair) => {
            let pair: tokenizers::InputSequence = if is_pretokenized {
                pre_tokenized_input_sequence(pair).unwrap()
            } else {
                text_input_sequence(pair).unwrap()
            };
            tokenizers::EncodeInput::Dual(sequence, pair)
        }
        extendr_api::wrapper::Nullable::Null => tokenizers::EncodeInput::Single(sequence),
    }
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
        let input = make_encode_input(sequence, pair, is_pretokenized);

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

    fn encode_batch (&self, inputs: Robj, add_special_tokens: bool) -> Vec<Robj> {

        let mut encoding_inputs = Vec::<tokenizers::EncodeInput>::new();

        if let Some(list) = inputs.as_list() {

            let mut inputs_str : Vec<Option<Vec<String>>> = list
                .iter()
                .map(|(_nm, val)| val.as_string_vector())
                .collect();

            if inputs_str.len() == 1 {
                inputs_str.push(Option::None);
            };

            match &inputs_str[0] {
                Some(sequence) => {
                    match &inputs_str[1] {
                        Some(pair) => {
                            for (s, p) in sequence.iter().zip(pair.iter()) {
                                encoding_inputs.push(
                                    tokenizers::EncodeInput::Dual(
                                        tokenizers::InputSequence::from(s.clone()),
                                        tokenizers::InputSequence::from(p.clone())
                                    )
                                );
                            }
                        },
                        None => {
                            for s in sequence.iter() {
                                encoding_inputs.push(
                                    tokenizers::EncodeInput::Single(
                                        tokenizers::InputSequence::from(s.clone())
                                    )
                                );
                            }
                        }
                    }
                },
                None => {
                    panic!("Need at a least a sequence.");
                }
            };
             
        } else {
            panic!("Not a list.")
        }

        self
            .tokenizer
            .encode_batch(encoding_inputs, add_special_tokens)
            .unwrap()
            .iter()
            .map(|v| Robj::from(REncoding{encoding: v.clone()}))
            .collect()
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

    fn get_pre_tokenizer (&self) -> RPreTokenizer {
        self.tokenizer.get_pre_tokenizer().unwrap().clone()
    }
 
    fn set_pre_tokenizer (&mut self, pre_tokenizer: &RPreTokenizer) {
        self.tokenizer.with_pre_tokenizer(pre_tokenizer.clone());
    }

    fn get_post_processor (&self) -> RPostProcessor {
        self.tokenizer.get_post_processor().unwrap().clone()
    }
 
    fn set_post_processor (&mut self, post_processor: &RPostProcessor) {
        self.tokenizer.with_post_processor(post_processor.clone());
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

    fn token_to_id (&self, token: &str) -> Option<u32> {
        self.tokenizer.token_to_id(token)
    }

    fn enable_padding (&mut self, direction: Nullable<&str>, pad_id : Nullable<u32>,
        pad_type_id : Nullable<u32>, pad_token: Nullable<&str>, length : Nullable<u32>,
        pad_to_multiple_of : Nullable<u32>) {
        let mut params = tokenizers::PaddingParams::default();

        match direction {
            Nullable::NotNull(value) => {
                params.direction = match value {
                    "left" => tokenizers::PaddingDirection::Left,
                    "right" => tokenizers::PaddingDirection::Right,
                    _ => {panic!("Unknown padding direction")}
                };
            },
            _ => {}
        }

        match pad_id {
            Nullable::NotNull(value) => params.pad_id = value,
            Nullable::Null => {}
        }

        match pad_type_id {
            Nullable::NotNull(value) => params.pad_type_id = value,
            Nullable::Null => {}
        }

        match pad_token {
            Nullable::NotNull(value) => params.pad_token = String::from(value),
            Nullable::Null => {}
        }

        match length {
            Nullable::NotNull(value) => params.strategy = tokenizers::PaddingStrategy::Fixed(value as usize),
            Nullable::Null => params.strategy = tokenizers::PaddingStrategy::BatchLongest
        }

        match pad_to_multiple_of {
            Nullable::NotNull(value) => params.pad_to_multiple_of = Option::Some(value as usize),
            Nullable::Null => {}
        }


        self.tokenizer.with_padding(Some(params));
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

