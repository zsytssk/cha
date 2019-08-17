mod node;

use super::lexer::{Keyword, Punc, Token, TokenData};
use super::utils::expect::{ExpectList, ExpectType};
use node::NodeType::*;
pub use node::{Node, NodePosition, NodeType, NodeVal};
use std::sync::Mutex;

pub struct Parser{
    scope_tree: Mutex<Vec<Node>>,
    /* 当前语句 */
    cur_state: Mutex<Vec<Node>>,
}

impl Parser {
    pub fn parse(tokens: Vec<Token>) {
        let mut parser = Parser {
            scope_tree: Mutex::new(Vec::new()),
            cur_state: Mutex::new(Vec::new()),
        };
        parser.parsing(tokens)
    }
    pub fn parsing<'a>(&'a mut self, tokens: Vec<Token>) {
        let mut expect_list: ExpectList<NodeType, Node> = ExpectList::new();
        for token in tokens {
            let Token { data, pos } = token;

            match data {
                TokenData::Bool(_boolean) => {}
                TokenData::Keyword(keyword) => match keyword {
                    Keyword::Let => {
                        let expect_fn = |ident| {
                        let mut ref_cur_state = &self.cur_state.lock().unwrap();
                            let mut cur_node =
                                Node::new(NodeType::Define, NodePosition::new(pos.x, pos.y));
                            let val = cur_node.get_val();
                            match val {
                                NodeVal::Define(ref mut define) => {
                                    define.set_right(ident);
                                }
                                _ => {}
                            };
                            ref_cur_state.push(cur_node);
                        };

                        expect_list.add(
                            NodeType::Variable,
                            ExpectType::After,
                            Box::new(expect_fn),
                        );
                    }
                    _ => {}
                },
                TokenData::Identifier(_ident) => {
                    let cur_node = Node::new(NodeType::Variable, NodePosition::new(pos.x, pos.y));
                    expect_list
                        .is_match(&NodeType::Variable, &ExpectType::After, cur_node);
                }
                TokenData::Punc(punc) => match punc {
                    Punc::Assign => {
                        let cur_node = Node::new(NodeType::Assign, NodePosition::new(pos.x, pos.y));
                        let mut ref_scope_tree = self.scope_tree.lock().unwrap();
                        ref_scope_tree.push(cur_node);
                    }
                    Punc::OpenBlock => {
                        let cur_node = Node::new(NodeType::Scope, NodePosition::new(pos.x, pos.y));
                        let mut ref_scope_tree = self.scope_tree.lock().unwrap();
                        ref_scope_tree.push(cur_node);
                        let expect_fn: Box<FnOnce(Node) + 'a> = Box::new(|child_scope| {
                            let mut ref_scope_tree = self.scope_tree.lock().unwrap();
                            self.last_scope_add_child(child_scope);
                        });
                        expect_list
                            .add(NodeType::Scope, ExpectType::After, expect_fn);
                    }
                    Punc::CloseBlock => {
                        let mut ref_scope_tree = self.scope_tree.lock().unwrap();
                        let last_index = ref_scope_tree.len() - 1;
                        let cur_scope = ref_scope_tree.remove(last_index);
                        expect_list
                            .is_match(&NodeType::Scope, &ExpectType::After, cur_scope);
                    }
                    _ => {}
                },
                TokenData::EOL => {}
                TokenData::EOF => {}
            }

            println!("{:?}", self.scope_tree);
        }
    }
    fn last_scope_add_child(&mut self, child: Node) {
        let mut ref_scope_tree = self.scope_tree.lock().unwrap();
        let last_scope = ref_scope_tree.last_mut().unwrap();
        let val = last_scope.get_val();
        match val {
            NodeVal::Scope(ref mut scope) => {
                scope.add_child(child);
            }
            _ => {}
        }
    }
    fn test(&mut self) {}
}
