#[macro_use]
extern crate criterion;
extern crate wasm_cel;

use criterion::black_box;
use criterion::Criterion;
use wasm_cel::interpreter;
use wasm_cel::model::*;
use wasm_cel::parser;

fn criterion_benchmark(c: &mut Criterion) {
    let input = parser::parse(
        r#"
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
    "#,
    )
    .unwrap();

    c.bench_function("interpret 20 lets", move |b| {
        b.iter(|| black_box(interpreter::EvalContext::default().evaluate(input.clone())))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
