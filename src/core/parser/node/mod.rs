mod assign;
mod node_type;
mod scope;
mod variable;
mod statement;

pub mod sign;
pub mod define;
pub use crate::core::utils::pos::NodePosition;
pub use node_type::{NodeType, NodeVal};
pub use crate::core::lexer::{Punc, TokenData};

#[derive(Debug)]
pub struct Node {
    pub val: NodeVal,
    pub position: NodePosition,
}

impl Node {
    pub fn new(node_type: NodeType, position: NodePosition, ori_data: Option<&TokenData>) -> Node {
        let node_val = NodeVal::new(node_type, ori_data);
        Node {
            val: node_val,
            position
        }
    }
    pub fn get_val(&mut self) -> &mut NodeVal {
        return &mut self.val;
    }
}
