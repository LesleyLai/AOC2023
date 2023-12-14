use crate::grid::Grid;
use std::collections::{hash_map::Entry, HashMap};

mod grid;

const TEST_INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

const INPUT: &str = include_str!("./input.txt");

fn parse_grid(input: &str) -> Grid<u8> {
    let nested: Vec<_> = input.lines().map(|line| line.as_bytes().to_vec()).collect();
    Grid::from_nested(&nested)
}

// Print grid for debugging purpose
#[allow(dead_code)]
fn print_grid(grid: &Grid<u8>) {
    for row in grid.rows() {
        println!("{}", std::str::from_utf8(row).unwrap());
    }
}

fn tilt_west(grid: &mut Grid<u8>) {
    let mut rows: Vec<_> = grid.rows().map(|row| row.to_vec()).collect();

    for row in rows.iter_mut() {
        row.split_mut(|&c| c == b'#').for_each(|free_slice| {
            // partition
            let rounded_rock_count = free_slice.iter().filter(|&&c| c == b'O').count();
            for x in 0..rounded_rock_count {
                free_slice[x] = b'O';
            }
            for x in rounded_rock_count..free_slice.len() {
                free_slice[x] = b'.';
            }
        })
    }

    *grid = Grid::from_nested(&rows);
}

fn tilt_east(grid: &mut Grid<u8>) {
    grid.flip_x();
    tilt_west(grid);
    grid.flip_x();
}

fn tilt_north(grid: &mut Grid<u8>) {
    grid.transpose();
    tilt_west(grid);
    grid.transpose();
}

fn tilt_south(grid: &mut Grid<u8>) {
    grid.transpose();
    tilt_east(grid);
    grid.transpose();
}

fn cycle(grid: &mut Grid<u8>) {
    tilt_north(grid);
    tilt_west(grid);
    tilt_south(grid);
    tilt_east(grid);
}

fn calculate_load(grid: &Grid<u8>) -> usize {
    grid.rows()
        .enumerate()
        .map(|(i, row)| row.iter().filter(|&&c| c == b'O').count() * (grid.height as usize - i))
        .sum()
}

fn part1(mut grid: Grid<u8>) -> usize {
    tilt_north(&mut grid);
    calculate_load(&grid)
}

fn part2(mut grid: Grid<u8>) -> usize {
    let mut table = HashMap::new();
    let mut i: usize = 0;
    let remaining = loop {
        match table.entry(grid.clone()) {
            Entry::Vacant(v) => {
                v.insert(i);
            }
            Entry::Occupied(ref o) => {
                let repetition_iterations = i - o.get();
                break (1000000000 - i) % repetition_iterations;
            }
        }

        cycle(&mut grid);
        i += 1;
    };

    for _ in 0..remaining {
        cycle(&mut grid);
    }

    calculate_load(&grid)
}

fn main() {
    let mut grid = parse_grid(TEST_INPUT);
    cycle(&mut grid);
    cycle(&mut grid);

    assert_eq!(part1(parse_grid(TEST_INPUT)), 136);
    assert_eq!(part1(parse_grid(INPUT)), 108918);

    assert_eq!(part2(parse_grid(TEST_INPUT)), 64);
    assert_eq!(part2(parse_grid(INPUT)), 100310);
}
