use std::fs::read_to_string;
mod core;
use crate::core::{parser::Parser, executer::Executer};

fn main() {
    let source = read_to_string("src/spec/demo.cha").unwrap();
    Parser::parse(&source);
}
