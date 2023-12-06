use std::fs::read_to_string;

const TEST: &str = "Time:      7  15   30
Distance:  9  40  200";

pub fn part_1() -> usize {
    let contents = read_to_string("input/day-6.txt").unwrap();
    let (time, distance) = contents.split_once('\n').unwrap();
    let time = time
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split_ascii_whitespace()
        .map(|t| t.parse().unwrap());
    let distance = distance
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split_ascii_whitespace()
        .map(|d| d.parse().unwrap());
    time.zip(distance)
        .map(|(t, d)| (0..=t).filter(|i| i * (t - i) > d).count())
        .product()
}

pub fn part_2() -> usize {
    let contents = read_to_string("input/day-6.txt").unwrap();
    let (time, distance) = contents.split_once('\n').unwrap();
    let time: usize = time
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .replace(' ', "")
        .parse()
        .unwrap();
    let distance = distance
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .replace(' ', "")
        .parse()
        .unwrap();
    (0..=time).filter(|i| i * (time - i) > distance).count()
}
