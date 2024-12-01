use std::{
    fs::File,
    io::{BufRead, BufReader}
};

pub fn start() {
    let file = File::open("./input/day_2.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = vec![];
    for l in reader.lines(){
        let temp = l.unwrap();
        lines.push(String::from(temp));
    }
    part_1(lines.clone());
    
}

fn part_1(lines: Vec<String>){

}