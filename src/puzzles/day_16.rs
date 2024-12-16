use std::{
    cmp::Reverse, collections::{BinaryHeap, HashMap}, fs::File, io::{BufRead, BufReader}
};

pub fn start() {
    let start = std::time::Instant::now();
    let file = File::open("./input/day_16.txt").unwrap();
    let reader = BufReader::new(file);
    let mut map: Vec<Vec<bool>> = vec![];
    let mut reindeer:Reindeer = Reindeer{loc: (0,0), dir: Direction::E, score: 0};
    let mut goal: (usize, usize) = (0,0);
    let mut i = 0_usize;
    for l in reader.lines() {
        let temp = l.unwrap();
        if let Some(x) = temp.find('S') {
            reindeer.loc = (i,x);
        }
        if let Some(x) = temp.find('E') {
            goal = (i,x);
        }
        let tvec: Vec<bool> = temp.chars().map(|c| match c {
            '#' => false,
            _ => true
        }).collect();
        map.push(tvec);
        i += 1;
    }
    part_1(reindeer, map, goal);
    // part_2(lines);
    println!("Time: {:?}", start.elapsed());
}

fn part_1(mut r: Reindeer, map: Vec<Vec<bool>>, g: (usize, usize)) {
    let mut visited: HashMap<(usize, usize), [u32;4]> = HashMap::new();
    let mut locations: BinaryHeap<Reverse<Reindeer>> = BinaryHeap::new();
    visited.insert(r.loc, [u32::MAX, 0, u32::MAX, u32::MAX]);
    while r.loc != g {
        match r.dir {
            Direction::N => {
                if map[r.loc.0 - 1][r.loc.1] && (!visited.contains_key(&(r.loc.0 - 1, r.loc.1)) || visited[&(r.loc.0 - 1, r.loc.1)][Direction::N.to_int()] > r.score + 1) {
                    locations.push(Reverse(Reindeer{ score: r.score + 1, loc: (r.loc.0 - 1, r.loc.1), dir: Direction::N}));
                }
                if map[r.loc.0][r.loc.1 + 1] && (!visited.contains_key(&(r.loc.0, r.loc.1 + 1)) || visited[&(r.loc.0, r.loc.1 + 1)][Direction::E.to_int()] > r.score + 1000) {
                    locations.push(Reverse(Reindeer{ score: r.score + 1000, loc: r.loc, dir: Direction::E}));
                }
                if map[r.loc.0][r.loc.1 - 1] && (!visited.contains_key(&(r.loc.0, r.loc.1 - 1)) || visited[&(r.loc.0, r.loc.1 - 1)][Direction::W.to_int()] > r.score + 1000) {
                    locations.push(Reverse(Reindeer{ score: r.score + 1000, loc: r.loc, dir: Direction::W}));
                }
            },
            Direction::E => {
                if map[r.loc.0][r.loc.1 + 1] && (!visited.contains_key(&(r.loc.0, r.loc.1 + 1)) || visited[&(r.loc.0, r.loc.1 + 1)][Direction::E.to_int()] > r.score + 1) {
                    locations.push(Reverse(Reindeer{ score: r.score + 1, loc: (r.loc.0, r.loc.1 + 1), dir: Direction::E}))
                }
                if map[r.loc.0 + 1][r.loc.1] && (!visited.contains_key(&(r.loc.0 + 1, r.loc.1)) || visited[&(r.loc.0 + 1, r.loc.1)][Direction::S.to_int()] > r.score + 1000) {
                    locations.push(Reverse(Reindeer{ score: r.score + 1000, loc: r.loc, dir: Direction::S}))
                }
                if map[r.loc.0 - 1][r.loc.1] && (!visited.contains_key(&(r.loc.0 - 1, r.loc.1)) || visited[&(r.loc.0 - 1, r.loc.1)][Direction::N.to_int()] > r.score + 1000) {
                    locations.push(Reverse(Reindeer{ score: r.score + 1000, loc: r.loc, dir: Direction::N}))
                }
            },
            Direction::S => {
                if map[r.loc.0 + 1][r.loc.1] && (!visited.contains_key(&(r.loc.0 + 1, r.loc.1)) || visited[&(r.loc.0 + 1, r.loc.1)][Direction::S.to_int()] > r.score + 1) {
                    locations.push(Reverse(Reindeer{ score: r.score + 1, loc: (r.loc.0 + 1, r.loc.1), dir: Direction::S}));
                }
                if map[r.loc.0][r.loc.1 + 1] && (!visited.contains_key(&(r.loc.0, r.loc.1 + 1)) || visited[&(r.loc.0, r.loc.1 + 1)][Direction::E.to_int()] > r.score + 1000) {
                    locations.push(Reverse(Reindeer{ score: r.score + 1000, loc: r.loc, dir: Direction::E}));
                }
                if map[r.loc.0][r.loc.1 - 1] && (!visited.contains_key(&(r.loc.0, r.loc.1 - 1)) || visited[&(r.loc.0, r.loc.1 - 1)][Direction::W.to_int()] > r.score + 1000) {
                    locations.push(Reverse(Reindeer{ score: r.score + 1000, loc: r.loc, dir: Direction::W}));
                }
            },
            Direction::W => {
                if map[r.loc.0][r.loc.1 - 1] && (!visited.contains_key(&(r.loc.0, r.loc.1 - 1)) || visited[&(r.loc.0, r.loc.1 - 1)][Direction::W.to_int()] > r.score + 1) {
                    locations.push(Reverse(Reindeer{ score: r.score + 1, loc: (r.loc.0, r.loc.1 - 1), dir: Direction::W}))
                }
                if map[r.loc.0 + 1][r.loc.1] && (!visited.contains_key(&(r.loc.0 + 1, r.loc.1)) || visited[&(r.loc.0 + 1, r.loc.1)][Direction::S.to_int()] > r.score + 1000) {
                    locations.push(Reverse(Reindeer{ score: r.score + 1000, loc: r.loc, dir: Direction::S}))
                }
                if map[r.loc.0 - 1][r.loc.1] && (!visited.contains_key(&(r.loc.0 - 1, r.loc.1)) || visited[&(r.loc.0 - 1, r.loc.1)][Direction::N.to_int()] > r.score + 1000) {
                    locations.push(Reverse(Reindeer{ score: r.score + 1000, loc: r.loc, dir: Direction::N}))
                }
            }
        }
        r = locations.pop().unwrap().0;
        let mut tmp = [u32::MAX;4];
        if visited.contains_key(&r.loc) {
            tmp = visited[&r.loc];
        }
        tmp[r.dir.to_int()] = r.score;
        visited.insert(r.loc, tmp);
}
    println!("{}", r.score);
}

fn part_2(lines: Vec<String>) {

}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    N,
    E,
    S,
    W
}

impl Direction {
    fn to_int(&self) -> usize {
        match self {
            Direction::N => 0,
            Direction::E => 1,
            Direction::S => 2,
            Direction::W => 3
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Reindeer {
    score: u32,
    loc: (usize, usize),
    dir: Direction,
}

