#![allow(unused)]

use criterion::Criterion;
use rayon::slice::ParallelSliceMut;
use std::hint::black_box;

use renderer::math::RadixSorter;

use crate::common::*;

// Sequential Morton Code sorting on u32

pub fn stable_sort_lucky_u32(c: &mut Criterion) {
    let morton_codes = mortonic_lucky::<u32>();

    c.bench_function("<std::sort on Morton Codes: Lucky u32>", |b| {
        b.iter_batched(
            || morton_codes.clone(),
            |mut codes| {
                codes.sort();
                black_box(codes)
            },
            criterion::BatchSize::SmallInput,
        );
    });
}

pub fn unstable_sort_lucky_u32(c: &mut Criterion) {
    let morton_codes = mortonic_lucky::<u32>();

    c.bench_function("<std::unstable_sort on Morton Codes: Lucky u32>", |b| {
        b.iter_batched(
            || morton_codes.clone(),
            |mut codes| {
                codes.sort_unstable();
                black_box(codes)
            },
            criterion::BatchSize::SmallInput,
        );
    });
}

pub fn radix_sort_lucky_u32(c: &mut Criterion) {
    let morton_codes = mortonic_lucky::<u32>();

    c.bench_function("<Radix Sort on Morton Codes: Lucky u32>", |b| {
        b.iter_batched(
            || morton_codes.clone(),
            |mut codes| {
                codes.radix_sort();
                black_box(codes)
            },
            criterion::BatchSize::SmallInput,
        );
    });
}

pub fn stable_sort_maria_u32(c: &mut Criterion) {
    let morton_codes = mortonic_maria::<u32>();

    c.bench_function("<std::sort on Morton Codes: Maria u32>", |b| {
        b.iter_batched(
            || morton_codes.clone(),
            |mut codes| {
                codes.sort();
                black_box(codes)
            },
            criterion::BatchSize::SmallInput,
        );
    });
}

pub fn unstable_sort_maria_u32(c: &mut Criterion) {
    let morton_codes = mortonic_maria::<u32>();

    c.bench_function("<std::unstable_sort on Morton Codes: Maria u32>", |b| {
        b.iter_batched(
            || morton_codes.clone(),
            |mut codes| {
                codes.sort_unstable();
                black_box(codes)
            },
            criterion::BatchSize::SmallInput,
        );
    });
}

pub fn radix_sort_maria_u32(c: &mut Criterion) {
    let morton_codes = mortonic_maria::<u32>();

    c.bench_function("<Radix Sort on Morton Codes: Maria u32>", |b| {
        b.iter_batched(
            || morton_codes.clone(),
            |mut codes| {
                codes.radix_sort();
                black_box(codes)
            },
            criterion::BatchSize::SmallInput,
        );
    });
}

// Parallel Morton Code sorting on u32

pub fn par_stable_sort_lucky_u32(c: &mut Criterion) {
    let morton_codes = mortonic_lucky::<u32>();

    c.bench_function("<rayon::par_sort on Morton Codes: Lucky u32>", |b| {
        b.iter_batched(
            || morton_codes.clone(),
            |mut codes| {
                codes.par_sort();
                black_box(codes)
            },
            criterion::BatchSize::SmallInput,
        );
    });
}

pub fn par_unstable_sort_lucky_u32(c: &mut Criterion) {
    let morton_codes = mortonic_lucky::<u32>();

    c.bench_function(
        "<rayon::par_sort_unstable on Morton Codes: Lucky u32>",
        |b| {
            b.iter_batched(
                || morton_codes.clone(),
                |mut codes| {
                    codes.par_sort_unstable();
                    black_box(codes)
                },
                criterion::BatchSize::SmallInput,
            );
        },
    );
}

pub fn par_radix_sort_lucky_u32(c: &mut Criterion) {
    let morton_codes = mortonic_lucky::<u32>();

    c.bench_function("<Parallel Radix Sort on Morton Codes: Lucky u32>", |b| {
        b.iter_batched(
            || morton_codes.clone(),
            |mut codes| {
                codes.par_radix_sort();
                black_box(codes)
            },
            criterion::BatchSize::SmallInput,
        );
    });
}

