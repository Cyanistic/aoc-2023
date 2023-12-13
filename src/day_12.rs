use std::{collections::HashMap, fs::read_to_string, usize};

const TEST: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

pub fn part_1() -> usize {
    let contents = read_to_string("input/day-12.txt").unwrap();
    contents
        .lines()
        .map(|line| {
            let (rows, nums) = line.split_once(' ').unwrap();
            let nums: Vec<usize> = nums
                .trim()
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            recurse(
                &rows.chars().collect::<Box<[char]>>(),
                &nums,
                &mut HashMap::new(),
                0,
                0,
                0,
            )
        })
        .sum::<usize>()
}

// Logic adapted from https://github.com/jonathanpaulson/AdventOfCode/blob/master/2023/12.py
// I did make my own recursive function for part 1 but it didn't work for part 2 and I couldn't
// find a way to memoize it easily so i just copied this one
fn recurse(
    rows: &[char],
    nums: &[usize],
    cache: &mut HashMap<(usize, usize, usize), usize>,
    ind: usize,
    b_ind: usize,
    b_len: usize,
) -> usize {
    let key = (ind, b_ind, b_len);
    if let Some(entry) = cache.get(&key) {
        return *entry;
    }
    if ind == rows.len() {
        if (b_ind == nums.len() && b_len == 0) || (b_ind == nums.len() - 1 && b_len == nums[b_ind])
        {
            return 1;
        } else {
            return 0;
        }
    }
    let mut res = 0;
    for c in ['.', '#'] {
        if rows[ind] == c || rows[ind] == '?' {
            if c == '.' && b_len == 0 {
                res += recurse(rows, nums, cache, ind + 1, b_ind, 0);
            } else if c == '.' && nums.get(b_ind).is_some_and(|x| *x == b_len) {
                res += recurse(rows, nums, cache, ind + 1, b_ind + 1, 0);
            } else if c == '#' {
                res += recurse(rows, nums, cache, ind + 1, b_ind, b_len + 1);
            }
        }
    }
    cache.insert(key, res);
    res
}

pub fn part_2() -> usize {
    const REPEATS: usize = 5;
    let contents = read_to_string("input/day-12.txt").unwrap();
    contents
        .lines()
        .map(|line| {
            let (rows, nums) = line.split_once(' ').unwrap();
            let nums: Vec<usize> = [nums.trim(); REPEATS]
                .join(",")
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            recurse(
                &[rows; REPEATS].join("?").chars().collect::<Box<[char]>>(),
                &nums,
                &mut HashMap::new(),
                0,
                0,
                0,
            )
        })
        .sum::<usize>()
}
