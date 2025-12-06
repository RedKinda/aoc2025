pub const SAMPLE_OUTPUT: i64 = 3263827;
/*

123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
*/
pub fn run_naive(inp: &str) -> i64 {
    let mut total_sum: u64 = 0;

    let lines = inp.lines().map(|l| l.as_bytes()).collect::<Vec<&[u8]>>();

    let longest_line_len = lines.iter().map(|l| l.len()).max().unwrap();

    let mut operator = b'+';
    let mut num_acc: u64 = 0;
    for col in 0..longest_line_len {
        if matches!(lines.last().unwrap().get(col).unwrap_or(&b' '), b'+' | b'*') {
            total_sum += num_acc;
            operator = *lines.last().unwrap().get(col).unwrap();
            match operator {
                b'+' => num_acc = 0,
                b'*' => num_acc = 1,
                _ => unreachable!(),
            }
        }

        let mut col_nums = vec![];
        for row in 0..lines.len() - 1 {
            let c = lines[row].get(col).unwrap_or(&b' ');
            if *c == b' ' {
                continue;
            }
            col_nums.push(c);
        }

        // convert to decimal number
        let mut n: u64 = 0;
        for c in col_nums {
            n = n * 10 + (*c - b'0') as u64;
        }

        if n == 0 {
            continue;
        }

        match operator {
            b'+' => num_acc += n,
            b'*' => num_acc *= n,
            _ => unreachable!(),
        }
    }

    total_sum += num_acc;

    total_sum as i64
}

pub fn run(inp: &str) -> i64 {
    let mut total_sum: u64 = 0;
    let inp = inp.as_bytes();

    let line_length = inp.iter().position(|&c| c == b'\n').unwrap() + 1; // include newline
    let line_count = inp.len() / line_length;

    let index = |line, col| line * line_length + col;

    let mut operator = b'+';
    let mut num_acc: u64 = 0;
    for col in 0..line_length - 1 {
        // let local_operator = inp[index(line_count - 1, col)];
        let local_operator = unsafe { *inp.get_unchecked(index(line_count - 1, col)) };

        match local_operator {
            b'+' => {
                total_sum += num_acc;
                operator = local_operator;
                num_acc = 0
            }
            b'*' => {
                total_sum += num_acc;
                operator = local_operator;
                num_acc = 1
            }
            _ => {}
        }

        let mut n: u64 = 0;
        for row in 0..line_count - 1 {
            // let c = inp[index(row, col)];
            let c = unsafe { *inp.get_unchecked(index(row, col)) };
            if c == b' ' {
                continue;
            }
            n = n * 10 + (c - b'0') as u64;
        }

        if n == 0 {
            continue;
        }

        match operator {
            b'+' => num_acc += n,
            b'*' => num_acc *= n,
            _ => unreachable!(),
        }
    }

    total_sum += num_acc;

    total_sum as i64
}
