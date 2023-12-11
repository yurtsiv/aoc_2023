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

fn parse_input(input: &str) -> (usize, usize) {
    let lines: Vec<_> = input.split("\n").collect();
    let time = extract_numbers(&lines[0].replace(" ", ""))[0];
    let distance = extract_numbers(&lines[1].replace(" ", ""))[0];
    (time, distance)
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
    let (time, distance) = parse_input(input);
    win_possibilities(time, distance)
}
