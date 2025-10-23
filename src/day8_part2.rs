use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref MAP_LINE_RE: Regex =
        Regex::new(r"(?<source>[A-Z]*) = \((?<left>[A-Z]*), (?<right>[A-Z]*)\)").unwrap();
}

#[derive(Debug)]
struct Map {
    // 1 - source, 2 - transitive, 3 - destination
    node_types: Vec<u8>,
    // Right and left index for the node on the given index
    directions: Vec<(usize, usize)>,
}

type Dirs = Vec<bool>;

fn parse_map_line(line: &str) -> (String, String, String) {
    let Some(caps) = MAP_LINE_RE.captures(line) else { panic!() };
    (
        (caps["source"]).to_string(),
        (caps["left"]).to_string(),
        (caps["right"]).to_string(),
    )
}

fn build_map(lines: Vec<&str>) -> Map {
    let mut raw_directions: HashMap<String, (String, String)> = HashMap::new();
    let mut node_indexes: HashMap<String, usize> = HashMap::new();
    let mut node_names: Vec<String> = vec![];

    for (index, line) in lines.iter().enumerate() {
        let (source, left, right) = parse_map_line(&line);
        raw_directions.insert(source.clone(), (left, right));
        node_indexes.insert(source.clone(), index);
        node_names.push(source.clone());
    }

    let mut directions: Vec<(usize, usize)> = vec![];
    let mut node_types: Vec<u8> = vec![];

    for node_name in &node_names {
        let dirs = raw_directions.get(node_name).unwrap();
        let left_index = *node_indexes.get(&dirs.0).unwrap();
        let right_index = *node_indexes.get(&dirs.1).unwrap();

        if node_name.chars().nth(2) == Some('A') {
            node_types.push(1);
        } else if node_name.chars().nth(2) == Some('Z') {
            node_types.push(3);
        } else {
            node_types.push(2);
        }

        directions.push((left_index, right_index))
    }

    Map {
        node_types,
        directions: directions,
    }
}

fn build_dirs(line: &str) -> Dirs {
    line.chars().map(|c| c == 'R').collect()
}

fn find_path_length(map: &Map, dirs: &Dirs, source: usize) -> i64 {
    let mut steps: i64 = 0;
    let mut point = source;

    loop {
        for (dir_index, dir) in dirs.iter().enumerate() {
            if map.node_types[point] == 3 {
                return steps;
            }

            let (left, right) = map.directions[point];
            point = if *dir { right } else { left };

            steps += 1;
        }
    }
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    loop {
        if a == 0 {
            return b;
        }

        b %= a;

        if b == 0 {
            return a;
        }

        a %= b;
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    let x = gcd(a, b);
    if x != 0 {
        a / x * b
    } else {
        0
    }
}

fn vec_lcm(v: Vec<i64>) -> i64 {
    let mut r = lcm(v[0], v[1]);

    for i in 2..v.len() {
        r = lcm(r, v[i])
    }

    r
}

pub fn solve(input: &str) -> i64 {
    let lines: Vec<_> = input.split("\n").collect();
    let dirs = build_dirs(lines[0]);
    let map = build_map((lines[2..]).to_vec());

    let mut sources: Vec<usize> = vec![];

    for (index, node_type) in map.node_types.iter().enumerate() {
        if *node_type == 1u8 {
            sources.push(index)
        }
    }

    let mut path_lengths: Vec<i64> = vec![];
    for source in sources {
        path_lengths.push(find_path_length(&map, &dirs, source));
    }

    // Not a general solution. Just so happens that each path in test data
    // goes over the same end node with equal number of steps.
    vec_lcm(path_lengths)
}
