mod puzzles;

fn main() {
    let full_run = true;
    if full_run {
        println!("Day 1 solutions:");
        puzzles::day_1::start();
        println!("Day 2 solutions:");
        puzzles::day_2::start();
    }
}
