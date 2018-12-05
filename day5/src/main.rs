fn part1(input: &str) -> usize {
    let mut chemicals = String::from(input);
    eprintln!("chemicals_len: {}", chemicals.len());
    loop {
	let mut iter = chemicals.chars().peekable();
	let mut accum = String::new();
	let mut letter = iter.next();
	let mut changed = false;
	while let Some(l) = letter {
	    if let Some(p) = iter.peek() {
		if l != *p && l.to_ascii_lowercase() == p.to_ascii_lowercase() {
		    changed = true;
		    accum.push_str(iter.skip(1).collect::<String>().as_str());
		    break;
		}
	    }
	    accum.push(l);
	    letter = iter.next()
	}
	if !changed {
	    return chemicals.len();
	}
	chemicals = accum;
    }
}

fn part2(input: &str) -> usize {
    (b'a'..b'z').map(|c| c as char).map(|lowercase| {
	let uppercase = lowercase.to_ascii_uppercase();

	let input = input.chars()
	                 .filter(|c| *c != lowercase)
			 .filter(|c| *c != uppercase)
			 .collect::<String>();
	part1(input.as_str())
    }).min().unwrap()
}

fn main() {
    let input = include_str!("input.txt").trim();
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
	assert_eq!(part1("dabAcCaCBAcCcaDA"), 10);
	assert_eq!(part1("Aa"), 0);
	assert_eq!(part1("aA"), 0);
	assert_eq!(part1("Aab"), 1);
	assert_eq!(part1("BAab"), 0);
	assert_eq!(part1("BAcab"), 5);
	assert_eq!(part1("AbBca"), 3);
    }
}
