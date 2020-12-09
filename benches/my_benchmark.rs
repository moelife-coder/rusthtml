use criterion::{criterion_group, criterion_main, Criterion, black_box};
use rusthtml;

pub fn criterion_benchmark(c: &mut Criterion) {
    let example_source = black_box(include_str!("example.html"));
    let example_full_source = black_box(include_str!("full_example.html"));
    let habrahabr = black_box(include_str!("page_habrahabr-70330.html"));
    c.bench_function("example", |b| b.iter(|| rusthtml::ElementContent::parse(rusthtml::HtmlTag::parse(example_source)).unwrap()));
    c.bench_function("full example", |b| b.iter(|| rusthtml::ElementContent::parse(rusthtml::HtmlTag::parse(example_full_source)).unwrap()));
    c.bench_function("habrahabr-70330", |b| b.iter(|| rusthtml::ElementContent::parse(rusthtml::HtmlTag::parse(habrahabr)).unwrap()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

