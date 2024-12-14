use std::{
    collections::HashMap, fs::File, io::{BufRead, BufReader}
};
use memoize::memoize;

pub fn start() {
    let start = std::time::Instant::now();
    let file = File::open("./input/day_11.txt").unwrap();
    let reader = BufReader::new(file);
    let mut values: Vec<u64> = vec![];
    for l in reader.lines() {
        let temp = l.unwrap();
        values = temp
            .split_whitespace()
            .map(|c| c.parse::<u64>().unwrap())
            .collect();
    }
    part_1(values.clone());
    part_2(values);
    println!("Time: {:?}", start.elapsed());
}

fn part_1(values: Vec<u64>) {
    let mut result = 0_u64;
    for v in values {
        result += blink_memo(v, 25);
    }
    println! {"{result}"};
}

fn part_2(values: Vec<u64>) {
    let mut result = 0_u64;
    for v in values {
        result += blink_memo(v, 75);
    }
    println! {"{result}"};
}

#[memoize]
fn blink_memo(v: u64, n: u8) -> u64 {
    if n == 0 { return 1; }
    let mut result = 0_u64;
    if v == 0 {
        result += blink_memo(1, n - 1);
    } else if count_digits(v) % 2 == 0 {
        let (a, b) = cleave(v);
        result += blink_memo(a, n-1);
        result += blink_memo(b, n-1);
    } else {
        result += blink_memo(v * 2024, n-1);
    }
    result
}


fn cleave(n: u64) -> (u64, u64) {
    let l = u64::pow(10, count_digits(n) / 2);
    let a = n % l;
    let b = n / l;
    (a, b)
}

fn count_digits(n: u64) -> u32 {
    let tmp = u64::ilog10(n) + 1;
    tmp
}
