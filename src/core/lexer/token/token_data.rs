use super::super::config::keyword::Keyword;
use super::super::config::punc::Punc;

#[derive(Debug)]
pub enum TokenData {
    Bool(bool),
    Keyword(Keyword),
    /** 变量 */
    Identifier(String),
    /** 字符串 */
    r#String(String),
    /** 标点符号 */
    Punc(Punc),
    EOL,
    SPACE,
    EOF,
}

impl TokenData {
    pub fn from_str(val: &str) -> Option<TokenData> {
        match val {
            "true" => Some(TokenData::Bool(true)),
            "false" => Some(TokenData::Bool(false)),
            _ => {
                if let Some(keyword) = Keyword::from_str(val) {
                    Some(TokenData::Keyword(keyword))
                } else {
                    Some(TokenData::Identifier(val.to_owned()))
                }
            }
        }
    }
    pub fn from_punc(val: &str) -> Option<TokenData> {
        match Punc::from_str(val) {
            Some(punc) => Some(TokenData::Punc(punc)),
            _ => None,
        }
    }
    pub fn from_string(val: &str) -> Option<TokenData> {
        Some(TokenData::r#String(val.to_owned()))
    }
}
