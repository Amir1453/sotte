use rayon::prelude::*;

pub trait RadixSorter {
    fn radix_sort(&mut self);
    fn par_radix_sort(&mut self);
}

pub fn radix_sort_by_key<F, N>(slice: &mut [F], bits: usize, f: N)
where
    F: Clone,
    N: Fn(&F) -> usize,
{
    const BUCKET_SIZE: usize = 6;
    const N_BUCKETS: usize = 1 << BUCKET_SIZE;
    const MASK: usize = (1 << BUCKET_SIZE) - 1;
    let passes = bits.div_ceil(BUCKET_SIZE);

    if slice.len() <= 1 {
        return;
    }

    let mut source = slice.to_vec();
    let mut destination = vec![source[0].clone(); slice.len()];

    let mut shift = 0;
    for _ in 0..passes {
        let mut frequency = [0usize; N_BUCKETS];

        for code in &source {
            let key = (f(code) >> shift) & MASK;
            frequency[key] += 1;
        }

        let mut start = 0;
        for bucket_start in &mut frequency {
            let bucket_count = *bucket_start;
            *bucket_start = start;
            start += bucket_count;
        }

        for code in &source {
            let key = (f(code) >> shift) & MASK;
            let idx = frequency[key];

            destination[idx] = code.clone();
            frequency[key] = idx + 1;
        }

        std::mem::swap(&mut source, &mut destination);

        shift += BUCKET_SIZE;
    }

    slice.clone_from_slice(&source);
}

pub fn par_radix_sort_by_key<R, F>(slice: &mut [R], bits: usize, f: F)
where
    R: Clone + Sync + Send,
    F: Fn(&R) -> usize + Sync + Send,
{
    const BUCKET_BITS: usize = 6;
    const N_BUCKETS: usize = 1 << BUCKET_BITS;
    const MASK: usize = N_BUCKETS - 1;
    const CHUNK_SIZE: usize = 8192;

    let passes = bits.div_ceil(BUCKET_BITS);

    if slice.len() <= 1 {
        return;
    }

    let mut source = slice.to_vec();
    let mut destination = vec![source[0].clone(); source.len()];

    for pass in 0..passes {
        let shift = pass * BUCKET_BITS;

        let histograms: Vec<[usize; N_BUCKETS]> = source
            .par_chunks(CHUNK_SIZE)
            .map(|chunk| {
                let mut hist = [0usize; N_BUCKETS];
                for code in chunk {
                    let bucket = (f(code) >> shift) & MASK;
                    hist[bucket] += 1;
                }
                hist
            })
            .collect();

        let mut bucket_totals = [0usize; N_BUCKETS];
        for hist in &histograms {
            for b in 0..N_BUCKETS {
                bucket_totals[b] += hist[b];
            }
        }

        let mut bucket_starts = [0usize; N_BUCKETS];
        let mut acc = 0usize;
        for b in 0..N_BUCKETS {
            bucket_starts[b] = acc;
            acc += bucket_totals[b];
        }

        let mut chunk_bases = vec![[0usize; N_BUCKETS]; histograms.len()];
        let mut running = [0usize; N_BUCKETS];
        for (i, hist) in histograms.iter().enumerate() {
            chunk_bases[i] = running;
            for b in 0..N_BUCKETS {
                running[b] += hist[b];
            }
        }

        let dst_addr = destination.as_mut_ptr() as usize;

        source
            .par_chunks(CHUNK_SIZE)
            .enumerate()
            .for_each(|(chunk_idx, chunk)| {
                let mut local = [0usize; N_BUCKETS];
                let base = chunk_bases[chunk_idx];

                for code in chunk {
                    let bucket = (f(code) >> shift) & MASK;
                    let idx = bucket_starts[bucket] + base[bucket] + local[bucket];

                    // SAFETY:
                    // - idx = bucket_starts[bucket] + chunk_bases[chunk_idx][bucket] + local[bucket]
                    unsafe {
                        (dst_addr as *mut R).add(idx).write(code.clone());
                    }
                    local[bucket] += 1;
                }
            });

        std::mem::swap(&mut source, &mut destination);
    }

    slice.clone_from_slice(&source);
}
