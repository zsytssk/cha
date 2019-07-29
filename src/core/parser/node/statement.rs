use super::{Node};

pub struct Statement {
    children: Vec<Node>,
}

impl Statement {
    pub fn new() -> Statement {
        Statement {
            children: Vec::new(),
        }
    }
    pub fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }
}
