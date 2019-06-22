use crate::model::{Expression, Identifier, Literal};

use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

use std::convert::TryFrom;

#[derive(Parser)]
#[grammar = "cel.pest"]
struct CelParser;

pub fn parse(input: &str) -> Result<Expression, String> {
    let mut parsed =
        CelParser::parse(Rule::Expression, input).map_err(|err| format!("{:?}", err))?;
    Ok(extract_expression(parsed.next().unwrap()))
}

fn extract_expression(pair: Pair<Rule>) -> Expression {
    assert_eq!(pair.as_rule(), Rule::Expression);
    let mut pairs = pair.into_inner();

    let mut bindings = vec![];
    let body = loop {
        let p = pairs.next().unwrap();
        match p.as_rule() {
            Rule::LetBinding => {
                bindings.push(extract_binding(p));
            }
            _ => break extract_disjunction(p),
        }
    };

    bindings
        .into_iter()
        .rfold(body, |expr, (id, value)| Expression::LetBinding {
            id,
            value: Box::new(value),
            body: Box::new(expr),
        })
}

fn extract_binding(pair: Pair<Rule>) -> (Identifier, Expression) {
    assert_eq!(pair.as_rule(), Rule::LetBinding);
    let mut pairs = pair.into_inner();
    let id = extract_identifier(pairs.next().unwrap());
    let value = extract_disjunction(pairs.next().unwrap());
    (id, value)
}

fn extract_disjunction(pair: Pair<Rule>) -> Expression {
    assert_eq!(pair.as_rule(), Rule::Disjunction);
    let mut exprs: Vec<Expression> = pair.into_inner().map(extract_conjunction).collect();
    if exprs.len() == 1 {
        exprs.swap_remove(0)
    } else {
        Expression::Or(exprs)
    }
}

fn extract_conjunction(pair: Pair<Rule>) -> Expression {
    assert_eq!(pair.as_rule(), Rule::Conjunction);
    let mut exprs: Vec<Expression> = pair.into_inner().map(extract_relation).collect();
    if exprs.len() == 1 {
        exprs.swap_remove(0)
    } else {
        Expression::And(exprs)
    }
}

fn extract_relation(pair: Pair<Rule>) -> Expression {
    assert_eq!(pair.as_rule(), Rule::Relation);
    let mut pairs = pair.into_inner();
    let a = extract_addition(pairs.next().unwrap());
    match pairs.next() {
        None => a,
        Some(op) => {
            assert_eq!(op.as_rule(), Rule::RelOp);
            let b = extract_addition(pairs.next().unwrap());
            match op.as_str() {
                "==" => Expression::Eq(Box::new(a), Box::new(b)),
                "!=" => Expression::Neq(Box::new(a), Box::new(b)),
                "<" => Expression::Lt(Box::new(a), Box::new(b)),
                "<=" => Expression::Lte(Box::new(a), Box::new(b)),
                ">=" => Expression::Gte(Box::new(a), Box::new(b)),
                ">" => Expression::Gt(Box::new(a), Box::new(b)),
                _ => unreachable!(),
            }
        }
    }
}

fn extract_addition(pair: Pair<Rule>) -> Expression {
    assert_eq!(pair.as_rule(), Rule::Addition);
    let mut pairs = pair.into_inner();
    let mut a = extract_multiplication(pairs.next().unwrap());
    while let Some(op) = pairs.next() {
        assert_eq!(op.as_rule(), Rule::AddOp);
        let b = extract_multiplication(pairs.next().unwrap());
        a = match op.as_str() {
            "+" => Expression::Add(Box::new(a), Box::new(b)),
            "-" => Expression::Sub(Box::new(a), Box::new(b)),
            _ => unreachable!(),
        }
    }
    a
}

fn extract_multiplication(pair: Pair<Rule>) -> Expression {
    assert_eq!(pair.as_rule(), Rule::Multiplication);
    let mut pairs = pair.into_inner();
    let mut a = extract_unary(pairs.next().unwrap());
    while let Some(op) = pairs.next() {
        assert_eq!(op.as_rule(), Rule::MulOp);
        let b = extract_unary(pairs.next().unwrap());
        a = match op.as_str() {
            "*" => Expression::Mul(Box::new(a), Box::new(b)),
            "/" => Expression::Div(Box::new(a), Box::new(b)),
            "%" => Expression::Mod(Box::new(a), Box::new(b)),
            _ => unreachable!(),
        }
    }
    a
}

