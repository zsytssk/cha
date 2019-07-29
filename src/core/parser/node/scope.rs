use super::{Node};

pub struct Scope {
    children: Vec<Node>,
}

impl Scope {
    pub fn new() -> Scope {
        Scope {
            children: Vec::new(),
        }
    }
    pub fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }
}
