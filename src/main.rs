use std::fs::read_to_string;
mod core;

use crate::core::{executer::Executer, lexer::Lexer, parser::Parser};

fn main() {
    let source = read_to_string("src/spec/demo.cha").unwrap();
    let tokens = Lexer::parse(&source);
    Parser::parse(tokens);
}
