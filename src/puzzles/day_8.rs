use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

pub fn start() {
    let start = std::time::Instant::now();
    let file = File::open("./input/day_8.txt").unwrap();
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
    let mut map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
    let limit: (usize, usize) = (lines.len(), lines[0].len());
    for y in 0..lines.len() {
        let line: Vec<char> = lines[y].chars().collect();
        for x in 0..line.len() {
            if line[x] == '.' {
                continue;
            }
            if map.contains_key(&line[x]) {
                let mut tmp = map.get(&line[x]).unwrap().clone();
                tmp.push((x, y));
                map.insert(line[x], tmp);
            } else {
                map.insert(line[x], vec![(x, y)]);
            }
        }
    }
    for k in map.keys() {
        let antennas: Vec<(usize, usize)> = map.get(k).unwrap().to_owned();
        for i in 0..antennas.len() {
            for j in i + 1..antennas.len() {
                let distance: (i32, i32) = (
                    antennas[i].0 as i32 - antennas[j].0 as i32,
                    antennas[i].1 as i32 - antennas[j].1 as i32,
                );
                if let Some(v) = vec_add(antennas[i], distance, limit) {
                    antinodes.insert(v);
                }
                if let Some(v) = vec_sub(antennas[j], distance, limit) {
                    antinodes.insert(v);
                }
            }
        }
    }
    println!("{}", antinodes.len());
}

fn part_2(lines: Vec<String>) {
    let mut map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
    let limit: (usize, usize) = (lines.len(), lines[0].len());
    for y in 0..lines.len() {
        let line: Vec<char> = lines[y].chars().collect();
        for x in 0..line.len() {
            if line[x] == '.' {
                continue;
            }
            if map.contains_key(&line[x]) {
                let mut tmp = map.get(&line[x]).unwrap().clone();
                tmp.push((x, y));
                map.insert(line[x], tmp);
            } else {
                map.insert(line[x], vec![(x, y)]);
            }
        }
    }
    for k in map.keys() {
        let antennas: Vec<(usize, usize)> = map.get(k).unwrap().to_owned();
        for i in 0..antennas.len() {
            for j in i + 1..antennas.len() {
                let distance: (i32, i32) = (
                    antennas[i].0 as i32 - antennas[j].0 as i32,
                    antennas[i].1 as i32 - antennas[j].1 as i32,
                );
                let mut tmpvec = antennas[i].clone();
                while let Some(v) = vec_add(tmpvec, distance, limit) {
                    antinodes.insert(v);
                    tmpvec = v.clone();
                }
                tmpvec = antennas[j].clone();
                while let Some(v) = vec_sub(tmpvec, distance, limit) {
                    antinodes.insert(v);
                    tmpvec = v.clone();
                }
                antinodes.insert(antennas[i]);
                antinodes.insert(antennas[j]);
            }
        }
    }
    println!("{}", antinodes.len());
}

fn vec_add(a: (usize, usize), b: (i32, i32), limit: (usize, usize)) -> Option<(usize, usize)> {
    let x = a.0 as i32 + b.0;
    let y = a.1 as i32 + b.1;
    if x < 0 || x >= limit.0 as i32 || y < 0 || y >= limit.1 as i32 {
        return None;
    }
    Some((x as usize, y as usize))
}

fn vec_sub(a: (usize, usize), b: (i32, i32), limit: (usize, usize)) -> Option<(usize, usize)> {
    let x = a.0 as i32 - b.0;
    let y = a.1 as i32 - b.1;
    if x < 0 || x >= limit.0 as i32 || y < 0 || y >= limit.1 as i32 {
        return None;
    }
    Some((x as usize, y as usize))
}
