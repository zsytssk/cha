#[derive(Debug)]
pub enum Punc {
    /// `=`
    Assign,
    /// `}`
    CloseBlock,
    /// `)`
    CloseParen,
    /// `{`
    OpenBlock,
    /// `(`
    OpenParen,
    /// `"`
    DoubleQuote,
    /// `;`
    Semicolon,
}

impl Punc {
    pub fn from_str(val: &str) -> Option<Punc> {
        match val {
            "=" => Some(Punc::Assign),
            "}" => Some(Punc::CloseBlock),
            ")" => Some(Punc::CloseParen),
            "{" => Some(Punc::OpenBlock),
            "(" => Some(Punc::OpenParen),
            "\"" => Some(Punc::DoubleQuote),
            ";" => Some(Punc::Semicolon),
            _ => None,
        }
    }
    pub fn is_punc(val: &str) -> bool {
        match val {
            "=" => true,
            "}" => true,
            ")" => true,
            "{" => true,
            "(" => true,
            "\"" => true,
            ";" => true,
            _ => false,
        }
    }
}
