use serde::Serialize;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum Expression {
    Eq(Box<Expression>, Box<Expression>),
    Neq(Box<Expression>, Box<Expression>),
    Lt(Box<Expression>, Box<Expression>),
    Lte(Box<Expression>, Box<Expression>),
    Gte(Box<Expression>, Box<Expression>),
    Gt(Box<Expression>, Box<Expression>),
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
    Mod(Box<Expression>, Box<Expression>),
    Neg(Box<Expression>),
    Not(Box<Expression>),
    Member(Box<Expression>, Identifier),
    Method(Box<Expression>, Identifier, Vec<Expression>),
    Lit(Literal),
    Binding(Identifier),
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum Literal {
    I64(i64),
    F64(f64),
    Bool(bool),
    String(String),
    Bytes(Vec<u8>),
    List(Vec<Expression>),
    Null,
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum Value {
    I64(i64),
    F64(f64),
    Bool(bool),
    String(String),
    Bytes(Vec<u8>),
    List(Vec<Value>),
    Null,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
pub struct Identifier(pub String);

impl FromStr for Identifier {
    type Err = ();
    fn from_str(input: &str) -> Result<Identifier, ()> {
        Ok(Identifier(input.to_owned()))
    }
}

pub type EvalResult = Result<Value, String>;
