mod node;

use super::lexer::{Keyword, Punc, Token, TokenData};
use node::define::Define;
use node::sign::Sign;
use node::NodeType::*;
pub use node::{Node, NodePosition, NodeType, NodeVal};

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    pub fn parse(tokens: Vec<Token>) -> Vec<Node> {
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
            // println!("test:>2 {:?}", &next_node);
            let Node { ref mut val, .. } = &mut next_node;
            match val {
                NodeVal::Define(_) => {
                    cur_state.push(next_node);
                }
                NodeVal::Assign(_) => {
                    cur_state.push(next_node);
                }
                NodeVal::Variable(_) => {
                    cur_state.push(next_node);
                }
                NodeVal::r#String(_) => {
                    cur_state.push(next_node);
                }
                NodeVal::Scope(_) => {
                    scope_tree.push(next_node);
                }
                NodeVal::Sign(ref sign_box) => match sign_box {
                    Sign::ScopeClose => {
                        let scope = scope_tree.remove(scope_tree.len() - 1);
                        add_to_last_scope(scope, &mut scope_tree, &mut node_list);
                    }
                    Sign::EndOfLine => {
                        // 将当前statement 放到scope中...
                        let save_state = analysis_cur_state(&mut cur_state);
                        add_state_to_scope(save_state, &mut scope_tree, &mut node_list);
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
    fn get_next(&mut self) -> Option<Node> {
        loop {
            match self.preview_next() {
                None => {
                    self.index += 1;
                }
                Some(node) => {
                    self.index += 1;
                    let Node { val, .. } = &node;

                    match val {
                        NodeVal::Sign(Sign::EndOfFile) => return None,
                        _ => {}
                    };

                    return Some(node);
                }
            };
        }
    }
    fn preview_next(&self) -> Option<Node> {
        if self.index >= self.tokens.len() {
            None
        } else {
            let index = self.index;
            let next_token = &self.tokens[index];
            let Token { data, pos } = next_token;

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
                    Some(data),
                )),
                TokenData::r#String(_str) => Some(Node::new(
                    NodeType::r#String,
                    NodePosition::new(pos.x, pos.y),
                    Some(data),
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
                _ => None,
            }
        }
    }
}

fn add_state_to_scope(
    mut cur_state: Vec<Node>,
    scope_tree: &mut Vec<Node>,
    node_list: &mut Vec<Node>,
) {
    if cur_state.len() > 0 {
        let mut state = Node::new(NodeType::Statement, NodePosition::new(0, 0), None);
        match state.val {
            NodeVal::Statement(ref mut in_state) => {
                for i in (0..cur_state.len()).rev() {
                    let node = cur_state.remove(i);
                    in_state.add_child(node);
                }
            }
            _ => {}
        };

        add_to_last_scope(state, scope_tree, node_list);
    }
}

fn add_to_last_scope(item: Node, scope_tree: &mut Vec<Node>, node_list: &mut Vec<Node>) {
    let last_index = (scope_tree.len() as i32) - 1;
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

fn analysis_cur_state(cur_state: &mut Vec<Node>) -> Vec<Node> {
    let mut save_state: Vec<Node> = Vec::new();
    loop {
        if cur_state.len() == 0 {
            break;
        }
        let mut cur_item = cur_state.remove(0);
        let Node { ref mut val, .. } = &mut cur_item;
        match val {
            NodeVal::Assign(ref mut assign) => {
                let next_item = cur_state.remove(0);
                let pre_item = save_state.remove(save_state.len() - 1);
                assign.set_left(pre_item);
                assign.set_right(next_item);
            }
            NodeVal::Define(ref mut define) => {
                let next_item = cur_state.remove(0);
                define.set_right(next_item);
            }
            _ => {}
        };
        save_state.push(cur_item);
    }
    save_state
}
