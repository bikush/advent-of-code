use std::io;

fn main() {
    println!("AOC!");
    let mut numbers: Vec<u32> = vec![];
    loop {
		let mut line = String::new();
		io::stdin().read_line(&mut line).expect("Failed to read the line");
		let number: u32 = match line.trim().parse() {
			Ok(num) => num,
			Err(_) => break
		};
		numbers.push(number);
	}
    let day1_1 = advent_of_code::day_1_part_1(&numbers);
    let day1_2 = advent_of_code::day_1_part_2(&numbers);

    println!("Day 1: {}, {}", day1_1, day1_2);
}
