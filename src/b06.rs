pub const SAMPLE_OUTPUT: i64 = 3263827;
/*

123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
*/
pub fn run(inp: &str) -> i64 {
    let mut total_sum = 0;

    let lines = inp.lines().collect::<Vec<&str>>();

    let longest_line_len = lines.iter().map(|l| l.len()).max().unwrap();

    let mut operator = '+';
    let mut num_acc = 0;
    for col in 0..longest_line_len {
        if matches!(
            lines.last().unwrap().chars().nth(col).unwrap_or(' '),
            '+' | '*'
        ) {
            total_sum += num_acc;
            operator = lines.last().unwrap().chars().nth(col).unwrap();
            match operator {
                '+' => num_acc = 0,
                '*' => num_acc = 1,
                _ => unreachable!(),
            }
        }

        let mut col_nums = vec![];
        for row in 0..lines.len() - 1 {
            let c = lines[row].chars().nth(col).unwrap_or(' ');
            if c == ' ' {
                continue;
            }
            col_nums.push(c);
        }

        // convert to decimal number
        let mut n = 0;
        for c in col_nums {
            n = n * 10 + (c as i64 - '0' as i64);
        }

        if n == 0 {
            continue;
        }

        match operator {
            '+' => num_acc += n,
            '*' => num_acc *= n,
            _ => unreachable!(),
        }
    }

    total_sum += num_acc;

    total_sum
}
