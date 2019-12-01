use crate::model::{Expression, Literal, Value};
use crate::stack::Operation;

pub fn linearize(e: Expression) -> Vec<Operation> {
    let mut walker = Walker::new();
    walker.walk(e);
    walker.0
}

struct Walker(Vec<Operation>);
impl Walker {
    fn new() -> Walker {
        Walker(Vec::new())
    }
    fn walk(&mut self, e: Expression) {
        match e {
            Expression::LetBinding { .. } => unimplemented!(),
            Expression::Ternary { .. } => unimplemented!(),
            Expression::Or(_) => unimplemented!(),
            Expression::And(_) => unimplemented!(),
            Expression::Eq(_, _) => unimplemented!(),
            Expression::Neq(_, _) => unimplemented!(),
            Expression::Lt(_, _) => unimplemented!(),
            Expression::Lte(_, _) => unimplemented!(),
            Expression::Gte(_, _) => unimplemented!(),
            Expression::Gt(_, _) => unimplemented!(),
            Expression::Add(a, b) => {
                self.0.push(Operation::Add);
                self.walk(*a);
                self.walk(*b);
            }
            Expression::Sub(a, b) => {
                self.0.push(Operation::Sub);
                self.walk(*a);
                self.walk(*b);
            }
            Expression::Mul(a, b) => {
                self.0.push(Operation::Mul);
                self.walk(*a);
                self.walk(*b);
            }
            Expression::Div(a, b) => {
                self.0.push(Operation::Div);
                self.walk(*a);
                self.walk(*b);
            }
            Expression::Mod(a, b) => {
                self.0.push(Operation::Mod);
                self.walk(*a);
                self.walk(*b);
            }
            Expression::Neg(a) => {
                self.0.push(Operation::Neg);
                self.walk(*a);
            }
            Expression::Not(a) => {
                self.0.push(Operation::Not);
                self.walk(*a);
            }
            Expression::Member(_, _) => unimplemented!(),
            Expression::Method(_, _, _) => unimplemented!(),
            Expression::Lit(lit) => self.walk_literal(lit),
            Expression::Binding(_) => unimplemented!(),
        }
    }

    fn walk_literal(&mut self, lit: Literal) {
        match lit {
            Literal::Null => self.0.push(Operation::Lit(Value::Null)),
            Literal::I64(v) => self.0.push(Operation::Lit(Value::I64(v))),
            Literal::F64(v) => self.0.push(Operation::Lit(Value::F64(v))),
            Literal::Bool(v) => self.0.push(Operation::Lit(Value::Bool(v))),
            Literal::String(v) => self.0.push(Operation::Lit(Value::String(v))),
            Literal::Bytes(v) => self.0.push(Operation::Lit(Value::Bytes(v))),
            Literal::List(vs) => {
                self.0.push(Operation::MakeList(vs.len()));
                for v in vs {
                    self.walk(v);
                }
            }
            Literal::Map(vs) => {
                self.0.push(Operation::MakeMap(vs.len()));
                for (k, v) in vs {
                    self.walk(k);
                    self.walk(v);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::parser::parse;

    use super::*;

    #[test]
    fn linearize_add() {
        let expr = parse(r#" 1 + 1 "#).unwrap();
        assert_eq!(
            linearize(expr),
            vec![
                Operation::Add,
                Operation::Lit(Value::I64(1)),
                Operation::Lit(Value::I64(1)),
            ]
        );
    }

    #[test]
    fn linearize_sub() {
        let expr = parse(r#" 1 - 2 "#).unwrap();
        assert_eq!(
            linearize(expr),
            vec![
                Operation::Sub,
                Operation::Lit(Value::I64(1)),
                Operation::Lit(Value::I64(2)),
            ]
        );
    }

    #[test]
    fn linearize_list() {
        let expr = parse(r#" [1, 2 + 3] "#).unwrap();
        assert_eq!(
            linearize(expr),
            vec![
                Operation::MakeList(2),
                Operation::Lit(Value::I64(1)),
                Operation::Add,
                Operation::Lit(Value::I64(2)),
                Operation::Lit(Value::I64(3)),
            ]
        );
    }

    #[test]
    fn linearize_list_empty() {
        let expr = parse(r#" [] "#).unwrap();
        assert_eq!(
            linearize(expr),
            vec![
                Operation::MakeList(0),
            ]
        );
    }
}
