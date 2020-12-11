
pub fn day_1_part_1(input: &Vec<u32>) -> u32 {
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

pub fn day_1_part_2(input: &Vec<u32>) -> u32 {
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
