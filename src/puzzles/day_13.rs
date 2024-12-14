use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn start() {
    let start = std::time::Instant::now();
    let file = File::open("./input/day_13s.txt").unwrap();
    let reader = BufReader::new(file);
    //let mut lines: Vec<String> = vec![];
    let mut machines: Vec<Machine> = vec![];
    let re = Regex::new(r"(\d+)").unwrap();
    let mut a: (u64, u64) = (0, 0);
    let mut b: (u64, u64) = (0, 0);
    let mut prize: (u64, u64) = (0, 0);
    for l in reader.lines() {
        let temp = l.unwrap();
        if temp.starts_with("Button A") {
            let matches = re.captures_iter(&temp);
            let mut t: Vec<u64> = vec![];
            for m in matches {
                let x: Vec<u64> = m
                    .iter()
                    .filter(|x| x.is_some())
                    .map(|x| x.unwrap().as_str().parse::<u64>().unwrap())
                    .collect();
                t.push(x[0]);
            }
            a = (t[0], t[1]);
        } else if temp.starts_with("Button B") {
            let matches = re.captures_iter(&temp);
            let mut t: Vec<u64> = vec![];
            for m in matches {
                let x: Vec<u64> = m
                    .iter()
                    .filter(|x| x.is_some())
                    .map(|x| x.unwrap().as_str().parse::<u64>().unwrap())
                    .collect();
                t.push(x[0]);
            }
            b = (t[0], t[1]);
        } else if temp.starts_with("Prize") {
            let matches = re.captures_iter(&temp);
            let mut t: Vec<u64> = vec![];
            for m in matches {
                let x: Vec<u64> = m
                    .iter()
                    .filter(|x| x.is_some())
                    .map(|x| x.unwrap().as_str().parse::<u64>().unwrap())
                    .collect();
                t.push(x[0]);
            }
            prize = (t[0], t[1]);
        } else {
            machines.push(Machine {
                button_a: a,
                button_b: b,
                prize,
            });
        }
    }
    part_1(&machines);
    part_2(&machines);
    println!("Time: {:?}", start.elapsed());
}

fn part_1(machines: &Vec<Machine>) {
    let mut result: u64 = 0;
    for m in machines {
        let mut found: Vec<u64> = vec![];
        for i in 0..101_u64 {
            if m.button_a.0 * i >= m.prize.0 || m.button_a.1 * i >= m.prize.1 {
                break;
            }
            if (m.prize.0 - m.button_a.0 * i) % m.button_b.0 == 0
                && (m.prize.1 - m.button_a.1 * i) % m.button_b.1 == 0
                && (m.prize.0 - m.button_a.0 * i) / m.button_b.0
                    == (m.prize.1 - m.button_a.1 * i) / m.button_b.1
                && (m.prize.1 - m.button_a.1 * i) / m.button_b.1 <= 100
            {
                found.push((3 * i) + (((m.prize.0 - m.button_a.0 * i) / m.button_b.0) * 1));
            }
        }
        result += found.iter().min().unwrap_or(&0);
    }
    println!("{result}");
}

fn part_2(machines: &Vec<Machine>) {
    let mut result: u64 = 0;
    for m in machines {
        let p = (m.prize.0 + 10000000000000, m.prize.1 + 10000000000000);
        //need to figure out algorithm to do this more efficiently.
    }
    println!("{result}");
}
struct Machine {
    button_a: (u64, u64),
    button_b: (u64, u64),
    prize: (u64, u64),
}
