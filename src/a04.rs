use std::{
    ops::{Add as _, AddAssign, Index, IndexMut},
    simd::{self, Simd, cmp::SimdPartialEq, num::SimdInt as _},
};

pub const SAMPLE_OUTPUT: i64 = 13;
/*
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
*/
pub fn run_naive(inp: &str) -> i64 {
    let line_length = inp.lines().next().unwrap().len();
    let line_count = inp.lines().count();

    // vec of vecs, pad one layer of false arouynd
    let mut grid = vec![vec![false; line_length + 2]; line_count + 2];
    let mut count_grid = vec![vec![0u8; line_length + 2]; line_count + 2];

    for (y, line) in inp.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '@' {
                grid[y + 1][x + 1] = true;
                // increment neighbors in count_grid
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dy == 0 && dx == 0 {
                            continue;
                        }
                        count_grid[(y as isize + 1 + dy) as usize]
                            [(x as isize + 1 + dx) as usize] += 1;
                    }
                }
            }
        }
    }

    // find all true where count is < 4
    let mut safe_count = 0;
    for y in 1..=line_count {
        for x in 1..=line_length {
            if grid[y][x] && count_grid[y][x] < 4 {
                safe_count += 1;
            }
        }
    }
    safe_count
}

// const LINE_COUNT: usize = 139;
// const LINE_LENGTH: usize = 140; // this has a newline at the end

const BUFFER_LENGTH: usize = 150 * 150;

struct FlatGrid<'a> {
    data: &'a mut [u8; BUFFER_LENGTH],
    line_count: usize,
    line_length: usize,
}

impl<'a> FlatGrid<'a> {
    fn as_flattened(&self) -> &[u8] {
        &self.data[0..(self.line_count * self.line_length)]
    }

    fn as_flattened_mut(&mut self) -> &mut [u8] {
        &mut self.data[0..(self.line_count * self.line_length)]
    }
}

impl<'a> Index<(usize, usize)> for FlatGrid<'a> {
    type Output = u8;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (y, x) = index;
        &self.data[y * (self.line_length) + x]
    }
}

impl<'a> IndexMut<(usize, usize)> for FlatGrid<'a> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (y, x) = index;
        &mut self.data[y * (self.line_length) + x]
    }
}

struct FlatGridImmutable<'a> {
    data: &'a [u8; BUFFER_LENGTH],
    line_count: usize,
    line_length: usize,
}

impl<'a> Index<(usize, usize)> for FlatGridImmutable<'a> {
    type Output = u8;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (y, x) = index;
        &self.data[y * (self.line_length) + x]
    }
}

pub fn run_fast(inp: &str) -> i64 {
    let line_length = inp.find('\n').unwrap() + 1;
    let line_count = inp.len() / (line_length);

    // println!("line_length {}, line_count {}", line_length, line_count);

    // let raw_input = inp.as_bytes();

    // let mut grid = [[false; LINE_LENGTH + 1]; LINE_COUNT + 2];

    assert!(inp.len() < BUFFER_LENGTH);

    let lines: FlatGridImmutable = FlatGridImmutable {
        data: unsafe { &*(inp.as_bytes() as *const [u8] as *const [u8; BUFFER_LENGTH]) },
        line_count,
        line_length,
    };

    let mut raw_buffer = [1u8 << 4; BUFFER_LENGTH];
    let mut count_grid = FlatGrid {
        data: &mut raw_buffer,
        line_count: line_count + 2,
        line_length: line_length + 1,
    };

    let mut safe_count = 0;

    for y in 1..line_count + 1 {
        for x in 1..line_length + 1 {
            let c = lines[(y - 1, x - 1)];

            if c == b'@' {
                // grid[y][x] = true;
                count_grid[(y, x)] ^= 1 << 4; // flip off the "false" flag

                // increment neighbors in count_grid
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dy == 0 && dx == 0 {
                            continue;
                        }
                        count_grid[((y as isize + dy) as usize, (x as isize + dx) as usize)] += 1;
                    }
                }
            }

            // this means the count is <4, and also that the flag is false, aka the @ is there
            if count_grid[(y - 1, x - 1)] < 4 {
                safe_count += 1;
            }
        }
    }

    // count last line manually
    for x in 1..line_length + 1 {
        if count_grid[(line_count, x)] < 4 {
            safe_count += 1;
        }
    }

    safe_count
}

