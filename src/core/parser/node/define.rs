use super::Node;

#[derive(Debug)]
pub struct Define {
    right: Option<Node>,
    is_mut: bool,
}

impl Define {
    pub fn new() -> Define {
        Define {
            right: None,
            is_mut: false,
        }
    }
    pub fn set_right(&mut self, node: Node) {
        self.right = Some(node);
    }
    pub fn set_mut(&mut self, is_mut: bool) {
        self.is_mut = is_mut;
    }
}
