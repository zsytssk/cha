mod node;
mod node_type;

use super::lexer::{Keyword, Punc, Token, TokenData};
use node::{Node, NodePosition, NodeType};

pub struct Parser {}

impl Parser {
    pub fn parse(tokens: Vec<Token>) {
        println!("{:?}", tokens);
        let mut node_list: Vec<Node> = Vec::new();
        let mut scope_tree: Vec<Node> = Vec::new();
        for token in tokens {
            let Token { data, pos } = token;

            match data {
                TokenData::Bool(boolean) => {}
                TokenData::Keyword(keyword) => match keyword {
                    Keyword::Let => {
                        let cur_node =
                            Node::new(NodeType::Definition, NodePosition::new(pos.x, pos.y));
                    }
                    _ => {}
                },
                TokenData::Identifier(ident) => {}
                TokenData::Punc(punc) => match punc {
                    Punc::OpenBlock => {
                        let cur_node = Node::new(NodeType::Scope, NodePosition::new(pos.x, pos.y));
                        scope_tree.push(cur_node);
                    }
                    Punc::CloseBlock => {
                        let cur_scope = scope_tree.pop().unwrap();
                        if scope_tree.len() != 0 {
                            let latest_scope = scope_tree.last_mut().unwrap();
                            latest_scope.addChild(cur_scope);
                        } else {
                            node_list.push(cur_scope);
                        }
                    }
                    _ => {}
                },
                TokenData::EOF => {}
            }
        }
    }
}
