use super::assign::Assign;
use super::define::Define;
use super::scope::Scope;
use super::statement::Statement;
use super::variable::Variable;

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
}

#[derive(Debug)]
pub enum NodeVal {
    Variable(Box<Variable>),
    Assign(Box<Assign>),
    Scope(Box<Scope>),
    Define(Box<Define>),
    Statement(Box<Statement>),
}

impl NodeVal {
    pub fn new(node_type: NodeType) -> NodeVal {
        match node_type {
            NodeType::Scope => NodeVal::Scope(Box::new(Scope::new())),
            NodeType::Statement => NodeVal::Statement(Box::new(Statement::new())),
            NodeType::Assign => NodeVal::Assign(Box::new(Assign::new())),
            NodeType::Variable => NodeVal::Variable(Box::new(Variable::new())),
            NodeType::Define => NodeVal::Define(Box::new(Define::new())),
        }
    }
    pub fn unwrap() {}
}
