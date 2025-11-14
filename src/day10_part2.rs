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

type Point = (usize, usize);
type Matrix = Vec<Vec<char>>;

fn get_polygon(matrix: Matrix, start_x: usize, start_y: usize) -> (Vec<Point>, usize) {
    let mut points: Vec<Point> = vec![(start_x, start_y)];
    let mut perimeter = 0usize;

    let max_x = matrix[0].len() - 1;
    let max_y = matrix.len() - 1;
    let mut x = start_x;
    let mut y = start_y;
    let mut skip_dir: Option<Dir> = None;

    loop {
        for dir in [Dir::Left, Dir::Right, Dir::Top, Dir::Bottom] {
            let (next_x, next_y) = next_coords(x, y, max_x, max_y, &dir);

            if skip_dir != Some(dir)
                && (next_x != x || next_y != y)
                && is_connected(matrix[y][x], matrix[next_y][next_x], &dir)
            {
                // Turn
                if skip_dir.is_some() && skip_dir != Some(opposite_dir(&dir)) {
                    points.push((x, y));
                }

                perimeter += 1;
                x = next_x;
                y = next_y;
                skip_dir = Some(opposite_dir(&dir));
                break;
            }
        }

        if matrix[y][x] == 'S' {
            return (points, perimeter);
        }
    }
}

// https://en.wikipedia.org/wiki/Shoelace_formula
fn polygon_area(points: &Vec<Point>) -> usize {
    let sum: i64 = points.windows(2).map(|pair| (pair[0].1 + pair[1].1) as i64 * (pair[0].0 as i64 - pair[1].0 as i64)).sum();

    (sum.abs() / 2) as usize
}

fn parse_input(input: &str) -> (Matrix, usize, usize) {
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

    (matrix, start_x, start_y)
}

pub fn solve(input: &str) -> usize {
    let (matrix, start_x, start_y) = parse_input(&input);
    let (points, perimeter) = get_polygon(matrix, start_x, start_y);
    let area = polygon_area(&points);

    area + 1 - perimeter / 2
}
