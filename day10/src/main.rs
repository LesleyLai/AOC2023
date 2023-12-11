mod grid;

use std::collections::VecDeque;

use crate::grid::Grid;

const TEST_INPUT: &str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF
";

const TEST_INPUT2: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

const TEST_INPUT3: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
";

const TEST_INPUT4: &str = "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........
";

const TEST_INPUT5: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
";

const TEST_INPUT6: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";

const TEST_INPUT7: &str = "F-7..
|.|..
|FJS7
||.||
|L-J|
L---J";

const INPUT: &str = include_str!("./input.txt");

fn part1(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<_>>();
    let grid = Grid::from_nested(&grid);

    let grid_width = grid.width;
    let grid_height = grid.height;

    let mut distance_grid = Grid::new(grid_width, grid_height);

    let mut start_coord = (grid_width, grid_height);
    for y in 0..grid_height {
        for x in 0..grid_width {
            if grid[(x, y)] == b'S' {
                start_coord = (x, y);
                distance_grid[(x, y)] = Some(0);
            }
        }
    }

    // breadth-first traversal
    let mut queue = VecDeque::new();
    queue.push_back(start_coord);
    while let Some(current) = queue.pop_front() {
        let current_distance = distance_grid[current].unwrap();
        let (x, y) = current;

        let up = (x, y - 1);
        if y > 0 && distance_grid[up].is_none() {
            match (grid[(x, y)], grid[up]) {
                (b'|' | b'L' | b'J' | b'S', b'|' | b'7' | b'F') => {
                    distance_grid[up] = Some(current_distance + 1);
                    queue.push_back(up);
                }
                _ => {}
            }
        }

        let down = (x, y + 1);
        if y < grid_height - 1 && distance_grid[down].is_none() {
            match (grid[current], grid[down]) {
                (b'|' | b'7' | b'F' | b'S', b'|' | b'L' | b'J') => {
                    distance_grid[down] = Some(current_distance + 1);
                    queue.push_back(down);
                }
                _ => {}
            }
        }

        let left = (x - 1, y);
        if x > 0 && distance_grid[left].is_none() {
            match (grid[current], grid[left]) {
                (b'-' | b'J' | b'7' | b'S', b'-' | b'L' | b'F') => {
                    distance_grid[left] = Some(current_distance + 1);
                    queue.push_back(left);
                }
                _ => {}
            }
        }

        let right = (x + 1, y);
        if x < grid_width - 1 && distance_grid[right].is_none() {
            match (grid[current], grid[right]) {
                (b'-' | b'L' | b'F' | b'S', b'-' | b'J' | b'7') => {
                    distance_grid[right] = Some(current_distance + 1);
                    queue.push_back(right);
                }
                _ => {}
            }
        }
    }

    distance_grid
        .rows()
        .iter()
        .filter_map(|row| row.iter().filter_map(|opt| *opt).max())
        .max()
        .unwrap()
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Status {
    Boundary,
    Interior,
    Exterior,
}

fn flood_fill(status_grid: &mut Grid<Option<Status>>, start: (isize, isize), replacement: Status) {
    let mut stack = vec![start];

    while let Some((x, y)) = stack.pop() {
        if status_grid[(x, y)].is_some() {
            assert_ne!(
                status_grid[(x, y)].unwrap(),
                if replacement == Status::Exterior {
                    Status::Interior
                } else {
                    Status::Exterior
                }
            );

            continue; // Already have something. Can't fill
        }
        status_grid[(x, y)] = Some(replacement);

        let up = (x, y - 1);
        let down = (x, y + 1);
        let left = (x - 1, y);
        let right = (x + 1, y);

        if y > 0 {
            stack.push(up);
        }
        if y < status_grid.height - 1 {
            stack.push(down);
        }
        if x > 0 {
            stack.push(left);
        }
        if x < status_grid.width - 1 {
            stack.push(right);
        }
    }
}

fn part2(input: &str) -> isize {
    let mut grid = input
        .lines()
        .map(|line| {
            let mut row = vec![b'.'];
            row.extend_from_slice(line.as_bytes());
            row.push(b'.');
            row
        })
        .collect::<Vec<_>>();
    let grid_width = grid.first().unwrap().len();
    grid.insert(0, vec![b'.'; grid_width]);
    grid.push(vec![b'.'; grid_width]);

    let grid = Grid::from_nested(&grid);

    let mut status_grid: Grid<Option<Status>> = Grid::new(grid.width, grid.height);

    let mut start_coord = (grid.width, grid.height);
    for y in 0..grid.height {
        for x in 0..grid.width {
            if grid[(x, y)] == b'S' {
                start_coord = (x, y);
                status_grid[(x, y)] = Some(Status::Boundary);
            }
        }
    }

    let mut queue = VecDeque::new();
    queue.push_back(start_coord);
    while let Some(current) = queue.pop_front() {
        let (x, y) = current;

        let up = (x, y - 1);
        if y > 0 && status_grid[up].is_none() {
            if matches!(
                (grid[current], grid[up]),
                (b'|' | b'L' | b'J' | b'S', b'|' | b'7' | b'F')
            ) {
                status_grid[up] = Some(Status::Boundary);
                queue.push_back(up);
            }
        }

        let down = (x, y + 1);
        if y < grid.height - 1 && status_grid[down].is_none() {
            match (grid[current], grid[down]) {
                (b'|' | b'7' | b'F' | b'S', b'|' | b'L' | b'J') => {
                    status_grid[down] = Some(Status::Boundary);
                    queue.push_back(down);
                }
                _ => {}
            }
        }

        let left = (x - 1, y);
        if x > 0 && status_grid[left].is_none() {
            match (grid[current], grid[left]) {
                (b'-' | b'J' | b'7' | b'S', b'-' | b'L' | b'F') => {
                    status_grid[left] = Some(Status::Boundary);
                    queue.push_back(left);
                }
                _ => {}
            }
        }

        let right = (x + 1, y);
        if x < grid.width - 1 && status_grid[right].is_none() {
            match (grid[current], grid[right]) {
                (b'-' | b'L' | b'F' | b'S', b'-' | b'J' | b'7') => {
                    status_grid[right] = Some(Status::Boundary);
                    queue.push_back(right);
                }
                _ => {}
            }
        }
    }

    // Flood fill outside
    flood_fill(&mut status_grid, (0, 0), Status::Exterior);

    // Go through loop
    let mut is_counter_clockwise = false;

    {
        let mut current_coord = start_coord;
        let mut prev_coord = current_coord;
        loop {
            let (x, y) = current_coord;

            let up = (x, y - 1);
            let down = (x, y + 1);
            let left = (x - 1, y);
            let right = (x + 1, y);
            if up != prev_coord
                && matches!(
                    (grid[current_coord], grid[up]),
                    (b'|' | b'L' | b'J' | b'S', b'|' | b'7' | b'F' | b'S')
                )
            {
                if status_grid[right] == Some(Status::Exterior) {
                    is_counter_clockwise = true;
                }
                prev_coord = current_coord;
                current_coord = up;
            } else if down != prev_coord
                && matches!(
                    (grid[current_coord], grid[down]),
                    (b'|' | b'7' | b'F' | b'S', b'|' | b'L' | b'J' | b'S')
                )
            {
                if status_grid[left] == Some(Status::Exterior) {
                    is_counter_clockwise = true;
                }
                prev_coord = current_coord;
                current_coord = down;
            } else if left != prev_coord
                && matches!(
                    (grid[current_coord], grid[left]),
                    (b'-' | b'J' | b'7' | b'S', b'-' | b'L' | b'F' | b'S')
                )
            {
                if status_grid[up] == Some(Status::Exterior) {
                    is_counter_clockwise = true;
                }
                prev_coord = current_coord;
                current_coord = left;
            } else if right != prev_coord
                && matches!(
                    (grid[current_coord], grid[right]),
                    (b'-' | b'L' | b'F' | b'S', b'-' | b'J' | b'7' | b'S')
                )
            {
                if status_grid[down] == Some(Status::Exterior) {
                    is_counter_clockwise = true;
                }
                prev_coord = current_coord;
                current_coord = right;
            } else {
                panic!("Should not happen!");
            }
            assert_ne!(current_coord, prev_coord);

            if current_coord == start_coord {
                break;
            }
        }
    }

    {
        let mut current_coord = start_coord;
        let mut prev_coord = current_coord;
        loop {
            let (x, y) = current_coord;

            let up = (x, y - 1);
            let down = (x, y + 1);
            let left = (x - 1, y);
            let right = (x + 1, y);
            if up != prev_coord
                && matches!(
                    (grid[current_coord], grid[up]),
                    (b'|' | b'L' | b'J' | b'S', b'|' | b'7' | b'F' | b'S')
                )
            {
                if is_counter_clockwise {
                    flood_fill(&mut status_grid, left, Status::Interior);
                    flood_fill(&mut status_grid, right, Status::Exterior);
                } else {
                    flood_fill(&mut status_grid, right, Status::Interior);
                    flood_fill(&mut status_grid, left, Status::Exterior);
                }
                prev_coord = current_coord;
                current_coord = up;
            } else if down != prev_coord
                && matches!(
                    (grid[current_coord], grid[down]),
                    (b'|' | b'7' | b'F' | b'S', b'|' | b'L' | b'J' | b'S')
                )
            {
                if is_counter_clockwise {
                    flood_fill(&mut status_grid, right, Status::Interior);
                    flood_fill(&mut status_grid, left, Status::Exterior);
                } else {
                    flood_fill(&mut status_grid, left, Status::Interior);
                    flood_fill(&mut status_grid, right, Status::Exterior);
                }
                prev_coord = current_coord;
                current_coord = down;
            } else if left != prev_coord
                && matches!(
                    (grid[current_coord], grid[left]),
                    (b'-' | b'J' | b'7' | b'S', b'-' | b'L' | b'F' | b'S')
                )
            {
                if is_counter_clockwise {
                    flood_fill(&mut status_grid, down, Status::Interior);
                    flood_fill(&mut status_grid, up, Status::Exterior);
                } else {
                    flood_fill(&mut status_grid, up, Status::Interior);
                    flood_fill(&mut status_grid, down, Status::Exterior);
                }
                prev_coord = current_coord;
                current_coord = left;
            } else if right != prev_coord
                && matches!(
                    (grid[current_coord], grid[right]),
                    (b'-' | b'L' | b'F' | b'S', b'-' | b'J' | b'7' | b'S')
                )
            {
                if is_counter_clockwise {
                    flood_fill(&mut status_grid, up, Status::Interior);
                    flood_fill(&mut status_grid, down, Status::Exterior);
                } else {
                    flood_fill(&mut status_grid, down, Status::Interior);
                    flood_fill(&mut status_grid, up, Status::Exterior);
                }
                prev_coord = current_coord;
                current_coord = right;
            } else {
                panic!("Should not happen!");
            }
            assert_ne!(current_coord, prev_coord);

            if current_coord == start_coord {
                break;
            }
        }
    }

    let mut result = 0;
    for y in 0..status_grid.height {
        for x in 0..status_grid.width {
            if status_grid[(x, y)] == Some(Status::Interior) {
                result += 1;
            }
        }
    }
    result
}

fn main() {
    assert_eq!(part1(INPUT), 6649);

    assert_eq!(part2(TEST_INPUT), 1);
    assert_eq!(part2(TEST_INPUT2), 1);
    assert_eq!(part2(TEST_INPUT3), 4);
    assert_eq!(part2(TEST_INPUT4), 4);
    assert_eq!(part2(TEST_INPUT5), 8);
    assert_eq!(part2(TEST_INPUT6), 10);

    assert_eq!(part2(TEST_INPUT7), 1);

    assert_eq!(part2(INPUT), 601);
}
