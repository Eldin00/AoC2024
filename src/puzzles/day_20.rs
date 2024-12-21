use std::{
    collections::HashMap, fs::File, io::{BufRead, BufReader}
};

pub fn start() {
    let start = std::time::Instant::now();
    let file = File::open("./input/day_20.txt").unwrap();
    let reader = BufReader::new(file);
    let mut i = 0_usize;
    let mut end = (0_usize,0_usize);
    let mut begin = end;
    let mut map: Vec<Vec<bool>> = vec![];
    for l in reader.lines() {
        let temp = l.unwrap();
        if let Some(x) = temp.find('E') {
            end = (x, i);
        }
        if let Some(x) = temp.find('S') {
            begin = (x, i);
        }
        map.push(temp.chars().map(|c| match c {'#' => false, _ => true}).collect());
        i += 1;
    }
    part_1(&map, begin, end);
    //part_2(lines);
    println!("Time: {:?}", start.elapsed());
}

fn part_1(m: &Vec<Vec<bool>>, s: (usize, usize), e: (usize, usize)) {
    let mut scores: Vec<Vec<u32>> = vec![vec![u32::MAX; m[0].len()]; m.len()];
    let mut loc = s.clone();
    let mut cur_dist = 0;
    let mut path: Vec<(usize, usize)> = vec![s];
    let mut cheats: HashMap<u32, u32> = HashMap::new();    
    scores[loc.1][loc.0] = cur_dist;
    while loc != e {
        cur_dist += 1;
        if m[loc.1 - 1][loc.0] && scores[loc.1 - 1][loc.0] > cur_dist {
            loc.1 -= 1;
            path.push(loc);
            scores[loc.1][loc.0] = cur_dist;
            continue;
        }
        if m[loc.1][loc.0 + 1] && scores[loc.1][loc.0 + 1] > cur_dist {
            loc.0 += 1;
            path.push(loc);
            scores[loc.1][loc.0] = cur_dist;
            continue;
        }
        if m[loc.1 + 1][loc.0] && scores[loc.1 + 1][loc.0] > cur_dist {
            loc.1 += 1;
            path.push(loc);
            scores[loc.1][loc.0] = cur_dist;
            continue;
        }
        if m[loc.1][loc.0 - 1] && scores[loc.1][loc.0 - 1] > cur_dist {
            loc.0 -= 1;
            path.push(loc);
            scores[loc.1][loc.0] = cur_dist;
            continue;
        }
        break;
    }
    while let Some(loc) = path.pop() {
        if loc.1 > 1 && scores[loc.1 - 2][loc.0] < scores[loc.1][loc.0] {
            let savings = scores[loc.1][loc.0] - scores[loc.1 - 2][loc.0] - 2;
            *cheats.entry(savings).or_default() += 1;
        }
        if loc.1 + 2 < scores.len() && scores[loc.1 + 2][loc.0] < scores[loc.1][loc.0] {
            let savings = scores[loc.1][loc.0] - scores[loc.1 + 2][loc.0] - 2;
            *cheats.entry(savings).or_default() += 1;
        }
        if loc.0 > 1 && scores[loc.1][loc.0 - 2] < scores[loc.1][loc.0] {
            let savings = scores[loc.1][loc.0] - scores[loc.1][loc.0 - 2] - 2;
            *cheats.entry(savings).or_default() += 1;
        }
        if loc.0 + 2 < scores.len() && scores[loc.1][loc.0 + 2] < scores[loc.1][loc.0] {
            let savings = scores[loc.1][loc.0] - scores[loc.1][loc.0 + 2] - 2;
            *cheats.entry(savings).or_default() += 1;
        }
    }
    let k: Vec<&u32> = cheats.keys().collect();
    println!("{}", k.iter().filter(|n| **n >= &(100_u32)).fold(0, |acc, n| cheats[n] + acc));
}


fn part_2(lines: Vec<String>) {

}

