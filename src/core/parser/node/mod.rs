mod assign;
mod node_type;
mod scope;
mod variable;
mod define;
mod statement;

pub use crate::core::utils::pos::NodePosition;
pub use node_type::{NodeType, NodeVal};

#[derive(Debug)]
pub struct Node {
    pub val: NodeVal,
    pub position: NodePosition,
}

impl Node {
    pub fn new(node_type: NodeType, position: NodePosition) -> Node {
        let node_val = NodeVal::new(node_type);
        Node {
            val: node_val,
            position
        }
    }
    pub fn get_val(&mut self) -> &mut NodeVal {
        return &mut self.val;
    }
}
