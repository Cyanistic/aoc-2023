use std::{collections::HashMap, fs::read_to_string};

const TEST: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

pub fn part_1() -> usize {
    let contents = read_to_string("input/day-14.txt").unwrap();
    let mut cur_pos: HashMap<usize, usize> = HashMap::new();
    let line_count = contents.lines().count();
    let mut sum = 0;
    for (row, line) in contents.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            match char {
                'O' => {
                    cur_pos
                        .entry(col)
                        .and_modify(|x| *x = x.saturating_sub(1))
                        .or_insert(line_count);
                    sum += cur_pos.get(&col).unwrap();
                }
                '#' => {
                    cur_pos.insert(col, line_count.saturating_sub(row));
                }
                '.' => (),
                _ => unreachable!(),
            }
        }
    }
    sum
}

pub fn part_2() -> usize {
    const ITERATIONS: usize = 1000000000;
    let contents = read_to_string("input/day-14.txt").unwrap();
    let mut grid: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let mut cache: HashMap<Vec<Vec<char>>, usize> = HashMap::new();
    let mut i = 0;
    while i < ITERATIONS {
        if let Some(entry) = cache.get(&grid) {
            i = ITERATIONS - ((ITERATIONS - i) % (i - entry));
            cache.clear();
        }
        cache.insert(grid.clone(), i);
        for _ in 0..4 {
            let mut new_grid = vec![vec!['.'; grid.len()]; grid[0].len()];
            let mut cur_pos: HashMap<usize, usize> = HashMap::new();
            for row in 0..grid.len() {
                for col in 0..grid[row].len() {
                    match grid[row][col] {
                        'O' => {
                            cur_pos
                                .entry(col)
                                .and_modify(|x| *x = x.saturating_sub(1))
                                .or_insert(grid.len());
                            new_grid[col][*cur_pos.get(&col).unwrap() - 1] = 'O';
                        }
                        '#' => {
                            new_grid[col][grid.len() - row - 1] = '#';
                            cur_pos.insert(col, grid.len().saturating_sub(row));
                        }
                        '.' => (),
                        _ => unreachable!(),
                    }
                }
            }
            grid = new_grid;
        }
        i += 1;
    }
    grid.iter()
        .enumerate()
        .map(|(ok, ov)| {
            ov.iter()
                .filter_map(|iv| (*iv == 'O').then_some(grid.len() - ok))
                .sum::<usize>()
        })
        .sum()
}