fn extract_unary(pair: Pair<Rule>) -> Expression {
    assert_eq!(pair.as_rule(), Rule::Unary);
    let mut pairs = pair.into_inner();
    let a = pairs.next().unwrap();
    match a.as_rule() {
        Rule::Member => extract_member(a),
        Rule::UnaryOp => {
            assert_eq!(a.as_rule(), Rule::UnaryOp);
            match a.as_str() {
                "-" => Expression::Neg(Box::new(extract_unary(pairs.next().unwrap()))),
                "!" => Expression::Not(Box::new(extract_unary(pairs.next().unwrap()))),
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}

fn extract_member(pair: Pair<Rule>) -> Expression {
    assert_eq!(pair.as_rule(), Rule::Member);
    let mut pairs = pair.into_inner();
    let mut a = extract_operand(pairs.next().unwrap());

    while let Some(pair) = pairs.next() {
        match pair.as_rule() {
            Rule::MethodCall => {
                let (id, args) = extract_method_call(pair);
                a = Expression::Method(Box::new(a), id, args);
            }
            Rule::MemberRef => {
                let id = extract_member_ref(pair);
                a = Expression::Member(Box::new(a), id);
            }
            _ => unreachable!(),
        };
    }

    a
}

fn extract_operand(pair: Pair<Rule>) -> Expression {
    assert_eq!(pair.as_rule(), Rule::Operand);
    let a = pair.into_inner().next().unwrap();
    match a.as_rule() {
        Rule::Literal => Expression::Lit(extract_literal(a)),
        Rule::Identifier => Expression::Binding(extract_identifier(a)),
        Rule::Relation => extract_relation(a),
        _ => unreachable!(),
    }
}

fn extract_method_call(pair: Pair<Rule>) -> (Identifier, Vec<Expression>) {
    assert_eq!(pair.as_rule(), Rule::MethodCall);
    let mut pairs = pair.into_inner();
    (
        extract_identifier(pairs.next().unwrap()),
        extract_args(pairs.next().unwrap()),
    )
}

fn extract_member_ref(pair: Pair<Rule>) -> Identifier {
    assert_eq!(pair.as_rule(), Rule::MemberRef);
    extract_identifier(pair.into_inner().next().unwrap())
}

fn extract_identifier(pair: Pair<Rule>) -> Identifier {
    assert_eq!(pair.as_rule(), Rule::Identifier);
    pair.as_str().parse().expect("parse identifier")
}

fn extract_args(pair: Pair<Rule>) -> Vec<Expression> {
    assert_eq!(pair.as_rule(), Rule::Args);
    pair.into_inner().map(extract_relation).collect()
}

fn extract_literal(pair: Pair<Rule>) -> Literal {
    assert_eq!(pair.as_rule(), Rule::Literal);
    let pair = pair.into_inner().next().unwrap();
    match pair.as_rule() {
        Rule::StringLiteral => Literal::String(extract_string(pair)),
        Rule::BytesLiteral => Literal::Bytes(extract_bytes(pair)),
        Rule::FloatLiteral => Literal::F64(pair.as_str().parse().unwrap()),
        Rule::IntLiteral => Literal::I64(pair.as_str().parse().unwrap()),
        Rule::ListLiteral => extract_list(pair),
        Rule::MapLiteral => extract_map(pair),
        Rule::BoolLiteral => Literal::Bool(pair.as_str().parse().unwrap()),
        Rule::NullLiteral => Literal::Null,
        _ => unreachable!(),
    }
}

fn extract_string(pair: Pair<Rule>) -> String {
    assert_eq!(pair.as_rule(), Rule::StringLiteral);
    let mut buf = String::new();
    for p in pair.into_inner() {
        match unescape_sequence(&p) {
            Unescaped::Byte(b) => buf.push(b as char),
            Unescaped::Unicode(ch) => buf.push(ch),
        };
    }
    buf
}

fn extract_bytes(pair: Pair<Rule>) -> Vec<u8> {
    assert_eq!(pair.as_rule(), Rule::BytesLiteral);
    let mut buf = Vec::new();
    for p in pair.into_inner() {
        match unescape_sequence(&p) {
            Unescaped::Byte(b) => buf.push(b),
            Unescaped::Unicode(ch) => buf.extend_from_slice(ch.encode_utf8(&mut [0; 4]).as_bytes()),
        };
    }
    buf
}

enum Unescaped {
    Byte(u8),
    Unicode(char),
}
fn unescape_sequence(pair: &Pair<Rule>) -> Unescaped {
    match pair.as_rule() {
        Rule::CharLiteral => Unescaped::Unicode(pair.as_str().chars().next().unwrap()),
        Rule::Escape => {
            let s = &pair.as_str()[1..];
            match &s[..1] {
                "t" => Unescaped::Byte(b'\t'),
                "n" => Unescaped::Byte(b'\n'),
                "\"" => Unescaped::Byte(b'"'),
                "x" => Unescaped::Byte(u8::from_str_radix(&s[1..], 16).unwrap()),
                "u" => Unescaped::Unicode(
                    char::try_from(u32::from_str_radix(&s[1..], 16).unwrap()).unwrap(),
                ),
                "0" | "1" | "2" | "3" => Unescaped::Byte(u8::from_str_radix(s, 8).unwrap()),
                _ => unreachable!("unexpected string literal {}", s),
            }
        }
        _ => unreachable!(),
    }
}

fn extract_list(pair: Pair<Rule>) -> Literal {
    assert_eq!(pair.as_rule(), Rule::ListLiteral);
    let mut vs = Vec::new();
    for p in pair.into_inner() {
        vs.push(extract_disjunction(p));
    }
    Literal::List(vs)
}

fn extract_map(pair: Pair<Rule>) -> Literal {
    assert_eq!(pair.as_rule(), Rule::MapLiteral);
    let mut fields = Vec::new();
    for p in pair.into_inner() {
        fields.push(extract_map_field(p));
    }
    Literal::Map(fields)
}

fn extract_map_field(pair: Pair<Rule>) -> (Expression, Expression) {
    assert_eq!(pair.as_rule(), Rule::MapField);
    let mut pairs = pair.into_inner();
    (
        extract_disjunction(pairs.next().unwrap()),
        extract_disjunction(pairs.next().unwrap()),
    )
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::model::Literal;
    use std::any::Any;

    fn assert_valid(input: &str) {
        parse(input).expect("failed to parse");
    }

    fn assert_invalid(input: &str) {
        let parsed = parse(input);
        assert!(parsed.is_err(), "{} was accepted as {:?}", input, parsed);
    }

    fn literal(x: &dyn Any) -> Expression {
        if let Some(&s) = x.downcast_ref::<&str>() {
            return Expression::Lit(Literal::String(String::from(s)));
        }
        if let Some(&b) = x.downcast_ref::<&[u8]>() {
            return Expression::Lit(Literal::Bytes(b.to_vec()));
        }
        unimplemented!("literal of type {:?}", x.type_id())
    }

    #[test]
    fn cel_valid() {
        assert_valid("22 * (4 + 15)");
        assert_valid("22 * -4");
        assert_valid("!false");
    }

    #[test]
    fn valid_floats() {
        assert_valid("3.1415926");
        assert_invalid(".1415926");
        assert_invalid("3.");
    }

    #[test]
    fn null_literal() {
        assert_valid("null");
    }

    #[test]
    fn int_literals() {
        assert_valid("1");
        assert_valid("31415");
    }

    #[test]
    #[should_panic] // TODO: return an error instead of panicking
    fn int_literal_overflow() {
        let _ = parse("9999999999999999999999999");
    }

    #[test]
    fn cel_smoke() {
        let input = "22 * (4 + 15)";
        assert_eq!(
            parse(input),
            Ok(Expression::Mul(
                Box::new(Expression::Lit(Literal::I64(22))),
                Box::new(Expression::Add(
                    Box::new(Expression::Lit(Literal::I64(4))),
                    Box::new(Expression::Lit(Literal::I64(15))),
                ))
            ))
        );
    }

    #[test]
    fn cel_list() {
        let input = "[0, false || true]";
        assert_eq!(
            parse(input),
            Ok(Expression::Lit(Literal::List(vec![
                Expression::Lit(Literal::I64(0)),
                Expression::Or(vec![
                    Expression::Lit(Literal::Bool(false)),
                    Expression::Lit(Literal::Bool(true)),
                ])
            ])))
        );
    }

    #[test]
    fn valid_list() {
        assert_valid(r#" [] "#); // empty
        assert_valid(r#" [x, y, z] "#); // bindings
        assert_valid(r#" [ "a", "b", "c",  ] "#); // trailing comma
        assert_valid(r#" [[[[[0]]]]] "#); // nested
    }

    #[test]
    fn valid_maps() {
        assert_valid(r#" { "foo": "bar" } "#); // string literals
        assert_valid(r#" { foo: bar } "#); // bindings
        assert_valid(r#" { a: b, c: d, e: f } "#); // multiple fields
        assert_valid(r#" {} "#); // empty
        assert_valid(
            r#" {
          "a": "b",
          "p": "q"
        } "#,
        ); // multi-line
        assert_valid(
            r#" {
          "a": "b",
          "p": "q",
        } "#,
        ); // trailing comma
    }

    #[test]
    fn cel_string() {
        let input = r#""asdf""#;
        assert_eq!(
            parse(input),
            Ok(Expression::Lit(Literal::String(String::from("asdf"))))
        );
    }

    #[test]
    fn valid_string_literals() {
        assert_valid(r#" "asdf" "#);
        assert_valid(r#" 'asdf' "#);
        assert_valid(r#" '¢' "#);
    }

    #[test]
    fn cel_escaped_quote_string() {
        let input = r#""as\"df""#;
        assert_eq!(
            parse(input),
            Ok(Expression::Lit(Literal::String(String::from("as\"df"))))
        );
    }

    #[test]
    fn invalid_octal_escapes() {
        assert_invalid(r#" "\0" "#);
        assert_invalid(r#" "\7" "#);
        assert_invalid(r#" "\07" "#);
        assert_invalid(r#" "\77" "#);
        assert_invalid(r#" "\8" "#);
        assert_invalid(r#" "\378" "#);
        assert_invalid(r#" "\400" "#);
    }

    #[test]
    fn valid_octal_escapes() {
        assert_eq!(parse(r#" "\000" "#).unwrap(), literal(&"\u{0000}"));
        assert_eq!(parse(r#" "\007" "#).unwrap(), literal(&"\u{0007}"));
        assert_eq!(parse(r#" "\377" "#).unwrap(), literal(&"\u{00FF}"));
    }

    #[test]
    fn valid_hex_escapes() {
        assert_eq!(parse(r#" "\x00" "#).unwrap(), literal(&"\u{0000}"));
        assert_eq!(parse(r#" "\xFF" "#).unwrap(), literal(&"\u{00FF}"));
    }

    #[test]
    fn valid_unicode_escapes() {
        assert_eq!(parse(r#" "\u0000" "#).unwrap(), literal(&"\u{0000}"));
        assert_eq!(parse(r#" "\u00FF" "#).unwrap(), literal(&"\u{00FF}"));
        assert_eq!(parse(r#" "\uFF00" "#).unwrap(), literal(&"\u{FF00}"));
        assert_eq!(parse(r#" "\uFFFF" "#).unwrap(), literal(&"\u{FFFF}"));
    }

    #[test]
    fn valid_bytes() {
        assert_eq!(parse(r#" b"asdf" "#).unwrap(), literal(&"asdf".as_bytes()));
    }

    #[test]
    fn method_call() {
        assert_valid(r#" [1, 2, 3].len() "#);
        assert_valid(r#" 42.pow(42) "#);
        assert_valid(r#" ([1] + [2]).len() "#);
        assert_valid(r#" ([1] + [2]).len().pow(2) "#);
        assert_valid(r#" ([1] + [2]).foo.bar.baz(1,2,3).length.asdf("asdf") "#);
    }

    #[test]
    fn member_access() {
        assert_valid(r#" foo "#);
        assert_valid(r#" foo.bar.baz "#);
    }

    #[test]
    fn multi_conjunction() {
        assert_valid(r#" true && true && true && true "#);
    }

    #[test]
    fn multi_disjunction() {
        assert_valid(r#" true || true || true || true "#);
    }

    #[test]
    fn relations() {
        assert_valid(r#" 0 < 1 && 1 <= 2 && 3 == 3 && 4 >= 3 && 4 > 3 && 0 != 0 "#);
    }

    #[test]
    fn let_binding_smoke() {
        assert_valid(r#" let x = 42; x "#);
    }

    #[test]
    fn let_binding_no_expression() {
        assert_invalid(r#" let x = 42; "#);
    }

    #[test]
    fn let_rebinding() {
        assert_valid(r#" let x = 42; let x = x*x; x "#);
    }
}
