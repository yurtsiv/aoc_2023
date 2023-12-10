use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref NUMBER_RE: Regex = Regex::new(r"(\d+)").unwrap();
}

fn is_gear(c: char) -> bool {
    c == '*'
}

fn get_neighboring_positions(
    start: usize,
    end: usize,
    line_index: usize,
    width: usize,
    height: usize,
) -> Vec<usize> {
    let mut res: Vec<usize> = vec![(line_index * width) + end];

    if start > 0 {
        res.push((line_index * width) + start - 1)
    }

    let from = if start == 0 { 0 } else { start - 1 };
    let to = if end == width - 1 { end } else { end + 1 };

    if line_index > 0 {
        for i in from..to {
            res.push((line_index - 1) * width + i)
        }
    }

    if line_index < (height - 1) {
        for i in from..to {
            res.push((line_index + 1) * width + i)
        }
    }

    res
}

fn process_line(
    line: &str,
    line_index: usize,
    width: usize,
    height: usize,
    input: &Vec<char>,
    gear_adjacency: &mut HashMap<usize, Vec<i32>>,
) {
    for caps in NUMBER_RE.captures_iter(line) {
        let m = caps.get(0).unwrap();

        let positions = get_neighboring_positions(m.start(), m.end(), line_index, width, height)
            .into_iter()
            .filter(|pos| is_gear(*input.get(*pos).unwrap()));

        let (_, [value_str]) = caps.extract();
        let value = value_str.parse().unwrap();

        for pos in positions {
            match gear_adjacency.get_mut(&pos) {
                None => {
                  gear_adjacency.insert(pos, vec![value]);
                }
                Some(values) => {
                  values.push(value);
                }
            }
        }
    }
}

pub fn solve(input: &str) -> i32 {
    let lines = input.split("\n").collect::<Vec<&str>>();
    let input_joined: Vec<char> = lines.join("").chars().collect();
    let width = lines.get(0).unwrap().len();
    let height = lines.len();

    let mut gear_adjacency: HashMap<usize, Vec<i32>> = HashMap::new();

    for (idx, line) in lines.iter().enumerate() {
        process_line(line, idx, width, height, &input_joined, &mut gear_adjacency)
    }

    println!("Hello: {:?}", gear_adjacency);

    gear_adjacency
        .values()
        .filter(|values| values.iter().len() == 2)
        .map(|values| values.get(0).unwrap() * values.get(1).unwrap())
        .sum()
}
