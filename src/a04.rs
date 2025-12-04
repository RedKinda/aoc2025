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

const LINE_COUNT: usize = 139;
const LINE_LENGTH: usize = 140; // this has a newline at the end

pub fn run(inp: &str) -> i64 {
    #[cfg(debug_assertions)]
    if inp.len() < 1000 {
        return run_naive(inp);
    }

    let lines: &[[u8; LINE_LENGTH]; LINE_COUNT] = unsafe {
        inp.as_bytes()
            .as_chunks_unchecked::<LINE_LENGTH>()
            .try_into()
            .unwrap_unchecked()
    };

    // let mut grid = [[false; LINE_LENGTH + 1]; LINE_COUNT + 2];

    // 1 << 4 is a flag for "false" - the @ is NOT there
    let mut count_grid = [[1u8 << 4; LINE_LENGTH + 1]; LINE_COUNT + 2];

    // find all true where count is < 4
    let mut safe_count = 0;

    for y in 1..LINE_COUNT + 1 {
        for x in 1..LINE_LENGTH + 1 {
            let c = lines[y - 1][x - 1];

            if c == b'@' {
                // grid[y][x] = true;
                count_grid[y][x] ^= 1 << 4; // flip off the "false" flag

                // increment neighbors in count_grid
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dy == 0 && dx == 0 {
                            continue;
                        }
                        count_grid[(y as isize + dy) as usize][(x as isize + dx) as usize] += 1;
                    }
                }
            }

            // this means the count is <4, and also that the flag is false, aka the @ is there
            if count_grid[y - 1][x - 1] < 4 {
                safe_count += 1;
            }
        }
    }

    // count last line manually
    for x in 1..LINE_LENGTH {
        if count_grid[LINE_COUNT][x] < 4 {
            safe_count += 1;
        }
    }

    safe_count
}
