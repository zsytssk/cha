use super::Node;

#[derive(Debug)]
pub struct Assign {
    left: Option<Node>,
    right: Option<Node>,
}

impl Assign  {
    pub fn new() -> Assign {
        Assign {
            left: None,
            right: None,
        }
    }
    pub fn set_left(&mut self, node: Node) {
        self.left = Some(node);
    }
    pub fn set_right(&mut self, node: Node) {
        self.right = Some(node);
    }
}

