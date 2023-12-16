use crate::grid::Grid;

mod grid;

const TEST_INPUT: &str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
"#;

const INPUT: &str = include_str!("input.txt");

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}
use Direction::*;

impl Direction {
    fn to_vec2(self) -> (isize, isize) {
        match self {
            Left => (-1, 0),
            Right => (1, 0),
            Up => (0, -1),
            Down => (0, 1),
        }
    }

    fn tag(self) -> usize {
        match self {
            Left => 0,
            Right => 1,
            Up => 2,
            Down => 3,
        }
    }
}

type Vector2 = (isize, isize);

fn march(point: Vector2, dir: Direction) -> Vector2 {
    let dir = dir.to_vec2();
    (point.0 + dir.0, point.1 + dir.1)
}

#[derive(Copy, Clone, Debug, Hash)]
struct Ray {
    origin: Vector2,
    direction: Direction,
}

impl Ray {
    fn new(origin: Vector2, direction: Direction) -> Self {
        Ray { origin, direction }
    }

    fn march_one(self) -> Self {
        self.march_toward(self.direction)
    }

    fn march_toward(self, direction: Direction) -> Self {
        Ray {
            origin: march(self.origin, direction),
            direction,
        }
    }
}

fn energize(grid: &Grid<u8>, start: Ray) -> usize {
    use Direction::*;

    // ray marching
    let mut energized_grid: Grid<bool> = Grid::new(grid.width, grid.height);
    let mut traced_grid: Grid<[bool; 4]> = Grid::new(grid.width, grid.height);
    let mut stack = vec![start];

    while let Some(ray) = stack.pop() {
        if let Some(traced_dirs) = traced_grid.get_mut(ray.origin) {
            let traced = &mut traced_dirs[ray.direction.tag()];
            if *traced {
                continue;
            }
            *traced = true;
        } else {
            // Ray goes out of bound
            continue;
        }

        energized_grid[ray.origin] = true;

        match grid[ray.origin] {
            b'|' if matches!(ray.direction, Left | Right) => {
                stack.push(ray.march_toward(Up));
                stack.push(ray.march_toward(Down));
            }
            b'-' if matches!(ray.direction, Up | Down) => {
                stack.push(ray.march_toward(Left));
                stack.push(ray.march_toward(Right));
            }
            b'/' => {
                let new_dir = match ray.direction {
                    Left => Down,
                    Right => Up,
                    Up => Right,
                    Down => Left,
                };
                stack.push(ray.march_toward(new_dir));
            }
            b'\\' => {
                let new_dir = match ray.direction {
                    Left => Up,
                    Right => Down,
                    Up => Left,
                    Down => Right,
                };
                stack.push(ray.march_toward(new_dir));
            }
            _ => {
                // pass through
                stack.push(ray.march_one())
            }
        }
    }

    energized_grid
        .rows()
        .map(|row| row.iter().filter(|&&b| b).count())
        .sum()
}

fn parse(input: &str) -> Grid<u8> {
    let nested: Vec<_> = input.lines().map(|line| line.as_bytes().to_vec()).collect();
    Grid::from_nested(&nested)
}

fn part1(grid: &Grid<u8>) -> usize {
    energize(grid, Ray::new((0, 0), Right))
}

fn part2(grid: &Grid<u8>) -> usize {
    let starts = (0..grid.width)
        .map(|x| Ray::new((x, 0), Down))
        .chain((0..grid.width).map(|x| Ray::new((x, grid.height - 1), Up)))
        .chain((0..grid.height).map(|y| Ray::new((0, y), Left)))
        .chain((0..grid.height).map(|y| Ray::new((grid.width - 1, y), Right)));

    starts.map(|start| energize(&grid, start)).max().unwrap()
}

fn main() {
    let test_grid = parse(TEST_INPUT);
    let grid = parse(INPUT);

    assert_eq!(part1(&test_grid), 46);
    assert_eq!(part1(&grid), 7498);

    assert_eq!(part2(&test_grid), 51);
    assert_eq!(part2(&grid), 7846);
}
