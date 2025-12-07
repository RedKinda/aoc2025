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

pub fn run(inp: &str) -> i64 {
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

    for line in lines {
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
