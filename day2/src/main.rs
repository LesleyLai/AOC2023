const INPUT: &'static str = include_str!("./input.txt");

struct Set {
    r: i32,
    g: i32,
    b: i32,
}

impl Set {
    fn new() -> Self {
        Set { r: 0, g: 0, b: 0 }
    }

    fn max(&self, other: &Self) -> Self {
        Set {
            r: self.r.max(other.r),
            g: self.g.max(other.g),
            b: self.b.max(other.b),
        }
    }
}

struct Game {
    id: i32,
    sets: Vec<Set>,
}

fn parse(input: &str) -> Option<Vec<Game>> {
    input
        .lines()
        .map(|line| {
            let (id, games) = line.strip_prefix("Game ")?.split_once(":")?;
            let id = id.parse().ok()?;
            let sets: Vec<_> = games
                .split("; ")
                .map(str::trim)
                .map(|set_str| {
                    let mut set = Set::new();
                    for single_color_cubes in set_str.split(", ") {
                        let (count, color) = single_color_cubes.split_once(" ").unwrap();
                        let count = count.parse().ok().unwrap();
                        match color {
                            "red" => set.r = count,
                            "green" => set.g = count,
                            "blue" => set.b = count,
                            _ => {}
                        }
                    }
                    set
                })
                .collect();

            Some(Game { id, sets })
        })
        .collect()
}

fn part1(games: &[Game]) -> i32 {
    let has_enough_balls = |game: &&Game| {
        game.sets
            .iter()
            .all(|set| set.r <= 12 && set.g <= 13 && set.b <= 14)
    };

    games
        .iter()
        .filter(has_enough_balls)
        .map(|game| game.id)
        .sum()
}

fn part2(games: &[Game]) -> i32 {
    games.iter().fold(0, |result, game| {
        let max_set = game.sets.iter().fold(Set::new(), |set, acc| set.max(acc));
        result + max_set.r * max_set.g * max_set.b
    })
}

fn main() {
    let data = parse(INPUT).unwrap();

    let result1 = part1(&data);
    let result2 = part2(&data);
    assert_eq!(result1, 3059);
    assert_eq!(result2, 65371);
}
