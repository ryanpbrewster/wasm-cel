#[macro_use]
extern crate criterion;
extern crate wasm_cel;

use criterion::black_box;
use criterion::Criterion;
use wasm_cel::interpreter;
use wasm_cel::parser;

fn benchmark_addition(c: &mut Criterion) {
    let lots_of_ones = r#"
    1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1
      + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1
      + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1
      + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1
      + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1
      + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1
      + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1
      + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1
      + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1
      + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1
      + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1
    "#;
    let lots_of_ones = parser::parse(lots_of_ones).unwrap();
    c.bench_function("lots of ones", move |b| {
        b.iter(|| black_box(interpreter::EvalContext::default().evaluate(lots_of_ones.clone())))
    });
}

fn benchmark_bindings(c: &mut Criterion) {
    let deep_lookup = r#"
        let n00 = 0;
        let n01 = 1 + n00;
        let n02 = 1 + n00;
        let n03 = 1 + n00;
        let n04 = 1 + n00;
        let n05 = 1 + n00;
        let n06 = 1 + n00;
        let n07 = 1 + n00;
        let n08 = 1 + n00;
        let n09 = 1 + n00;
        let n10 = 1 + n00;
        let n11 = 1 + n00;
        let n12 = 1 + n00;
        let n13 = 1 + n00;
        let n14 = 1 + n00;
        let n15 = 1 + n00;
        let n16 = 1 + n00;
        let n17 = 1 + n00;
        let n18 = 1 + n00;
        let n19 = 1 + n00;
        n00 + n01 + n02 + n03 + n04 + n05 + n06 + n07 + n08 + n09 +
        n10 + n11 + n12 + n13 + n14 + n15 + n16 + n17 + n18 + n19
    "#;
    let deep_lookup = parser::parse(deep_lookup).unwrap();

    let shallow_lookup = r#"
        let n00 = 0;
        let n01 = 1 + n00;
        let n02 = 1 + n01;
        let n03 = 1 + n02;
        let n04 = 1 + n03;
        let n05 = 1 + n04;
        let n06 = 1 + n05;
        let n07 = 1 + n06;
        let n08 = 1 + n07;
        let n09 = 1 + n08;
        let n10 = 1 + n09;
        let n11 = 1 + n10;
        let n12 = 1 + n11;
        let n13 = 1 + n12;
        let n14 = 1 + n13;
        let n15 = 1 + n14;
        let n16 = 1 + n15;
        let n17 = 1 + n16;
        let n18 = 1 + n17;
        let n19 = 1 + n18;
        n00 + n01 + n02 + n03 + n04 + n05 + n06 + n07 + n08 + n09 +
        n10 + n11 + n12 + n13 + n14 + n15 + n16 + n17 + n18 + n19
    "#;
    let shallow_lookup = parser::parse(shallow_lookup).unwrap();

    c.bench_function("20 shallow lets", move |b| {
        b.iter(|| black_box(interpreter::EvalContext::default().evaluate(shallow_lookup.clone())))
    });
    c.bench_function("20 deep lets", move |b| {
        b.iter(|| black_box(interpreter::EvalContext::default().evaluate(deep_lookup.clone())))
    });
}

criterion_group!(benches, benchmark_addition, benchmark_bindings);
criterion_main!(benches);
