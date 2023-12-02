use std::collections::HashMap;

lazy_static! {
    static ref DIGIT_VALUES: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("one", "1");
        m.insert("two", "2");
        m.insert("three", "3");
        m.insert("four", "4");
        m.insert("five", "5");
        m.insert("six", "6");
        m.insert("seven", "7");
        m.insert("eight", "8");
        m.insert("nine", "9");

        m
    };
    static ref TOKENS: Vec<&'static str> = vec![
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine"
    ];
}

fn convert_token(token: &str) -> &str {
    match DIGIT_VALUES.get(token) {
        Some(val) => *val,
        _ => token
    }
}
    
fn number_from_line(line: &str) -> i32 {
    let mut digits: Vec<(usize, &str)> = TOKENS
        .iter()
        .flat_map(|token| line.match_indices(token))
        .collect();

    digits.sort_by(|(pos1, _), (pos2, _)| pos1.cmp(pos2));

    let first_digit = convert_token(digits.first().unwrap().1);
    let last_digit = convert_token(digits.last().unwrap().1);

    format!("{}{}", first_digit, last_digit).parse().unwrap()
}

pub fn solve(input: &str) -> i32 {
    input.split("\n").map(|line| number_from_line(line)).sum()
}
