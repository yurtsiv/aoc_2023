use itertools::Itertools;

fn parse_line(line: &str) -> (&str, usize) {
    let parts: Vec<_> = line.split(" ").collect();
    (parts[0], parts[1].parse().unwrap())
}

fn hand_type(hand: &str) -> &str {
    let mut chars: Vec<_> = hand.chars().collect();
    chars.sort_by(|a, b| a.cmp(b));

    let mut card_group_sizes: Vec<_> = chars
        .into_iter()
        .group_by(|c| *c)
        .into_iter()
        .map(|(_, items)| items.collect::<Vec<_>>().len())
        .collect();

    card_group_sizes.sort_by(|a, b| b.cmp(a));

    match card_group_sizes.len() {
        5 => "1", // High card
        4 => "2", // One pair
        3 => {
            if card_group_sizes[0] == 2 {
                "3" // Two pair
            } else {
                "4" // Three of a kind
            }
        }
        2 => {
            if card_group_sizes[0] == 3 {
                "5" // Full house
            } else {
                "6" // Four of a kind
            }
        }
        _ => "7", // Five of a kind
    }
}

fn hand_value(hand: &str) -> usize {
    let hand_type = hand_type(hand);
    let substituted_hand = hand
        .replace("1", "11")
        .replace("2", "12")
        .replace("3", "13")
        .replace("4", "14")
        .replace("5", "15")
        .replace("6", "16")
        .replace("7", "17")
        .replace("8", "18")
        .replace("9", "19")
        .replace("T", "20")
        .replace("J", "21")
        .replace("Q", "22")
        .replace("K", "23")
        .replace("A", "24");

    format!("{hand_type}{substituted_hand}").parse().unwrap()
}

pub fn solve(input: &str) -> usize {
    input
        .split("\n")
        .map(|line| parse_line(line))
        .map(|(hand, bid)| (hand_value(hand), bid))
        .sorted_by(|a, b| a.0.cmp(&b.0))
        .map(|(_, bid)| bid)
        .enumerate()
        .map(|(idx, bid)| (idx + 1) * bid)
        .sum()
}
