mod node;
mod node_type;

use super::lexer::{Keyword, Punc, Token, TokenData};
use node::Node;

pub struct Parser {}

impl Parser {
    pub fn parse(tokens: Vec<Token>) {
        println!("{:?}", tokens);
        let mut node_list: Vec<Node> = Vec::new();
        for token in tokens {
            let Token { data, .. } = token;

            match data {
                TokenData::Bool(boolean) => {}
                TokenData::Keyword(Keyword) => {}
                TokenData::Identifier(ident) => {}
                TokenData::Punc(punc) => {}
                TokenData::EOF => {}
            }
        }
    }
}
