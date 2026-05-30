use criterion::{criterion_group, criterion_main};

mod common;

mod math;
use crate::math::*;

criterion_group!(
    math,
    // Morton Sorting u32
    // morton::stable_sort_lucky_u32,
    // morton::unstable_sort_lucky_u32,
    // morton::radix_sort_lucky_u32,
    // morton::stable_sort_maria_u32,
    // morton::unstable_sort_maria_u32,
    // morton::radix_sort_maria_u32,
    // Morton Sorting u64
    morton::stable_sort_lucky_u64,
    morton::unstable_sort_lucky_u64,
    morton::radix_sort_lucky_u64,
    morton::stable_sort_maria_u64,
    morton::unstable_sort_maria_u64,
    morton::radix_sort_maria_u64,
);
criterion_main!(math);
