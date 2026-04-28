#![allow(non_snake_case)]
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use two_round_fully_adaptive::helpers::{next_pow2, rand_scalar};
use two_round_fully_adaptive::polynomial::{Params, Polynomial};

fn bench_eval_vs_ro_eval(c: &mut Criterion) {
    let mut group = c.benchmark_group("poly_eval_functions");

    // degrees = 2^4, 2^6, ..., 2^16
    for exp in (4..=16).step_by(1) {
        let degree = 1usize << exp;
        let n = degree + 1;
        let m = next_pow2(n);

        let pp = Params::setup(m, "PolyEvalFns");
        let poly = Polynomial::random(degree, &pp);
        let z = rand_scalar();
        let key = rand_scalar();

        group.bench_with_input(
            BenchmarkId::new("eval_stored", degree),
            &degree,
            |b, &_deg| {
                b.iter(|| {
                    let y = poly.eval(&black_box(z));
                    black_box(y);
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("eval_ro_chacha", degree),
            &degree,
            |b, &_deg| {
                b.iter(|| {
                    let y = Polynomial::eval_with_prf_chacha8(&black_box(z), degree, &key);
                    black_box(y);
                });
            },
        );

        group.bench_with_input(BenchmarkId::new("eval_ro", degree), &degree, |b, &_deg| {
            b.iter(|| {
                let y = Polynomial::eval_with_ro(&black_box(z), degree, &key);
                black_box(y);
            });
        });
    }

    group.finish();
}

criterion_group!(benches, bench_eval_vs_ro_eval);
criterion_main!(benches);
