use std::fs::read_to_string;
fn main() {
    let buffer = read_to_string("src/spec/demo.cha").unwrap();
    println!("{}", buffer);
}
