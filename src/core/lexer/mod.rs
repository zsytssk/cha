mod config;
mod error;
mod token;

use error::Error;
use std::iter::Peekable;
use std::str::Chars;
pub use token::{Token, TokenData};
pub use config::punc::Punc;
pub use config::keyword::Keyword;

#[derive(Debug)]
pub struct Lexer<'a> {
    pub chars: Peekable<Chars<'a>>,
    pub tokens: Vec<Token>,
    line_number: u64,
    column_number: u64,
}

impl<'a> Lexer<'a> {
    pub fn parse(source: &str) -> Vec<Token> {
        let chars = source.chars().peekable();
        let mut parser = Lexer {
            chars,
            tokens: Vec::new(),
            line_number: 0,
            column_number: 0,
        };

        match parser.lex() {
            Err(err) => println!("{:?}", err),
            _ => {}
        }

        parser.tokens
    }
    pub fn lex(&mut self) -> Result<(), Error> {
        let err: Error;
        loop {
            // Check if we've reached the end
            match self.preview_next() {
                Some(_) => (),
                None => {
                    return Ok(());
                }
            }
            let ch = self.next()?;

            match ch {
                '\n' | ' ' | '\t' | '\r' => {
                    if ch == '\r' {
                        self.line_number += 1;
                        self.column_number = 0;
                    }
                }
                _ if Punc::is_punc(&ch.to_string()) => match Punc::from_str(&ch.to_string()) {
                    Some(punc) => {
                        let token_data = TokenData::Punc(punc);
                        self.push_token(token_data);
                    }
                    None => {
                        panic!("cant generate Punc from {}", ch);
                    }
                },
                _ if ch.is_alphabetic() || ch == '$' || ch == '_' => {
                    let mut buf = ch.to_string();
                    loop {
                        let nch = match self.preview_next() {
                            Some(ch) => ch,
                            None => break,
                        };

                        match nch {
                            _ if nch.is_alphabetic() || nch.is_digit(10) || nch == '_' => {
                                buf.push(self.next()?);
                            }
                            _ => {
                                break;
                            }
                        }
                    }

                    match TokenData::from_str(&buf) {
                        Some(token_data) => {
                            self.push_token(token_data);
                        }
                        None => {
                            panic!("cant generate Punc from {}", &buf);
                        }
                    }
                }
                _ => {
                    println!("{:?}", &self.tokens);
                    let msg = format!(
                        "{}:{}: Unexpected char '{}'",
                        self.line_number,
                        self.column_number,
                        ch.to_string()
                    );

                    err = Error::new(&msg);
                    break;
                }
            }
        }

        Err(err)
    }
    pub fn next(&mut self) -> Result<char, Error> {
        self.column_number += 1;
        match self.chars.next() {
            Some(char) => Ok(char),
            None => Err(Error::new("finished")),
        }
    }

    fn preview_next(&mut self) -> Option<char> {
        // No need to return a reference, we can return a copy
        match self.chars.peek() {
            Some(v) => Some(*v),
            None => None,
        }
    }
    fn push_token(&mut self, tk: TokenData) {
        let token = Token::new(tk, self.column_number, self.line_number);
        self.tokens.push(token)
    }
}
