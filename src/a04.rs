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
pub fn run(inp: &str) -> i64 {
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
