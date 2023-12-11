use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

const TEST: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

pub fn part_1() -> usize {
    let contents = read_to_string("input/day-11.txt").unwrap();
    let grid: Box<[Box<[char]>]> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let galaxies: Vec<(usize, usize)> = grid
        .iter()
        .enumerate()
        .flat_map(|(ok, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, x)| **x == '#')
                .map(move |(ik, _)| (ok, ik))
        })
        .collect();
    let empty_rows: HashSet<usize> = grid
        .iter()
        .enumerate()
        .filter_map(|(k, v)| v.iter().all(|x| *x == '.').then_some(k))
        .collect();

    let mut empty_cols: HashSet<usize> = HashSet::new();
    'outer: for i in 0..grid[0].len() {
        for j in 0..grid.len() {
            if grid[j][i] != '.' {
                continue 'outer;
            }
        }
        empty_cols.insert(i);
    }

    let mut sum = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            sum += galaxies[j].0 - galaxies[i].0
                + (galaxies[j].1 as isize - galaxies[i].1 as isize).unsigned_abs()
                + (galaxies[i].0..galaxies[j].0)
                    .filter(|x| empty_rows.contains(x))
                    .count()
                + (galaxies[i].1.min(galaxies[j].1)..galaxies[i].1.max(galaxies[j].1))
                    .filter(|x| empty_cols.contains(x))
                    .count();
        }
    }
    sum
}

pub fn part_2() -> usize {
    let contents = read_to_string("input/day-11.txt").unwrap();
    let grid: Box<[Box<[char]>]> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let galaxies: Vec<(usize, usize)> = grid
        .iter()
        .enumerate()
        .flat_map(|(ok, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, x)| **x == '#')
                .map(move |(ik, _)| (ok, ik))
        })
        .collect();
    let empty_rows: HashSet<usize> = grid
        .iter()
        .enumerate()
        .filter_map(|(k, v)| v.iter().all(|x| *x == '.').then_some(k))
        .collect();

    let mut empty_cols: HashSet<usize> = HashSet::new();
    'outer: for i in 0..grid[0].len() {
        for j in 0..grid.len() {
            if grid[j][i] != '.' {
                continue 'outer;
            }
        }
        empty_cols.insert(i);
    }

    let mut sum = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            sum += galaxies[j].0 - galaxies[i].0
                + (galaxies[j].1 as isize - galaxies[i].1 as isize).unsigned_abs()
                + (galaxies[i].0..galaxies[j].0)
                    .filter(|x| empty_rows.contains(x))
                    .count()
                    * 999999
                + (galaxies[i].1.min(galaxies[j].1)..galaxies[i].1.max(galaxies[j].1))
                    .filter(|x| empty_cols.contains(x))
                    .count()
                    * 999999;
        }
    }
    sum
}
