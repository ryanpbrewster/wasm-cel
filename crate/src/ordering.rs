use crate::model::Value;
use std::cmp::{Ordering, PartialOrd};

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
        match (self, other) {
            (Value::Null, Value::Null) => Some(Ordering::Equal),
            (Value::I64(a), Value::I64(b)) => Some(Ord::cmp(&a, &b)),
            (Value::F64(a), Value::F64(b)) => f64::partial_cmp(&a, &b),
            (Value::Bool(a), Value::Bool(b)) => Some(Ord::cmp(&a, &b)),
            (Value::String(ref a), Value::String(ref b)) => Some(Ord::cmp(&a, &b)),
            (Value::Bytes(ref a), Value::Bytes(ref b)) => Some(Ord::cmp(&a, &b)),
            _ => None,
        }
    }
}
