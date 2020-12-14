use std::fs::File;
use std::io::prelude::*;

mod solutions;
pub use crate::solutions::day_1;
pub use crate::solutions::day_2;
pub use crate::solutions::day_3;
pub use crate::solutions::day_4;

fn day_runner(idx: u32, part_1: &dyn Fn(&str) -> u32, part_2: &dyn Fn(&str) -> u32) {
    let mut file = File::open(format!("data/day{}.txt", idx)).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("Day {}: {}, {}", idx, part_1(&contents), part_2(&contents));  
}


fn main() {
	println!("AOC!");

    day_runner(1, &day_1::part_1, &day_1::part_2);
    day_runner(2, &day_2::part_1, &day_2::part_2);
    day_runner(3, &day_3::part_1, &day_3::part_2);
    day_runner(4, &day_4::part_1, &day_4::part_2);
}
