use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn start() {
    let start = std::time::Instant::now();
    let file = File::open("./input/day_11.txt").unwrap();
    let reader = BufReader::new(file);
    let mut values: Vec<u64> = vec![];
    for l in reader.lines() {
        let temp = l.unwrap();
        values = temp.split_whitespace().map(|c| c.parse::<u64>().unwrap()).collect();
    }
    part_1(values.clone());
    println!("Time: {:?}", start.elapsed());
}

fn part_1(mut values: Vec<u64>) {
    for _ in 0..25 {
        values = blink(values);
    }
    println!{"{}", values.len()};
    part_2(values);
}

fn part_2(values: Vec<u64>) {
    let mut result: u64 = 0;
    let mut x = 0;
    for n in values {
        println!("{x}");
        x += 1;
        let mut tmpvec: Vec<u64> = vec![n];
        for i in 0..50 {
            if i % 10 == 0 {println!("at {i}");}
            tmpvec = blink(tmpvec);
        }
        result += tmpvec.len() as u64;
    }
    println!("{}", result);
}

fn blink(v: Vec<u64>) -> Vec<u64> {
    let mut tmp: Vec<u64> = vec![];
    for i in v {
        if i == 0 {
            tmp.push(1);
        } else if count_digits(i) % 2 == 0 {
            let (a, b) = cleave(i);
            tmp.push(a);
            tmp.push(b);             
        }
        else {
            tmp.push(i * 2024);
        }
    }
    tmp
}

fn cleave(n: u64) -> (u64, u64){
    let l = u64::pow(10,count_digits(n)/2);
    let a = n % l;
    let b = n / l;
    (a, b)
}

fn count_digits(n: u64) -> u32 {
    let tmp = u64::ilog10(n) + 1;
    tmp
}