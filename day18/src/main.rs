mod direction;
mod vec2;

use crate::direction::Direction;
use crate::vec2::Vec2;
use std::isize;

const TEST_INPUT: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
";

const INPUT: &str = include_str!("./input.txt");

impl Direction {
    pub fn from_byte(byte: u8) -> Self {
        match byte {
            b'U' => Direction::Up,
            b'D' => Direction::Down,
            b'L' => Direction::Left,
            b'R' => Direction::Right,
            _ => panic!("Bad input!"),
        }
    }
}

fn part1(input: &str) -> isize {
    let parsed_input: Vec<_> = input
        .lines()
        .map(|line| {
            let [dir, distance, _]: [&str; 3] = line
                .split_whitespace()
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            let dir = Direction::from_byte(dir.as_bytes()[0]);
            let distance: isize = distance.parse().unwrap();

            (dir, distance)
        })
        .collect();

    solve(&parsed_input)
}

fn part2(input: &str) -> isize {
    let parsed_input: Vec<_> = input
        .lines()
        .map(|line| {
            let [_, _, color]: [&str; 3] = line
                .split_whitespace()
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();

            let color = color.trim_start_matches("(#").trim_end_matches(")");
            let distance = isize::from_str_radix(&color[0..color.len() - 1], 16).unwrap();

            let direction = match color.as_bytes().last().unwrap() {
                b'0' => Direction::Right,
                b'1' => Direction::Down,
                b'2' => Direction::Left,
                b'3' => Direction::Up,
                _ => panic!("Bad input"),
            };

            (direction, distance)
        })
        .collect();

    solve(&parsed_input)
}

fn shoelace_theorem(vertices: &[Vec2]) -> isize {
    // Shoelace Theorem for area
    // https://artofproblemsolving.com/wiki/index.php/Shoelace_Theorem
    let mut two_area = 0;
    for i in 0..vertices.len() {
        let (current, next) = (vertices[i], vertices[(i + 1) % vertices.len()]);
        two_area += current.x * next.y - current.y * next.x;
    }
    two_area / 2
}

fn solve(parsed_input: &[(Direction, isize)]) -> isize {
    let vertices: Vec<_> = parsed_input
        .iter()
        .scan(Vec2::new(0, 0), |current, &(dir, distance)| {
            *current = *current + Vec2::from(dir) * distance;
            Some(*current)
        })
        .collect();

    let boundary_point_count: isize = parsed_input.iter().map(|(_, distance)| distance).sum();

    let area = shoelace_theorem(&vertices);

    // Picks theorem
    // https://artofproblemsolving.com/wiki/index.php/Pick%27s_Theorem
    // area = interior_point_count + 1 / 2 * boundary_point_count - 1
    // interior_point_count = area - 1 / 2 * boundary_point_count + 1
    // total points = interior_point_count + boundary_point_count = area + 1 / 2 * boundary_point_count + 1
    area + boundary_point_count / 2 + 1
}

fn main() {
    assert_eq!(part1(TEST_INPUT), 62);
    assert_eq!(part1(INPUT), 70026);

    assert_eq!(part2(TEST_INPUT), 952408144115);
    assert_eq!(part2(INPUT), 68548301037382);
}
