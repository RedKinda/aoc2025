pub const SAMPLE_OUTPUT: i64 = 7;

/*
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
*/

#[derive(Clone, Copy)]
struct BitVec {
    inner: u16,
}
impl BitVec {
    fn new() -> Self {
        BitVec { inner: 0 }
    }

    fn set(&mut self, index: usize, value: bool) {
        if value {
            self.inner |= 1 << index;
        } else {
            self.inner &= !(1 << index);
        }
    }

    fn get(&self, index: usize) -> bool {
        (self.inner & (1 << index)) != 0
    }

    fn count_ones(&self) -> u32 {
        self.inner.count_ones()
    }
}
impl PartialEq for BitVec {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}
impl Eq for BitVec {}

impl std::ops::BitXor for BitVec {
    type Output = BitVec;

    fn bitxor(self, rhs: Self) -> Self::Output {
        BitVec {
            inner: self.inner ^ rhs.inner,
        }
    }
}
fn recurse(
    state: BitVec,
    index: usize,
    switch_bitvecs: &[BitVec],
    target_bitvec: BitVec,
) -> Option<BitVec> {
    if index == switch_bitvecs.len() {
        return None;
    }
    let flipped_state = state ^ switch_bitvecs[index];
    if flipped_state == target_bitvec {
        let mut result_bitvec = BitVec::new();
        result_bitvec.set(index, true);
        return Some(result_bitvec);
    }

    let sol_noflip = recurse(state, index + 1, switch_bitvecs, target_bitvec);
    let sol_flip =
        recurse(flipped_state, index + 1, switch_bitvecs, target_bitvec).map(|mut bv| {
            bv.set(index, true);
            bv
        });

    // return whichever has less ones
    match (sol_noflip, sol_flip) {
        (Some(bv1), Some(bv2)) => {
            if bv1.count_ones() <= bv2.count_ones() {
                Some(bv1)
            } else {
                Some(bv2)
            }
        }
        (Some(bv1), None) => Some(bv1),
        (None, Some(bv2)) => Some(bv2),
        (None, None) => None,
    }
}

pub fn run(inp: &str) -> i64 {
    let mut total = 0;
    for line in inp.lines() {
        let parts: Vec<&str> = line.split(" ").collect();

        let target_light_config_str = parts[0];
        let mut target_bitvec = BitVec::new();
        for (i, c) in target_light_config_str
            .chars()
            .skip(1)
            .take(target_light_config_str.len() - 2)
            .enumerate()
        {
            if c == '#' {
                target_bitvec.set(i, true);
            }
        }

        let mut switch_bitvecs: Vec<BitVec> = Vec::new();
        for switch_part in &parts[1..parts.len() - 1] {
            let indices_str = switch_part.trim_start_matches('(').trim_end_matches(')');
            let mut switch_bitvec = BitVec::new();
            for index_str in indices_str.split(',') {
                let index = index_str.parse::<usize>().unwrap();
                switch_bitvec.set(index, true);
            }

            switch_bitvecs.push(switch_bitvec);
        }

        let sol =
            recurse(BitVec::new(), 0, &switch_bitvecs, target_bitvec).expect("No solution found");

        total += sol.count_ones() as i64;
    }

    total
}
