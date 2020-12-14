
fn check_sled_rental(data: &str) -> bool {
	let vec: Vec<&str> = data.split(|c| c == '-' || c == ' ' || c == ':').filter(|x| !x.is_empty()).collect();
	let min: u32 = vec[0].parse().unwrap();
	let max: u32 = vec[1].parse().unwrap();
	let count = vec[3].matches(vec[2]).count() as u32;
	count >= min && count <= max
}

fn check_tobogan(data: &str) -> bool {
	let vec: Vec<&str> = data.split(|c| c == '-' || c == ' ' || c == ':').filter(|x| !x.is_empty()).collect();
	let pos_1: usize = vec[0].parse().unwrap();
	let pos_2: usize = vec[1].parse().unwrap();
	let target = vec[2].chars().next().unwrap();
	vec[3].chars().enumerate().map(
			|(idx, one_char)| {
				let idx = idx + 1;
				if one_char == target && (idx == pos_1 || idx == pos_2) {
					1u32 
				} else { 
					0u32
				}
			}
		).sum::<u32>() == 1u32
}

pub fn part_1(input: &str) -> u32 {
	input.lines().filter(|line| check_sled_rental(line)).count() as u32
}

pub fn part_2(input: &str) -> u32 {
	input.lines().filter(|line| check_tobogan(line)).count() as u32
}
