use crate::model::Value;

#[derive(Debug, PartialEq)]
pub enum Operation {
    Lit(Value),
    MakeList(usize),
    MakeMap(usize),
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Neg,
    Not,
}

pub mod walker;
pub mod runtime;
