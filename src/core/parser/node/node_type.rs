use super::assign::Assign;
use super::define::Define;
use super::scope::Scope;
use super::sign::Sign;
use super::statement::Statement;
use super::variable::Variable;
use super::TokenData;

#[derive(PartialEq)]
pub enum NodeType {
    Scope,
    /** 定义 */
    Define,
    /** 赋值 */
    Assign,
    /** 变量 */
    Variable,
    /** 语句 */
    Statement,
    /** 标识:> closeScope, endOfLine */
    Sign,
}

#[derive(Debug)]
pub enum NodeVal {
    Variable(Box<Variable>),
    Assign(Box<Assign>),
    Scope(Box<Scope>),
    Define(Box<Define>),
    Statement(Box<Statement>),
    Sign(Sign),
}

impl NodeVal {
    pub fn new(node_type: NodeType, ori_data: &TokenData) -> NodeVal {
        match node_type {
            NodeType::Scope => NodeVal::Scope(Box::new(Scope::new())),
            NodeType::Statement => NodeVal::Statement(Box::new(Statement::new())),
            NodeType::Assign => NodeVal::Assign(Box::new(Assign::new())),
            NodeType::Variable => NodeVal::Variable(Box::new(Variable::new())),
            NodeType::Define => NodeVal::Define(Box::new(Define::new())),
            NodeType::Sign => {
                let sign = Sign::new(ori_data).unwrap();
                NodeVal::Sign(sign)
            },
        }
    }
    pub fn unwrap() {}
}
