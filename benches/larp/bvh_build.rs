use criterion::Criterion;
use std::hint::black_box;

use renderer::larp::{Bvh};

use crate::common;

// Sequential BVH Building benchmarks

pub fn seq_cat(c: &mut Criterion) {
    let mesh = common::cat().build_soup();

    c.bench_function("<Sequential BVH Build: Cat>", |b| {
        b.iter(|| {
            black_box(Bvh::build_seq(black_box(&mesh)));
        });
    });
}

pub fn seq_lucky(c: &mut Criterion) {
    let mesh = common::lucky().build_soup();

    c.bench_function("<Sequential BVH Build: Lucky>", |b| {
        b.iter(|| {
            black_box(Bvh::build_seq(black_box(&mesh)));
        });
    });
}

pub fn seq_maria(c: &mut Criterion) {
    let mesh = common::maria().build_soup();

    c.bench_function("<Sequential BVH Build: Maria>", |b| {
        b.iter(|| {
            black_box(Bvh::build_seq(black_box(&mesh)));
        });
    });
}

// Parallel BVH Building benchmarks

pub fn par_cat(c: &mut Criterion) {
    let mesh = common::cat().build_soup();

    c.bench_function("<Parallel BVH Build: Cat>", |b| {
        b.iter(|| {
            black_box(Bvh::build_par(black_box(&mesh)));
        });
    });
}

pub fn par_lucky(c: &mut Criterion) {
    let mesh = common::lucky().build_soup();

    c.bench_function("<Parallel BVH Build: Lucky>", |b| {
        b.iter(|| {
            black_box(Bvh::build_par(black_box(&mesh)));
        });
    });
}

pub fn par_maria(c: &mut Criterion) {
    let mesh = common::maria().build_soup();

    c.bench_function("<Parallel BVH Build: Maria>", |b| {
        b.iter(|| {
            black_box(Bvh::build_par(black_box(&mesh)));
        });
    });
}
