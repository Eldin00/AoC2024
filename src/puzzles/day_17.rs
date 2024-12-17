use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn start() {
    let start = std::time::Instant::now();
    let file = File::open("./input/day_17.txt").unwrap();
    let  (mut a, mut b, mut c) = (0_u64,0_u64,0_u64);
    let reader = BufReader::new(file);
    let mut program: Vec<u8> = vec![];
    for l in reader.lines() {
        let temp = l.unwrap();
        if temp.starts_with("Register A:") {
            a = temp.split_once(": ").unwrap().1.parse::<u64>().unwrap();
        }
        if temp.starts_with("Register B:") {
            b = temp.split_once(": ").unwrap().1.parse::<u64>().unwrap();
        }
        if temp.starts_with("Register C:") {
            c = temp.split_once(": ").unwrap().1.parse::<u64>().unwrap();
        }
        if temp.starts_with("Program:") {
            program = temp.split_once(": ").unwrap().1.split(',').map(|c| c.parse::<u8>().unwrap()).collect();
        }

    }
    part_1(a, b, c, &program);
    part_2(b, c, &program);
    println!("Time: {:?}", start.elapsed());
}

fn part_1(a: u64, b: u64, c: u64, prog: &Vec<u8>) {
    let output: Vec<u64> = compute(a, b, c, &prog, 0);
    println!("{}", output.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(","));
}

fn part_2(b: u64, c: u64, prog: &Vec<u8>) {
    let mut a: u64 = 0;
    let mut values = prog.clone();
    while values.len() > 0 {
        let target = values.pop().unwrap();
        for i in 0..8 {
            let next_a = (a * 8) + i; //because we need 3 bits per output digit.
            let output = compute(next_a, b, c, prog, 0);
            if output[0] == target as u64 {
                a = next_a;
                break;
            }
        }
    }
    println!("{a}");
}

fn compute(mut a: u64, mut b: u64, mut c: u64, prog: &Vec<u8>, mut ins_ptr: usize) -> Vec<u64>{
    let mut output: Vec<u64> = vec![];
    while ins_ptr < prog.len() {
        let combo = match prog[ins_ptr + 1] {
            4 => a,
            5 => b,
            6 => c,
            _ => prog[ins_ptr + 1] as u64
        };
        match prog[ins_ptr] {
            0 => {
                a = a/2_u64.pow(combo.clamp(0,63) as u32);
                ins_ptr += 2;
            },
            1 => {
                b = b ^ prog[ins_ptr + 1] as u64;
                ins_ptr += 2;
            },
            2 => {
                b = combo % 8; 
                ins_ptr += 2;
            },
            3 => {
                if a == 0 { 
                    ins_ptr += 2;
                    continue; 
                }
                ins_ptr = prog[ins_ptr + 1] as usize;
            },
            4 => {
                b = b ^ c;
                ins_ptr += 2;
            },
            5 => {
                output.push(combo % 8);
                ins_ptr += 2;
            },
            6 => {
                b = a/2_u64.pow(combo.clamp(0,63) as u32);
                ins_ptr += 2;
            },
            7 => {
                c = a/2_u64.pow(combo.clamp(0,63) as u32);
                ins_ptr += 2;
            },
            _ => ()
        }
    }
    output
}