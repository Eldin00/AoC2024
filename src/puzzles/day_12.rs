use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn start() {
    let start = std::time::Instant::now();
    let file = File::open("./input/day_12.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines: Vec<Vec<u8>> = vec![];
    for l in reader.lines() {
        let temp = l.unwrap();
        lines.push(temp.bytes().collect());
    }
    part_1(lines);
    println!("Time: {:?}", start.elapsed());
}

fn part_1(lines: Vec<Vec<u8>>) {
    let mut areas: Vec<Vec<u32>> = vec![vec![0; lines.len()]; lines[0].len()];
    let mut regions: Vec<Region> = vec![];
    let mut area_id: u32 = 0;
    for y in 0..lines[0].len() {
        for x in 0..lines.len() {
            if areas[x][y] == 0 {
                area_id += 1;
                regions.push(map_area((x, y), &mut areas, &lines, area_id));
            }
        }
    }
    println!(
        "{}",
        regions.iter().fold(0, |acc, x| acc + x.area * x.perimeter)
    );
    part_2(areas, regions);
}

fn part_2(areas: Vec<Vec<u32>>, mut regions: Vec<Region>) {
    for x in 0..areas[0].len() {
        for y in 0..areas.len() {
            let i = regions.iter().position(|r| r.id == areas[y][x]).unwrap();
            regions[i].sides += check_corner(&areas, (x, y));
        }
    }
    println!(
        "{}",
        regions.iter().fold(0, |acc, x| acc + x.area * x.sides)
    );
}

fn check_corner(areas: &Vec<Vec<u32>>, loc: (usize, usize)) -> u32 {
    let mut result: u32 = 0;
    if ((loc.1 == 0 || areas[loc.1 - 1][loc.0] != areas[loc.1][loc.0])
        && (loc.0 == 0 || areas[loc.1][loc.0 - 1] != areas[loc.1][loc.0]))
        || (loc.0 != 0
            && loc.1 != 0
            && areas[loc.1 - 1][loc.0] == areas[loc.1][loc.0]
            && areas[loc.1][loc.0 - 1] == areas[loc.1][loc.0]
            && areas[loc.1 - 1][loc.0 - 1] != areas[loc.1][loc.0])
    {
        result += 1;
    }
    if ((loc.1 == 0 || areas[loc.1 - 1][loc.0] != areas[loc.1][loc.0])
        && (loc.0 == areas[0].len() - 1 || areas[loc.1][loc.0 + 1] != areas[loc.1][loc.0]))
        || (loc.0 != areas[0].len() - 1
            && loc.1 != 0
            && areas[loc.1 - 1][loc.0] == areas[loc.1][loc.0]
            && areas[loc.1][loc.0 + 1] == areas[loc.1][loc.0]
            && areas[loc.1 - 1][loc.0 + 1] != areas[loc.1][loc.0])
    {
        result += 1;
    }
    if ((loc.1 == areas.len() - 1 || areas[loc.1 + 1][loc.0] != areas[loc.1][loc.0])
        && (loc.0 == areas[0].len() - 1 || areas[loc.1][loc.0 + 1] != areas[loc.1][loc.0]))
        || (loc.0 != areas[0].len() - 1
            && loc.1 != areas.len() - 1
            && areas[loc.1 + 1][loc.0] == areas[loc.1][loc.0]
            && areas[loc.1][loc.0 + 1] == areas[loc.1][loc.0]
            && areas[loc.1 + 1][loc.0 + 1] != areas[loc.1][loc.0])
    {
        result += 1;
    }
    if ((loc.1 == areas.len() - 1 || areas[loc.1 + 1][loc.0] != areas[loc.1][loc.0])
        && (loc.0 == 0 || areas[loc.1][loc.0 - 1] != areas[loc.1][loc.0]))
        || (loc.0 != 0
            && loc.1 != areas.len() - 1
            && areas[loc.1 + 1][loc.0] == areas[loc.1][loc.0]
            && areas[loc.1][loc.0 - 1] == areas[loc.1][loc.0]
            && areas[loc.1 + 1][loc.0 - 1] != areas[loc.1][loc.0])
    {
        result += 1;
    }
    return result;
}

fn map_area(
    loc: (usize, usize),
    areas: &mut Vec<Vec<u32>>,
    lines: &Vec<Vec<u8>>,
    area_id: u32,
) -> Region {
    let mut stack: Vec<(usize, usize)> = vec![];
    stack.push(loc);
    let mut area: u32 = 0;
    let mut perimeter: u32 = 0;
    while stack.len() > 0 {
        let l = stack.pop().unwrap();
        if areas[l.0][l.1] != 0 {
            continue;
        }
        area += 1;
        areas[l.0][l.1] = area_id;

        if l.0 != 0 && areas[l.0 - 1][l.1] == 0 && lines[l.0][l.1] == lines[l.0 - 1][l.1] {
            stack.push((l.0 - 1, l.1));
        }
        if l.0 < lines[0].len() - 1
            && areas[l.0 + 1][l.1] == 0
            && lines[l.0][l.1] == lines[l.0 + 1][l.1]
        {
            stack.push((l.0 + 1, l.1));
        }
        if l.1 != 0 && areas[l.0][l.1 - 1] == 0 && lines[l.0][l.1] == lines[l.0][l.1 - 1] {
            stack.push((l.0, l.1 - 1));
        }
        if l.1 < lines.len() - 1
            && areas[l.0][l.1 + 1] == 0
            && lines[l.0][l.1] == lines[l.0][l.1 + 1]
        {
            stack.push((l.0, l.1 + 1));
        }
        if l.0 == 0 || lines[l.0 - 1][l.1] != lines[l.0][l.1] {
            perimeter += 1;
        }
        if l.0 == lines[0].len() - 1 || lines[l.0 + 1][l.1] != lines[l.0][l.1] {
            perimeter += 1;
        }
        if l.1 == 0 || lines[l.0][l.1 - 1] != lines[l.0][l.1] {
            perimeter += 1;
        }
        if l.1 == lines.len() - 1 || lines[l.0][l.1 + 1] != lines[l.0][l.1] {
            perimeter += 1;
        }
    }
    Region {
        id: area_id,
        area,
        perimeter,
        sides: 0,
    }
}

struct Region {
    id: u32,
    area: u32,
    perimeter: u32,
    sides: u32,
}
