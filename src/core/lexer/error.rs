#[derive(Debug)]
pub struct Error {
    msg: String,
}

impl Error {
    pub fn new(msg: &str) -> Error {
        Error {
            msg: msg.to_owned()
        }
    }
}