lazy_static! {
    static ref TOKENS: Vec<&'static str> = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"];
}

fn number_from_line(line: &str) -> i32 {
    let mut digits: Vec<(usize, &str)> = TOKENS
        .iter()
        .flat_map(|token| line.match_indices(token))
        .collect();

    digits.sort_by(|(pos1, _), (pos2, _)| pos1.cmp(pos2));

    let first_digit = digits.first().unwrap().1;
    let last_digit = digits.last().unwrap().1;

    format!("{}{}", first_digit, last_digit).parse().unwrap()
}

pub fn solve(input: &str) -> i32 {
    input.split("\n").map(|line| number_from_line(line)).sum()
}
