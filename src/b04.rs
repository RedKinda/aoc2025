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

        // println!("\n---\n");
        // // debug print the grids
        // for y in 1..=line_count {
        //     for x in 1..=line_length {
        //         print!("{}", if grid[y][x] { '@' } else { '.' });
        //     }
        //     println!();
        // }

        if !removed {
            break;
        }
    }

    safe_count
}

pub fn run(inp: &str) -> i64 {
    #[cfg(debug_assertions)]
    if inp.len() < 1000 {
        return run_naive(inp);
    }

    let lines: &[[u8; 140]; 139] = unsafe {
        inp.as_bytes()
            .as_chunks_unchecked::<140>()
            .try_into()
            .unwrap_unchecked()
    };

    let mut grid = [[false; 142]; 141];
    let mut count_grid = [[0u8; 142]; 141];
    let mut safe_count = 0;

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == b'@' {
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

    let mut y = 1;
    let mut x = 1;

    while x < 141 {
        while y < 140 {
            if grid[y][x] && count_grid[y][x] < 4 {
                safe_count += 1;

                let mut shift = None;

                // sub 1 from neighbors in count_grid
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if dy == 0 && dx == 0 {
                            continue;
                        }
                        let v =
                            &mut count_grid[(y as isize + dy) as usize][(x as isize + dx) as usize];

                        *v -= 1;

                        if *v == 3
                            && (dx == -1 || (dy == -1 && dx == 0))
                            // && grid[(y as isize + dy) as usize][(x as isize + dx) as usize]
                        {
                            // we made a neighbor safe, back up to it
                            // println!("shifting by {}, {}", -dx, -dy);
                            shift.get_or_insert((dx, dy));
                        }
                    }
                }

                grid[y][x] = false;

                if let Some((sx, sy)) = shift {
                    y = (y as isize + sy) as usize;
                    x = (x as isize + sx) as usize;
                    continue;
                }
            }

            y += 1;
        }
        y = 1;
        x += 1;
    }

    safe_count
}
