use std::{
    fs::File,
    io::{BufRead, BufReader}
};

pub fn start() {
    let start = std::time::Instant::now();
    let file = File::open("./input/day_9.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = vec![];
    for l in reader.lines() {
        let temp = l.unwrap();
        lines.push(temp);
    }
    part_1(lines.clone());
    part_2(lines);
    println!("Time: {:?}", start.elapsed());
}

fn part_1(lines: Vec<String>) {
    let line: Vec<u8> = lines[0].chars().map(|c| c.to_string().parse::<u8>().unwrap()).collect();
    let mut is_file: bool = true;
    let mut drivemap: Vec<Option<u32>> = vec![];
    let mut compacted: Vec<u32> = vec![];
    let mut i: u32 = 0;
    for l in line {
        if is_file {
            let mut a = vec![Some(i); l as usize];
            drivemap.append(a.as_mut());
            i += 1;
            is_file = false;
        }
        else {
            let mut a: Vec<Option<u32>> = vec![None; l as usize];
            drivemap.append(a.as_mut());
            is_file = true;
        }
    }
    let mut i = 0;
    while i < drivemap.len() {
        if let Some(n) = drivemap[i] {
            compacted.push(n);
            i += 1;
        } else {
            let element = drivemap.pop();
            if let Some(Some(n)) = element {
                compacted.push(n);
                i += 1
            }
        }
    }
    let mut result: u64 = 0;
    for i in 0..compacted.len() {
        result += (i as u32 * compacted[i]) as u64;
    }
    println!{"{result}"};
}

fn part_2(lines: Vec<String>) { //gets wrong answer on real input, but works on test input.
    let line: Vec<u8> = lines[0].chars().map(|c| c.to_string().parse::<u8>().unwrap()).collect();
    let mut is_file: bool = true;
    let mut drivemap: Vec<Option<u32>> = vec![];
    let mut gaps: Vec<(usize, usize)> = vec![];
    let mut files: Vec<(usize, usize)> = vec![];
    let mut i: u32 = 0;
    for l in line {
        if is_file {
            let mut a = vec![Some(i); l as usize];
            files.push((drivemap.len(), l as usize));
            drivemap.append(a.as_mut());
            i += 1;
            is_file = false;
        }
        else {
            let mut a: Vec<Option<u32>> = vec![None; l as usize];
            gaps.push((drivemap.len(), l as usize));
            drivemap.append(a.as_mut());
            is_file = true;
        }
    }
    while files.len() > 0 {
        let file = files.pop().unwrap();
        for i in 0..gaps.len(){
            if i >= file.0 {break;};
            if gaps[i].1 >= file.1 {
                for j in 0..file.1 {
                    drivemap[gaps[i].0 + j] = drivemap[file.0 + j];
                    drivemap[file.0 + j] = None;
                }
                if gaps[i].1 == file.1 {
                    gaps.remove(i);
                } else {
                    gaps[i].0 += file.1;
                    gaps[i].1 -= file.1;
                }
                break;
            }
        }
    }
    let mut result: u64 = 0;
    for i in 0..drivemap.len() {
        if drivemap[i].is_some() {
            result += (i as u32 * drivemap[i].unwrap()) as u64;
        }
    }
    println!{"{result} WRONG!"};
}
// result < 8705204127569