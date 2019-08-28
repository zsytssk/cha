use super::super::{Punc, TokenData};

#[derive(Debug)]
pub enum Sign {
    EndOfLine,
    EndOfFile,
    ScopeClose,
}

impl Sign {
    pub fn new(ori_data: &TokenData) -> Option<Sign> {
        match ori_data {
            TokenData::Punc(punc) => match punc {
                Punc::CloseBlock => Some(Sign::ScopeClose),
                _ => None,
            },
            TokenData::EOL => Some(Sign::EndOfLine),
            TokenData::EOF => Some(Sign::EndOfFile),
            _ => None,
        }
    }
}