pub fn run_simd(inp: &str) -> i64 {
    // let lines: &[[u8; LINE_LENGTH]; LINE_COUNT] = unsafe {
    //     inp.as_bytes()
    //         .as_chunks_unchecked::<LINE_LENGTH>()
    //         .try_into()
    //         .unwrap_unchecked()
    // };

    let line_length = inp.find('\n').unwrap() + 1;
    let line_count = inp.len() / (line_length);

    // println!("line_length {}, line_count {}", line_length, line_count);

    // let raw_input = inp.as_bytes();

    // let mut grid = [[false; LINE_LENGTH + 1]; LINE_COUNT + 2];

    assert!(inp.len() < BUFFER_LENGTH);

    let lines: FlatGridImmutable = FlatGridImmutable {
        data: unsafe { &*(inp.as_bytes() as *const [u8] as *const [u8; BUFFER_LENGTH]) },
        line_count,
        line_length: line_length,
    };

    let mut raw_buffer = [1u8 << 4; BUFFER_LENGTH];
    let mut count_grid = FlatGrid {
        data: &mut raw_buffer,
        line_count: line_count + 2,
        line_length: line_length + 1,
    };

    // 1 << 4 is a flag for "false" - the @ is NOT there
    // let mut count_grid: [[u8; 141]; 141] = [[1u8 << 4; line_length + 1]; LINE_COUNT + 2];

    // find all true where count is < 4
    let mut safe_count = 0;

    let simd_offsets = simd::Simd::<isize, 8>::from_array([
        -(line_length as isize + 1) - 1,
        -(line_length as isize + 1),
        -(line_length as isize + 1) + 1,
        -1,
        1,
        line_length as isize + 1 - 1,
        line_length as isize + 1,
        line_length as isize + 1 + 1,
    ]);
    let simd_adder = simd::Simd::<u8, 8>::splat(1);

    for y in 1..line_count + 1 {
        for x in 1..line_length + 1 {
            let c = lines[(y - 1, x - 1)];

            if c == b'@' {
                // grid[y][x] = true;
                // flip off the "false" flag
                count_grid[(y, x)] ^= 1 << 4;

                let simd_indices =
                    simd::Simd::<isize, 8>::splat((y * (line_length + 1) + x) as isize)
                        .add(simd_offsets)
                        .cast::<usize>();

                unsafe {
                    let mut simd = Simd::gather_select_unchecked(
                        count_grid.as_flattened(),
                        simd::Mask::splat(true),
                        simd_indices,
                        Simd::splat(Default::default()),
                    );
                    simd += simd_adder;
                    // write back
                    simd.scatter_select_unchecked(
                        count_grid.as_flattened_mut(),
                        simd::Mask::splat(true),
                        simd_indices,
                    )
                }

                // simd::Simd::gather

                // increment neighbors in count_grid
                // for dy in -1..=1 {
                //     for dx in -1..=1 {
                //         if dy == 0 && dx == 0 {
                //             continue;
                //         }
                //         count_grid[(y as isize + dy) as usize][(x as isize + dx) as usize] += 1;
                //     }
                // }
            }

            // this means the count is <4, and also that the flag is false, aka the @ is there
            if count_grid[(y - 1, x - 1)] < 4 {
                safe_count += 1;
            }
        }
    }

    // count last line manually
    for x in 1..line_length + 1 {
        if count_grid[(line_count, x)] < 4 {
            safe_count += 1;
        }
    }

    // debug print

    // for y in 1..=LINE_COUNT {
    //     for x in 1..=LINE_LENGTH {
    //         let c = if (count_grid[y][x] & (1 << 4)) == 0 {
    //             '@'
    //         } else {
    //             '.'
    //         };
    //         print!("{}", c);
    //     }
    //     println!();
    // }

    safe_count
}

struct UnsafeSlice<'a> {
    data: &'a [u8],
}

impl<'a> From<&'a [u8]> for UnsafeSlice<'a> {
    fn from(data: &'a [u8]) -> Self {
        UnsafeSlice { data }
    }
}

impl Index<usize> for UnsafeSlice<'_> {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { self.data.get_unchecked(index) }
    }
}

impl UnsafeSlice<'_> {
    #[track_caller]
    unsafe fn slice_unchecked(&self, range: std::ops::Range<usize>) -> UnsafeSlice<'_> {
        UnsafeSlice {
            data: unsafe { self.data.get_unchecked(range) },
        }
    }

    fn as_slice(&self) -> &[u8] {
        self.data
    }
}

