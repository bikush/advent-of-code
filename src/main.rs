use std::fs::File;
use std::io::prelude::*;

fn read(path: &str) -> std::io::Result<Vec<u32>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut numbers: Vec<u32> = vec!();
    for line in contents.lines() {
    	let number: u32 = match line.trim().parse() {
			Ok(num) => num,
			Err(_) => continue
		};
		numbers.push(number);
    }
    Ok(numbers)
}

fn main() {
    println!("AOC!");

    // Day1
    let numbers = read("data/day1.txt").unwrap();
    let day1_1 = advent_of_code::day_1_part_1(&numbers);
    let day1_2 = advent_of_code::day_1_part_2(&numbers);
    println!("Day 1: {}, {}", day1_1, day1_2);
}
