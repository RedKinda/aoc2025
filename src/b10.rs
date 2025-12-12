use std::collections::HashMap;

use rayon::iter::{ParallelBridge, ParallelIterator};

pub const SAMPLE_OUTPUT: i64 = 33;

/*
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
*/

#[derive(Clone, Copy, Hash)]
struct UVec([u16; 16]);
impl UVec {
    fn new() -> Self {
        UVec([0; 16])
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn elements_le(&self, other: &UVec) -> bool {
        for i in 0..self.len() {
            if self.0[i] > other.0[i] {
                return false;
            }
        }
        true
    }

    fn reduce_sum(&self) -> u16 {
        let mut sum = 0;
        for val in &self.0 {
            sum += *val;
        }
        sum
    }

    fn elements_full_divided_by(&self, other: &UVec) -> u16 {
        // ensure that all elements are divided fully, by the same integer
        let mut coeff = 0;
        for i in 0..self.len() {
            match (self.0[i] == 0, other.0[i] == 0) {
                (true, true) => continue,
                (true, false) => return 0,
                (false, true) => return 0,
                (false, false) => {
                    if !self.0[i].is_multiple_of(other.0[i]) {
                        return 0;
                    }
                    let current_coeff = self.0[i] / other.0[i];
                    if coeff == 0 {
                        coeff = current_coeff;
                    } else if coeff != current_coeff {
                        return 0;
                    }
                }
            }
        }

        coeff
    }
}
impl std::ops::Index<usize> for UVec {
    type Output = u16;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
impl std::ops::IndexMut<usize> for UVec {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
impl PartialEq for UVec {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl Eq for UVec {}
impl std::ops::Add for UVec {
    type Output = UVec;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = UVec::new();
        for i in 0..self.len() {
            result[i] = self[i] + rhs[i];
        }
        result
    }
}
impl std::ops::Sub for UVec {
    type Output = UVec;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = UVec::new();
        for i in 0..self.len() {
            result[i] = self[i] - rhs[i];
        }
        result
    }
}

#[inline(always)]
fn recurse_bruteforce(
    state: UVec,
    index: usize,
    switch_uvecs: &[UVec],
    target_uvec: &UVec,
    // press_count_tracker: &mut UVec,
    // dp_cache: &mut HashMap<UVec, (usize, u16)>,
) -> u16 {
    if state == *target_uvec {
        return 0;
    }

    if index == switch_uvecs.len() - 1 {
        let to_fulfill = *target_uvec - state;
        let modifier = &switch_uvecs[index];
        let coeff = to_fulfill.elements_full_divided_by(modifier);
        if coeff != 0 {
            return coeff;
        }
        return u16::MAX;
    }

    // if let Some((smallest_index_verified, new_state_cache)) = dp_cache.get(&state)
    //     && smallest_index_verified <= &index
    // {
    //     println!("Cache hit at index {}", index);
    //     return *new_state_cache;
    // }

    let mut press_count: u16 = 0;
    let modifier = switch_uvecs[index];
    let mut new_state = state;

    let mut least_presses = u16::MAX;
    // let mut least_press_count = *press_count_tracker;

    while new_state.elements_le(target_uvec) && press_count <= least_presses {
        let sol_presses = recurse_bruteforce(
            new_state,
            index + 1,
            switch_uvecs,
            target_uvec,
            // press_count_tracker,
            // dp_cache,
        );
        if sol_presses != u16::MAX {
            let total_presses = press_count.saturating_add(sol_presses);
            return total_presses;
        }

        new_state = new_state + modifier;
        press_count += 1;
        // press_count_tracker[index] = press_count;

        // if press_count_tracker.0 == [5, 0, 5, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] {
        //     println!("Debug hit at index {}", index);
        // }
    }

    // *press_count_tracker = least_press_count;
    // dp_cache.insert(state, (index, least_presses));
    u16::MAX
}

pub fn run(inp: &str) -> i64 {
    inp.lines()
        .enumerate()
        .par_bridge()
        .map(|(line_index, line)| {
            let parts: Vec<&str> = line.split(" ").collect();

            let mut switch_uvecs: Vec<UVec> = Vec::new();
            for switch_part in &parts[1..parts.len() - 1] {
                let indices_str = switch_part.trim_start_matches('(').trim_end_matches(')');
                let mut switch_uvec = UVec::new();
                for index_str in indices_str.split(',') {
                    let index = index_str.parse::<usize>().unwrap();
                    switch_uvec[index] = 1;
                }

                switch_uvecs.push(switch_uvec);
            }

            // last part is target light config
            let target_jolt_config_str = parts.last().unwrap();
            let mut target_uvec = UVec::new();
            for (ind, num) in target_jolt_config_str
                .trim_start_matches('{')
                .trim_end_matches('}')
                .split(',')
                .enumerate()
            {
                let val = num.parse::<u16>().unwrap();
                target_uvec[ind] = val;
            }

            // println!("Target UVec: {:?}", target_uvec.0);
            // println!("Switch UVecs:");
            // for su in &switch_uvecs {
            //     println!("             {:?}", su.0);
            // }

            // let mut press_count_tracker = UVec::new();

            // sort switch uvecs by reduce sum ascending
            switch_uvecs.sort_by_key(|uvec| uvec.reduce_sum());

            // let mut cache = HashMap::new();
            let sol = recurse_bruteforce(
                UVec::new(),
                0,
                &switch_uvecs,
                &target_uvec,
                // &mut press_count_tracker,
                // &mut cache,
            );

            // debug print tracker
            // println!("Press counts per switch: {:?}", press_count_tracker.0);
            println!("Solution presses for line {}: {}", line_index, sol);

            if sol == u16::MAX {
                panic!("No solution found");
            }

            sol as i64
        })
        .sum::<i64>()
}
