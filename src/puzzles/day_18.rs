use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    fs::File,
    io::{BufRead, BufReader},
    u32,
};

pub fn start() {
    let start = std::time::Instant::now();
    let file = File::open("./input/day_18.txt").unwrap();
    let reader = BufReader::new(file);
    let mut drops: Vec<(usize, usize)> = vec![];
    for l in reader.lines() {
        let temp = l.unwrap();
        let split_line = temp.split_once(",").unwrap();
        drops.push((
            split_line.0.parse::<usize>().unwrap(),
            split_line.1.parse::<usize>().unwrap(),
        ));
    }

    let mapsize: usize = 71;

    part_1(&drops, mapsize);
    part_2(&drops, mapsize);
    println!("Time: {:?}", start.elapsed());
}

fn part_1(drops: &Vec<(usize, usize)>, size: usize) {
    let mut map: Vec<Vec<bool>> = vec![vec![true; size]; size];
    let mut visited: Vec<Vec<u32>> = vec![vec![u32::MAX; size]; size];
    let mut unvisited: Vec<Vec<u32>> = vec![];
    let mut cell = MapCell {
        wdist: 0,
        dist: 0,
        loc: (0, 0),
    };
    let mut locations: BinaryHeap<Reverse<MapCell>> = BinaryHeap::new();
    locations.push(Reverse(cell));
    for i in 0..size {
        unvisited.push(vec![]);
        for j in 0..size {
            unvisited[i].push(((size * 2) - 2 - (j + i)) as u32);
        }
    }
    for i in 0..1024 {
        map[drops[i].1][drops[i].0] = false;
    }
    while cell.loc != (size - 1, size - 1) {
        cell = step(
            locations.pop().unwrap().0,
            &map,
            &mut visited,
            &mut locations,
            &unvisited
        );
    }
    println!("{}", cell.dist);
}

fn part_2(drops: &Vec<(usize, usize)>, size: usize) {
    let mut map: Vec<Vec<bool>> = vec![vec![true; size]; size];
    let mut unvisited: Vec<Vec<u32>> = vec![];
    //locations.push(Reverse(cell));
    for i in 0..size {
        unvisited.push(vec![]);
        for j in 0..size {
            unvisited[i].push((j + i) as u32);
        }
    }
    for i in 0..1024_usize{
        map[drops[i].1][drops[i].0] = false;
    }
    for n in 1024..drops.len() {
        let mut visited: Vec<Vec<u32>> = vec![vec![u32::MAX; size]; size];
        let mut locations: BinaryHeap<Reverse<MapCell>> = BinaryHeap::new();
        map[drops[n].1][drops[n].0] = false;
        let mut cell = MapCell {
            wdist: 0,
            dist: 0,
            loc: (size - 1, size - 1),
        };
        locations.push(Reverse(cell));
        let mut flag = false;
        while cell.loc != (0,0) {
            if locations.is_empty() {
                println!("{} - {:?}", n, drops[n]);
                flag = true;
                break
            }
            //println!("{n}, {:?}", cell.loc);
            cell = step(
                locations.pop().unwrap().0,
                &map,
                &mut visited,
                &mut locations,
                &unvisited
            );
            // for i in 0..map[0].len() {
            //     for j in 0..map.len() {
            //         if cell.loc == (i,j) { print!("@") }
            //         else { print!("{}", match map[j][i] { true => '.', false => '#' }) }
            //     }
            //     println!();
            // }
            // println!("{n}");
        }
        if flag {break;}
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct MapCell {
    wdist: u32,
    dist: u32,
    loc: (usize, usize),
}

fn step(
    cell: MapCell,
    m: &Vec<Vec<bool>>,
    v: &mut Vec<Vec<u32>>,
    l: &mut BinaryHeap<Reverse<MapCell>>,
    u: &Vec<Vec<u32>>,
) -> MapCell {
    //println!("{}, {:?}", cell.dist, cell.loc);
    if cell.loc.0 > 0
        && m[cell.loc.1][cell.loc.0 - 1]
        && v[cell.loc.1][cell.loc.0 - 1] > cell.dist + 1
    {
        l.push(Reverse(MapCell {
            wdist: cell.dist + 1 + u[cell.loc.1][cell.loc.0 - 1],
            dist: cell.dist + 1,
            loc: (cell.loc.0 - 1, cell.loc.1),
        }));
        v[cell.loc.1][cell.loc.0 - 1] = cell.dist + 1;
    }
    if cell.loc.1 < m[0].len() - 1
        && m[cell.loc.1 + 1][cell.loc.0]
        && v[cell.loc.1 + 1][cell.loc.0] > cell.dist + 1
    {
        l.push(Reverse(MapCell {
            wdist: cell.dist + 1 + u[cell.loc.1 + 1][cell.loc.0],
            dist: cell.dist + 1,
            loc: (cell.loc.0, cell.loc.1 + 1),
        }));
        v[cell.loc.1 + 1][cell.loc.0] = cell.dist + 1;
    }
    if cell.loc.0 < m.len() - 1
        && m[cell.loc.1][cell.loc.0 + 1]
        && v[cell.loc.1][cell.loc.0 + 1] > cell.dist + 1
    {
        l.push(Reverse(MapCell {
            wdist: cell.dist + 1 + u[cell.loc.1][cell.loc.0 + 1],
            dist: cell.dist + 1,
            loc: (cell.loc.0 + 1, cell.loc.1),
        }));
        v[cell.loc.1][cell.loc.0 + 1] = cell.dist + 1;
    }
    if cell.loc.1 > 0
        && m[cell.loc.1 - 1][cell.loc.0]
        && v[cell.loc.1 - 1][cell.loc.0] > cell.dist + 1
    {
        l.push(Reverse(MapCell {
            wdist: cell.dist + 1 + u[cell.loc.1 - 1][cell.loc.0],
            dist: cell.dist + 1,
            loc: (cell.loc.0, cell.loc.1 - 1),
        }));
        v[cell.loc.1 - 1][cell.loc.0] = cell.dist + 1;
    }
    cell
}

