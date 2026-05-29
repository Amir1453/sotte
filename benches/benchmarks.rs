use criterion::{criterion_group, criterion_main};

mod common;

mod larp;
use crate::larp::*;

criterion_group!(
    benches,
    // Build
    bvh_build::seq_cat,
    bvh_build::seq_lucky,
    bvh_build::seq_maria,
    bvh_build::par_cat,
    bvh_build::par_lucky,
    bvh_build::par_maria,
    // Traversal
    bvh_traversal::seq_cat,
    bvh_traversal::seq_lucky,
    bvh_traversal::seq_maria,
    bvh_traversal::par_cat,
    bvh_traversal::par_lucky,
    bvh_traversal::par_maria,
);
criterion_main!(benches);
