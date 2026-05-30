use criterion::{criterion_group, criterion_main};

mod common;

mod math;
use crate::math::*;

criterion_group!(
    math,
    // Sequential Morton Sorting Lucky u32
    morton::stable_sort_lucky_u32,
    morton::unstable_sort_lucky_u32,
    morton::radix_sort_lucky_u32,
    // Sequential Morton Sorting Maria u32
    morton::stable_sort_maria_u32,
    morton::unstable_sort_maria_u32,
    morton::radix_sort_maria_u32,
    // Parallel Morton Sorting Lucky u32
    morton::par_stable_sort_lucky_u32,
    morton::par_unstable_sort_lucky_u32,
    morton::par_radix_sort_lucky_u32,
    // Parallel Morton Sorting Lucky u32
    morton::par_stable_sort_maria_u32,
    morton::par_unstable_sort_maria_u32,
    morton::par_radix_sort_maria_u32,
    // Morton Sorting u64
    // morton::stable_sort_lucky_u64,
    // morton::unstable_sort_lucky_u64,
    // morton::radix_sort_lucky_u64,
    // morton::stable_sort_maria_u64,
    // morton::unstable_sort_maria_u64,
    // morton::radix_sort_maria_u64,
);
criterion_main!(math);
