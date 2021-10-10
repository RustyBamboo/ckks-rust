use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use num_bigint::ToBigInt;
use rlwe::*;

fn keygen_benchmark(c: &mut Criterion) {
    let ciph_modulus = 1.to_bigint().unwrap() << 600;

    let mut group = c.benchmark_group("keygen");
    for i in 1..13 {
        let poly_degree = 1 << i;
        group.bench_with_input(
            BenchmarkId::from_parameter(poly_degree),
            &poly_degree,
            |b, &p| {
                b.iter(|| Rwle::keygen(&ciph_modulus, p as usize, p as usize));
            },
        );
    }
    group.finish()
}

fn keygen_relin_benchmark(c: &mut Criterion) {
    let ciph_modulus = 1.to_bigint().unwrap() << 600;
    let big_modulus = 1.to_bigint().unwrap() << 1200;

    let mut group = c.benchmark_group("keygen_relin");
    for i in 1..13 {
        let poly_degree = 1 << i;
        let key = Rwle::keygen(&ciph_modulus, poly_degree as usize, poly_degree as usize);

        group.bench_with_input(BenchmarkId::from_parameter(poly_degree), &key, |b, k| {
            b.iter(|| k.relin_key(&big_modulus));
        });
    }
    group.finish()
}

fn encoder_creation_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("encoder_creation");
    for i in 1..13 {
        let poly_degree = 1 << i;
        group.bench_with_input(
            BenchmarkId::from_parameter(poly_degree),
            &poly_degree,
            |b, &p| {
                b.iter(|| encoder::CKKSEncoder::new(p as usize * 2));
            },
        );
    }
    group.finish()
}

fn encoder_benchmark(c: &mut Criterion) {
    let scaling_factor = 1_usize << 30;

    let mut group = c.benchmark_group("encoder");
    for i in 1..13 {
        let poly_degree = 1 << i;
        let encoder = encoder::CKKSEncoder::new(poly_degree as usize * 2);
        let msg = vec![0f64; poly_degree >> 1];
        group.bench_with_input(BenchmarkId::from_parameter(poly_degree), &msg, |b, m| {
            b.iter(|| encode(&m, scaling_factor, &encoder));
        });
    }
    group.finish()
}

fn encryption_benchmark(c: &mut Criterion) {
    let scaling_factor = 1_usize << 30;
    let ciph_modulus = 1.to_bigint().unwrap() << 600;

    let mut group = c.benchmark_group("encryption");
    for i in 1..13 {
        let poly_degree = 1 << i;
        let key = Rwle::keygen(&ciph_modulus, poly_degree as usize, poly_degree as usize);
        let encoder = encoder::CKKSEncoder::new(poly_degree as usize * 2);
        let msg = vec![0f64; poly_degree >> 1];
        let plain = encode(&msg, scaling_factor, &encoder);
        group.bench_with_input(BenchmarkId::from_parameter(poly_degree), &plain, |b, p| {
            b.iter(|| encrypt(&key.public(), &ciph_modulus, &p));
        });
    }
    group.finish()
}

fn addition_benchmark(c: &mut Criterion) {
    let ciph_modulus = 1.to_bigint().unwrap() << 600;
    let scaling_factor = 1_usize << 30;

    let mut group = c.benchmark_group("addition");
    for i in 1..13 {
        let poly_degree = 1 << i;
        let key = Rwle::keygen(&ciph_modulus, poly_degree as usize, poly_degree as usize);
        let encoder = encoder::CKKSEncoder::new(poly_degree as usize * 2);
        let msg = vec![0f64; poly_degree >> 1];
        let plain = encode(&msg, scaling_factor, &encoder);
        let cipher = encrypt(&key.public(), &ciph_modulus, &plain);
        group.bench_function(BenchmarkId::from_parameter(poly_degree), |b| {
            b.iter(|| &cipher + &cipher)
        });
    }
    group.finish()
}

fn multiplication_benchmark(c: &mut Criterion) {
    let ciph_modulus = 1.to_bigint().unwrap() << 600;
    let scaling_factor = 1_usize << 30;
    let big_modulus = 1.to_bigint().unwrap() << 1200;

    let mut group = c.benchmark_group("multiplication");
    group.sample_size(10);
    for i in 1..13 {
        let poly_degree = 1 << i;
        let key = Rwle::keygen(&ciph_modulus, poly_degree as usize, poly_degree as usize);
        let relin_key = key.relin_key(&big_modulus);
        let encoder = encoder::CKKSEncoder::new(poly_degree as usize * 2);
        let msg = vec![0f64; poly_degree >> 1];
        let plain = encode(&msg, scaling_factor, &encoder);
        let cipher = encrypt(&key.public(), &ciph_modulus, &plain);
        group.bench_function(BenchmarkId::new("Multiplication", poly_degree), |b| {
            b.iter(|| &cipher * &cipher)
        });
        group.bench_function(
            BenchmarkId::new("Multiplication w/ Relin", poly_degree),
            |b| b.iter(|| (&cipher * &cipher).relin(&relin_key, &big_modulus)),
        );
    }
    group.finish()
}

criterion_group!(
    benches,
    keygen_benchmark,
    keygen_relin_benchmark,
    encoder_creation_benchmark,
    encoder_benchmark,
    encryption_benchmark,
    addition_benchmark,
    multiplication_benchmark
);
criterion_main!(benches);
