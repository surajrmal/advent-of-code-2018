fn part1(input: usize) -> String {
    let mut recipes = vec![3, 7];
    let mut elf1 = 0;
    let mut elf2 = 1;
    recipes.reserve(input + 10);
    while recipes.len() < (input + 10) {
	let new_recipe = recipes[elf1] + recipes[elf2];
	if new_recipe >= 10 {
	    recipes.push(1);
	    recipes.push(new_recipe % 10);
	} else {
	    recipes.push(new_recipe);
	}
	elf1 = (elf1 + recipes[elf1] + 1) % recipes.len();
	elf2 = (elf2 + recipes[elf2] + 1) % recipes.len();
    }
    recipes.iter().skip(input).take(10).map(|&i| {
	std::char::from_digit(i as u32, 10).unwrap()
    }).collect()
}

fn part2(input: &str) -> usize {
    let scores : Vec<usize> = input.chars().map(|c| c.to_digit(10).unwrap() as usize).collect();
    let mut recipes = vec![3, 7];
    let mut elf1 = 0;
    let mut elf2 = 1;
    let make_recipe = |recipes: &mut Vec<usize>, elf1: &mut usize, elf2: &mut usize| {
	let new_recipe = recipes[*elf1] + recipes[*elf2];
	if new_recipe >= 10 {
	    recipes.push(1);
	    recipes.push(new_recipe % 10);
	} else {
	    recipes.push(new_recipe);
	}
	*elf1 = (*elf1 + recipes[*elf1] + 1) % recipes.len();
	*elf2 = (*elf2 + recipes[*elf2] + 1) % recipes.len();
    };
    while recipes.len() < scores.len() {
	make_recipe(&mut recipes, &mut elf1, &mut elf2);
    }
    loop {
	let before = recipes.len() - scores.len();
	if recipes.iter().skip(before).zip(scores.iter()).all(|(x, y)| x == y) {
	    return before;
	}
	if before > 0 && recipes.iter().skip(before- 1).zip(scores.iter()).all(|(x, y)| x == y) {
	    return before -1;
	}
	make_recipe(&mut recipes, &mut elf1, &mut elf2);
    }
}

fn main() {
    println!("part1: {}", part1(824501));
    println!("part2: {}", part2("824501"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
	assert_eq!(part1(9), "5158916779".to_string());
	assert_eq!(part1(5), "0124515891".to_string());
	assert_eq!(part1(18), "9251071085".to_string());
	assert_eq!(part1(2018), "5941429882".to_string());
    }

    #[test]
    fn part2_test() {
	assert_eq!(part2("51589"), 9);
	assert_eq!(part2("01245"), 5);
	assert_eq!(part2("92510"), 18);
	assert_eq!(part2("59414"), 2018);
    }
}
