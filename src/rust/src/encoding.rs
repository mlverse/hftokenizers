use extendr_api::*;

#[extendr]
pub struct REncoding {
    pub encoding: tokenizers::Encoding
}

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

  fn get_attention_mask (&self) {
      self.encoding.get_attention_mask().to_vec();
  }
}

extendr_module! {
    mod encoding;
    impl REncoding;
}
