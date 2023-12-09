#![feature(array_windows)]

const TEST_INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

const INPUT: &str = include_str!("input.txt");

fn parse_numbers(line: &str) -> Vec<isize> {
    line.split_whitespace()
        .filter_map(|n| n.parse().ok())
        .collect::<Vec<_>>()
}

fn adjacent_difference(slice: &[isize]) -> Vec<isize> {
    slice
        .array_windows::<2>()
        .map(|&[v1, v2]| v2 - v1)
        .collect()
}

fn make_grid(values: Vec<isize>) -> Vec<Vec<isize>> {
    let mut grid = vec![values];

    while !grid.last().unwrap().iter().all(|v| *v == 0) {
        grid.push(adjacent_difference(grid.last().unwrap()));
    }

    grid
}

fn part1(input: &str) -> isize {
    let mut sum = 0;
    for values in input.lines().map(|line| parse_numbers(line)) {
        let grid = make_grid(values);

        let grid_height = grid.len();
        let mut acc = 0;
        for i in 1..grid_height {
            let column_index = grid.len() - i;
            let y_last_index = grid[column_index].len() - 1;
            acc += grid[column_index - 1][y_last_index] + grid[column_index][y_last_index];
        }

        let next_value = acc;
        sum += next_value;
    }
    sum
}

fn part2(input: &str) -> isize {
    let mut sum = 0;
    for values in input.lines().map(|line| parse_numbers(line)) {
        let grid = make_grid(values);
        let grid_height = grid.len();
        let mut acc = 0;
        for i in 1..grid_height {
            acc = grid[grid.len() - i - 1][0] - acc;
        }
        let next_value = acc;
        sum += next_value;
    }
    sum
}

fn main() {
    assert_eq!(part1(TEST_INPUT), 114);
    assert_eq!(part1(INPUT), 1581679977);
    assert_eq!(part2("10 13 16 21 30 45"), 5);

    assert_eq!(part2(TEST_INPUT), 2);
    assert_eq!(part2(INPUT), 889);
}
