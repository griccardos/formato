use criterion::{black_box, criterion_group, criterion_main, Criterion};
use formato::Formato;

pub fn criterion_benchmark(c: &mut Criterion) {
    //float thousands
    let mut gr = c.benchmark_group("std");
    gr.bench_function("usize small no decimal", |b| b.iter(|| tostring(black_box(20))));
    gr.bench_function("usize small decimal", |b| b.iter(|| format_usize(black_box(20))));
    gr.bench_function("usize big no decimal", |b| b.iter(|| tostring(black_box(1000000))));
    gr.bench_function("usize big decimal", |b| b.iter(|| format_usize(black_box(1000000))));
    gr.bench_function("f64 small no decimal", |b| b.iter(|| tostringf(black_box(20.))));
    gr.bench_function("f64 small decimal", |b| b.iter(|| format(black_box(20.))));
    gr.bench_function("f64 big no decimal", |b| b.iter(|| tostringf(black_box(1000000.))));
    gr.bench_function("f64 big decimal", |b| b.iter(|| format(black_box(1000000.))));
    gr.bench_function("no round", |b| b.iter(|| format_no_round(black_box(9999.999))));
    gr.bench_function("round", |b| b.iter(|| format(black_box(9999.999))));
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
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

fn tostring(i: usize) {
    i.to_string();
}

fn format_usize(i: usize) {
    format!("{i:.1}");
}
fn format(i: f64) {
    format!("{i:.1}");
}
fn format_no_round(i: f64) {
    format!("{i}");
}

fn tostringf(i: f64) {
    i.to_string();
}

fn f_usize(i: usize, format: &str) {
    i.formato(format);
}

fn f_f64(i: f64, format: &str) {
    i.formato(format);
}
