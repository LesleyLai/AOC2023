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
            find_line::<false>(grid, 0, None)
                .map(|x| x * 100)
                .unwrap_or_else(|| find_vertical_line(grid, 0, None).unwrap_or(0))
        })
        .sum()
}

fn part2(input: &[Grid<u8>]) -> isize {
    input
        .iter()
        .map(|grid| {
            let h = find_horizontal_line(grid, 0, None);
            find_horizontal_line(grid, 1, h)
                .map(|x| x * 100)
                .unwrap_or_else(|| {
                    let v = find_vertical_line(grid, 0, None);
                    find_vertical_line(grid, 1, v).unwrap_or(0)
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

fn find_line<const IS_VERTICAL: bool>(
    grid: &Grid<u8>,
    max_diffs: isize,
    skip_line_index: Option<isize>,
) -> Option<isize> {
    let length = if IS_VERTICAL { grid.width } else { grid.height };
    (0..length - 1).find_map(|n| {
        (has_reflection::<IS_VERTICAL>(grid, max_diffs, n) && Some(n + 1) != skip_line_index)
            .then(|| n + 1)
    })
}

fn find_horizontal_line(
    grid: &Grid<u8>,
    max_diffs: isize,
    skip_line_index: Option<isize>,
) -> Option<isize> {
    find_line::<false>(grid, max_diffs, skip_line_index)
}

fn find_vertical_line(
    grid: &Grid<u8>,
    max_diffs: isize,
    skip_line_index: Option<isize>,
) -> Option<isize> {
    find_line::<true>(grid, max_diffs, skip_line_index)
}

fn has_reflection<const IS_VERTICAL: bool>(
    grid: &Grid<u8>,
    max_diffs: isize,
    line_index: isize,
) -> bool {
    let length = if IS_VERTICAL { grid.width } else { grid.height };
    let iterations = (line_index + 1).min(length - line_index - 1);

    (0..iterations)
        .scan(0isize, |diffs, i| {
            let lesser_index = (line_index - i) as usize;
            let greater_index = (line_index + 1 + i) as usize;

            *diffs += if IS_VERTICAL {
                difference_count(
                    grid.columns().nth(lesser_index).unwrap(),
                    grid.columns().nth(greater_index).unwrap(),
                )
            } else {
                difference_count(
                    grid.rows().nth(lesser_index).unwrap(),
                    grid.rows().nth(greater_index).unwrap(),
                )
            };
            Some(*diffs)
        })
        .all(|diffs| diffs <= max_diffs)
}

fn main() {
    assert_eq!(part1(&parse_input(&TEST_INPUT)), 405);
    assert_eq!(part1(&parse_input(&INPUT)), 35360);

    assert_eq!(part2(&parse_input(&INPUT)), 36755);
}
