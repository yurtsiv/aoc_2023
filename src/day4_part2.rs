use regex::Regex;
use std::collections::HashSet;

lazy_static! {
  static ref NUMBER_RE: Regex = Regex::new(r"(\d+)").unwrap();
}

fn extract_numbers(s: &str) -> HashSet<u32> {
  NUMBER_RE
    .captures_iter(s)
    .map(|caps| {
      let (_, [value_str]) = caps.extract();
      value_str.parse().unwrap()
    })
    .collect()
}

fn card_value(line: &str) -> u32 {
  let parts: Vec<&str> = line.split(":").collect::<Vec<_>>().get(1).unwrap().split("|").collect();

  let winning_numbers = extract_numbers(parts.get(0).unwrap());
  let possessed_numbers = extract_numbers(parts.get(1).unwrap());

  winning_numbers.intersection(&possessed_numbers).collect::<Vec<_>>().len() as u32
}

pub fn solve(input: &str) -> u32 {
  let card_values: Vec<u32> = input
    .split("\n")
    .map(|line| card_value(line))
    .collect();

  let mut card_copies: Vec<u32> = vec![1; card_values.len()];

  for (card_idx, val) in card_values.iter().enumerate() {
    for i in 1..(*val + 1) {
      card_copies[card_idx + (i as usize)] += card_copies[card_idx];
    }
  }

  card_copies.iter().sum()
}
