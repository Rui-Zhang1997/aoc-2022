mod day3;
mod utils;

fn main() {
    let fp = "day3/input.txt";
    println!("soln 1: {}", day3::solns::solution1(fp));
    println!("soln 2: {}", day3::solns::solution2(fp));
}