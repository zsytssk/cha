mod variable_type;

use variable_type::VariableType;

#[derive(Debug)]
pub struct Variable {
    name: String,
    /** variable 的类型  */
    variable_type: Option<VariableType>,
}

impl Variable {
    pub fn new() -> Variable {
        Variable {
            name: String::new(),
            variable_type: None,
        }
    }
    pub fn set_type(&mut self, variable_type: VariableType) {
        self.variable_type = Some(variable_type);
    }
}
