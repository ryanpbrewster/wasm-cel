use serde::Serialize;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum Expression {
    LetBinding {
        id: Identifier,
        value: Box<Expression>,
        body: Box<Expression>,
    },
    Ternary {
        condition: Box<Expression>,
        true_branch: Box<Expression>,
        else_branch: Box<Expression>,
    },
    Or(Vec<Expression>),
    And(Vec<Expression>),
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
            Expression::Ternary { .. } => Op::Ternary,
            Expression::LetBinding { .. } => Op::LetBinding,
            Expression::Or(_) => Op::Or,
            Expression::And(_) => Op::And,
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
    LetBinding,
    Ternary,
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

    pub fn size(&self) -> usize {
        let transitive = match self {
            Value::I64(_) => 0,
            Value::F64(_) => 0,
            Value::Bool(_) => 0,
            Value::String(s) => s.len(),
            Value::Bytes(b) => b.len(),
            Value::Null => 0,
            Value::List(children) => children.iter().map(|v| v.size()).sum(),
            Value::Map(children) => children.iter().map(|(k, v)| k.len() + v.size()).sum(),
        };
        std::mem::size_of_val(self) + transitive
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
#[serde(tag = "t", content = "c")]
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
    EvaluationTooLarge,
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sizeof_primitive() {
        assert_eq!(Value::Null.size(), 64);
        assert_eq!(Value::Bool(true).size(), 64);
        assert_eq!(Value::I64(42).size(), 64);
        assert_eq!(Value::F64(2.78).size(), 64);
    }

    #[test]
    fn sizeof_string() {
        let v = Value::String("asdf".to_owned());
        assert_eq!(v.size(), 64 + 4);
    }

    #[test]
    fn sizeof_bytes() {
        let v = Value::Bytes("asdf".as_bytes().to_owned());
        assert_eq!(v.size(), 64 + 4);
    }

    #[test]
    fn sizeof_list() {
        let v = Value::List(vec![
            Value::String("asdf".to_owned()),
            Value::Null,
            Value::List(vec![]),
        ]);
        assert_eq!(v.size(), 4 * 64 + 4);
    }

    #[test]
    fn sizeof_map() {
        let v = Value::Map(vec![("a".to_owned(), Value::Null)].into_iter().collect());
        assert_eq!(v.size(), 2 * 64 + 1);
    }
}
