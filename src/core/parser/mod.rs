mod node;

use super::lexer::{Keyword, Punc, Token, TokenData};
use node::NodeType::*;
use node::sign::Sign;
pub use node::{Node, NodePosition, NodeType, NodeVal};
use std::sync::Mutex;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn parse(tokens: Vec<Token>) -> Vec<Node> {
        let mut parser = Parser { tokens, pos: 0 };
        parser.parsing()
    }
    pub fn parsing(&mut self) -> Vec<Node> {
        let mut scope_tree: Vec<Node> = Vec::new();
        let mut cur_state: Vec<Node> = Vec::new();

        loop {
            let mut next_node = match self.get_next() {
                Some(node) => node,
                None => {
                    return scope_tree;
                }
            };

            let Node { ref mut val, .. } = &mut next_node;
            match val {
                NodeVal::Define(ref mut define) => {
                    let var_node = self.get_next().unwrap();
                    let Node { val, .. } = &var_node;
                    match val {
                        NodeVal::Variable(_) => {
                            define.set_right(var_node);
                        }
                        _ => {}
                    };
                    cur_state.push(next_node);
                },
                NodeVal::Scope(_) => {
                    scope_tree.push(next_node);
                },
                NodeVal::Sign(ref sign_box) =>  {
                    match sign_box {
                        Sign::ScopeClose => {
                            let scope = scope_tree.remove(scope_tree.len() -1);
                            let last_index = scope_tree.len() -1;
                            let Node { ref mut val, .. } = &mut scope_tree[last_index];
                            match val {
                                NodeVal::Scope(ref mut scope_parent) => {
                                    scope_parent.add_child(scope);
                                }
                                _ => {}
                            }
                        },
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
    fn get_next(&mut self) -> Option<Node> {
        if self.pos >= self.tokens.len() {
            None
        } else {
            self.pos += 1;
            let next_token = &self.tokens[self.pos];
            let Token { data, pos } = next_token;

            match data {
                TokenData::Bool(_boolean) => None,
                TokenData::Keyword(keyword) => match keyword {
                    Keyword::Let => Some(Node::new(
                        NodeType::Define,
                        NodePosition::new(pos.x, pos.y),
                        data,
                    )),
                    _ => None,
                },
                TokenData::Identifier(_ident) => Some(Node::new(
                    NodeType::Variable,
                    NodePosition::new(pos.x, pos.y),
                    data,
                )),
                TokenData::Punc(punc) => match punc {
                    Punc::Assign => {
                        Some(Node::new(NodeType::Assign, NodePosition::new(pos.x, pos.y),data))
                    }
                    Punc::OpenBlock => {
                        Some(Node::new(NodeType::Scope, NodePosition::new(pos.x, pos.y), data))
                    }
                    Punc::CloseBlock => {
                        Some(Node::new(NodeType::Scope, NodePosition::new(pos.x, pos.y), data))
                    }
                    _ => None,
                },
                TokenData::EOL => None,
                TokenData::EOF => None,
            }
        }
    }
    fn last_scope_add_child(&mut self, child: Node) {
        // let mut ref_scope_tree = scope_tree.lock().unwrap();
        // let last_scope = ref_scope_tree.last_mut().unwrap();
        // let val = last_scope.get_val();
        // match val {
        //     NodeVal::Scope(ref mut scope) => {
        //         scope.add_child(child);
        //     }
        //     _ => {}
        // }
    }
}
