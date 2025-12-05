use std::ops::{Add as _, AddAssign, Index, IndexMut};

pub const SAMPLE_OUTPUT: i64 = 43;
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

    let mut safe_count = 0;
    // find all true where count is < 4
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

    loop {
        let mut removed = false;

        for y in 1..=line_count {
            for x in 1..=line_length {
                if grid[y][x] && count_grid[y][x] < 4 {
                    safe_count += 1;
                    removed = true;
                    grid[y][x] = false;

                    // dec neighbors in count_grid
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            if dy == 0 && dx == 0 {
                                continue;
                            }
                            count_grid[(y as isize + dy) as usize][(x as isize + dx) as usize] -= 1;
                        }
                    }
                }
            }
        }

        if !removed {
            break;
        }
    }
    println!("\n---\n");
    // debug print the grids
    for y in 1..=line_count {
        for x in 1..=line_length {
            print!("{}", if grid[y][x] { '@' } else { '.' });
        }
        println!();
    }

    safe_count
}

const BUFFER_LENGTH: usize = 150 * 150;

struct FlatGrid<'a> {
    data: &'a mut [u8; BUFFER_LENGTH],
    line_count: usize,
    line_length: usize,
}

impl<'a> Index<(usize, usize)> for FlatGrid<'a> {
    type Output = u8;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (y, x) = index;
        debug_assert!(y < self.line_count);
        debug_assert!(x < self.line_length);
        let index = y * (self.line_length) + x;
        // &self.data[y * (self.line_length) + x]
        unsafe { self.data.get_unchecked(index) }
    }
}

impl<'a> IndexMut<(usize, usize)> for FlatGrid<'a> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (y, x) = index;
        debug_assert!(y < self.line_count);
        debug_assert!(x < self.line_length);
        let index = y * (self.line_length) + x;
        // &mut self.data[y * (self.line_length) + x]
        unsafe { self.data.get_unchecked_mut(index) }
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
        debug_assert!(y < self.line_count);
        debug_assert!(x < self.line_length);
        &self.data[y * (self.line_length) + x]
    }
}

pub fn run(inp: &str) -> i64 {
    // run_naive(inp);
    assert!(inp.len() < BUFFER_LENGTH);
    let line_length = inp.find('\n').unwrap() + 1;
    let line_count = inp.len() / (line_length);

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

    let deltas_first = [(-1, -1), (-1, 0), (-1, 1), (0, -1)];
    let deltas_second = [(0, 1), (1, -1), (1, 0), (1, 1)];

    // for (y, line) in lines.iter().enumerate() {
    //     for (x, c) in line.iter().enumerate() {

    for y in 0..line_count {
        for x in 0..line_length {
            if lines[(y, x)] == b'@' {
                // grid[y + 1][x + 1] = true;
                count_grid[(y + 1, x + 1)] &= !(1 << 4); // flip off the "false" flag

                // increment neighbors in count_grid
                for (dy, dx) in deltas_first.iter().chain(deltas_second.iter()) {
                    count_grid[(
                        (y as isize + dy + 1) as usize,
                        (x as isize + dx + 1) as usize,
                    )] += 1;
                }
            }
        }
    }

    let dbgprint = |count_grid: &FlatGrid<'_>, mark_y: usize, mark_x: usize| {
        // // debugpring the counts, including padding
        // println!("\n---\n");
        // for y in 0..line_count + 2 {
        //     for x in 0..line_length + 1 {
        //         let v = count_grid[(y, x)];
        //         if y == mark_y && x == mark_x {
        //             print!("! ");
        //         } else if v & (1 << 4) != 0 {
        //             print!(". ");
        //         } else {
        //             print!("{} ", v);
        //         }
        //     }
        //     println!();
        // }
    };

    dbgprint(&count_grid, 0, 0);

    #[allow(clippy::drop_non_drop)]
    drop(lines);

    let mut y = 1;
    let mut x = 1;

    while y < line_count + 1 {
        while x < line_length + 1 {
            if count_grid[(y, x)] < 4 {
                safe_count += 1;

                let mut shift = None;

                let mut inner = |deltas: (isize, isize), do_shift: bool| {
                    let (dy, dx) = deltas;
                    let v =
                        &mut count_grid[((y as isize + dy) as usize, (x as isize + dx) as usize)];

                    *v -= 1;

                    if do_shift && *v < 4
                    // && grid[(y as isize + dy) as usize][(x as isize + dx) as usize]
                    {
                        // we made a neighbor safe, back up to it
                        // println!("shifting by {}, {}", -dx, -dy);
                        shift.get_or_insert((dy, dx));
                    }
                };

                // sub 1 from neighbors in count_grid
                for delta in deltas_first {
                    inner(delta, true);
                }
                for delta in deltas_second {
                    inner(delta, false);
                }

                // grid[y][x] = false;
                count_grid[(y, x)] |= 1 << 4; // set the "false" flag back on
                dbgprint(&count_grid, y, x);

                if let Some((sy, sx)) = shift {
                    y = (y as isize + sy) as usize;
                    x = (x as isize + sx) as usize;
                    // println!("shifting to {}, {}", y, x);
                    dbgprint(&count_grid, y, x);
                    continue;
                }
            }

            x += 1;
        }
        x = 1;
        y += 1;
    }

    // println!("\n---\n");
    // // debug print the grids
    // for y in 1..=line_count {
    //     for x in 1..=line_length {
    //         print!(
    //             "{}",
    //             if (count_grid[(y, x)] & (1 << 4)) == 0 {
    //                 '@'
    //             } else {
    //                 '.'
    //             }
    //         );
    //     }
    //     println!();
    // }

    safe_count
}
