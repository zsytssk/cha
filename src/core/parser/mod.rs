mod node;

use super::lexer::{Keyword, Punc, Token, TokenData};
use node::sign::Sign;
use node::NodeType::*;
pub use node::{Node, NodePosition, NodeType, NodeVal};

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    pub fn parse(tokens: Vec<Token>) -> Vec<Node> {
        println!("{:?}", &tokens);
        let mut parser = Parser { tokens, index: 0 };
        parser.parsing()
    }
    pub fn parsing(&mut self) -> Vec<Node> {
        /* 最后返回的的Node列表 */
        let mut node_list: Vec<Node> = Vec::new();
        let mut scope_tree: Vec<Node> = Vec::new();
        let mut cur_state: Vec<Node> = Vec::new();

        loop {
            let mut next_node = match self.get_next() {
                Some(node) => node,
                None => {
                    println!("{:?}", &node_list);
                    return node_list;
                }
            };

            let Node {
                ref mut val,
                position,
                ..
            } = &mut next_node;
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
                }
                NodeVal::Scope(_) => {
                    scope_tree.push(next_node);
                }
                NodeVal::Sign(ref sign_box) => match sign_box {
                    Sign::ScopeClose => {
                        let scope = scope_tree.remove(scope_tree.len() - 1);
                        addToLastScope(scope, &mut scope_tree, &mut node_list);
                    }
                    Sign::EndOfLine => {
                        let pos = &next_node.position.pos;
                        let mut state =
                            Node::new(NodeType::Statement, NodePosition::new(pos.x, pos.y), None);
                        match state.val {
                            NodeVal::Statement(ref mut in_state) => {
                                for i in 0..cur_state.len() {
                                    let node = cur_state.remove(i);
                                    in_state.add_child(node);
                                }
                            }
                            _ => {}
                        };

                        addToLastScope(state, &mut scope_tree, &mut node_list);
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
    fn get_next(&mut self) -> Option<Node> {
        match self.preview_next() {
            None => None,
            Some(node) => {
                self.index += 1;
                Some(node)
            }
        }
    }
    fn preview_next(&self) -> Option<Node> {
        if self.index >= self.tokens.len() {
            None
        } else {
            let index = self.index;
            let next_token = &self.tokens[index];
            let Token { data, pos } = next_token;

            println!("test:> {:?}", &data);
            match data {
                TokenData::Bool(_boolean) => None,
                TokenData::Keyword(keyword) => match keyword {
                    Keyword::Let => Some(Node::new(
                        NodeType::Define,
                        NodePosition::new(pos.x, pos.y),
                        None,
                    )),
                    _ => None,
                },
                TokenData::Identifier(_ident) => Some(Node::new(
                    NodeType::Variable,
                    NodePosition::new(pos.x, pos.y),
                    None,
                )),
                TokenData::Punc(punc) => match punc {
                    Punc::Assign => Some(Node::new(
                        NodeType::Assign,
                        NodePosition::new(pos.x, pos.y),
                        None,
                    )),
                    Punc::OpenBlock => Some(Node::new(
                        NodeType::Scope,
                        NodePosition::new(pos.x, pos.y),
                        None,
                    )),
                    Punc::CloseBlock => Some(Node::new(
                        NodeType::Sign,
                        NodePosition::new(pos.x, pos.y),
                        Some(data),
                    )),
                    _ => None,
                },
                TokenData::EOL => Some(Node::new(
                    NodeType::Sign,
                    NodePosition::new(pos.x, pos.y),
                    Some(data),
                )),
                TokenData::EOF => Some(Node::new(
                    NodeType::Sign,
                    NodePosition::new(pos.x, pos.y),
                    Some(data),
                )),
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

fn addToLastScope(item: Node, scope_tree: &mut Vec<Node>, node_list: &mut Vec<Node>) {
    let last_index = (scope_tree.len() - 1) as i32;
    if last_index >= 0 {
        let Node { ref mut val, .. } = &mut scope_tree[last_index as usize];
        match val {
            NodeVal::Scope(ref mut scope_parent) => {
                scope_parent.add_child(item);
            }
            _ => {}
        }
    } else {
        node_list.push(item);
    }
}
