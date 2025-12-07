pub const SAMPLE_OUTPUT: i64 = 21;

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
    let mut state = vec![];

    let mut lines = inp.lines();

    let mut splitcount = 0;

    let first_line = lines.next().unwrap();
    state.resize(first_line.len(), false);
    state[first_line
        .chars()
        .enumerate()
        .find(|c| c.1 == 'S')
        .unwrap()
        .0] = true;

    for line in lines {
        for (ind, char) in line.chars().enumerate() {
            if char == '^' && state[ind] {
                state[ind - 1] = true;
                state[ind + 1] = true;
                state[ind] = false;
                splitcount += 1;
            }
        }
    }

    splitcount
}
