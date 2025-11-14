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

type Matrix = Vec<Vec<char>>;

fn loop_length(matrix: Matrix, start_x: usize, start_y: usize) -> i64 {
    let max_x = matrix[0].len() - 1;
    let max_y = matrix.len() - 1;
    let mut x = start_x;
    let mut y = start_y;
    let mut skip_dir: Option<Dir> = None;
    let mut cycle_length = 0i64;

    loop {
        for dir in [Dir::Top, Dir::Bottom, Dir::Left, Dir::Right] {
            let (next_x, next_y) = next_coords(x, y, max_x, max_y, &dir);

            if skip_dir != Some(dir)
                && (next_x != x || next_y != y)
                && is_connected(matrix[y][x], matrix[next_y][next_x], &dir)
            {
                x = next_x;
                y = next_y;
                skip_dir = Some(opposite_dir(&dir));
                break;
            }
        }

        cycle_length += 1;

        if matrix[y][x] == 'S' {
            return cycle_length;
        }
    }
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

    ((loop_length(matrix, start_x, start_y) as f64) / 2.0).ceil() as usize
}
