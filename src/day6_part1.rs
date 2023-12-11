use regex::Regex;

lazy_static! {
    static ref NUMBER_RE: Regex = Regex::new(r"(\d+)").unwrap();
}

fn extract_numbers(s: &str) -> Vec<usize> {
    NUMBER_RE
        .captures_iter(s)
        .map(|caps| {
            let (_, [value_str]) = caps.extract();
            value_str.parse().unwrap()
        })
        .collect()
}

fn parse_input(input: &str) -> (Vec<usize>, Vec<usize>) {
    let lines: Vec<_> = input.split("\n").collect();
    let times = extract_numbers(lines[0]);
    let distances = extract_numbers(lines[1]);
    (times, distances)
}

fn win_possibilities(time: usize, distance: usize) -> usize {
  let mut res: usize = 0;
  for t in 1..time {
    if t * (time - t) > distance {
      res += 1
    }
  }
  res
}

pub fn solve(input: &str) -> usize {
    let (times, distances) = parse_input(input);

    times
        .into_iter()
        .enumerate()
        .map(|(idx, time)| win_possibilities(time, distances[idx]))
        .product()
}
