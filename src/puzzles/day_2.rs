use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn start() {
    let file = File::open("./input/day_2.txt").unwrap();
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
    let mut safe_count: u32 = 0;
    for l in lines {
        let vs: Vec<&str> = l.as_str().split_whitespace().collect();
        let vi: Vec<u32> = vs.iter().map(|x| x.parse::<u32>().unwrap()).collect();
        if safe_test(vi) {
            safe_count += 1;
        }
    }
    println!("{safe_count}");
}

fn part_2(lines: Vec<String>) {
    let mut safe_count: u32 = 0;
    for l in lines {
        let vs: Vec<&str> = l.as_str().split_whitespace().collect();
        let vi: Vec<u32> = vs.iter().map(|x| x.parse::<u32>().unwrap()).collect();
        if safe_test2(vi) {
            safe_count += 1;
        }
    }
    println!("{safe_count}");
}

fn safe_test(list: Vec<u32>) -> bool {
    let ascending = list[0] < list[1];
    for i in 1..list.len() {
        if (list[i - 1] < list[i]) != ascending
            || list[i].abs_diff(list[i - 1]) > 3
            || list[i] == list[i - 1]
        {
            return false;
        }
    }
    true
}

fn safe_test2(mut list: Vec<u32>) -> bool {
    let ascending = list[0] < list[1];
    for i in 1..list.len() {
        if (list[i - 1] < list[i]) != ascending
            || list[i].abs_diff(list[i - 1]) > 3
            || list[i] == list[i - 1]
        {
            // It's possible that if the direction switches at the 2nd value in the list that removing the head of the list will fix 
            // it, but removing the 2nd and 3rd values (the pair that failed) will not. In all other cases only the 2 values from the
            // first pair to fail need to be checked.
            if i == 2 && (list[i - 1] < list[i]) != ascending {
                let mut list_copy_1 = list.clone();
                let mut list_copy_2 = list.clone();
                _ = list_copy_1.remove(i);
                _ = list_copy_2.remove(i-1); 
                _ = list.remove(0);
                return safe_test(list_copy_1) || safe_test(list_copy_2) || safe_test(list);
            }
            let mut list_copy = list.clone();
            _ = list_copy.remove(i);
            _ = list.remove(i - 1);
            return safe_test(list_copy) || safe_test(list);
        }
    }
    true
}
