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

fn day_1() {
	// Day1
	let numbers = read("data/day1.txt").unwrap();
	let part_1 = advent_of_code::day_1::part_1(&numbers);
	let part_2 = advent_of_code::day_1::part_2(&numbers);
	println!("Day 1: {}, {}", part_1, part_2);  
}

fn day_2() {
    // Day1
    let mut file = File::open("data/day2.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let part_1 = advent_of_code::day_2::part_1(&contents);
    println!("Day 2: {}, {}", part_1, 0);  
}

fn main() {
	println!("AOC!");

    day_1();
    day_2();
}
