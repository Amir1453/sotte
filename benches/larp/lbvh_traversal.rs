use criterion::Criterion;
use std::hint::black_box;

use renderer::larp::{Boundable, LinearBvh};

use crate::common;

// Sequential Traversal benchmarks

const RAY_COUNT: usize = 100_000;

pub fn seq_cat(c: &mut Criterion) {
    let mesh = common::cat().build_soup();
    let bvh = LinearBvh::build_seq(&mesh);
    let root_bbox = bvh.bounding_box();

    let rays = common::rays_into_bbox(root_bbox, RAY_COUNT);

    c.bench_function("<Sequential LBVH Traversal: Cat>", |b| {
        b.iter(|| {
            for ray in &rays {
                black_box(bvh.intersect_ray(ray, f64::EPSILON, f64::INFINITY));
            }
        });
    });
}

pub fn seq_lucky(c: &mut Criterion) {
    let mesh = common::lucky().build_soup();
    let bvh = LinearBvh::build_seq(&mesh);
    let root_bbox = bvh.bounding_box();

    let rays = common::rays_into_bbox(root_bbox, RAY_COUNT);

    c.bench_function("<Sequential LBVH Traversal: Lucky>", |b| {
        b.iter(|| {
            for ray in &rays {
                black_box(bvh.intersect_ray(ray, f64::EPSILON, f64::INFINITY));
            }
        });
    });
}

pub fn seq_maria(c: &mut Criterion) {
    let mesh = common::maria().build_soup();
    let bvh = LinearBvh::build_seq(&mesh);
    let root_bbox = bvh.bounding_box();

    let rays = common::rays_into_bbox(root_bbox, RAY_COUNT);

    c.bench_function("<Sequential LBVH Traversal: Maria>", |b| {
        b.iter(|| {
            for ray in &rays {
                black_box(bvh.intersect_ray(ray, f64::EPSILON, f64::INFINITY));
            }
        });
    });
}

// Parallel Traversal benchmarks

pub fn par_cat(c: &mut Criterion) {
    let mesh = common::cat().build_soup();
    let bvh = LinearBvh::build_par(&mesh);
    let root_bbox = bvh.bounding_box();

    let rays = common::rays_into_bbox(root_bbox, RAY_COUNT);

    c.bench_function("<Parallel LBVH Traversal: Cat>", |b| {
        b.iter(|| {
            for ray in &rays {
                black_box(bvh.intersect_ray(ray, f64::EPSILON, f64::INFINITY));
            }
        });
    });
}

pub fn par_lucky(c: &mut Criterion) {
    let mesh = common::lucky().build_soup();
    let bvh = LinearBvh::build_par(&mesh);
    let root_bbox = bvh.bounding_box();

    let rays = common::rays_into_bbox(root_bbox, RAY_COUNT);

    c.bench_function("<Parallel LBVH Traversal: Lucky>", |b| {
        b.iter(|| {
            for ray in &rays {
                black_box(bvh.intersect_ray(ray, f64::EPSILON, f64::INFINITY));
            }
        });
    });
}

pub fn par_maria(c: &mut Criterion) {
    let mesh = common::maria().build_soup();
    let bvh = LinearBvh::build_par(&mesh);
    let root_bbox = bvh.bounding_box();

    let rays = common::rays_into_bbox(root_bbox, RAY_COUNT);

    c.bench_function("<Parallel LBVH Traversal: Maria>", |b| {
        b.iter(|| {
            for ray in &rays {
                black_box(bvh.intersect_ray(ray, f64::EPSILON, f64::INFINITY));
            }
        });
    });
}
