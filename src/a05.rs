pub const SAMPLE_OUTPUT: i64 = 3;

/*
3-5
10-14
16-20
12-18

1
5
8
11
17
32
*/

pub fn run_naive(inp: &str) -> i64 {
    let mut ranges = vec![];

    let mut lines = inp.lines();

    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }

        let mut parts = line.split('-');
        let start: i64 = parts.next().unwrap().parse().unwrap();
        let end: i64 = parts.next().unwrap().parse().unwrap();

        ranges.push((start, end));
    }

    let mut sum_valid = 0;

    while let Some(line) = lines.next() {
        let ingredient: i64 = line.parse().unwrap();

        for r in ranges.iter() {
            if ingredient >= r.0 && ingredient <= r.1 {
                sum_valid += 1;
                break;
            }
        }
    }

    sum_valid
}

use std::simd::{Mask, Simd, cmp::SimdPartialOrd as _};

pub fn run(inp: &str) -> i64 {
    // let mut ranges: smallvec::SmallVec<[(i64, i64); 256]> = smallvec::SmallVec::new();

    // for line in inp.lines() {
    //     if line.is_empty() {
    //         break;
    //     }
    //     let mut parts = line.split('-');
    //     let start: i64 = parts.next().unwrap().parse().unwrap();
    //     let end: i64 = parts.next().unwrap().parse().unwrap();

    //     debug_assert!(end >= start);

    //     ranges.push((start, end + 1));
    // }

    const ARRLEN: usize = 3;

    let mut range_starts: [Simd<u64, 64>; ARRLEN] = [Simd::splat(u64::MAX); ARRLEN];
    let mut range_ends: [Simd<u64, 64>; ARRLEN] = [Simd::splat(0); ARRLEN];

    let mut input_bytes = inp.as_bytes().iter();
    let mut char = *input_bytes.next().unwrap();
    let mut it = 0;
    loop {
        if char == b'\n' {
            char = *input_bytes.next().unwrap();
            if char == b'\n' {
                break;
            }
        }

        // let mut parts = inp.split('-');
        // let start: i64 = parts.next().unwrap().parse().unwrap();
        // let end: i64 = parts.next().unwrap().parse().unwrap();

        let mut start = 0;
        while char != b'-' {
            start = start * 10 + (char - b'0') as i64;
            char = *input_bytes.next().unwrap();
        }
        char = *input_bytes.next().unwrap(); // skip '-'

        let mut end = 0;
        while char != b'\n' {
            end = end * 10 + (char - b'0') as i64;
            char = *input_bytes.next().unwrap();
        }

        debug_assert!(end >= start);
        // ranges.push((start, end + 1));
        let simd_index = it / 64;
        let lane_index = it % 64;

        range_starts[simd_index][lane_index] = start as u64;
        range_ends[simd_index][lane_index] = (end + 1) as u64;
        it += 1;
    }

    let mut matching = 0;

    while let Some(byte) = input_bytes.next() {
        let mut ingredient = 0;
        let mut char = *byte;
        while char != b'\n' {
            ingredient = ingredient * 10 + (char - b'0') as i64;
            char = *input_bytes.next().unwrap();
        }

        let ingredient_u64 = ingredient as u64;
        let ingredient_simd = Simd::splat(ingredient_u64);

        let mut gather_mask = Mask::splat(false);

        for i in 0..ARRLEN {
            let ge_start = ingredient_simd.simd_ge(range_starts[i]);
            let lt_end = ingredient_simd.simd_lt(range_ends[i]);
            gather_mask |= ge_start & lt_end;
        }

        if gather_mask.any() {
            matching += 1;
        }
    }

    matching
}
