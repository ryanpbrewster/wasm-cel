use serde::Serialize;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum Expression {
    Or(Box<Expression>, Box<Expression>),
    And(Box<Expression>, Box<Expression>),
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

impl Expression {
    pub fn op(&self) -> Op {
        match self {
            Expression::Or(_, _) => Op::Or,
            Expression::And(_, _) => Op::And,
            Expression::Eq(_, _) => Op::Eq,
            Expression::Neq(_, _) => Op::Neq,
            Expression::Lt(_, _) => Op::Lt,
            Expression::Lte(_, _) => Op::Lte,
            Expression::Gte(_, _) => Op::Gte,
            Expression::Gt(_, _) => Op::Gt,
            Expression::Add(_, _) => Op::Plus,
            Expression::Sub(_, _) => Op::Minus,
            Expression::Mul(_, _) => Op::Times,
            Expression::Div(_, _) => Op::Div,
            Expression::Mod(_, _) => Op::Mod,
            Expression::Neg(_) => Op::Neg,
            Expression::Not(_) => Op::Not,
            Expression::Member(_, id) => Op::Member(id.clone()),
            Expression::Method(_, id, _) => Op::Method(id.clone()),
            Expression::Lit(_) => Op::Lit,
            Expression::Binding(_) => Op::Lookup,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize)]
pub enum Kind {
    I64,
    F64,
    Bool,
    String,
    Bytes,
    List,
    Map,
    Null,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
#[serde(tag = "t", content = "c")]
pub enum Op {
    Not,
    Neg,
    Plus,
    Minus,
    Times,
    Div,
    Mod,
    Or,
    And,
    Eq,
    Neq,
    Lte,
    Lt,
    Gt,
    Gte,
    Lit,
    Lookup,
    Member(Identifier),
    Method(Identifier),
}

impl Value {
    pub fn kind(&self) -> Kind {
        match *self {
            Value::I64(_) => Kind::I64,
            Value::F64(_) => Kind::F64,
            Value::Bool(_) => Kind::Bool,
            Value::String(_) => Kind::String,
            Value::Bytes(_) => Kind::Bytes,
            Value::List(_) => Kind::List,
            Value::Map(_) => Kind::Map,
            Value::Null => Kind::Null,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum Literal {
    I64(i64),
    F64(f64),
    Bool(bool),
    String(String),
    Bytes(Vec<u8>),
    List(Vec<Expression>),
    Map(Vec<(Expression, Expression)>),
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
    Map(HashMap<String, Value>),
    Null,
}

#[derive(Debug, Eq, PartialEq, Serialize, Clone)]
pub enum Error {
    NoMethod(Identifier),
    NoMethodOnType(Kind, Identifier),
    NoMethodWithSignature(Kind, Identifier, Vec<Kind>),
    InvalidTypeForOperator(Kind, Op),
    InvalidTypesForOperator(Kind, Kind, Op),
    DivisionByZero,
    NoSuchBinding(Identifier),
    NoSuchMember(Identifier),
    InvalidMapKey(Kind),
    DuplicateMapKey(String),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
pub struct Identifier(pub String);
impl Identifier {
    pub fn new(name: &str) -> Identifier {
        Identifier(name.to_owned())
    }
}

impl FromStr for Identifier {
    type Err = ();
    fn from_str(input: &str) -> Result<Identifier, ()> {
        Ok(Identifier::new(input))
    }
}

pub type EvalResult = Result<Value, Error>;
