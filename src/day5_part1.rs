type ChainBlock = Vec<(u32, u32, u32)>;
type Chain = Vec<ChainBlock>;

fn parse_block(block: &str) -> ChainBlock {
    let mut res: ChainBlock = vec![];

    for (idx, line) in block.split("\n").enumerate() {
        if idx > 0 {
            let parts: Vec<_> = line.split(" ").collect();
            res.push((
                parts.get(0).unwrap().parse().unwrap(),
                parts.get(1).unwrap().parse().unwrap(),
                parts.get(2).unwrap().parse().unwrap(),
            ))
        }
    }

    res
}

fn parse_seeds(line: &str) -> Vec<u32> {
    let mut res: Vec<u32> = vec![];

    for (idx, val) in line.split(" ").enumerate() {
        if idx > 0 {
            res.push(val.parse().unwrap())
        }
    }

    res
}

fn parse_input(input: &str) -> (Vec<u32>, Chain) {
    let mut seeds: Vec<u32> = vec![];
    let mut chain: Chain = vec![];

    for (idx, block) in input.split("\n\n").enumerate() {
        if idx == 0 {
            seeds = parse_seeds(block);
        } else {
            chain.push(parse_block(block));
        }
    }

    (seeds, chain)
}

fn seed_location(seed: u32, chain: &Chain) -> u32 {
    let mut current_source = seed;

    for block in chain {
      for (dest_start, source_start, range) in block {
          if current_source >= *source_start && current_source < *source_start + range {
              current_source = dest_start + (current_source - *source_start);
              break;
          }
      }
    }

    current_source
}

pub fn solve(input: &str) -> u32 {
    let (seeds, chain) = parse_input(input);

    seeds
        .into_iter()
        .map(|seed| seed_location(seed, &chain))
        .min()
        .unwrap()
}
