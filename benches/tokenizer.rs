use criterion::{criterion_group, criterion_main, Criterion};
use flit::core::tokenizer::tokenize;

fn bench_tokenizer(c: &mut Criterion) {
    let text = "The quick brown fox jumps over the lazy dog. ".repeat(1000);
    c.bench_function("tokenize_1000_sentences", |b| {
        b.iter(|| tokenize(std::iter::once(text.clone())))
    });
}

criterion_group!(benches, bench_tokenizer);
criterion_main!(benches);
