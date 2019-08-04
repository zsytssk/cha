mod node;

use std::sync::{Mutex};
use super::lexer::{Keyword, Punc, Token, TokenData};
use super::utils::expect::{ExpectList, ExpectType};
use node::NodeType::*;
pub use node::{Node, NodePosition, NodeType, NodeVal};

pub struct Parser {}

impl Parser {
    pub fn parse(tokens: Vec<Token>) {
        let scope_tree: Mutex<Vec<Node>> = Mutex::new(Vec::new());
        let mut expect_list:ExpectList<NodeType, Node> = ExpectList::new();
        for token in tokens {
            let Token { data, pos } = token;

            match data {
                TokenData::Bool(_boolean) => {}
                TokenData::Keyword(keyword) => match keyword {
                    Keyword::Let => {
                        let mut cur_node = Node::new(NodeType::Define, NodePosition::new(pos.x, pos.y));
                        let mut ref_scope_tree = scope_tree.lock().unwrap();
                        Parser::last_scope_add_child(&mut *ref_scope_tree, cur_node);
                        expect_list.add(NodeType::Variable, ExpectType::After, Box::new(|ident| {
                            let val = &cur_node.get_val();
                            match val {
                                NodeVal::Define(define) => {
                                    define.set_right(ident);
                                }
                                _ => {}
                            }
                        }));
                    },
                    _ => {}
                },
                TokenData::Identifier(_ident) => {
                    let cur_node = Node::new(NodeType::Variable, NodePosition::new(pos.x, pos.y));
                    expect_list.is_match(&NodeType::Variable, &ExpectType::After, cur_node);
                }
                TokenData::Punc(punc) => match punc {
                    Punc::Assign => {
                        let cur_node = Node::new(NodeType::Assign, NodePosition::new(pos.x, pos.y));
                        let mut ref_scope_tree = scope_tree.lock().unwrap();
                        ref_scope_tree.push(cur_node);
                    }
                    Punc::OpenBlock => {
                        let cur_node = Node::new(NodeType::Scope, NodePosition::new(pos.x, pos.y));
                        let mut ref_scope_tree = scope_tree.lock().unwrap();
                        ref_scope_tree.push(cur_node);

                        expect_list.add(NodeType::Scope, ExpectType::After, Box::new(|child_scope| {
                            let mut ref_scope_tree = &scope_tree.lock().unwrap();
                            Parser::last_scope_add_child(&mut ref_scope_tree, child_scope);
                        }));
                    }
                    Punc::CloseBlock => {
                        let mut ref_scope_tree = scope_tree.lock().unwrap();
                        let last_index = ref_scope_tree.len() - 1;
                        let cur_scope = ref_scope_tree.remove(last_index);
                        expect_list.is_match(&NodeType::Scope, &ExpectType::After, cur_scope);
                    }
                    _ => {}
                },
                TokenData::EOL => {}
                TokenData::EOF => {}
            }

            println!("{:?}", scope_tree);
        }
    }
    fn last_scope_add_child(scope_tree: &mut Vec<Node>, child: Node) {
        let last_scope = scope_tree.last_mut().unwrap();
        let val = last_scope.get_val();
        match val {
            NodeVal::Scope(ref mut scope) => {
                scope.add_child(child);
            },
            _=> {}
        }
    }
}
