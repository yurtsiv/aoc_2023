// Could also be done by feeding a reversed seq to part1.
// This is an explicit and slightly more efficient approach.

fn solve_sequence(seq: &mut Vec<i64>) -> i64 {
    let mut has_non_zero = true;
    let mut seq_min_idx = 0;

    while has_non_zero {
        has_non_zero = false;

        for i in ((seq_min_idx + 1)..seq.len()).rev() {
            let diff = seq[i] - seq[i - 1];
            seq[i] = diff;
            has_non_zero = has_non_zero || diff != 0;
        }

        seq_min_idx += 1;
    }

    let mut res: i64 = 0;
    for i in (0..seq_min_idx).rev() {
        res = seq[i] - res;
    }

    res
}

pub fn solve(input: &str) -> i64 {
    input
        .split("\n")
        .map(|line| {
            line.split(" ")
                .map(|val| val.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .map(|mut seq| solve_sequence(&mut seq))
        .sum()
}
