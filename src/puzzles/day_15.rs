use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn start() {
    let start = std::time::Instant::now();
    let file = File::open("./input/day_15s.txt").unwrap();
    let reader = BufReader::new(file);
    let mut map: Vec<Vec<char>> = vec![];
    let mut bot: (usize, usize) = (0, 0);
    let mut commands: Vec<char> = vec![];
    let mut n: usize = 0;
    for l in reader.lines() {
        let temp = l.unwrap();
        if temp.starts_with('#') {
            let v: Vec<char> = temp.chars().collect();
            let mut tvec: Vec<char> = vec![];
            for i in 0..temp.len() {
                tvec.push(match v[i] {
                    '#' => '#',
                    '.' => '.',
                    'O' => 'O',
                    '@' => {
                        bot = (n, i);
                        '.'
                    }
                    _ => 'X',
                });
            }
            map.push(tvec);
        } else {
            commands.extend(temp.chars());
        }
        n += 1;
    }
    part_1(bot.clone(), map.clone(), commands.clone());
    part_2(bot, map, commands);
    println!("Time: {:?}", start.elapsed());
}

fn part_1(mut bot: (usize, usize), mut map: Vec<Vec<char>>, cmd: Vec<char>) {
    println!("{map:?}");
    for c in cmd {
        bot = try_move(bot, &mut map, c);
    }
    println!("{}", compute_gps(map));
}

fn part_2(mut bot: (usize, usize), mut map: Vec<Vec<char>>, cmd: Vec<char>) {
    // let mut m: Vec<Vec<char>> = vec![];
    // bot.1 *= 2;
    // for r in map.clone(){
    //     let mut tmp_vec: Vec<char> = vec![];
    //     for c in r {
    //         match c {
    //             '#' => {tmp_vec.extend_from_slice(&['#','#']);},
    //             '.' => {tmp_vec.extend_from_slice(&['.','.']);},
    //             'O' => {tmp_vec.extend_from_slice(&['[',']']);},
    //             _ => ()
    //         }
    //     }
    //     m.push(tmp_vec);
    // }
    // bot.0;
    // for c in cmd {
    //     bot = try_move2(bot, &mut map, c);
    // }
    // println!("{}", compute_gps(map));
}

fn try_move(bot: (usize, usize), map: &mut Vec<Vec<char>>, dir: char) -> (usize, usize) {
    match dir {
        '^' => match map[bot.0 - 1][bot.1] {
            '#' => {
                return bot;
            }
            '.' => {
                return (bot.0 - 1, bot.1);
            }
            'O' => {
                let mut i = bot.0 - 1;
                while i > 0 {
                    i -= 1;
                    if map[i][bot.1] == '#' {
                        return bot;
                    }
                    if map[i][bot.1] == '.' {
                        map[i][bot.1] = 'O';
                        map[bot.0 - 1][bot.1] = '.';
                        return (bot.0 - 1, bot.1);
                    }
                }
            }
            _ => {
                return bot;
            }
        },
        '>' => match map[bot.0][bot.1 + 1] {
            '#' => {
                return bot;
            }
            '.' => {
                return (bot.0, bot.1 + 1);
            }
            'O' => {
                let mut i = bot.1 + 1;
                while i < map[0].len() - 1 {
                    i += 1;
                    if map[bot.0][i] == '#' {
                        return bot;
                    }
                    if map[bot.0][i] == '.' {
                        map[bot.0][i] = 'O';
                        map[bot.0][bot.1 + 1] = '.';
                        return (bot.0, bot.1 + 1);
                    }
                }
            }
            _ => {
                return bot;
            }
        },
        'v' => match map[bot.0 + 1][bot.1] {
            '#' => {
                return bot;
            }
            '.' => {
                return (bot.0 + 1, bot.1);
            }
            'O' => {
                let mut i = bot.0 + 1;
                while i < map.len() - 1 {
                    i += 1;
                    if map[i][bot.1] == '#' {
                        return bot;
                    }
                    if map[i][bot.1] == '.' {
                        map[i][bot.1] = 'O';
                        map[bot.0 + 1][bot.1] = '.';
                        return (bot.0 + 1, bot.1);
                    }
                }
            }
            _ => {
                return bot;
            }
        },
        '<' => match map[bot.0][bot.1 - 1] {
            '#' => {
                return bot;
            }
            '.' => {
                return (bot.0, bot.1 - 1);
            }
            'O' => {
                let mut i = bot.1 - 1;
                while i > 0 {
                    i -= 1;
                    if map[bot.0][i] == '#' {
                        return bot;
                    }
                    if map[bot.0][i] == '.' {
                        map[bot.0][i] = 'O';
                        map[bot.0][bot.1 - 1] = '.';
                        return (bot.0, bot.1 - 1);
                    }
                }
            }
            _ => {
                return bot;
            }
        },
        _ => {
            return bot;
        }
    };
    bot
}

// fn try_move2(mut bot: (usize, usize), map: &mut Vec<Vec<char>>, dir: char) -> (usize, usize) {

//     bot
// }

fn compute_gps(map: Vec<Vec<char>>) -> u32 {
    let mut result: u32 = 0;
    for y in 0..map.len() as usize {
        for x in 0..map[0].len() as usize {
            if map[y][x] == 'O' || map[y][x] == '[' {
                result += (y * 100 + x) as u32;
            }
        }
    }
    result
}
