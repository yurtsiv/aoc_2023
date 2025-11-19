#[derive(Debug, PartialEq, Clone)]
enum Cell {
    Galaxy,
    Empty,
}

type Matrix = Vec<Vec<Cell>>;

fn transpose(matrix: Matrix) -> Matrix {
    let len = matrix[0].len();
    let mut iters: Vec<_> = matrix.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<Cell>>()
        })
        .collect()
}

fn find_galaxy_y_coords(matrix: &Matrix, space_multiplier: usize) -> Vec<usize> {
    let mut res = vec![];
    let mut space_added = 0; 

    for (y, row) in matrix.iter().enumerate() {
        let mut has_galaxy = false;
        for cell in row.iter() {
            if *cell == Cell::Galaxy {
                has_galaxy = true;
                res.push(y + space_added);
            }
        }

        if !has_galaxy {
            space_added += (space_multiplier - 1).max(1);
        }
    }

    return res
}

fn parse_input(input: &str) -> Matrix {
    input
        .split("\n")
        .map(|line| {
            line.chars()
                .map(|c| if c == '#' { Cell::Galaxy } else { Cell::Empty })
                .collect()
        })
        .collect::<Matrix>()
}

// Sum of the distances between each pair of numbers
fn distance_sum(nums: &Vec<usize>) -> usize {
    let mut sum = 0;
    for i in 0..(nums.len() - 1) {
        for j in (i + 1)..nums.len() {
            sum += nums[i].max(nums[j]) - nums[j].min(nums[i]);
        }
    }
    return sum
}

pub fn solve_inner(input: &str, space_multiplier: usize) -> usize {
    let matrix = parse_input(input);
    let y_coords = find_galaxy_y_coords(&matrix, space_multiplier);
    let x_coords = find_galaxy_y_coords(&transpose(matrix), space_multiplier);

    // A distance between two galaxies is the sum of the distance between their x and y coords.
    // Since the final result is a sum of all distances, there's no need to know (x, y) for each galaxy,
    // we just sum the distances between all x and y coordinates separately.
    distance_sum(&x_coords) + distance_sum(&y_coords)
}

pub fn solve(input: &str) -> usize {
    solve_inner(input, 1)
}