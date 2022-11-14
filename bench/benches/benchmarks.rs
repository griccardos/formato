use criterion::{black_box, criterion_group, criterion_main, Criterion};
use formato::Formato;

pub fn criterion_benchmark(c: &mut Criterion) {
    asserts();

    let mut gr = c.benchmark_group("std format");
    gr.bench_function("usize small no decimal", |b| b.iter(|| format_usize(black_box(20))));
    gr.bench_function("usize small decimal", |b| b.iter(|| format_usize_d(black_box(20))));
    gr.bench_function("usize big no decimal", |b| b.iter(|| format_usize(black_box(1000000))));
    gr.bench_function("usize big decimal", |b| b.iter(|| format_usize_d(black_box(1000000))));
    gr.bench_function("f64 small no decimal", |b| b.iter(|| format(black_box(20.))));
    gr.bench_function("f64 small decimal", |b| b.iter(|| format_d(black_box(20.))));
    gr.bench_function("f64 big no decimal", |b| b.iter(|| format(black_box(1000000.))));
    gr.bench_function("f64 big decimal", |b| b.iter(|| format_d(black_box(1000000.))));
    gr.bench_function("no round", |b| b.iter(|| format_no_round(black_box(9999.999))));
    gr.bench_function("round", |b| b.iter(|| format_d(black_box(9999.999))));
    gr.finish();

    let mut gr = c.benchmark_group("formato");
    gr.bench_function("usize small no decimal", |b| b.iter(|| f_usize(black_box(20), black_box("0"))));
    gr.bench_function("usize small decimal", |b| b.iter(|| f_usize(black_box(20), black_box("0.0"))));
    gr.bench_function("usize big no decimal", |b| b.iter(|| f_usize(black_box(1000000), black_box("#,###"))));
    gr.bench_function("usize big decimal", |b| b.iter(|| f_usize(black_box(1000000), black_box("#,###"))));
    gr.bench_function("f64 small no decimal", |b| b.iter(|| f_f64(black_box(20.), black_box("0"))));
    gr.bench_function("f64 small decimal", |b| b.iter(|| f_f64(black_box(20.), black_box("0.0"))));
    gr.bench_function("f64 big no decimal", |b| b.iter(|| f_f64(black_box(1000000.), black_box("#,###"))));
    gr.bench_function("f64 big decimal", |b| b.iter(|| f_f64(black_box(1000000.), black_box("#,###.0"))));
    gr.bench_function("no round", |b| b.iter(|| f_f64(black_box(9999.999), black_box("#,###.000"))));
    gr.bench_function("round", |b| b.iter(|| f_f64(black_box(9999.999), black_box("#,###.00"))));
    gr.finish();
}

fn asserts() {
    assert_eq!(format_usize(1), f_usize(1, "0"));
    assert_eq!(format_usize_d(1), f_usize(1, "0.0"));

    assert_eq!(format(1.1), f_f64(1.1, "0"));
    assert_eq!(format_d(1.1), f_f64(1.1, "0.0"));

    assert_eq!(format_no_round(9999.999), f_f64(9999.999, "0.000"));
    assert_eq!(format_d(9999.999), f_f64(9999.999, "0.00"));
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

fn format_usize_d(i: usize) {
    format!("{i:.1}");
}
fn format_usize(i: usize) {
    format!("{i}");
}
fn format_d(i: f64) {
    format!("{i:.1}");
}
fn format(i: f64) {
    format!("{i:.0}");
}
fn format_no_round(i: f64) {
    format!("{i}");
}

fn f_usize(i: usize, format: &str) {
    i.formato(format);
}

fn f_f64(i: f64, format: &str) {
    i.formato(format);
}
