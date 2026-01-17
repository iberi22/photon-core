use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use photon_core::{encode_data, decode_data};

pub fn benchmark_encoding(c: &mut Criterion) {
    let data = vec![0xAB; 1000]; // 1KB of data
    c.bench_function("encode_1kb", |b| b.iter(|| encode_data(black_box(&data))));
}

pub fn benchmark_decoding(c: &mut Criterion) {
    let data = vec![0xAB; 1000];
    let voxels = encode_data(&data);
    c.bench_function("decode_1kb", |b| b.iter(|| decode_data(black_box(&voxels), false)));
}

pub fn benchmark_decoding_with_noise(c: &mut Criterion) {
    let data = vec![0xAB; 1000];
    let voxels = encode_data(&data);
    c.bench_function("decode_1kb_noise", |b| b.iter(|| decode_data(black_box(&voxels), true)));
}

criterion_group!(benches, benchmark_encoding, benchmark_decoding, benchmark_decoding_with_noise);
criterion_main!(benches);
