use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn start() {
    let start = std::time::Instant::now();
    let file = File::open("./input/day_14.txt").unwrap();
    let reader = BufReader::new(file);
    let mut bots: Vec<Bot> = vec![];
    for l in reader.lines() {
        let temp = l.unwrap();
        let temp_vec: Vec<&str> = temp.split(&['=', ',', ' ']).collect();
        let b: Bot = Bot {
            loc: (
                temp_vec[1].parse::<usize>().unwrap(),
                temp_vec[2].parse::<usize>().unwrap(),
            ),
            veloc: (
                temp_vec[4].parse::<i32>().unwrap(),
                temp_vec[5].parse::<i32>().unwrap(),
            ),
        };
        bots.push(b);
    }
    part_1(bots.clone());
    part_2(bots);
    println!("Time: {:?}", start.elapsed());
}

fn part_1(mut bots: Vec<Bot>) {
    let m: (i32, i32) = (101, 103);
    for i in 0..bots.len() {
        let mut x = (bots[i].loc.0 as i32 + bots[i].veloc.0 * 100) % m.0;
        if x < 0 {
            x = m.0 + x;
        }
        let mut y = (bots[i].loc.1 as i32 + bots[i].veloc.1 * 100) % m.1;
        if y < 0 {
            y = m.1 + y;
        }
        bots[i].loc = (x as usize, y as usize);
    }
    let mut quads: Vec<usize> = vec![0; 4];
    for b in bots {
        if b.loc.0 < m.0 as usize / 2 && b.loc.1 < m.1 as usize / 2 {
            quads[0] += 1;
        }
        if b.loc.0 > m.0 as usize / 2 && b.loc.1 < m.1 as usize / 2 {
            quads[1] += 1;
        }
        if b.loc.0 < m.0 as usize / 2 && b.loc.1 > m.1 as usize / 2 {
            quads[2] += 1;
        }
        if b.loc.0 > m.0 as usize / 2 && b.loc.1 > m.1 as usize / 2 {
            quads[3] += 1;
        }
    }
    println!("{}", quads[0] * quads[1] * quads[2] * quads[3]);
}

// not sure enough what the christmas tree would look like to know how to detect a christmas tree other than just 
// pretty-printing the map and looking at it. But it didn't show up in the first 5k
fn part_2(mut bots: Vec<Bot>) {
    println!("time 0");
    pretty_print(&bots);
    let m: (i32, i32) = (101, 103);
    for n in 0..5000 {
        for i in 0..bots.len() {
            let x = (bots[i].loc.0 as i32 + bots[i].veloc.0 + m.0) % m.0;
            let y = (bots[i].loc.1 as i32 + bots[i].veloc.1 + m.1) % m.1;
            bots[i].loc = (x as usize, y as usize);
        }
        println!("time {}", n + 1);
        pretty_print(&bots);
    }
}

#[derive(Clone, Debug)]
struct Bot {
    loc: (usize, usize),
    veloc: (i32, i32),
}

fn pretty_print(bots: &Vec<Bot>) {
    for x in 0..101_usize {
        for y in 0..103_usize {
            let n = bots.iter().fold(0, |acc, b| {
                if b.loc.0 == x && b.loc.1 == y {
                    acc + 1
                } else {
                    acc
                }
            });
            if n > 0 {
                print!("{n}");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}