pub fn run(inp: &str) -> i64 {
    unsafe {
        let line_length = inp.find('\n').unwrap() + 1;
        let line_count = inp.len() / (line_length);
        // println!("line_length {}, line_count {}", line_length, line_count);
        let inp: UnsafeSlice = inp.as_bytes().into();

        // let mut debug_field = [[b'.'; 150]; 150];

        let get_index = |y: usize, x: usize| y * (line_length) + x;

        let empty_dummy = [b'.'; 150];

        let mut row_a;
        let mut row_b = Simd::splat(b'.');
        let mut row_c = {
            Simd::load_or_default(
                inp.slice_unchecked(get_index(0, 0)..get_index(0, line_length))
                    .as_slice(),
            )
        };
        // let mut row_c = inp.slice_unchecked(get_index(1, 0)..get_index(1, line_length));

        let simd_detect = simd::Simd::splat(b'@');
        // these will be rotated left at the start of the loop

        let mut res = 0;
        for y in 0..line_count - 1 {
            row_a = row_b;
            row_b = row_c;

            row_c = Simd::load_or_default(
                inp.slice_unchecked(get_index(y + 1, 0)..get_index(y + 1, line_length))
                    .as_slice(),
            );

            for x in 1..line_length {
                row_a = row_a.rotate_elements_left();
                row_b = row_b.rotate_elements_left();
                row_c = row_c.rotate_elements_left();

                let simd_a: Simd<u8, 3> = row_a.resize(0);
                let mut simd_b: Simd<u8, 3> = row_b.resize(0);
                let mut simd_c: Simd<u8, 3> = row_c.resize(0);

                // println!(
                //     "--- y {} x {}\nsimd_a {:?} \nsimd_b {:?} \nsimd_c {:?}",
                //     y,
                //     x,
                //     simd_a.as_array().map(|c| c as char),
                //     simd_b.as_array().map(|c| c as char),
                //     simd_c.as_array().map(|c| c as char)
                // );

                if simd_b[1] != b'@' {
                    continue;
                }

                let detect_a = simd_a.simd_eq(simd_detect);
                let detect_b = simd_b.simd_eq(simd_detect);
                let detect_c = simd_c.simd_eq(simd_detect);

                // sum detections
                let summed = detect_a.to_bitmask().count_ones()
                    + detect_b.to_bitmask().count_ones()
                    + detect_c.to_bitmask().count_ones();

                // println!("y {} x {} sum {}", y, x, summed);

                if summed < 5 {
                    res += 1;
                    // debug_field[y][x] = b'X';
                } else {
                    // debug_field[y][x] = b'.'; // y as u8 + b'0';
                }
            }
        }

        {
            row_a = row_b;
            row_b = row_c;

            simd_a = simd::Simd::<u8, 3>::from_array([row_a[1], b'.', row_a[0]]);
            simd_b = simd::Simd::<u8, 3>::from_array([row_b[1], b'.', row_b[0]]);

            // last row
            for x in 1..line_length {
                simd_a = simd_a.rotate_elements_left::<1>();
                simd_b = simd_b.rotate_elements_left::<1>();

                simd_a[2] = row_a[x];
                simd_b[2] = row_b[x];

                // println!(
                //     "--- y (last) x {}\nsimd_a {:?} \nsimd_b {:?}",
                //     x,
                //     simd_a.as_array().map(|c| c as char),
                //     simd_b.as_array().map(|c| c as char),
                // );

                if simd_b[1] != b'@' {
                    continue;
                }

                let detect_a = simd_a.simd_eq(simd_detect);
                let detect_b = simd_b.simd_eq(simd_detect);

                // sum detections
                let summed =
                    detect_a.to_bitmask().count_ones() + detect_b.to_bitmask().count_ones();

                // println!("y {} x {} sum {}", y, x, summed);

                if summed < 5 {
                    res += 1;
                }
            }
        }

        // print debug field
        // println!("\n---\n");
        // for y in 0..line_count + 2 {
        //     for x in 0..line_length + 2 {
        //         print!("{} ", debug_field[y][x] as char);
        //     }
        //     println!();
        // }

        // panic!("meow {}", res);

        res
    }
}
