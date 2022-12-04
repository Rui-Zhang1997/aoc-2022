mod day4;
mod utils;

fn main() {
    let fp = "day4/input.txt";
    println!("soln 1: {}", day4::solns::solution1(fp));
    println!("soln 2: {}", day4::solns::solution2(fp));
}