#[derive(Debug)]
pub struct Pos {
    pub x: u64,
    pub y: u64,
}

impl Pos {
    pub fn new(x: u64, y: u64) -> Pos {
        Pos {
            x, y
        }
    }
}

pub struct NodePosition {
    uid: String,
    pos: Pos,
}

impl NodePosition {
    pub fn new(x: u64, y: u64) -> NodePosition {
        NodePosition {
            uid: "".to_owned(),
            pos: Pos::new(x, y)
        }
    }
}