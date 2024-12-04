use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn start() {
    let file = File::open("./input/day_4.txt").unwrap();
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
    let mut result: u32 = 0;
    for i in 0..lines.len() {
        //look in line
        result += lines[i].match_indices("XMAS").collect::<Vec<_>>().len() as u32;
        result += lines[i].match_indices("SAMX").collect::<Vec<_>>().len() as u32;
        if i < 3 {
            continue;
        }
        //Look up
        for end in lines[i].match_indices("X") {
            if lines[i - 1].chars().nth(end.0) == Some('M')
                && lines[i - 2].chars().nth(end.0) == Some('A')
                && lines[i - 3].chars().nth(end.0) == Some('S')
            {
                result += 1;
            }
        }
        for end in lines[i].match_indices("S") {
            if lines[i - 1].chars().nth(end.0) == Some('A')
                && lines[i - 2].chars().nth(end.0) == Some('M')
                && lines[i - 3].chars().nth(end.0) == Some('X')
            {
                result += 1;
            }
        }

        //look diagonal right
        for end in lines[i].match_indices("X") {
            if end.0 < lines.len() - 2
                && lines[i - 1].chars().nth(end.0 + 1) == Some('M')
                && lines[i - 2].chars().nth(end.0 + 2) == Some('A')
                && lines[i - 3].chars().nth(end.0 + 3) == Some('S')
            {
                result += 1;
            }
        }
        for end in lines[i].match_indices("S") {
            if end.0 < lines.len() - 2
                && lines[i - 1].chars().nth(end.0 + 1) == Some('A')
                && lines[i - 2].chars().nth(end.0 + 2) == Some('M')
                && lines[i - 3].chars().nth(end.0 + 3) == Some('X')
            {
                result += 1;
            }
        }

        //Look diagonal left
        for end in lines[i].match_indices("X") {
            if end.0 >= 3
                && lines[i - 1].chars().nth(end.0 - 1) == Some('M')
                && lines[i - 2].chars().nth(end.0 - 2) == Some('A')
                && lines[i - 3].chars().nth(end.0 - 3) == Some('S')
            {
                result += 1;
            }
        }
        for end in lines[i].match_indices("S") {
            if end.0 >= 3
                && lines[i - 1].chars().nth(end.0 - 1) == Some('A')
                && lines[i - 2].chars().nth(end.0 - 2) == Some('M')
                && lines[i - 3].chars().nth(end.0 - 3) == Some('X')
            {
                result += 1;
            }
        }
    }

    println!("{result}");
}

fn part_2(lines: Vec<String>) {
    let mut result: u32 = 0;
    for i in 1..lines.len() - 1 {
        for cnt in lines[i].match_indices("A") {
            if cnt.0 > 0
                && cnt.0 < lines[i].len() - 1
                && ((lines[i - 1].chars().nth(cnt.0 - 1) == Some('M')
                    && lines[i + 1].chars().nth(cnt.0 + 1) == Some('S'))
                    || (lines[i - 1].chars().nth(cnt.0 - 1) == Some('S')
                        && lines[i + 1].chars().nth(cnt.0 + 1) == Some('M')))
                && ((lines[i - 1].chars().nth(cnt.0 + 1) == Some('M')
                    && lines[i + 1].chars().nth(cnt.0 - 1) == Some('S'))
                    || (lines[i - 1].chars().nth(cnt.0 + 1) == Some('S')
                        && lines[i + 1].chars().nth(cnt.0 - 1) == Some('M')))
            {
                result += 1;
            }
        }
    }
    println!("{result}")
}
