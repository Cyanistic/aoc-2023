use std::fs::read_to_string;

const TEST: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

pub fn part_1() -> usize {
    let contents = read_to_string("input/day-15.txt").unwrap();
    contents
        .trim()
        .split(',')
        .map(|group| {
            group.chars().fold(0, |mut acc, x| {
                acc += x as usize;
                acc *= 17;
                acc %= 256;
                acc
            })
        })
        .sum()
}

pub fn part_2() -> usize {
    let contents = read_to_string("input/day-15.txt").unwrap();
    let mut boxes: Vec<Vec<(&str, usize)>> = vec![Vec::new(); 256];
    for group in contents.trim().split(',') {
        match group.split_once('=') {
            Some((label, num)) => {
                let box_num = label.chars().fold(0, |mut acc, x| {
                    acc += x as usize;
                    acc *= 17;
                    acc %= 256;
                    acc
                });
                match boxes[box_num].iter_mut().find(|(l, _)| l == &label) {
                    Some(lens) => lens.1 = num.parse().unwrap(),
                    None => boxes[box_num].push((label, num.parse().unwrap())),
                }
            }
            None => {
                let box_num = group[..group.len() - 1].chars().fold(0, |mut acc, x| {
                    acc += x as usize;
                    acc *= 17;
                    acc %= 256;
                    acc
                });
                if let Some(pos) = boxes[box_num]
                    .iter()
                    .position(|(l, _)| *l == &group[..group.len() - 1])
                {
                    boxes[box_num].remove(pos);
                }
            }
        }
    }
    boxes
        .into_iter()
        .enumerate()
        .map(|(ok, ov)| {
            ov.into_iter()
                .enumerate()
                .map(|(ik, (_, num))| (ok + 1) * (ik + 1) * num)
                .sum::<usize>()
        })
        .sum::<usize>()
}
