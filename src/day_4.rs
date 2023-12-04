use std::{collections::HashMap, fs::read_to_string};

const TEST: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

pub fn part_1() -> usize {
    let contents = read_to_string("input/day-4.txt").unwrap();
    contents
        .lines()
        .map(|line| {
            let (card, you) = line.split_once('|').unwrap();
            let nums = card
                .split_once(':')
                .unwrap()
                .1
                .trim()
                .split(' ')
                .filter(|x| !x.is_empty())
                .collect::<Vec<&str>>();
            let points = you.trim().split(' ').filter(|x| nums.contains(x)).count();
            if points > 1 {
                2usize.pow(points as u32 - 1)
            } else {
                points
            }
        })
        .sum()
}

pub fn part_2() -> usize {
    let contents = read_to_string("input/day-4.txt").unwrap();
    let mut cards: HashMap<usize, usize> = HashMap::new();
    contents
        .lines()
        .enumerate()
        .map(|(k, line)| {
            let (card, you) = line.split_once('|').unwrap();
            let nums = card
                .split_once(':')
                .unwrap()
                .1
                .trim()
                .split(' ')
                .filter(|x| !x.is_empty())
                .collect::<Vec<&str>>();
            let points = you
                .trim()
                .split(' ')
                .filter(|x| !x.is_empty() && nums.contains(x))
                .count();
            let times = *cards.get(&k).unwrap_or(&0) + 1;
            for i in 1..=points {
                cards
                    .entry(k + i)
                    .and_modify(|x| *x += times)
                    .or_insert(times);
            }
            times
        })
        .sum()
}
