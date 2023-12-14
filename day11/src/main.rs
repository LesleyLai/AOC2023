use crate::grid::Grid;
use std::ops::Not;

mod grid;

const TEST_INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";

const INPUT: &str = include_str!("./input.txt");

fn parse(input: &str) -> Grid<u8> {
    let nested: Vec<_> = input.lines().map(|line| line.as_bytes().to_vec()).collect();
    Grid::from_nested(&nested)
}

fn sum_of_lengths(grid: &Grid<u8>, expansion: isize) -> isize {
    let x_to_expand: Vec<_> = grid
        .columns()
        .enumerate()
        .filter_map(|(x, mut col)| col.find(|&&c| c == b'#').is_none().then(|| x as isize))
        .collect();

    let y_to_expand: Vec<_> = grid
        .rows()
        .enumerate()
        .filter_map(|(y, row)| row.contains(&b'#').not().then(|| y as isize))
        .collect();

    let mut galaxies = vec![];
    {
        let mut real_y = 0;
        for y in 0..grid.height {
            let mut real_x = 0;
            for x in 0..grid.width {
                let coord = (x, y);
                if grid[coord] == b'#' {
                    galaxies.push((real_x, real_y));
                }
                let is_x_expanding = x_to_expand.binary_search(&x).is_ok();
                real_x += if is_x_expanding { expansion } else { 1 };
            }
            let is_y_expanding = y_to_expand.binary_search(&y).is_ok();
            real_y += if is_y_expanding { expansion } else { 1 };
        }
    }

    let mut sum = 0;
    for i in 0..galaxies.len() - 1 {
        for j in (i + 1)..galaxies.len() {
            let ((x1, y1), (x2, y2)) = (galaxies[i], galaxies[j]);
            let distance = (x1 - x2).abs() + (y1 - y2).abs();
            sum += distance;
        }
    }

    sum
}

fn main() {
    let test_grid = parse(TEST_INPUT);
    let grid = parse(INPUT);

    // part 1
    assert_eq!(sum_of_lengths(&test_grid, 2), 374);
    assert_eq!(sum_of_lengths(&grid, 2), 9648398);

    // part 2
    assert_eq!(sum_of_lengths(&test_grid, 10), 1030);
    assert_eq!(sum_of_lengths(&test_grid, 100), 8410);
    assert_eq!(sum_of_lengths(&grid, 1000000), 618800410814);
}
