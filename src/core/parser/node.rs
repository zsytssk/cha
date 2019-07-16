pub use super::node_type::NodeType;
pub use crate::core::utils::pos::NodePosition;

pub struct Node {
    node_type: NodeType,
    position: NodePosition,
    left: Box<Option<Node>>,
    right: Box<Option<Node>>,
    children: Vec<Node>,
}

impl Node {
    pub fn new(node_type: NodeType, position: NodePosition) -> Node {
        Node {
            node_type,
            position,
            left: Box::new(None),
            right: Box::new(None),
            children: Vec::new(),
        }
    }
    pub fn addChild(&mut self, child: Node) {
        self.children.push(child);
    }
}
