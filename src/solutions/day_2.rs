
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
