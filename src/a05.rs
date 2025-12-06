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

use std::{
    hint::{black_box, unreachable_unchecked},
    simd::{Mask, Simd, cmp::SimdPartialOrd as _, num::SimdUint as _, u8x8, u8x16, u64x8, u64x16},
};

fn read_number_fast(input_bytes: &[u8]) -> u64 {
    let len = input_bytes.len();

    #[inline]
    fn parse_int_simd_16(input_bytes: &[u8]) -> u64 {
        let mut bytes = [0u8; 16];
        bytes[..input_bytes.len()].copy_from_slice(input_bytes);
        let digits = u8x16::from_array(bytes);
        let zero = u8x16::splat(b'0');
        let values = digits - zero;

        let multipliers = match input_bytes.len() {
            3 => u64x16::from_array([100, 10, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
            4 => u64x16::from_array([1000, 100, 10, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
            5 => u64x16::from_array([10000, 1000, 100, 10, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
            6 => u64x16::from_array([
                100000, 10000, 1000, 100, 10, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ]),
            7 => u64x16::from_array([
                1000000, 100000, 10000, 1000, 100, 10, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ]),
            8 => u64x16::from_array([
                10000000, 1000000, 100000, 10000, 1000, 100, 10, 1, 0, 0, 0, 0, 0, 0, 0, 0,
            ]),
            9 => u64x16::from_array([
                100000000, 10000000, 1000000, 100000, 10000, 1000, 100, 10, 1, 0, 0, 0, 0, 0, 0, 0,
            ]),
            10 => u64x16::from_array([
                1000000000, 100000000, 10000000, 1000000, 100000, 10000, 1000, 100, 10, 1, 0, 0, 0,
                0, 0, 0,
            ]),
            11 => u64x16::from_array([
                10000000000,
                1000000000,
                100000000,
                10000000,
                1000000,
                100000,
                10000,
                1000,
                100,
                10,
                1,
                0,
                0,
                0,
                0,
                0,
            ]),
            12 => u64x16::from_array([
                100000000000,
                10000000000,
                1000000000,
                100000000,
                10000000,
                1000000,
                100000,
                10000,
                1000,
                100,
                10,
                1,
                0,
                0,
                0,
                0,
            ]),
            13 => u64x16::from_array([
                1000000000000,
                100000000000,
                10000000000,
                1000000000,
                100000000,
                10000000,
                1000000,
                100000,
                10000,
                1000,
                100,
                10,
                1,
                0,
                0,
                0,
            ]),
            14 => u64x16::from_array([
                10000000000000,
                1000000000000,
                100000000000,
                10000000000,
                1000000000,
                100000000,
                10000000,
                1000000,
                100000,
                10000,
                1000,
                100,
                10,
                1,
                0,
                0,
            ]),
            15 => u64x16::from_array([
                100000000000000,
                10000000000000,
                1000000000000,
                100000000000,
                10000000000,
                1000000000,
                100000000,
                10000000,
                1000000,
                100000,
                10000,
                1000,
                100,
                10,
                1,
                0,
            ]),
            16 => u64x16::from_array([
                1000000000000000,
                100000000000000,
                10000000000000,
                1000000000000,
                100000000000,
                10000000000,
                1000000000,
                100000000,
                10000000,
                1000000,
                100000,
                10000,
                1000,
                100,
                10,
                1,
            ]),
            _ => unsafe { unreachable_unchecked() },
        };

        let values_u64 = values.cast::<u64>();
        let products = values_u64 * multipliers;
        products.reduce_sum()
    }

    match len {
        0 => 0,
        1 => (input_bytes[0] - b'0') as u64,
        2 => {
            let d0 = (input_bytes[0] - b'0') as u64;
            let d1 = (input_bytes[1] - b'0') as u64;
            d0 * 10 + d1
        }
        3..=16 => parse_int_simd_16(input_bytes),
        _ => unsafe { unreachable_unchecked() },
    }
}

pub fn run(inp: &str) -> i64 {
    unsafe {
        const ARRLEN: usize = 3;

        let mut range_starts: [Simd<u64, 64>; ARRLEN] = [Simd::splat(u64::MAX); ARRLEN];
        let mut range_ends: [Simd<u64, 64>; ARRLEN] = [Simd::splat(0); ARRLEN];

        let input_bytes = inp.as_bytes();
        let mut ind = 0;

        let mut it = 0;
        loop {
            if *input_bytes.get_unchecked(ind) == b'\n' {
                // if input_bytes[ind - 1] == b'\n' {
                break;
                // }
                // ind += 1;
            }

            // let mut parts = inp.split('-');
            // let start: i64 = parts.next().unwrap().parse().unwrap();
            // let end: i64 = parts.next().unwrap().parse().unwrap();

            let start_num_ind = ind;
            while *input_bytes.get_unchecked(ind) != b'-' {
                ind += 1;
            }
            let start = read_number_fast(&input_bytes.get_unchecked(start_num_ind..ind));

            ind += 1; // skip '-'
            let end_num_ind = ind;
            while *input_bytes.get_unchecked(ind) != b'\n' {
                ind += 1;
            }
            let end = read_number_fast(&input_bytes.get_unchecked(end_num_ind..ind));

            debug_assert!(end >= start);
            // ranges.push((start, end + 1));
            let simd_index = it / 64;
            let lane_index = it % 64;

            // range_starts[simd_index][lane_index] = start as u64;
            *range_starts
                .get_unchecked_mut(simd_index)
                .as_mut_array()
                .get_unchecked_mut(lane_index) = start;
            // range_ends[simd_index][lane_index] = (end + 1) as u64;
            *range_ends
                .get_unchecked_mut(simd_index)
                .as_mut_array()
                .get_unchecked_mut(lane_index) = end + 1;

            it += 1;
            ind += 1; // skip '\n'
        }

        let mut matching = 0;

        while input_bytes.len() > ind {
            // let mut char = *byte;
            // while char != b'\n' {
            //     ingredient = ingredient * 10 + (char - b'0') as i64;
            //     char = *input_bytes.next().unwrap_unchecked();
            // }

            let ingredient_num_ind = ind;
            while *input_bytes.get_unchecked(ind) != b'\n' {
                ind += 1;
            }
            let ingredient = black_box(read_number_fast(&input_bytes.get_unchecked(ingredient_num_ind..ind)));
            ind += 1; // skip '\n'

            // let ingredient_simd = Simd::splat(ingredient);

            // let mut gather_mask = Mask::splat(false);

            // for i in 0..ARRLEN {
            //     let ge_start = ingredient_simd.simd_ge(range_starts[i]);
            //     let lt_end = ingredient_simd.simd_lt(range_ends[i]);
            //     gather_mask |= ge_start & lt_end;
            // }

            // if gather_mask.any() {
            //     matching += 1;
            // }
        }

        matching
    }
}
