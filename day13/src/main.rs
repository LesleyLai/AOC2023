use crate::grid::Grid;

mod grid;

const TEST_INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

const INPUT: &str = include_str!("./input.txt");

fn parse_input(input: &str) -> Vec<Grid<u8>> {
    input
        .split("\r\n\r\n")
        .flat_map(|s| s.split("\n\n")) // Line ending madness
        .map(|s| s.lines().map(|l| l.as_bytes().to_vec()).collect::<Vec<_>>())
        .map(|nested| Grid::from_nested(&nested))
        .collect()
}

fn part1(input: &[Grid<u8>]) -> isize {
    input
        .iter()
        .map(|grid| {
            find_horizontal_line(grid, 0, 0)
                .map(|x| x * 100)
                .unwrap_or_else(|| find_vertical_line(grid, 0, 0).unwrap_or(0))
        })
        .sum()
}

fn part2(input: &[Grid<u8>]) -> isize {
    input
        .iter()
        .map(|grid| {
            let h = find_horizontal_line(grid, 0, 0);
            find_horizontal_line(grid, 1, h.unwrap_or(0))
                .map(|x| x * 100)
                .unwrap_or_else(|| {
                    let v = find_vertical_line(grid, 0, 0);
                    find_vertical_line(grid, 1, v.unwrap_or(0)).unwrap_or(0)
                })
        })
        .sum()
}

fn difference_count<T: PartialEq>(
    first: impl IntoIterator<Item = T>,
    second: impl IntoIterator<Item = T>,
) -> isize {
    first
        .into_iter()
        .zip(second)
        .map(|(e1, e2)| (e1 != e2) as isize)
        .sum::<isize>()
}

fn find_horizontal_line(
    grid: &Grid<u8>,
    max_diffs: isize,
    skip_line_index: isize,
) -> Option<isize> {
    (0..grid.height - 1).find_map(|y| {
        (has_horizontal_reflection(grid, max_diffs, y) && y + 1 != skip_line_index).then(|| y + 1)
    })
}

fn find_vertical_line(grid: &Grid<u8>, max_diffs: isize, skip_line_index: isize) -> Option<isize> {
    (0..grid.width - 1).find_map(|x| {
        (has_vertical_reflection(grid, max_diffs, x) && x + 1 != skip_line_index).then(|| x + 1)
    })
}

fn has_horizontal_reflection(grid: &Grid<u8>, max_diffs: isize, line_index: isize) -> bool {
    let mut diffs = 0;

    let iterations = (line_index + 1).min(grid.height - line_index - 1);

    for i in 0..iterations {
        let up = (line_index - i) as usize;
        let down = (line_index + 1 + i) as usize;

        let up_row = grid.rows().nth(up).unwrap();
        let down_row = grid.rows().nth(down).unwrap();
        diffs += difference_count(up_row, down_row);

        if diffs > max_diffs {
            return false;
        }
    }
    true
}

fn has_vertical_reflection(grid: &Grid<u8>, max_diffs: isize, line_index: isize) -> bool {
    let mut diffs = 0;

    let iterations = (line_index + 1).min(grid.width - line_index - 1);

    for i in 0..iterations {
        let left = (line_index - i) as usize;
        let right = (line_index + 1 + i) as usize;
        let left_col = grid.columns().nth(left).unwrap();
        let right_col = grid.columns().nth(right).unwrap();
        diffs += difference_count(left_col, right_col);

        if diffs > max_diffs {
            return false;
        }
    }
    true
}

fn main() {
    assert_eq!(part1(&parse_input(&TEST_INPUT)), 405);
    assert_eq!(part1(&parse_input(&INPUT)), 35360);

    assert_eq!(part2(&parse_input(&INPUT)), 36755);
}
