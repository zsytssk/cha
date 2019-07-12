
struct NodePosition {
    uid: String;
    pos {
        x: i32;
        y: i32;
    }
}

struct Node {
    type: NodeType;
    position: NodePosition;
}