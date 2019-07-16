pub mod token_data;

pub use token_data::TokenData;
use crate::core::utils::pos::Pos;

#[derive(Debug)]
pub struct Token {
    pub data: TokenData,
    pub pos: Pos,
}

impl Token {
    pub fn new(data: TokenData, x: u64, y: u64) -> Token {
        Token {
            pos: Pos::new(x, y),
            data,
        }
    }
}