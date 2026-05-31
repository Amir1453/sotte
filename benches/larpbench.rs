use criterion::{criterion_group, criterion_main};

mod common;

mod larp;
use crate::larp::*;

criterion_group!(
    benches,
    // BVH Build
    bvh_build::cat,
    bvh_build::lucky,
    bvh_build::maria,
    // BVH Traversal
    bvh_traversal::cat,
    bvh_traversal::lucky,
    bvh_traversal::maria,
    // LBVH Build
    lbvh_build::cat,
    lbvh_build::lucky,
    lbvh_build::maria,
    // LBVH Traversal
    lbvh_traversal::cat,
    lbvh_traversal::lucky,
    lbvh_traversal::maria,
    // Cross Build
    cross::build_seq_bvh_lbvh,
    cross::build_par_bvh_lbvh,
    // Cross Traversal
    cross::traversal_seq_bvh_lbvh,
    cross::traversal_par_bvh_lbvh,
);
criterion_main!(benches);
