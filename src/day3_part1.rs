use regex::Regex;

lazy_static! {
    static ref NUMBER_RE: Regex = Regex::new(r"(\d+)").unwrap();
    static ref NON_SYMBOLS: Vec<char> = vec!['.', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
}

fn is_symbol(c: &char) -> bool {
    NON_SYMBOLS.iter().all(|x| *x != *c)
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

fn numbers_from_line(
    line: &str,
    line_index: usize,
    width: usize,
    height: usize,
    input: &Vec<char>,
) -> Vec<i32> {
    let mut res: Vec<i32> = vec![];

    for caps in NUMBER_RE.captures_iter(line) {
        let m = caps.get(0).unwrap();

        let positions = get_neighboring_positions(m.start(), m.end(), line_index, width, height);

        if positions
            .iter()
            .any(|pos| is_symbol(input.get(*pos).unwrap()))
        {
            let (_, [value_str]) = caps.extract();
            res.push(value_str.parse().unwrap());
        }
    }

    res
}

pub fn solve(input: &str) -> i32 {
    let lines = input.split("\n").collect::<Vec<&str>>();
    let input_joined: Vec<char> = lines.join("").chars().collect();
    let width = lines.get(0).unwrap().len();
    let height = lines.len();

    lines
        .iter()
        .enumerate()
        .flat_map(|(idx, line)| numbers_from_line(line, idx, width, height, &input_joined))
        .sum()
}
