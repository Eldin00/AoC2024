use std::{
    cmp::Ordering,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn start() {
    let mut rules: HashMap<u8, Vec<u8>> = HashMap::new();
    let mut jobs: Vec<Vec<u8>> = vec![];
    let file = File::open("./input/day_5.txt").unwrap();
    let reader = BufReader::new(file);
    let mut flag = false;
    for l in reader.lines() {
        let temp = l.unwrap();
        if !flag {
            if temp.trim() == "" {
                flag = true;
                continue;
            }
            let temp_rule: Vec<u8> = temp.split('|').map(|x| x.parse::<u8>().unwrap()).collect();
            if rules.contains_key(&temp_rule[0]) {
                let mut t = rules.get_mut(&temp_rule[0]).unwrap().clone();
                t.push(temp_rule[1].clone());
                rules.insert(temp_rule[0], t.clone());
            } else {
                rules.insert(temp_rule[0], vec![temp_rule[1]]);
            }
        } else {
            let temp_job: Vec<u8> = temp.split(',').map(|x| x.parse::<u8>().unwrap()).collect();
            jobs.push(temp_job.clone());
        }
    }
    part_1(rules, jobs);
}

fn part_1(rules: HashMap<u8, Vec<u8>>, jobs: Vec<Vec<u8>>) {
    let mut result: u32 = 0;
    let mut failed_jobs: Vec<Vec<u8>> = vec![];
    for job in jobs {
        let mut j2 = job.clone();
        j2.sort_by(|a, b| compare_page(&rules, a, b));
        if j2.clone().iter().zip(job).all(|a| *a.0 == a.1) {
            result += (j2[(j2.len()) / 2]) as u32
        } else {
            failed_jobs.push(j2.clone());
        }
    }
    println!("{result}");
    part_2(failed_jobs);
}

fn part_2(jobs: Vec<Vec<u8>>) {
    let result: u32 = jobs.iter().fold(0, |acc, v| acc + v[v.len() / 2] as u32);
    println!("{result}");
}

fn compare_page(rules: &HashMap<u8, Vec<u8>>, a: &u8, b: &u8) -> Ordering {
    if rules.contains_key(a) && rules.get(a).unwrap().contains(b) {
        return Ordering::Less;
    }
    if rules.contains_key(b) && rules.get(b).unwrap().contains(a) {
        return Ordering::Greater;
    }
    Ordering::Equal
}
