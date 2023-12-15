use std::collections::HashMap;

const TEST_INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

const INPUT: &str = include_str!("input.txt");

fn hash(val: &str) -> usize {
    val.as_bytes()
        .iter()
        .fold(0, |acc, c| (acc + *c as usize) * 17 % 256)
}

fn part1(input: &str) -> usize {
    input.split(",").map(|step| hash(step)).sum()
}

fn part2(input: &str) -> usize {
    let mut label_to_box: HashMap<String, usize> = HashMap::new();
    let mut boxes: Box<[Vec<(String, usize)>]> = vec![vec![]; 256].into_boxed_slice();

    for step in input.split(",") {
        if step.ends_with("-") {
            let label = step.trim_end_matches("-");
            if label_to_box.contains_key(label) {
                let box_index = *label_to_box.get(label).unwrap();

                label_to_box.remove(label);
                let remove_index = boxes[box_index]
                    .iter()
                    .position(|(l, _)| l == label)
                    .unwrap();
                boxes[box_index].remove(remove_index);
            }
        } else {
            let (label, focal_length) = step.split_once("=").unwrap();
            let focal_length: usize = focal_length.parse().unwrap();
            let box_index: usize = hash(label);
            match label_to_box.get(label) {
                None => {
                    label_to_box.insert(label.into(), box_index);
                    boxes[box_index].push((label.into(), focal_length));
                }
                Some(&index) => {
                    assert_eq!(index, box_index);
                    let (_, old_focal_length) = boxes[box_index]
                        .iter_mut()
                        .find(|(l, _)| l == label)
                        .unwrap();
                    *old_focal_length = focal_length;
                }
            }
        }
    }

    let mut sum = 0;
    for (i, b) in boxes.iter().enumerate() {
        for (slot_index, (_, focal_length)) in b.iter().enumerate() {
            sum += (i + 1) * (slot_index + 1) * focal_length;
        }
    }

    sum
}

fn main() {
    assert_eq!(hash("HASH"), 52);

    assert_eq!(part1(TEST_INPUT), 1320);
    assert_eq!(part1(INPUT), 511343);

    assert_eq!(part2(TEST_INPUT), 145);
    assert_eq!(part2(INPUT), 294474);
}
