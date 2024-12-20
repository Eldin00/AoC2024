use std::{
    collections::{HashSet, VecDeque}, fs::File, io::{BufRead, BufReader}
};

pub fn start() {
    let start = std::time::Instant::now();
    let file = File::open("./input/day_19.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = vec![];
    for l in reader.lines() {
        let temp = l.unwrap();
        lines.push(temp);
    }
    //println!("{lines:?}");
    let mut towels: Vec<String> = lines[0].split(", ").map(|s| String::from(s)).collect();
    towels.sort_by(|a,b| a.len().cmp(&b.len()));
    part_1(&lines[2..], towels);
    //part_2(&lines[2..], towels);
    println!("Time: {:?}", start.elapsed());
}

fn part_1(lines: &[String], towels: Vec<String>) {
    let mut result: u32 = 0;
    let r_towels = reduce_towels(towels.clone());
    let mut matches: Vec<String> = vec![];
    for l in lines {
        if check_pattern(l, &r_towels){
            result += 1;
            matches.push(String::from(l));
        }
    }
    println!("{result}");
    part_2(matches.as_slice(), towels);
}

fn part_2(lines: &[String], mut towels: Vec<String>) {
    let mut result: u32 = 0;
    towels.reverse();
    for l in lines{
        result += count_matches(l, &towels);
    }
    println!("{result} too slow or computing wrong."); 
}

fn reduce_towels(mut t: Vec<String>) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    while t.len() > 0 {
        let p = t.pop().unwrap();
        if !check_pattern(&p, &t) { result.push(String::from(p)); }
    }
    result
}

fn check_pattern(pat: &str, t: &Vec<String>) -> bool {
    let mut stack: Vec<&str> = vec![pat];
    while stack.len() > 0 {
        let p = stack.pop().unwrap();
        for s in t {

            if p.starts_with(s) && p.len() == s.len() {
                return true;
            }
            if p.starts_with(s) {
                stack.push(p.strip_prefix(s).unwrap())
            }
        }
    }
    false
}

fn count_matches(pat: &str, towels: &Vec<String>) -> u32 {
    let mut stack: Vec<&str> = vec![pat];
    let mut result: u32 = 0;
    while stack.len() > 0 {
        let p = stack.pop().unwrap();
        let t: Vec<String> = towels.iter().filter(|v| p.contains(*v)).map(|v| String::from(v)).collect();
        for s in t {
            if p.starts_with(&s) && p.len() == s.len() {
                result += 1;
                if result % 10000 == 0 { println!("Found match #{result} for {pat}"); }
            } 
            if p.starts_with(&s) {
                stack.push(p.strip_prefix(&s).unwrap())
            }
        }
    }
    result
}

