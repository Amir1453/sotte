use criterion::Criterion;
use std::hint::black_box;

use renderer::larp::{Boundable, Bvh};

use crate::common;

const RAY_COUNT: usize = 100_000;

pub fn cat(c: &mut Criterion) {
    let mesh = common::cat().build_soup();

    let seq_bvh = Bvh::build_seq(&mesh);
    let seq_rays = common::rays_into_bbox(seq_bvh.bounding_box(), RAY_COUNT);

    let par_bvh = Bvh::build_par(&mesh);
    let par_rays = common::rays_into_bbox(par_bvh.bounding_box(), RAY_COUNT);

    let mut group = c.benchmark_group("BVH Traversal: Cat");

    group.bench_function("seq_build", |b| {
        b.iter(|| {
            for ray in &seq_rays {
                black_box(seq_bvh.intersect_ray(ray, f64::EPSILON, f64::INFINITY));
            }
        });
    });

    group.bench_function("par_build", |b| {
        b.iter(|| {
            for ray in &par_rays {
                black_box(par_bvh.intersect_ray(ray, f64::EPSILON, f64::INFINITY));
            }
        });
    });

    group.finish();
}

pub fn lucky(c: &mut Criterion) {
    let mesh = common::lucky().build_soup();

    let seq_bvh = Bvh::build_seq(&mesh);
    let seq_rays = common::rays_into_bbox(seq_bvh.bounding_box(), RAY_COUNT);

    let par_bvh = Bvh::build_par(&mesh);
    let par_rays = common::rays_into_bbox(par_bvh.bounding_box(), RAY_COUNT);

    let mut group = c.benchmark_group("BVH Traversal: Lucky");

    group.bench_function("seq_build", |b| {
        b.iter(|| {
            for ray in &seq_rays {
                black_box(seq_bvh.intersect_ray(ray, f64::EPSILON, f64::INFINITY));
            }
        });
    });

    group.bench_function("par_build", |b| {
        b.iter(|| {
            for ray in &par_rays {
                black_box(par_bvh.intersect_ray(ray, f64::EPSILON, f64::INFINITY));
            }
        });
    });

    group.finish();
}

pub fn maria(c: &mut Criterion) {
    let mesh = common::maria().build_soup();

    let seq_bvh = Bvh::build_seq(&mesh);
    let seq_rays = common::rays_into_bbox(seq_bvh.bounding_box(), RAY_COUNT);

    let par_bvh = Bvh::build_par(&mesh);
    let par_rays = common::rays_into_bbox(par_bvh.bounding_box(), RAY_COUNT);

    let mut group = c.benchmark_group("BVH Traversal: Maria");

    group.bench_function("seq_build", |b| {
        b.iter(|| {
            for ray in &seq_rays {
                black_box(seq_bvh.intersect_ray(ray, f64::EPSILON, f64::INFINITY));
            }
        });
    });

    group.bench_function("par_build", |b| {
        b.iter(|| {
            for ray in &par_rays {
                black_box(par_bvh.intersect_ray(ray, f64::EPSILON, f64::INFINITY));
            }
        });
    });

    group.finish();
}
