use extendr_api::prelude::*;
use serde::{Deserialize, Serialize};

#[extendr]
#[derive(Clone, Deserialize, Serialize)]
pub struct REncoding {
    pub encoding: tokenizers::Encoding
}

/// @export
#[extendr]
impl REncoding {

  fn get_n_sequences(&self) -> usize {
     self.encoding.n_sequences()
  }

  fn set_sequence_id(&mut self, sequence_id: i32) {
    self.encoding.set_sequence_id(sequence_id as usize);
  }

  fn get_ids(&self) -> Vec<u32> {
    self.encoding.get_ids().to_vec()
  }

  fn get_tokens(&self) -> Vec<String> {
    self.encoding.get_tokens().to_vec()
  }

  fn get_word_ids(&self) -> Vec<Option<u32>> {
    self.encoding.get_word_ids().to_vec()
  }

  fn get_sequence_ids(&self) -> Vec<Option<i32>> {
    self.encoding.get_sequence_ids()
        .iter()
        .map(|v| {
            match v {
                Some(v) => Option::Some(*v as i32),
                None => Option::None
            }  
        })
        .collect::<Vec<Option<i32>>>()
  }

  fn get_type_ids(&self) -> Vec<u32> {
    self.encoding.get_type_ids().to_vec()
  }

  fn get_offsets(&self) -> Robj {
    let v = self.encoding.get_offsets().to_vec();
    let v_iter = v
      .iter()
      .map(|v| <Robj>::from([v.0 as i32, v.1 as i32]));


    List::from_values(v_iter).into()
  }

  fn get_special_tokens_mask(&self) -> Vec<u32> {
    self.encoding.get_special_tokens_mask().to_vec()
  }
  
  fn get_attention_mask (&self) -> Vec<u32> {
      self.encoding.get_attention_mask().to_vec()
  }
  
  fn token_to_word(&self, token_index: u32) -> Option<u32> {
    let (_, word_idx) = self.encoding.token_to_word(token_index as usize)?;
    Some(word_idx)
  }

  fn char_to_token(&self, char_pos: u32, sequence_index: u32) -> Option<u32> {
    match self.encoding.char_to_token(char_pos as usize, sequence_index as usize) {
        Some(v) => Option::Some(v as u32),
        None => Option::None
    }
  }

  fn char_to_word(&self, char_pos: u32, sequence_index: u32) -> Option<u32> {
    match self.encoding.char_to_word(char_pos as usize, sequence_index as usize) {
        Some(v) => Some(v as u32),
        None => Option::None
    }
  }

  fn truncate(&mut self, max_length: u32, stride: u32) -> Result<()> {
    self.encoding.truncate(max_length as usize, stride as usize);
    Ok(())
  }
  
}

extendr_module! {
    mod encoding;
    impl REncoding;
}
