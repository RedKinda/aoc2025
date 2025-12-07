pub const SAMPLE_OUTPUT: i64 = 40;

/*
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
*/

pub fn run_naive(inp: &str) -> i64 {
    let mut state: Vec<u64> = vec![];

    let mut lines = inp.lines();

    let first_line = lines.next().unwrap();
    state.resize(first_line.len(), 0);
    state[first_line
        .chars()
        .enumerate()
        .find(|c| c.1 == 'S')
        .unwrap()
        .0] = 1;

    lines.next().unwrap(); // only every second line matters

    for line in lines.step_by(2) {
        for (ind, char) in line.chars().enumerate() {
            if char == '^' && state[ind] > 0 {
                state[ind - 1] += state[ind];
                state[ind + 1] += state[ind];
                state[ind] = 0;
            }
        }
    }

    state.iter().sum::<u64>() as i64
}

use std::simd::{Simd, cmp::SimdPartialEq as _, num::SimdUint};
pub fn run(inp: &str) -> i64 {
    const LINE_COUNT: usize = 142;
    const LINE_LENGTH: usize = 142;
    const LANE_COUNT: usize = 32;
    const SIMD_COUNT: usize = LINE_LENGTH.div_ceil(LANE_COUNT);

    // assert_eq!(SIMD_COUNT, 3);

    #[cfg(debug_assertions)]
    if inp.len() < LINE_COUNT * LINE_LENGTH {
        return run_naive(inp);
    }

    let inp = inp.as_bytes();

    let mut state = [Simd::<u64, LANE_COUNT>::splat(0); SIMD_COUNT];
    let simd_start_mask = Simd::<u8, LANE_COUNT>::splat(b'S');
    for i in 0..SIMD_COUNT {
        // set state[i] based on inp masking on mask
        let inp_simd =
            Simd::<u8, LANE_COUNT>::from_slice(&inp[i * LANE_COUNT..(i + 1) * LANE_COUNT]);
        let mask: std::simd::Mask<i64, LANE_COUNT> = inp_simd.simd_eq(simd_start_mask).cast();
        state[i] = mask.select(Simd::splat(1), Simd::splat(0));
    }

    for line_ind in (2..LINE_COUNT).step_by(2) {
        for i in 0..SIMD_COUNT {
            let mask = Simd::splat(b'^');
            let inp_slice = &inp[line_ind * LINE_LENGTH + (i * LANE_COUNT)
                ..line_ind * LINE_LENGTH + ((i + 1) * LANE_COUNT)];
            let inp_simd = Simd::<u8, LANE_COUNT>::from_slice(inp_slice);
            let splitter_mask: std::simd::Mask<i64, LANE_COUNT> = inp_simd.simd_eq(mask).cast();

            if i != SIMD_COUNT - 1 && splitter_mask.test(LANE_COUNT - 1) {
                // copy rightmost of i to left most of i+1
                state[i + 1].as_mut_array()[0] += state[i].as_array()[LANE_COUNT - 1];
            }
            if i != 0 && splitter_mask.test(0) {
                // copy leftmost of i to right most of i-1
                state[i - 1].as_mut_array()[LANE_COUNT - 1] += state[i].as_array()[0];
            }

            let to_jitter = splitter_mask.select(state[i], Simd::splat(0));
            let jitterless_state = state[i] - to_jitter;
            state[i] = jitterless_state
                + to_jitter.shift_elements_left::<1>(0)
                + to_jitter.shift_elements_right::<1>(0);

            // state[i] = state[i].shift_elements_left::<1>(0) + state[i].shift_elements_right::<1>(0);
        }

        // debug print state
        // for i in 0..SIMD_COUNT {
        //     for j in 0..LANE_COUNT {
        //         let v = state[i].as_array()[j];
        //         print!(
        //             "{} ",
        //             if v > 0 {
        //                 v.to_string()
        //             } else {
        //                 ".".to_string()
        //             }
        //         );
        //     }
        //     print!("| ");
        // }
        // println!();
    }

    state
        .iter()
        .fold(Simd::splat(0), |acc, s| acc + *s)
        .reduce_sum() as i64
}
