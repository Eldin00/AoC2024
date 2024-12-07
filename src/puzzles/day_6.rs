use std::{
    collections::HashMap, fs::File, io::{BufRead, BufReader}, fmt::{self, Display, Formatter}
};

pub fn start() {
    let start = std::time::Instant::now();
    let file = File::open("./input/day_6.txt").unwrap();
    let reader = BufReader::new(file);
    let mut map: Vec<Vec<bool>> = vec![];
    let mut start_loc: (usize, usize) = (0,0);
    let mut i: usize = 0;
    for l in reader.lines() {
        let temp = l.unwrap();
        let tmp = temp.find('^');
        if tmp.is_some() {
            start_loc = (i, tmp.unwrap());
        };
        i += 1;
        map.push(temp.chars().map(|x| x == '#').collect());

    }
        part_1(&map, &start_loc);
    println!("Time: {:?}", start.elapsed());
}

fn part_1(map: &Vec<Vec<bool>>, start_loc: &(usize, usize)) {
    let mut guard = Guard {
        dir: Direction::N,
        loc: *start_loc,
        limit: (map.len(), map[0].len()),
        visited: HashMap::from([(*start_loc, 1_u32)])
    };
    while guard.step(&map) { }
    println!("{:?}", guard.visited.keys().len());
    part_2(map.clone(), start_loc, guard.visited)
}

// not happy with this brute force solution, but it worked.
fn part_2(mut map: Vec<Vec<bool>>, start: &(usize, usize), visited: HashMap<(usize, usize), u32>) {
    let empty_vec: Vec<Direction> = vec![];
    let mut result: u32 = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] || start == &(i, j) || !visited.contains_key(&(i,j))  { continue; }
            map[i][j] = true;
            let mut visited: HashMap<(usize, usize), Vec<Direction>> = HashMap::new();
            let mut guard = Guard {
                dir: Direction::N,
                loc: *start,
                limit: (map.len(), map[0].len()),
                visited: HashMap::from([(*start, 1_u32)]),
            };
            while guard.step(&map){
                if visited.contains_key(&guard.loc) && visited.get(&guard.loc).unwrap().contains(&guard.dir) {
                    result += 1;
                    break;
                }
                let mut temp = visited.get(&guard.loc).unwrap_or(&empty_vec).clone();
                temp.push(guard.dir);
                visited.insert(guard.loc, temp.clone());
            }
            map[i][j] = false;
        }
    }
    println!("{result}");
}

//for debugging
#[allow(dead_code)]
fn print_map(map: &Vec<Vec<bool>>) {
    for a in map {
        for b in a {
            if *b {print!("#");} else {print!(".");}
        }
        print!("\n");
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Direction {
    N,
    E,
    S,
    W
}

impl Direction {
    pub fn next(self) -> Direction {
        match self {
            Direction::N => Direction::E,
            Direction::E => Direction::S,
            Direction::S => Direction::W,
            Direction::W => Direction::N
        }
    }
}

//for debugging
impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Direction::N => write!(f,"N"),
            Direction::E => write!(f,"E"),
            Direction::S => write!(f,"S"),
            Direction::W => write!(f,"W")
        }
    }
}

#[derive(Debug)]

struct Guard {
    dir: Direction,
    loc: (usize,usize),
    limit: (usize,usize),
    visited: HashMap<(usize, usize), u32>
}

impl Guard {
    pub fn step(&mut self, map: &Vec<Vec<bool>>) -> bool{
        match self.dir {
            Direction::N => {
                if self.loc.0 == 0 { return false; }
                if map[self.loc.0 - 1][self.loc.1] {
                    self.dir = self.dir.next();
                    return true; 
                } 
                self.loc.0 -= 1;
                *self.visited.entry(self.loc).or_insert(0) += 1;
            },
            Direction::E => {
                if self.loc.1 + 1 == self.limit.1 { return false; }
                if map[self.loc.0][self.loc.1 + 1] {
                    self.dir = self.dir.next();
                    return true; 
                } 
                self.loc.1 += 1;
                *self.visited.entry(self.loc).or_insert(0) += 1;
            },
            Direction::S => {
                if self.loc.0 + 1 == self.limit.0 { return false; }
                if map[self.loc.0 + 1][self.loc.1] {
                    self.dir = self.dir.next();
                    return true; 
                } 
                self.loc.0 += 1;
                *self.visited.entry(self.loc).or_insert(0) += 1;
            },
            Direction::W => {
                if self.loc.1 == 0 { return false; }
                if map[self.loc.0][self.loc.1 - 1] {
                    self.dir = self.dir.next();
                    return true; 
                } 
                self.loc.1 -= 1;
                *self.visited.entry(self.loc).or_insert(0) += 1;
            }
        }

        true
    }
}
