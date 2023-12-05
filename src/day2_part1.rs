use regex::Regex;

lazy_static! {
    static ref RED_RE: Regex = Regex::new(r"(\d+) red").unwrap();
    static ref GREEN_RE: Regex = Regex::new(r"(\d+) green").unwrap();
    static ref BLUE_RE: Regex = Regex::new(r"(\d+) blue").unwrap();
}

fn check_constraint(re: &Regex, line: &str, max_value: i32) -> bool {
    for (_, [value_str]) in re.captures_iter(line).map(|c| c.extract()) {
        if value_str.parse::<i32>().unwrap() > max_value {
            return false;
        }
    }

    true
}

fn is_game_possible(line: &str) -> bool {
    check_constraint(&RED_RE, line, 12)
        && check_constraint(&GREEN_RE, line, 13)
        && check_constraint(&BLUE_RE, line, 14)
}

pub fn solve(input: &str) -> i32 {
    input
        .split("\n")
        .enumerate()
        .filter(|(_, line)| is_game_possible(line))
        .map(|(index, _)| (index + 1) as i32)
        .sum()
}
