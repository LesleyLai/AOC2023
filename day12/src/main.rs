use std::cell::RefCell;
use std::collections::HashMap;

#[allow(dead_code)]
const TEST_INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";

const INPUT: &str = include_str!("./input.txt");

#[derive(Hash, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Entry {
    row_index: usize,
    contiguous_group_index: usize,
}

struct ArrangementCountMemo<'a> {
    row: &'a [u8],
    continuous_groups: &'a [usize],
    memo_table: RefCell<HashMap<Entry, usize>>,
}

impl<'a> ArrangementCountMemo<'a> {
    fn new(row: &'a [u8], continuous_groups: &'a [usize]) -> Self {
        ArrangementCountMemo {
            memo_table: RefCell::new(HashMap::new()),
            row,
            continuous_groups,
        }
    }

    fn arrangement_count(&self, row_index: usize, contiguous_group_index: usize) -> usize {
        let entry = Entry {
            row_index,
            contiguous_group_index,
        };
        if let Some(memoized) = self.memo_table.borrow().get(&entry) {
            return *memoized;
        }

        let row = &self.row[row_index..];
        let continuous_groups = &self.continuous_groups[contiguous_group_index..];

        let skip_first_count = || self.arrangement_count(row_index + 1, contiguous_group_index);

        let res = match (row, continuous_groups) {
            // end of input
            (&[], &[]) => 1,
            // dead end
            (&[], _) => 0,
            // Should have no damaged left
            (_, &[]) => {
                if row.iter().all(|b| *b != b'#') {
                    1
                } else {
                    0
                }
            }
            // dead end
            _ if row.len() < continuous_groups[0] => 0,
            // remaining cases
            (&[first, ..], &[continuous_count, ..]) => {
                match first {
                    b'?' => {
                        if row[0..continuous_count].iter().all(|b| *b != b'.') {
                            if row.len() > continuous_count && row[continuous_count] != b'#' {
                                self.arrangement_count(
                                    row_index + continuous_count + 1,
                                    contiguous_group_index + 1,
                                ) + skip_first_count()
                            } else if row.len() == continuous_count {
                                self.arrangement_count(
                                    row_index + continuous_count,
                                    contiguous_group_index + 1,
                                )
                            } else {
                                skip_first_count()
                            }
                        } else {
                            skip_first_count()
                        }
                    }
                    b'#' => {
                        if row[0..continuous_count].iter().all(|b| *b != b'.') {
                            if row.len() > continuous_count && row[continuous_count] != b'#' {
                                self.arrangement_count(
                                    row_index + continuous_count + 1,
                                    contiguous_group_index + 1,
                                )
                            } else if row.len() == continuous_count {
                                self.arrangement_count(
                                    row_index + continuous_count,
                                    contiguous_group_index + 1,
                                )
                            } else {
                                // dead end
                                0
                            }
                        } else {
                            // dead end
                            0
                        }
                    }
                    b'.' => skip_first_count(),
                    _ => unreachable!("Bad input"),
                }
            }
        };

        self.memo_table.borrow_mut().insert(entry, res);

        res
    }
}

fn arrangement_count(row: &[u8], continuous_groups: &[usize]) -> usize {
    ArrangementCountMemo::new(row, continuous_groups).arrangement_count(0, 0)
}

fn part1(input: &str) -> usize {
    let records: Vec<_> = input
        .lines()
        .map(|line| {
            let (row, continuous_groups) = line.split_once(" ").unwrap();
            (
                row.as_bytes(),
                continuous_groups
                    .split(",")
                    .filter_map(|token| token.parse().ok())
                    .collect::<Vec<usize>>()
                    .into_boxed_slice(),
            )
        })
        .collect();

    records
        .iter()
        .map(|(row, continuous_groups)| arrangement_count(row, &continuous_groups))
        .sum()
}

fn part2(input: &str) -> usize {
    let records: Vec<_> = input
        .lines()
        .map(|line| {
            let (row, continuous_groups) = line.split_once(" ").unwrap();
            let unfolded_row = format!("{row}?{row}?{row}?{row}?{row}");
            let continuous_groups = continuous_groups
                .split(",")
                .filter_map(|token| token.parse().ok());
            let continuous_groups_count = continuous_groups.clone().count();
            let unfolded_continuous_group: Vec<_> = continuous_groups
                .cycle()
                .take(continuous_groups_count * 5)
                .collect();

            (unfolded_row, unfolded_continuous_group.into_boxed_slice())
        })
        .collect();

    records
        .iter()
        .map(|(row, continuous_groups)| arrangement_count(row.as_bytes(), &continuous_groups))
        .sum()
}

fn main() {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arrangement_count() {
        assert_eq!(arrangement_count(b"#.#.###", &[1, 1, 3]), 1);
        assert_eq!(arrangement_count(b".#...#....###.", &[1, 1, 3]), 1);
        assert_eq!(arrangement_count(b".#.###.#.######", &[1, 3, 1, 6]), 1);
        assert_eq!(arrangement_count(b"####.#...#...", &[4, 1, 1]), 1);
        assert_eq!(arrangement_count(b"#....######..#####.", &[1, 6, 5]), 1);
        assert_eq!(arrangement_count(b".###.##....#", &[3, 2, 1]), 1);

        assert_eq!(arrangement_count(b"???", &[1]), 3);
        assert_eq!(arrangement_count(b"????", &[1]), 4);

        assert_eq!(arrangement_count(b"???.###", &[1, 1, 3]), 1);
        assert_eq!(arrangement_count(b".??..??...?##.", &[1, 1, 3]), 4);
        assert_eq!(arrangement_count(b"?#?#?#?#?#?#?#?", &[1, 3, 1, 6]), 1);
        assert_eq!(arrangement_count(b"????.#...#..", &[4, 1, 1]), 1);
        assert_eq!(arrangement_count(b"????.######..#####.", &[1, 6, 5]), 4);
        assert_eq!(arrangement_count(b"?###????????", &[3, 2, 1]), 10);

        assert_eq!(arrangement_count(b"??#???#?##??.", &[4, 5]), 3);
        assert_eq!(
            arrangement_count(b".#?#??#.????#?#??#?", &[4, 1, 1, 5, 1],),
            3
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 21);
        assert_eq!(part1(INPUT), 7110);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 525152);
        assert_eq!(part2(INPUT), 1566786613613);
    }
}
