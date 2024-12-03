use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn start() {
    let file = File::open("./input/day_3.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = vec![];
    for l in reader.lines() {
        let temp = l.unwrap();
        lines.push(temp);
    }
    part_1(lines.clone());
    part_2(lines);
}

fn part_1(lines: Vec<String>) {
    let mut final_value: u32 = 0;
    let mut results: Vec<&str> = vec![];
    let re = Regex::new(r"(mul\(\d+\,\d+\))").unwrap();
    for i in 0..lines.len() {
        let matches = re.captures_iter(lines[i].as_str());
        for m in matches {
            results.push(
                m.iter()
                    .filter(|x| x.is_some())
                    .map(|x| x.unwrap().as_str())
                    .collect::<Vec<&str>>()[0],
            );
        }
    }

    for mul in results {
        let tmp = mul.replace(&['m', 'u', 'l', '(', ')'], "");

        let x: Vec<u32> = tmp.split(',').map(|n| n.parse::<u32>().unwrap()).collect();
        final_value += x[0] * x[1];
    }

    println!("{final_value}");
}

fn part_2(lines: Vec<String>) {
    let mut final_value: u32 = 0;
    let mut results: Vec<&str> = vec![];
    let re = Regex::new(r"(mul\(\d+\,\d+\))|(do\(\))|(don't\(\))").unwrap();
    for i in 0..lines.len() {
        let matches = re.captures_iter(lines[i].as_str());
        for m in matches {
            results.push(
                m.iter()
                    .filter(|x| x.is_some())
                    .map(|x| x.unwrap().as_str())
                    .collect::<Vec<&str>>()[0],
            );
        }
    }

    let mut inactive = false;
    for mul in results {
        if mul.starts_with("do(") {
            inactive = false;
        }
        if mul.starts_with("don") {
            inactive = true
        }
        if inactive {
            continue;
        }
        if mul.starts_with("mul") {
            let tmp = mul.replace(&['m', 'u', 'l', '(', ')'], "");
            let x: Vec<u32> = tmp.split(',').map(|n| n.parse::<u32>().unwrap()).collect();
            final_value += x[0] * x[1];
        }
    }

    println!("{final_value}");
}
