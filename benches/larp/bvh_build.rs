use criterion::Criterion;
use std::hint::black_box;

use renderer::larp::Bvh;

use crate::common;

pub fn cat(c: &mut Criterion) {
    let mesh = common::cat().build_soup();

    let mut group = c.benchmark_group("BVH Build: Cat");

    group.bench_function("seq", |b| {
        b.iter(|| {
            black_box(Bvh::build_seq(black_box(&mesh)));
        });
    });

    group.bench_function("par", |b| {
        b.iter(|| {
            black_box(Bvh::build_par(black_box(&mesh)));
        });
    });

    group.finish();
}

pub fn lucky(c: &mut Criterion) {
    let mesh = common::lucky().build_soup();

    let mut group = c.benchmark_group("BVH Build: Lucky");

    group.bench_function("seq", |b| {
        b.iter(|| {
            black_box(Bvh::build_seq(black_box(&mesh)));
        });
    });

    group.bench_function("par", |b| {
        b.iter(|| {
            black_box(Bvh::build_par(black_box(&mesh)));
        });
    });

    group.finish();
}

pub fn maria(c: &mut Criterion) {
    let mesh = common::maria().build_soup();

    let mut group = c.benchmark_group("BVH Build: Maria");

    group.bench_function("seq", |b| {
        b.iter(|| {
            black_box(Bvh::build_seq(black_box(&mesh)));
        });
    });

    group.bench_function("par", |b| {
        b.iter(|| {
            black_box(Bvh::build_par(black_box(&mesh)));
        });
    });

    group.finish();
}
