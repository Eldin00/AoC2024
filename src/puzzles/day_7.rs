use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn start() {
    let start = std::time::Instant::now();
    let file = File::open("./input/day_7.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines: Vec<(u64, Vec<u64>)> = vec![];
    for l in reader.lines() {
        let temp = l.unwrap();
        let (t, v) = temp.split_once(':').unwrap();
        let t = t.parse::<u64>().unwrap();
        let v: Vec<u64> = v
            .split_whitespace()
            .filter(|s| !s.is_empty())
            .map(|n| n.parse::<u64>().unwrap_or_default())
            .collect();

        lines.push((t, v));
    }
    part_1(&lines);
    part_2(&lines);
    println!("Time: {:?}", start.elapsed());
}

fn part_1(lines: &Vec<(u64, Vec<u64>)>) {
    let mut result: u64 = 0;
    for l in lines {
        if try_add(l.1[0], l.1[1..].to_vec(), l.0) + try_mult(l.1[0], l.1[1..].to_vec(), l.0) > 0 {
            result += l.0;
        }
    }
    println!("{result}");
}

fn part_2(lines: &Vec<(u64, Vec<u64>)>) {
    let mut result: u64 = 0;
    for l in lines {
        if try_add2(l.1[0], l.1[1..].to_vec(), l.0)
            + try_mult2(l.1[0], l.1[1..].to_vec(), l.0)
            + try_concat(l.1[0], l.1[1..].to_vec(), l.0)
            > 0
        {
            result += l.0;
        }
    }
    println!("{result}");
}

fn try_add(n: u64, values: Vec<u64>, target: u64) -> u64 {
    let tmp = n + values[0];
    if tmp > target {
        return 0;
    }
    if values.len() == 1 {
        if tmp == target {
            return tmp;
        }
        return 0;
    }
    try_add(tmp, values[1..].to_vec(), target) + try_mult(tmp, values[1..].to_vec(), target)
}

fn try_mult(n: u64, values: Vec<u64>, target: u64) -> u64 {
    let tmp = n * values[0];
    if tmp > target {
        return 0;
    }
    if values.len() == 1 {
        if tmp == target {
            return tmp;
        }
        return 0;
    }
    try_add(tmp, values[1..].to_vec(), target) + try_mult(tmp, values[1..].to_vec(), target)
}

fn try_add2(n: u64, values: Vec<u64>, target: u64) -> u64 {
    let tmp = n + values[0];
    if tmp > target {
        return 0;
    }
    if values.len() == 1 {
        if tmp == target {
            //println!("add found {tmp}");
            return tmp;
        }
        return 0;
    }
    try_add2(tmp, values[1..].to_vec(), target)
        + try_mult2(tmp, values[1..].to_vec(), target)
        + try_concat(tmp, values[1..].to_vec(), target)
}

fn try_mult2(n: u64, values: Vec<u64>, target: u64) -> u64 {
    let tmp = n * values[0];
    if tmp > target {
        return 0;
    }
    if values.len() == 1 {
        if tmp == target {
            //println!("mult found {tmp}");
            return tmp;
        }
        return 0;
    }
    try_add2(tmp, values[1..].to_vec(), target)
        + try_mult2(tmp, values[1..].to_vec(), target)
        + try_concat(tmp, values[1..].to_vec(), target)
}

fn try_concat(n: u64, values: Vec<u64>, target: u64) -> u64 {
    let mut tmp = n.to_string();
    //println!("combining {n} with {}", values[0]);
    tmp.push_str(values[0].to_string().as_str());
    //println!("to get {tmp}");
    let tmp = tmp.parse::<u64>().unwrap();
    if tmp > target {
        return 0;
    }
    if values.len() == 1 {
        if tmp == target {
            //println!("concat found {tmp}");
            return tmp;
        }
        return 0;
    }
    try_add2(tmp, values[1..].to_vec(), target)
        + try_mult2(tmp, values[1..].to_vec(), target)
        + try_concat(tmp, values[1..].to_vec(), target)
}
