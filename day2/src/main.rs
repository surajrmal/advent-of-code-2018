use std::collections::HashMap;

fn parse_ids<'a>(input: &'a str) -> Vec<&'a str> {
    input.split("\n").collect()
}

fn part1(id_list: &Vec<&str>) -> usize {
    let mut two = 0;
    let mut three = 0;
    for id in id_list.iter() {
	let mut letters = HashMap::new();
	for letter in id.chars() {
	    let entry = letters.entry(letter).or_insert(0);
	    *entry += 1;
	}
	if letters.values().any(|occurences| *occurences == 2) {
	    two += 1;
	}
	if letters.values().any(|occurences| *occurences == 3) {
	    three += 1;
	}
    }
    two * three
}

fn part2(id_list: &Vec<&str>) -> String {
    for id in id_list.iter() {
	for id2 in id_list.iter() {
           let common = id.chars()
	                  .zip(id2.chars())
		          .filter_map(|(c1, c2)| if c1 == c2 { Some(c1) } else { None })
		          .collect::<String>();
	   if common.len() == id.len() - 1 {
	       return common;
	   }
	}
    }
    return "".to_string();
}

fn main() {
    let input = include_str!("input.txt");
    let id_list = parse_ids(input);
    println!("part 1: {}", part1(&id_list));
    println!("part 2: {}", part2(&id_list));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
	let input = vec!["abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab"];
        assert_eq!(12, part1(&input));
    }

    #[test]
    fn part2_test() {
	let input = vec!["abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz"];
        assert_eq!("fgij".to_string(), part2(&input));
    }
}

