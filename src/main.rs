use day1::{day1part1, day1part2};
use day3::{day3part1, day3part2};
use day4::{part1, part2};
mod day1;
mod day3;
mod day4;
mod util;

fn main() {
    println!("{:?}", day1part1("day1.txt"));
    println!("{:?}", day1part2("day1.txt"));
    println!("{:?}", day3part1("day3.txt"));
    println!("{:?}", day3part2("day3.txt"));
    println!("{:?}", part1("day4.txt"));
    println!("{:?}", part2("day4.txt"));
}
