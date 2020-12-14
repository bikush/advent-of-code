
pub mod day_1 {

pub fn part_1(input: &Vec<u32>) -> u32 {
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

pub fn part_2(input: &Vec<u32>) -> u32 {
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

}

pub mod day_2 {

fn check(data: &str) -> bool {
	let vec: Vec<&str> = data.split(|c| c == '-' || c == ' ' || c == ':').filter(|x| !x.is_empty()).collect();
	let min: u32 = vec[0].parse().unwrap();
	let max: u32 = vec[1].parse().unwrap();
	let count = vec[3].matches(vec[2]).count() as u32;
	count >= min && count <= max
}



pub fn part_1(input: &str) -> u32 {
	input.lines().filter(|line| check(line)).count() as u32
}

}