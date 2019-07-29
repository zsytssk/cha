mod node;

use super::lexer::{Keyword, Punc, Token, TokenData};
pub use node::{Node, NodePosition, NodeType, NodeVal};
use node::NodeType::*;

pub struct Parser {}

impl Parser {
    pub fn parse(tokens: Vec<Token>) {
        let mut node_list: Vec<Node> = Vec::new();
        let mut scope_tree: Vec<Node> = Vec::new();
        for token in tokens {
            let Token { data, pos } = token;

            match data {
                TokenData::Bool(boolean) => {}
                TokenData::Keyword(keyword) => match keyword {
                    Keyword::Let => {
                        let cur_node =
                            Node::new(NodeType::Define, NodePosition::new(pos.x, pos.y));
                        scope_tree.push(cur_node);
                    }
                    _ => {}
                },
                TokenData::Identifier(ident) => {
                    let cur_node = Node::new(NodeType::Variable, NodePosition::new(pos.x, pos.y));
                    scope_tree.push(cur_node);
                }
                TokenData::Punc(punc) => match punc {
                    Punc::Assign => {
                        let cur_left = scope_tree.pop().unwrap();
                        let cur_node =
                            Node::new(NodeType::Assign, NodePosition::new(pos.x, pos.y));
                        scope_tree.push(cur_node);
                    }
                    Punc::OpenBlock => {
                        let cur_node = Node::new(NodeType::Scope, NodePosition::new(pos.x, pos.y));
                        scope_tree.push(cur_node);
                    }
                    Punc::CloseBlock => {
                        let cur_scope = scope_tree.last_mut().unwrap();
                        if let NodeVal::Scope(boxed_scope) = &mut cur_scope.val {
                            let scope = boxed_scope.as_mut();
                            for i in 0..node_list.len() {
                                let item = node_list.remove(i);
                                scope.add_child(item)
                            }
                        }
                    }
                    _ => {}
                },
                TokenData::EOF => {}
            }
        }
    }
}
