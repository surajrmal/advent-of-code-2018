use std::collections::HashSet;

#[derive(Clone, Debug)]
struct Rule([bool; 5], bool);

fn parse_initial_state(input: &str) -> HashSet<isize> {
    input.trim().chars().enumerate().filter_map(|(i, ch)| {
	match ch {
	    '.' => None,
	    '#' => Some(i as isize),
	    _ => panic!("invalid initial state char ({}) at index {}", ch, i),
	}
    }).collect()
}

fn parse_rules(input: &str) -> HashSet<Vec<bool>> {
    input.trim().lines().filter(|line| {
	// Only take rules where one is create.
	line.chars().last().map(|ch| {
	    match ch {
		'.' => false,
		'#' => true,
		_ => panic!("invalid rule char ({})", ch),
	    }
	}).expect("no last char")
    }).map(|line| {
	line.chars().take(5).map(|ch| {
	    match ch {
		'.' => false,
		'#' => true,
		_ => panic!("invalid rule char ({})", ch),
	    }
	}).collect()
    }).collect()
}

fn part1(mut pots: HashSet<isize>, rules: &HashSet<Vec<bool>>, generations: usize) -> isize {
    let mut lowest = 0isize;
    let mut highest = *pots.iter().max().unwrap() as isize;
    let mut prev_prev = 0;
    let mut prev = pots.iter().sum::<isize>();
    for gen in 0..generations {
	if gen % 10000 == 0 {
	    // Take advanatage of the fact that input seems to converge...
	    let current = pots.iter().sum::<isize>();
	    if prev - prev_prev ==  current - prev {
		let diff = current - prev;
		let iter_left = ((generations - gen) / 10000) as isize;
		return current + (iter_left * diff);
	    }
	    prev_prev = prev;
	    prev = current;
	}
	let mut new_highest = lowest;
	let mut new_lowest = highest;
	pots = ((lowest - 2)..=(highest + 2)).filter_map(|i| {
	    let mut substate = vec![false; 5];
	    substate[0] = pots.contains(&(i - 2));
	    substate[1] = pots.contains(&(i - 1));
	    substate[2] = pots.contains(&i);
	    substate[3] = pots.contains(&(i + 1));
	    substate[4] = pots.contains(&(i + 2));
	    if rules.contains(&substate) {
		new_lowest = std::cmp::min(new_lowest, i);
		new_highest = std::cmp::max(new_highest, i);
		Some(i)
	    } else {
		None
	    }
	}).collect();
	lowest = new_lowest;
	highest = new_highest;
    }
    pots.iter().sum()
}

fn main() {
    let initial_state = parse_initial_state("#...#..###.#.###.####.####.#..#.##..#..##..#.....#.#.#.##.#...###.#..##..#.##..###..#..##.#..##...");
    let rules = parse_rules(include_str!("input.txt"));
    println!("part1: {}", part1(initial_state.clone(), &rules, 20));
    println!("part2: {}", part1(initial_state.clone(), &rules, 50000000000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
	let initial_state = parse_initial_state("#..#.#..##......###...###");
	let rules = parse_rules(include_str!("sample.txt"));
	assert_eq!(part1(initial_state, &rules, 20), 325);
    }
}
