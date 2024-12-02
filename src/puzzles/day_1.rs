use std::{
    fs::File,
    io::{BufRead, BufReader}
};

pub fn start() {
    let file = File::open("./input/day_1.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = vec![];
    for l in reader.lines(){
        let temp = l.unwrap();
        lines.push(temp);
    }
    part_1(lines);
    
}

fn part_1(input: Vec<String>){
    let mut list1: Vec<u32> = vec![];
    let mut list2: Vec<u32> = vec![];
    let mut result: u32 = 0;

    for line in input {
        let v: Vec<&str> = line.as_str().split_whitespace().collect();
        list1.push(v[0].parse::<u32>().unwrap());
        list2.push(v[1].parse::<u32>().unwrap());
    }

    list1.sort();
    list2.sort();

    for i in 0 .. list1.len() {
        result += list1[i].abs_diff(list2[i]);
    }

    println!("{result}");

    part_2(list1, list2);
}

fn part_2(list1: Vec<u32>, list2: Vec<u32>){
    let result = list1.iter().fold(0,|acc: u32, x| acc + (x * list2.iter().filter(|&n| *n == *x).count() as u32));
    println!("{result}");
}
