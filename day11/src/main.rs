use crate::grid::Grid;

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

fn sum_of_lengths(input: &str, expansion: isize) -> isize {
    let grid: Vec<_> = input.lines().map(|line| line.as_bytes().to_vec()).collect();
    let grid = Grid::from_nested(&grid);

    let mut x_to_expand = vec![];
    let mut y_to_expand = vec![];
    for x in 0..grid.width {
        let mut contain_galaxy = false;
        for y in 0..grid.height {
            if grid[(x, y)] == b'#' {
                contain_galaxy = true;
                break;
            }
        }
        if !contain_galaxy {
            x_to_expand.push(x);
        }
    }

    for (y, row) in grid.rows().iter().enumerate() {
        if !row.contains(&b'#') {
            y_to_expand.push(y as isize);
        }
    }

    let mut real_x = 0;
    let mut real_y = 0;
    let mut galaxies = vec![];
    for y in 0..grid.height {
        for x in 0..grid.width {
            let coord = (x, y);
            if grid[coord] == b'#' {
                galaxies.push((real_x, real_y));
            }
            let is_x_expanding = x_to_expand.binary_search(&x).is_ok();
            real_x += if is_x_expanding { expansion } else { 1 };
        }
        real_x = 0;
        let is_y_expanding = y_to_expand.binary_search(&y).is_ok();
        real_y += if is_y_expanding { expansion } else { 1 };
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
    // part 1
    assert_eq!(sum_of_lengths(TEST_INPUT, 2), 374);
    assert_eq!(sum_of_lengths(INPUT, 2), 9648398);

    // part 2
    assert_eq!(sum_of_lengths(TEST_INPUT, 10), 1030);
    assert_eq!(sum_of_lengths(TEST_INPUT, 100), 8410);
    assert_eq!(sum_of_lengths(INPUT, 1000000), 618800410814);
}
