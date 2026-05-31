use criterion::Criterion;
use std::hint::black_box;

use renderer::larp::{Boundable, Bvh, LinearBvh};

use crate::common;

const RAY_COUNT: usize = 100_000;

pub fn build_seq_bvh_lbvh(c: &mut Criterion) {
    let mesh = common::maria().build_soup();

    let mut group = c.benchmark_group("BVH v. LBVH Build: Maria");

    group.bench_function("bvh", |b| {
        b.iter(|| {
            black_box(Bvh::build_seq(black_box(&mesh)));
        });
    });

    group.bench_function("lbvh", |b| {
        b.iter(|| {
            black_box(LinearBvh::build_seq(black_box(&mesh)));
        });
    });

    group.finish();
}

pub fn build_par_bvh_lbvh(c: &mut Criterion) {
    let mesh = common::maria().build_soup();

    let mut group = c.benchmark_group("BVH v. LBVH Build (Par): Maria");

    group.bench_function("bvh", |b| {
        b.iter(|| {
            black_box(Bvh::build_par(black_box(&mesh)));
        });
    });

    group.bench_function("lbvh", |b| {
        b.iter(|| {
            black_box(LinearBvh::build_par(black_box(&mesh)));
        });
    });

    group.finish();
}

pub fn traversal_seq_bvh_lbvh(c: &mut Criterion) {
    let mesh = common::maria().build_soup();

    let bvh = Bvh::build_seq(&mesh);
    let lbvh = LinearBvh::build_seq(&mesh);

    let bvh_rays = common::rays_into_bbox(bvh.bounding_box(), RAY_COUNT);
    let lbvh_rays = common::rays_into_bbox(lbvh.bounding_box(), RAY_COUNT);

    let mut group = c.benchmark_group("BVH v. LBVH Traversal: Maria");

    group.bench_function("bvh", |b| {
        b.iter(|| {
            for ray in &bvh_rays {
                black_box(bvh.intersect_ray(ray, f64::EPSILON, f64::INFINITY));
            }
        });
    });

    group.bench_function("lbvh", |b| {
        b.iter(|| {
            for ray in &lbvh_rays {
                black_box(lbvh.intersect_ray(ray, f64::EPSILON, f64::INFINITY));
            }
        });
    });

    group.finish();
}

pub fn traversal_par_bvh_lbvh(c: &mut Criterion) {
    let mesh = common::maria().build_soup();

    let bvh = Bvh::build_par(&mesh);
    let lbvh = LinearBvh::build_par(&mesh);

    let bvh_rays = common::rays_into_bbox(bvh.bounding_box(), RAY_COUNT);
    let lbvh_rays = common::rays_into_bbox(lbvh.bounding_box(), RAY_COUNT);

    let mut group = c.benchmark_group("BVH v. LBVH Traversal (Par): Maria");

    group.bench_function("bvh", |b| {
        b.iter(|| {
            for ray in &bvh_rays {
                black_box(bvh.intersect_ray(ray, f64::EPSILON, f64::INFINITY));
            }
        });
    });

    group.bench_function("lbvh", |b| {
        b.iter(|| {
            for ray in &lbvh_rays {
                black_box(lbvh.intersect_ray(ray, f64::EPSILON, f64::INFINITY));
            }
        });
    });

    group.finish();
}
