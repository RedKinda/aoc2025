
pub const SAMPLE_OUTPUT: i64 = 357;

/*
987654321111111
811111111111119
234234234234278
818181911112111
*/

pub fn run_naive(inp: &str) -> i64 {
    inp.lines()
        .enumerate()
        .map(|(ind, line)| {
            let mut max_digit = '0';
            let mut max_digit_index = 0;

            for (ind, c) in line.chars().take(line.len() - 1).enumerate() {
                if c > max_digit {
                    max_digit = c;
                    max_digit_index = ind;
                }
            }

            // find largest digit, located after first ind

            let mut second_max_digit = '0';
            for (ind, c) in line.chars().enumerate().skip(max_digit_index + 1) {
                if c > second_max_digit {
                    second_max_digit = c;
                }
            }

            let r = max_digit.to_digit(10).unwrap() * 10 + second_max_digit.to_digit(10).unwrap();

            println!("result[{}]: {}", ind, r);

            r
        })
        .sum::<u32>() as i64
}


use std::{
    ops::SubAssign,
    simd::{
        self, LaneCount, SupportedLaneCount,
        cmp::{SimdOrd, SimdPartialEq, SimdPartialOrd as _},
        num::SimdUint as _,
    },
};

fn do_simd<const N: usize>(lines: &[[u8; 101]; N]) -> u64
where
    LaneCount<N>: SupportedLaneCount,
{
    // let first_digit_simd: simd::Simd<u8, 64> = simd::Simd::splat(b'0');
    // load this to be the first element of each row

    let zero_vec = simd::Simd::splat(b'0');

    let mut first_digit_simd: simd::Simd<u8, N>;
    let mut second_digit_simd: simd::Simd<u8, N> = simd::Simd::splat(b'0');

    let mut first_max_digit_simd: simd::Simd<u8, N> = simd::Simd::splat(b'0');
    let mut second_max_digit_simd: simd::Simd<u8, N> = simd::Simd::splat(b'0');

    //  second_digit = line[0];
    for row in 0..N {
        let line_index = row;
        second_digit_simd[row] = lines[line_index][0];
    }

    for col in 1..100 {
        first_digit_simd = second_digit_simd;
        // load second_digit_simd from lines
        for row in 0..N {
            let line_index = row;
            second_digit_simd[row] = lines[line_index][col];
        }

        let mask = first_digit_simd.simd_gt(first_max_digit_simd);
        // reset second max where mask is true
        second_max_digit_simd = mask.select(zero_vec, second_max_digit_simd);
        first_max_digit_simd = mask.select(first_digit_simd, first_max_digit_simd);
        let mask2 = second_digit_simd.simd_gt(second_max_digit_simd);
        second_max_digit_simd = mask2.select(second_digit_simd, second_max_digit_simd);
    }

    // now we have first_max_digit_simd and second_max_digit_simd for this group of 64
    // convert them to final result by doing first * 10 + second
    let ten_vec = simd::Simd::splat(10u16);
    first_max_digit_simd.sub_assign(zero_vec);
    second_max_digit_simd.sub_assign(zero_vec);

    let first_u16: simd::Simd<u16, N> = first_max_digit_simd.cast();
    let second_u16: simd::Simd<u16, N> = second_max_digit_simd.cast();
    let result_u16: simd::Simd<u16, N> = first_u16 * ten_vec + second_u16;

    // print results
    // for i in 0..N {
    //     println!("result[{}]: {}", i, result_u16[i]);
    // }

    let result_scalar: u64 = result_u16.reduce_sum() as u64;

    result_scalar
}

pub fn run(inp: &str) -> i64 {
    #[cfg(debug_assertions)]
    {
        if inp.len() < 1000 {
            // println!("Using naive implementation");
            return run_naive(inp);
        }
    }

    let lines: &[[u8; 101]; 200] = unsafe {
        inp.as_bytes()
            .as_chunks_unchecked::<101>()
            .try_into()
            .unwrap_unchecked()
    };

    // we have 200 rows, processing 64 at a time, then the remaining 8
    let a = do_simd::<64>(&lines[0..64].try_into().unwrap());
    let b = do_simd::<64>(&lines[64..128].try_into().unwrap());
    let c = do_simd::<64>(&lines[128..192].try_into().unwrap());
    let d = do_simd::<8>(&lines[192..200].try_into().unwrap());

    (a + b + c + d) as i64

    // for line_ind in 0..200 {
    //     let line = &lines[line_ind];
    //     let line = line[..line.len() - 1].as_ref();

    //     let mut max_first_digit = b'0';
    //     let mut second_max_digit = b'0';

    //     let mut first_digit;
    //     let mut second_digit = line[0];

    //     for ind in 1..line.len() {
    //         first_digit = second_digit;
    //         second_digit = line[ind];

    //         if first_digit > max_first_digit {
    //             max_first_digit = first_digit;
    //             second_max_digit = second_digit;
    //         } else if second_digit > second_max_digit {
    //             second_max_digit = second_digit;
    //         }
    //     }

    //     // let r = max_digit.to_digit(10).unwrap() * 10 + second_max_digit.to_digit(10).unwrap();
    //     let r = (max_first_digit - b'0') as u16 * 10 + (second_max_digit - b'0') as u16;

    //     // println!(
    //     //     "line: {}, max: {}, second max: {}, r: {}",
    //     //     line, max_digit, second_max_digit, r
    //     // );

    //     r
    // }
}
