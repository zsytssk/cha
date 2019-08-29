mod variable_type;
use super::TokenData;
use variable_type::VariableType;

#[derive(Debug)]
pub struct Variable {
    name: String,
    /** variable 的类型  */
    variable_type: Option<VariableType>,
}

impl Variable {
    pub fn new(token_data: Option<&TokenData>) -> Variable {
        let name = match token_data.unwrap() {
            TokenData::Identifier(r#str) => r#str,
            _ => "",
        };
        Variable {
            name: String::from(name),
            variable_type: None,
        }
    }
    pub fn set_type(&mut self, variable_type: VariableType) {
        self.variable_type = Some(variable_type);
    }
}
