fn solve_sequence(seq: &mut Vec<i64>) -> i64 {
  let mut has_non_zero = true;
  let mut seq_max_idx = seq.len();

  while has_non_zero {
    has_non_zero = false;

    for i in 0..(seq_max_idx - 1) {
      let diff = seq[i + 1] - seq[i];
      seq[i] = diff;
      has_non_zero = has_non_zero || diff != 0;
    }

    seq_max_idx -= 1;
  }

  seq[(seq_max_idx - 1)..seq.len()].iter().sum()
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
