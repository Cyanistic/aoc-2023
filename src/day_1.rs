use std::{collections::HashMap, fs::read_to_string};
pub fn part_1() -> usize {
    let contents = read_to_string("input/day-1.txt").unwrap();
    contents
        .lines()
        .map(|x| {
            x.chars()
                .find(|y| y.is_ascii_digit())
                .map(|y| y as u8 - b'0')
                .unwrap() as usize
                * 10
                + x.chars()
                    .rfind(|y| y.is_ascii_digit())
                    .map(|y| y as u8 - b'0')
                    .unwrap() as usize
        })
        .sum::<usize>()
}

pub fn part_2() -> usize {
    let digits: HashMap<&str, usize> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);
    let contents = read_to_string("input/day-1.txt").unwrap();
    contents
        .lines()
        .map(|x| {
            digits
                .iter()
                .filter_map(|(&k, &v)| x.find(k).map(|ind| (ind, v)))
                .min_by_key(|(k, _)| *k)
                .unwrap()
                .1
                * 10
                + digits
                    .iter()
                    .filter_map(|(&k, &v)| x.rfind(k).map(|ind| (ind, v)))
                    .max_by_key(|(k, _)| *k)
                    .unwrap()
                    .1
        })
        .sum::<usize>()
}
