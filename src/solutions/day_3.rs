
pub fn count_tree_impacts(input: &str, stride_x: u32, stride_y: i32) -> u32 {
	let mut x_offset = 0usize;
	let mut y_offset = -1i32;
	let mut tree_counter = 0u32;
	for line in input.lines() {
		y_offset += 1;
		if y_offset % stride_y != 0 {
			continue
		}
		x_offset = x_offset % line.as_bytes().len();
		if line.as_bytes()[x_offset] == b'#' {
			tree_counter += 1u32;
		}
		x_offset = x_offset + stride_x as usize;
	}
	tree_counter
}

pub fn part_1(input: &str) -> u32 {
	count_tree_impacts(input, 3, 1)
}

pub fn part_2(input: &str) -> u32 {
	count_tree_impacts(input, 1, 1) *
	count_tree_impacts(input, 3, 1) *
	count_tree_impacts(input, 5, 1) *
	count_tree_impacts(input, 7, 1) *
	count_tree_impacts(input, 1, 2)
}