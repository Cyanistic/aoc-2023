use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

const TEST1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";

const TEST2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

const TEST3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

pub fn part_1() -> usize {
    let contents = read_to_string("input/day-8.txt").unwrap();
    let (guide, node_list) = contents.split_once('\n').unwrap();
    let guide: Box<[char]> = guide.trim().chars().collect();
    let map: HashMap<&str, (&str, &str)> = node_list
        .trim()
        .lines()
        .map(|line| {
            let (key, vals) = line.split_once(" = ").unwrap();
            (key, vals[1..vals.len() - 1].split_once(", ").unwrap())
        })
        .collect();
    let mut steps = 0;
    let mut cur = "AAA";
    let mut i = 0;
    while cur != "ZZZ" {
        let selected = map.get(&cur).unwrap();
        match guide[i] {
            'L' => cur = selected.0,
            'R' => cur = selected.1,
            _ => unreachable!(),
        }
        i = match i + 1 < guide.len() {
            true => i + 1,
            false => 0,
        };
        steps += 1;
    }

    steps
}

pub fn part_2() -> usize {
    let contents = read_to_string("input/day-8.txt").unwrap();
    let (guide, node_list) = contents.split_once('\n').unwrap();
    let guide: Box<[char]> = guide.trim().chars().collect();
    let map: HashMap<&str, (&str, &str)> = node_list
        .trim()
        .lines()
        .map(|line| {
            let (key, vals) = line.split_once(" = ").unwrap();
            (key, vals[1..vals.len() - 1].split_once(", ").unwrap())
        })
        .collect();
    let mut cur: Vec<&str> = map
        .keys()
        .filter_map(|k| match k.ends_with('A') {
            true => Some(*k),
            false => None,
        })
        .collect();
    let mut steps = vec![0; cur.len()];
    for (k, c) in cur.iter_mut().enumerate() {
        let mut i = 0;
        while !c.ends_with('Z') {
            let selected = map.get(c).unwrap();
            match guide[i] {
                'L' => *c = selected.0,
                'R' => *c = selected.1,
                _ => unreachable!(),
            }
            i = match i + 1 < guide.len() {
                true => i + 1,
                false => 0,
            };
            steps[k] += 1;
        }
    }

    steps
        .iter()
        // Calculate prime factors of all paths in steps
        .flat_map(|x| (2..*x).filter(|y| *x % *y == 0 && (2..*y).all(|z| *y % z != 0)))
        // collect into hashset to remove duplicates
        .collect::<HashSet<usize>>()
        .iter()
        // Find lcm by calculating product
        .product::<usize>()
}
