#[derive(Debug)]
pub enum Keyword {
    Let
}

impl Keyword {
    pub fn from_str(val: &str) -> Option<Keyword>  {
        match val {
            "let" => {
                Some(Keyword::Let)
            },
            _ => {
                None
            }
        }
    }
}