use std::collections::HashMap;
use regex::Regex;

lazy_static! {
    static ref MAP_LINE_RE: Regex = Regex::new(r"(?<source>[A-Z]*) = \((?<left>[A-Z]*), (?<right>[A-Z]*)\)").unwrap();
}

type Map = HashMap<String, (String, String)>;
type Dirs = Vec<bool>;

fn parse_map_line(line: &str) -> (String, String, String) {
  let Some(caps) = MAP_LINE_RE.captures(line) else { panic!() };
  (
    (caps["source"]).to_string(),
    (caps["left"]).to_string(),
    (caps["right"]).to_string()
  )
}

fn build_map(lines: Vec<&str>) -> Map {
  let mut map: Map = HashMap::new();

  for line in lines {
    let (source, left, right) = parse_map_line(&line);
    map.insert(source, (left, right));
  }

  map
}

fn build_dirs(line: &str) -> Dirs {
  line.chars().map(|c| c == 'R').collect()
}

fn walk(map: &Map, dirs: &Dirs) -> i32 {
  let mut source: &String = &"AAA".to_string();
  let mut steps: i32 = 0;

  loop {
    for dir in dirs {
      match map.get(source) {
        Some((left, right)) => {
          source = if *dir { right } else { left }
        }
        None => {}
      }


      println!("{}", source);

      steps += 1;

      if source == "ZZZ" {
        return steps
      }
    }
  }
} 


pub fn solve(input: &str) -> i32 {
  let lines: Vec<_> = input.split("\n").collect();
  let dirs = build_dirs(lines[0]);
  let map = build_map((lines[2..]).to_vec());

  walk(&map, &dirs)
}