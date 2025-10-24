use itertools::Itertools;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Dir {
    Top,
    Bottom,
    Left,
    Right,
}

fn is_connected(a: char, b: char, dir: &Dir) -> bool {
    let top_nodes = ['S', '|', '7', 'F'];
    let bottom_nodes = ['S', '|', 'L', 'J'];
    let left_nodes = ['S', '-', 'F', 'L'];
    let right_nodes = ['S', '-', 'J', '7'];

    let res = match dir {
        Dir::Top => bottom_nodes.contains(&a) && top_nodes.contains(&b),
        Dir::Bottom => top_nodes.contains(&a) && bottom_nodes.contains(&b),
        Dir::Left => right_nodes.contains(&a) && left_nodes.contains(&b),
        Dir::Right => left_nodes.contains(&a) && right_nodes.contains(&b),
    };

    res
}

fn opposite_dir(dir: &Dir) -> Dir {
    match dir {
        Dir::Top => Dir::Bottom,
        Dir::Bottom => Dir::Top,
        Dir::Left => Dir::Right,
        Dir::Right => Dir::Left,
    }
}

fn maybe_sub(x: usize, y: usize) -> usize {
    if x == 0 {
        x
    } else {
        x - y
    }
}

fn next_coords(x: usize, y: usize, max_x: usize, max_y: usize, dir: &Dir) -> (usize, usize) {
    match dir {
        Dir::Top => (x, maybe_sub(y, 1)),
        Dir::Bottom => (x, (y + 1).min(max_y)),
        Dir::Left => (maybe_sub(x, 1), y),
        Dir::Right => ((x + 1).min(max_x), y),
    }
}

type Segment = (usize, usize);
type RowSegments = Vec<Segment>;
type AllSegments = Vec<RowSegments>;

/*
A 2-dimensional array of column idexes of each loop chunk on each row

Example:

...........
.S-------7.
.|.F----7|.
.|.|....||.
.|.|....||.
.|.L7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........

Output:

[
  [(1, 9)],
  [(9, 9), (8, 3), (1, 1)],
  [(9, 9), (8, 8), (3, 3), (1, 1)],
  [(9, 9), (8, 8), (3, 3), (1, 1)],
  [(9, 9), (6, 8), (3, 4), (1, 1)],
  [(9, 9), (6, 6), (4, 4), (1, 1)],
  [(9, 6), (4, 1)],
]
*/
fn loop_segments(matrix: Vec<Vec<char>>, start_x: usize, start_y: usize) -> AllSegments {
    let mut result = matrix
        .iter()
        .map(|_row| Vec::new())
        .collect::<AllSegments>();

    let max_x = matrix[0].len() - 1;
    let max_y = matrix.len() - 1;
    let mut x = start_x;
    let mut y = start_y;
    let mut chunk_start_x = x;
    let mut skip_dir: Option<Dir> = None;

    loop {
        // Make sure to go horizontally first
        for dir in [Dir::Left, Dir::Right, Dir::Top, Dir::Bottom] {
            let (next_x, next_y) = next_coords(x, y, max_x, max_y, &dir);

            if skip_dir != Some(dir)
                && (next_x != x || next_y != y)
                && is_connected(matrix[y][x], matrix[next_y][next_x], &dir)
            {
                if dir == Dir::Top || dir == Dir::Bottom {
                    result[y].push((chunk_start_x.min(x), x.max(chunk_start_x)));
                    chunk_start_x = x;
                }

                x = next_x;
                y = next_y;
                skip_dir = Some(opposite_dir(&dir));
                break;
            }
        }

        if matrix[y][x] == 'S' {
            return result
                .into_iter()
                .filter(|chunks| chunks.len() > 0)
                .map(|row_segments| {
                    row_segments
                        .into_iter()
                        .sorted_by(|a, b| Ord::cmp(&a.0, &b.0))
                        .collect::<RowSegments>()
                })
                .collect::<AllSegments>();
        }
    }
}

fn enclosed_tiles_in_row(row_segments: &RowSegments) -> i64 {
    let res = row_segments
        .chunks(2)
        .map(|chunk| match chunk {
            [x, y] => (x.1 as i64 - y.0 as i64).abs() - 1,
            _ => 0,
        })
        .sum();

    println!("Row {:?}", row_segments);
    println!("Row len {:?}", row_segments.len());
    println!("Res {:?}", res);

    res
}

/*
Algo (WRONG):
First and last rows of the cycle can't have enclosed nodes.
For each row group chunks into disjoint pairs (a, b) and for each pair do (b.0 - a.1 - 1).

New idea:
The area of a polygon the loop creates - minus # of loop nodes.

https://en.wikipedia.org/wiki/Shoelace_formula
*/

pub fn solve(input: &str) -> i64 {
    let mut matrix: Vec<Vec<char>> = Vec::new();
    let mut start_x: usize = 0;
    let mut start_y: usize = 0;

    for (y, line) in input.split("\n").enumerate() {
        let mut row: Vec<char> = Vec::new();
        for (x, s) in line.split("").filter(|s| *s != "").enumerate() {
            let c = s.chars().next().unwrap();
            row.push(c);
            if c == 'S' {
                start_x = x;
                start_y = y;
            }
        }
        matrix.push(row);
    }

    let segments = loop_segments(matrix, start_x, start_y);

    let s = &segments.as_slice()[1..(segments.len() - 1)];

    // println!("{:?}", segments);
    println!("{:?}", s);

    s.iter()
        .map(|row_segments| enclosed_tiles_in_row(row_segments))
        .sum()
}
