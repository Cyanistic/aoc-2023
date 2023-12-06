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
    // O(r*t) solution
    // r = # of races
    // t = time
    // time.zip(distance)
    //     .map(|(t, d): (usize, usize)| (0..=t).filter(|i| i * (t - i) > d).count())
    //     .product()
    //
    // O(r) solution
    time.zip(distance)
        .map(|(t, d): (f64, f64)| {
            (((-t - (t * t - 4.0 * d).sqrt()) / (-2.0)).ceil() as usize - 1)
                - (((-t + (t * t - 4.0 * d).sqrt()) / (-2.0)) as usize)
        })
        .product()
}

pub fn part_2() -> usize {
    let contents = read_to_string("input/day-6.txt").unwrap();
    let (time, distance) = contents.split_once('\n').unwrap();
    let time: f64 = time
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .replace(' ', "")
        .parse()
        .unwrap();
    let distance: f64 = distance
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .replace(' ', "")
        .parse()
        .unwrap();
    // O(t) solution
    // (0..=time).filter(|i| i * (time - i) > distance).count()
    // O(1) solution
    (((-time - (time * time - 4.0 * distance).sqrt()) / (-2.0)).ceil() as usize - 1)
        - (((-time + (time * time - 4.0 * distance).sqrt()) / (-2.0)) as usize)
}
