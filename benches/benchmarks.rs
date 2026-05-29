use criterion::{criterion_group, criterion_main};

mod common;

mod larp;
use crate::larp::*;

criterion_group!(
    benches,
    bvh_build::seq_cat,
    bvh_build::seq_lucky,
    bvh_build::seq_maria,
    bvh_build::par_cat,
    bvh_build::par_lucky,
    bvh_build::par_maria,
);
criterion_main!(benches);
