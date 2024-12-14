use std::{
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn start() {
    let start = std::time::Instant::now();
    let file = File::open("./input/day_10.txt").unwrap();
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
    let mut result: u32 = 0;
    let mut map: Vec<Vec<u8>> = vec![];
    for line in lines {
        map.push(
            line.chars()
                .map(|c| c.to_string().parse::<u8>().unwrap())
                .collect(),
        )
    }
    for x in 0..map[0].len() {
        for y in 0..map.len() {
            if map[x][y] == 0 {
                let mut v = step((x, y), &map);
                v.retain(|x| x.is_some());
                let mut v: Vec<(usize, usize)> = v.iter().map(|x| x.unwrap()).collect();
                v.sort_by(|a, b| match a.0.cmp(&b.0) {
                    Ordering::Equal => a.1.cmp(&b.1),
                    v => v,
                });
                v.dedup_by(|a, b| a.0 == b.0 && a.1 == b.1);
                result += v.len() as u32;
            }
        }
    }
    println!("{result}");
}

fn part_2(lines: Vec<String>) {
    let mut result: u32 = 0;
    let mut map: Vec<Vec<u8>> = vec![];
    for line in lines {
        map.push(
            line.chars()
                .map(|c| c.to_string().parse::<u8>().unwrap())
                .collect(),
        )
    }
    for x in 0..map[0].len() {
        for y in 0..map.len() {
            if map[x][y] == 0 {
                result += step2((x, y), &map);
            }
        }
    }
    println!("{result}");
}

fn step(loc: (usize, usize), map: &Vec<Vec<u8>>) -> Vec<Option<(usize, usize)>> {
    if map[loc.0][loc.1] == 9 {
        return vec![Some(loc)];
    }
    let mut result: Vec<Option<(usize, usize)>> = vec![];
    if loc.0 > 0 && map[loc.0][loc.1] + 1 == map[loc.0 - 1][loc.1] {
        result.append(step((loc.0 - 1, loc.1), map).as_mut());
    }
    if loc.0 < map[0].len() - 1 && map[loc.0][loc.1] + 1 == map[loc.0 + 1][loc.1] {
        result.append(step((loc.0 + 1, loc.1), map).as_mut());
    }
    if loc.1 > 0 && map[loc.0][loc.1] + 1 == map[loc.0][loc.1 - 1] {
        result.append(step((loc.0, loc.1 - 1), map).as_mut());
    }
    if loc.1 < map.len() - 1 && map[loc.0][loc.1] + 1 == map[loc.0][loc.1 + 1] {
        result.append(step((loc.0, loc.1 + 1), map).as_mut());
    }
    result
}

fn step2(loc: (usize, usize), map: &Vec<Vec<u8>>) -> u32 {
    if map[loc.0][loc.1] == 9 {
        return 1;
    }
    let mut result: u32 = 0;
    if loc.0 > 0 && map[loc.0][loc.1] + 1 == map[loc.0 - 1][loc.1] {
        result += step2((loc.0 - 1, loc.1), map);
    }
    if loc.0 < map[0].len() - 1 && map[loc.0][loc.1] + 1 == map[loc.0 + 1][loc.1] {
        result += step2((loc.0 + 1, loc.1), map);
    }
    if loc.1 > 0 && map[loc.0][loc.1] + 1 == map[loc.0][loc.1 - 1] {
        result += step2((loc.0, loc.1 - 1), map);
    }
    if loc.1 < map.len() - 1 && map[loc.0][loc.1] + 1 == map[loc.0][loc.1 + 1] {
        result += step2((loc.0, loc.1 + 1), map);
    }
    result
}
