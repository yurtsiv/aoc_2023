use regex::Regex;

lazy_static! {
    static ref RED_RE: Regex = Regex::new(r"(\d+) red").unwrap();
    static ref GREEN_RE: Regex = Regex::new(r"(\d+) green").unwrap();
    static ref BLUE_RE: Regex = Regex::new(r"(\d+) blue").unwrap();
}

fn max_value(re: &Regex, line: &str) -> i32 {
    re.captures_iter(line)
        .map(|c| {
            let (_, [value_str]) = c.extract();
            value_str.parse::<i32>().unwrap()
        })
        .max()
        .unwrap()
}

fn min_set_power(line: &str) -> i32 {
    let max_red = max_value(&RED_RE, line);
    let max_green = max_value(&GREEN_RE, line);
    let max_blue = max_value(&BLUE_RE, line);

    max_red * max_green * max_blue
}

pub fn solve(input: &str) -> i32 {
    input.split("\n").map(|line| min_set_power(line)).sum()
}
