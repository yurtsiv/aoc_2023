type Range = (usize, usize);
type ChainBlock = Vec<(usize, usize, usize)>;
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

fn parse_seeds(line: &str) -> Vec<Range> {
    let mut parts: Vec<_> = line.split(" ").collect();
    parts.remove(0);

    parts
        .chunks(2)
        .map(|chunk| {
            (
                chunk.get(0).unwrap().parse().unwrap(),
                chunk.get(1).unwrap().parse().unwrap(),
            )
        })
        .collect::<Vec<_>>()
}

fn parse_input(input: &str) -> (Vec<Range>, Chain) {
    let mut seeds: Vec<Range> = vec![];
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

fn range_overlap(x: Range, y: Range) -> Option<Range> {
    let (larger, smaller) = if x.1 - x.0 > y.1 - y.0 {
        (x, y)
    } else {
        (y, x)
    };

    if larger.0 > smaller.1 || larger.1 < smaller.0 {
        None
    } else if larger.0 <= smaller.0 && larger.1 >= smaller.1 {
        Some(smaller)
    } else if smaller.0 >= larger.0 {
        Some((smaller.0, larger.1))
    } else {
        Some((larger.0, smaller.1))
    }
}

fn missing_range_parts(range: Range, existing_parts: &mut Vec<Range>) -> Vec<Range> {
    if existing_parts.len() == 0 {
        return vec![range];
    }

    existing_parts.sort_by(|a, b| a.0.cmp(&b.0));

    let first_existing_part = existing_parts[0];
    let last_existing_part = existing_parts.last().unwrap();

    let mut res: Vec<Range> = vec![];

    if first_existing_part.0 > range.0 {
        res.push((range.0, first_existing_part.0 - 1));
    }

    if last_existing_part.1 < range.1 {
        res.push((last_existing_part.1 + 1, range.1));
    }

    for i in 0..(existing_parts.len() - 1) {
        let curr_part = existing_parts[i];
        let next_part = existing_parts[i + 1];
        if curr_part.1 < (next_part.0 - 1) {
            res.push((curr_part.1 + 1, next_part.0 - 1))
        }
    }

    res
}

fn seed_location_ranges((seed_start, seed_range): Range, chain: &Chain) -> Vec<Range> {
    let mut prev_source_ranges = vec![(seed_start, seed_start + seed_range - 1)];

    for block in chain {
        let mut new_source_ranges: Vec<Range> = vec![];

        for prev_source_range in prev_source_ranges {
            let mut overlaps: Vec<Range> = vec![];

            for (dest_start, source_start, span) in block {
                let source_range = (*source_start, source_start + span - 1);
                match range_overlap(prev_source_range, source_range) {
                    None => {}
                    Some(overlap) => {
                        overlaps.push(overlap);
                        let mapped_range = (
                            dest_start + overlap.0 - source_start,
                            dest_start + span - (source_start + span - overlap.1),
                        );
                        new_source_ranges.push(mapped_range);
                    }
                }
            }

            if overlaps.len() > 0 {
                let mut missing_parts = missing_range_parts(prev_source_range, &mut overlaps);
                new_source_ranges.append(&mut missing_parts);
            } else {
                new_source_ranges.push(prev_source_range);
            }
        }

        prev_source_ranges = new_source_ranges;
    }

    prev_source_ranges
}

pub fn solve(input: &str) -> usize {
    let (seeds, chain) = parse_input(input);

    seeds
        .into_iter()
        .flat_map(|seed| seed_location_ranges(seed, &chain))
        .map(|l| l.0)
        .min()
        .unwrap()
}