pub fn par_stable_sort_maria_u32(c: &mut Criterion) {
    let morton_codes = mortonic_maria::<u32>();

    c.bench_function("<rayon::par_sort on Morton Codes: Maria u32>", |b| {
        b.iter_batched(
            || morton_codes.clone(),
            |mut codes| {
                codes.par_sort();
                black_box(codes)
            },
            criterion::BatchSize::SmallInput,
        );
    });
}

pub fn par_unstable_sort_maria_u32(c: &mut Criterion) {
    let morton_codes = mortonic_maria::<u32>();

    c.bench_function(
        "<rayon::par_sort_unstable on Morton Codes: Maria u32>",
        |b| {
            b.iter_batched(
                || morton_codes.clone(),
                |mut codes| {
                    codes.par_sort_unstable();
                    black_box(codes)
                },
                criterion::BatchSize::SmallInput,
            );
        },
    );
}

pub fn par_radix_sort_maria_u32(c: &mut Criterion) {
    let morton_codes = mortonic_maria::<u32>();

    c.bench_function("<Parallel Radix Sort on Morton Codes: Maria u32>", |b| {
        b.iter_batched(
            || morton_codes.clone(),
            |mut codes| {
                codes.par_radix_sort();
                black_box(codes)
            },
            criterion::BatchSize::SmallInput,
        );
    });
}

// Morton Code sorting on u64

pub fn stable_sort_lucky_u64(c: &mut Criterion) {
    let morton_codes = mortonic_lucky::<u64>();

    c.bench_function("<std::sort on Morton Codes: Lucky u64>", |b| {
        b.iter_batched(
            || morton_codes.clone(),
            |mut codes| {
                codes.sort();
                black_box(codes)
            },
            criterion::BatchSize::SmallInput,
        );
    });
}

pub fn unstable_sort_lucky_u64(c: &mut Criterion) {
    let morton_codes = mortonic_lucky::<u64>();

    c.bench_function("<std::unstable_sort on Morton Codes: Lucky u64>", |b| {
        b.iter_batched(
            || morton_codes.clone(),
            |mut codes| {
                codes.sort_unstable();
                black_box(codes)
            },
            criterion::BatchSize::SmallInput,
        );
    });
}

pub fn radix_sort_lucky_u64(c: &mut Criterion) {
    let morton_codes = mortonic_lucky::<u64>();

    c.bench_function("<Radix Sort on Morton Codes: Lucky u64>", |b| {
        b.iter_batched(
            || morton_codes.clone(),
            |mut codes| {
                codes.radix_sort();
                black_box(codes)
            },
            criterion::BatchSize::SmallInput,
        );
    });
}

pub fn stable_sort_maria_u64(c: &mut Criterion) {
    let morton_codes = mortonic_maria::<u64>();

    c.bench_function("<std::sort on Morton Codes: Maria u64>", |b| {
        b.iter_batched(
            || morton_codes.clone(),
            |mut codes| {
                codes.sort();
                black_box(codes)
            },
            criterion::BatchSize::SmallInput,
        );
    });
}

pub fn unstable_sort_maria_u64(c: &mut Criterion) {
    let morton_codes = mortonic_maria::<u64>();

    c.bench_function("<std::unstable_sort on Morton Codes: Maria u64>", |b| {
        b.iter_batched(
            || morton_codes.clone(),
            |mut codes| {
                codes.sort_unstable();
                black_box(codes)
            },
            criterion::BatchSize::SmallInput,
        );
    });
}

pub fn radix_sort_maria_u64(c: &mut Criterion) {
    let morton_codes = mortonic_maria::<u64>();

    c.bench_function("<Radix Sort on Morton Codes: Maria u64>", |b| {
        b.iter_batched(
            || morton_codes.clone(),
            |mut codes| {
                codes.radix_sort();
                black_box(codes)
            },
            criterion::BatchSize::SmallInput,
        );
    });
}
