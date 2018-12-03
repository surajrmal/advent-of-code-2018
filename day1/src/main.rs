use std::collections::HashSet;

fn parse_frequencies(input: &str) -> Vec<i32> {
    input.split("\n")
	 .filter(|num| !num.is_empty())
         .map(|num| num.parse::<i32>().unwrap())
         .collect()
}

fn part2(frequency_list: Vec<i32>) -> i32 {
    let mut frequencies = HashSet::new();
    let mut current = 0;
    loop {
	for frequency in frequency_list.iter() {
	    current += frequency;
	    if frequencies.contains(&current) {
		return current;
	    }
	    frequencies.insert(current);
	}
    }
}

fn main() {
    let input = include_str!("input.txt");
    let frequency_list = parse_frequencies(input);
    println!("part 1: {}", frequency_list.iter().sum());
    println!("part 2: {}", part2(frequency_list));
}
