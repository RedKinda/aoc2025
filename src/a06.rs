pub const SAMPLE_OUTPUT: i64 = 4277556;
/*

123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
*/
pub fn run_naive(inp: &str) -> i64 {
    let mut nums: Vec<Vec<i64>> = vec![];
    let mut total_sum = 0;

    let mut first_line = true;
    for line in inp.lines() {
        let mut operators = false;
        let mut parts = line.split_whitespace();
        for (ind, part) in parts.enumerate() {
            if !part.chars().next().unwrap().is_ascii_digit() {
                operators = true;
            }

            if operators {
                match part {
                    "+" => {
                        // reduce sum the nums[ind]
                        let reduced: i64 = nums[ind].iter().sum();
                        total_sum += reduced;
                    }
                    "*" => {
                        // reduce product the nums[ind]
                        let reduced: i64 = nums[ind].iter().product();
                        total_sum += reduced;
                    }
                    _ => unreachable!(),
                }

                // println!("sum: {}", total_sum);
            } else {
                let n = part.parse::<i64>().unwrap();
                if first_line {
                    nums.push(vec![n]);
                } else {
                    nums[ind].push(n);
                }
            }
        }
        first_line = false;
    }

    total_sum
}

use std::{
    hint::unreachable_unchecked, // mental illness
    simd::{
        Simd,
        cmp::{SimdPartialEq as _, SimdPartialOrd},
        num::{SimdInt, SimdUint as _},
    },
};

pub fn run(inp: &str) -> i64 {
    unsafe {
        const LANES: usize = 5;
        let mut total_acc: Simd<u64, LANES> = Simd::splat(0);
        let mut total_acc_scalar: u64 = 0;
        let inp = inp.as_bytes();

        let line_length = inp.iter().position(|&c| c == b'\n').unwrap() + 1; // include newline
        let line_count = inp.len() / line_length;

        let index = |line, col| line * line_length + col;

        let mut ind = 0;
        while ind < line_length {
            // take last line 8 chars, load into simd, find first non-space
            let inp_index = index(line_count - 1, ind + 1);
            let inp_index_end = std::cmp::min(inp_index + 8, inp.len());
            let mut last_line_simd: Simd<u8, 8> = Simd::splat(b' ');
            // copy from inp[inp_index..inp_index_end] into last_line_simd
            let len: usize = inp_index_end - inp_index;
            if len == 0 {
                break;
            }
            // last_line_simd[..len].copy_from_slice(&inp[inp_index..inp_index_end]);
            last_line_simd
                .as_mut_array()
                .get_unchecked_mut(..len)
                .copy_from_slice(inp.get_unchecked(inp_index..inp_index_end));

            let mask = last_line_simd.simd_ge(Simd::splat(b'*'));
            let ind_end = mask.first_set().map(|v| v + ind + 1).unwrap_or(line_length);

            // let operator = inp[index(line_count - 1, ind)];
            let operator = *inp.get_unchecked(index(line_count - 1, ind));
            let mut acc_simd: Simd<u64, LANES> = Simd::splat(0);
            let mut acc_scalar = 1;

            let len = ind_end - ind;
            for row in 0..line_count - 1 {
                let mut simd = Simd::<u8, LANES>::splat(b' ');
                let inp_index = index(row, ind);
                let inp_index_end = inp_index + len;
                // let slice = &inp[inp_index..inp_index_end];
                let slice = inp.get_unchecked(inp_index..inp_index_end);

                let n_trailing_spaces = { slice.iter().rev().take_while(|&&c| c <= b' ').count() };
                // simd[(LANES - len)..].copy_from_slice(slice);
                simd.as_mut_array()
                    .get_unchecked_mut((LANES - len)..)
                    .copy_from_slice(slice);
                simd = match n_trailing_spaces {
                    1 => simd.rotate_elements_right::<1>(),
                    2 => simd.rotate_elements_right::<2>(),
                    3 => simd.rotate_elements_right::<3>(),
                    4 => simd.rotate_elements_right::<4>(),
                    _ => simd,
                };

                let is_space = simd.simd_le(Simd::splat(b' '));
                let subtracted = simd - Simd::splat(b'0');
                simd = is_space.select(Simd::splat(0), subtracted);
                let multiplied =
                    simd.cast() * Simd::<u32, LANES>::from_array([10000, 1000, 100, 10, 1]);
                // println!("{:?}", multiplied);

                match operator {
                    b'+' => {
                        let to_add: Simd<u64, LANES> = multiplied.cast();
                        acc_simd += to_add;
                    }
                    b'*' => {
                        acc_scalar *= multiplied.reduce_sum() as u64;
                    }
                    c => unreachable_unchecked(),
                }
            }

            // total_acc += acc;
            match operator {
                b'+' => {
                    // println!("acc_simd: {:?}", acc_simd);
                    total_acc += acc_simd;
                }
                b'*' => {
                    // println!("acc_scalar: {:?}", acc_scalar);
                    total_acc_scalar += acc_scalar;
                }
                _ => unreachable_unchecked(),
            }

            // println!("sum: {}", total_acc.reduce_sum());

            ind = ind_end;

            //
        }

        total_acc.reduce_sum() as i64 + total_acc_scalar as i64
    }
}
