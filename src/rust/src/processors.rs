use extendr_api::*;
use crate::encoding::*;
use tokenizers::PostProcessor;
use std::convert::TryFrom;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct RPostProcessor {
    post_processor: tokenizers::PostProcessorWrapper
}

#[extendr]
impl RPostProcessor {
    fn process (&self, encoding: &REncoding, pair_encoding: Nullable<&REncoding>, add_special_tokens: bool) -> REncoding {
        
        let pair_encoding = match pair_encoding {
            Nullable::NotNull(v) => Option::Some(v.clone().encoding),
            Nullable::Null => Option::None
        };

        let out = self.post_processor.process(encoding.clone().encoding, pair_encoding, add_special_tokens);
        REncoding{encoding: out.unwrap()}
    }
}

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

pub struct RTemplate(tokenizers::processors::template::Template);

impl From<RTemplate> for tokenizers::processors::template::Template {
    fn from(v: RTemplate) -> Self {
        v.0
    }
}

impl<'a> FromRobj<'a> for RTemplate {
    fn from_robj (robj: &'a Robj) -> std::result::Result<Self, &'static str> {
        if let Some(s) = robj.as_str() {
            let template = tokenizers::processors::template::Template::try_from(s); 
            std::result::Result::Ok(RTemplate(template.unwrap()))
        } else if let Some(s) = robj.as_str_vector() {
            let template = tokenizers::processors::template::Template::try_from(s);
            std::result::Result::Ok(RTemplate(template.unwrap()))
        } else {
            Err("Expected a character vector.")
        }
    }
}

pub struct RSpecialToken(tokenizers::processors::template::SpecialToken);

#[extendr]
impl RSpecialToken {}

impl From<RSpecialToken> for tokenizers::processors::template::SpecialToken {
    fn from(v: RSpecialToken) -> Self {
        v.0
    }
}

impl<'a> FromRobj<'a> for RSpecialToken {
    fn from_robj (robj: &'a Robj) -> std::result::Result<Self, &'static str> {
        if let Some(s) = robj.as_list_iter() {            
            let values : Vec<Robj> = s.collect();
            Ok(Self(tokenizers::processors::template::SpecialToken::from(
                (values[0].as_str().unwrap(),
                values[1].as_integer().unwrap() as u32) 
            )))
        } else {
            Err("Expected a character vector.")
        }
    }
}


pub struct VecRSpecialToken(Vec<RSpecialToken>);
#[extendr]
impl VecRSpecialToken {}

impl<'a> FromRobj<'a> for VecRSpecialToken {
    fn from_robj(robj: &'a Robj) -> std::result::Result<Self, &'static str> {
        if let Some(x) = robj.as_list_iter() {
            let mut output = Vec::<RSpecialToken>::new();
            for v in x {
                output.push(RSpecialToken::from_robj(&v).unwrap())
            }
            Ok(VecRSpecialToken(output))
        } else {
            Err("Expected a vector of special tokens.")
        }
    }
}

pub struct RTemplateProcessing {}

#[extendr]
impl RTemplateProcessing {
    fn new(
        single: Nullable<RTemplate>,
        pair: Nullable<RTemplate>,
        special_tokens: Nullable<VecRSpecialToken>
    ) -> RPostProcessor {

        let mut builder = tokenizers::processors::template::TemplateProcessing::builder();

        if let Nullable::NotNull(seq) = single {
            builder.single(seq.into());
        }
        if let Nullable::NotNull(seq) = pair {
            builder.pair(seq.into());
        }
        if let Nullable::NotNull(sp) = special_tokens {
            builder.special_tokens(sp.0);
        }

        let post_processor = builder.build().unwrap();
        
        RPostProcessor{post_processor: tokenizers::PostProcessorWrapper::Template(post_processor)}
    }
}

extendr_module! {
    mod processors;
    impl RPostProcessor;
    impl RSpecialToken;
    impl VecRSpecialToken;
    impl RTemplateProcessing;
}