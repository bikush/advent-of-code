
fn get_numbers(contents: &str) -> Vec<u32> {
	let mut numbers: Vec<u32> = vec!();
	for line in contents.lines() {
		let number: u32 = match line.trim().parse() {
			Ok(num) => num,
			Err(_) => continue
		};
		numbers.push(number);
	}
	numbers
}

pub fn part_1(text: &str) -> u32 {
	let input = get_numbers(text);
	let count = input.len();
	for x in 0..count-1 {
		for y in x+1..count {
			if input[x] + input[y] == 2020u32 {
				return input[x] * input[y]
			}
		}
	}
	0
}

pub fn part_2(text: &str) -> u32 {
	let input = get_numbers(text);
	let count = input.len();
	for x in 0..count-2 {
		for y in x+1..count-1 {
			for z in y+1..count {
				if input[x] + input[y] + input[z] == 2020u32 {
					return input[x] * input[y] * input[z]
				}
			}
		}
	}
	0
}
