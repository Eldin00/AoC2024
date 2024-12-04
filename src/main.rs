mod puzzles;

fn main() {
    let full_run = false;
    if full_run {
        println!("Day 1 solutions:");
        puzzles::day_1::start();
        println!("Day 2 solutions:");
        puzzles::day_2::start();
        println!("Day 3 solutions:");
        puzzles::day_3::start();
        }
    println!("Day 4 solutions:");
    puzzles::day_4::start();

}
