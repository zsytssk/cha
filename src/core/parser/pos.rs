#[derive(Debug)]
pub struct Pos {
    x: u64,
    y: u64,
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