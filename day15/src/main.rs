#![feature(drain_filter)]

use std::fmt;
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Space {
    Wall,
    Empty,
    EmptyVisited,
    Unit(usize),
}

type Map = Vec<Vec<Space>>;

#[derive(Debug, Clone, PartialEq, Eq)]
enum UnitType {
    Goblin,
    Elf,
}

#[derive(Debug, Clone)]
struct Unit {
    x : usize,
    y : usize,
    unit_type : UnitType,
    hitpoints : u32,
    attack_power : u32,
    dead : bool,
}
impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	match self.unit_type {
	    UnitType::Goblin => write!(f, "G")?,
	    UnitType::Elf => write!(f, "E")?,
	}
	write!(f, "({}): ({}, {})", self.hitpoints, self.x, self.y)
    }
}

#[derive(Debug, Clone)]
struct Game {
    map: Map,
    units : Vec<Unit>,
    round : u32,
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	write!(f, "Round: {}\n", self.round)?;
	for line in self.map.iter() {
	    let line = line.iter().map(|space| {
		match space {
		    Space::Wall => '#',
		    Space::Empty | Space::EmptyVisited => '.',
		    Space::Unit(unit) => {
			match self.units[*unit].unit_type {
			    UnitType::Goblin => 'G',
			    UnitType::Elf => 'E',
			}
		    }
		}
	    }).collect::<String>();
	    write!(f, "{}\n", line)?;
	}
	for unit in self.units.iter() {
	    write!(f, "{}\n", unit)?;
	}
	Ok(())
    }
}

#[derive(Debug, Copy, Clone)]
enum Move {
    Up,
    Left,
    Right,
    Down,
}

impl Game {
    fn is_over(&self) -> bool {
	self.units.iter().filter(|u| u.unit_type == UnitType::Elf && !u.dead).count() == 0 ||
	    self.units.iter().filter(|u| u.unit_type == UnitType::Goblin && !u.dead).count() == 0
    }

    fn elf_died(&self ) -> bool {
	self.units.iter().filter(|u| u.unit_type == UnitType::Elf).any(|elf| elf.dead)
    }

    fn set_elf_attack_power(&mut self, attack_power: u32) {
	self.units.iter_mut()
	          .filter(|u| u.unit_type == UnitType::Elf)
		  .for_each(|elf| elf.attack_power = attack_power);
    }


    fn outcome(&self) -> u32 {
	assert!(self.is_over());
	self.units.iter()
		  .map(|u| u.hitpoints)
		  .sum::<u32>() * self.round
    }

    fn try_attack(&self, unit: usize) -> Option<usize> {
	let unit = &self.units[unit];
	let check = |x: usize, y: usize| {
	    match self.map[y][x] {
		Space::Unit(u) => {
		    if unit.unit_type != self.units[u].unit_type {
			Some(u)
		    } else {
			None
		    }
		}
		_ => None,
	    }
	};

	let check_best = |x, y, best: Option<usize>| {
	    if let Some(ref b) = best {
		if let Some(u) = check(x, y) {
		    if self.units[u].hitpoints < self.units[*b].hitpoints {
			return Some(u)
		    }
		}
		best
	    } else {
		check(x, y)
	    }
	};

	let x = unit.x;
	let y = unit.y;

	let best : Option<usize> = None;
	let best = check_best(x, y - 1, best);
	let best = check_best(x - 1, y, best);
	let best = check_best(x + 1, y, best);
	let best = check_best(x, y + 1, best);

	best
    }

    fn try_move(&self, unit: usize) -> Option<Move> {
	if let Some(_) = self.try_attack(unit) {
	    return None;
	}
	
	let unit = &self.units[unit];
	let mut map = self.map.clone();
	let mut queue = VecDeque::new();

	queue.push_back((unit.x, unit.y - 1, Move::Up));
	queue.push_back((unit.x - 1, unit.y, Move::Left));
	queue.push_back((unit.x + 1, unit.y, Move::Right));
	queue.push_back((unit.x, unit.y + 1, Move::Down));

	while let Some((x, y, move_dir)) = queue.pop_front() {
	    match map[y][x] {
		Space::Unit(u) => {
		    if unit.unit_type != self.units[u].unit_type {
			return Some(move_dir)
		    }
		}
		Space::Empty => {
		    map[y][x] = Space::EmptyVisited;
		    queue.push_back((x, y - 1, move_dir));
		    queue.push_back((x - 1, y, move_dir));
		    queue.push_back((x + 1, y, move_dir));
		    queue.push_back((x, y + 1, move_dir));
		}
		_ => (),
	    }
	}

	None
    }

    fn perform_round(&mut self) {
	let order = self.map.iter().flat_map(|line| {
	    line.iter().filter_map(|space| {
		match space {
		    Space::Wall | Space::Empty => None,
		    Space::Unit(u) => Some(*u),
		    Space::EmptyVisited => panic!("Space is visited!"),
		}
	    })
	}).collect::<Vec<_>>();

	let mut last_kill = 0;
	for unit in order.iter() {
	    if self.units[*unit].dead {
		// This unit was killed earlier in the round.
		continue;
	    }
	    // Move if we can/need to.
	    if let Some(move_dir) = self.try_move(*unit) {
		let u = &mut self.units[*unit];
		self.map[u.y][u.x] = Space::Empty;
		match move_dir {
		    Move::Up => u.y -= 1,
		    Move::Left => u.x -= 1,
		    Move::Right => u.x += 1,
		    Move::Down => u.y += 1,
		}
		assert!(self.map[u.y][u.x] == Space::Empty);
		self.map[u.y][u.x] = Space::Unit(*unit);
	    }
	    // Attack an enemy neighbor if possible.
	    if let Some(other) = self.try_attack(*unit) {
		let attack_power = self.units[*unit].attack_power;
		let other = &mut self.units[other];
		if other.hitpoints > attack_power  {
		    other.hitpoints -= attack_power;
		} else {
		    println!("{} killed by {}", other, unit);
		    last_kill = *unit;
		    other.hitpoints = 0;
		    other.dead = true;
		    self.map[other.y][other.x] = Space::Empty;
		}
	    }
	}
	let last_to_go = order.into_iter().rev().find(|unit| !self.units[*unit].dead).unwrap();
	if !self.is_over() || (self.is_over() && last_kill == last_to_go) {
	    self.round += 1;
	}
	println!("{}", self);
    }
}

fn parse_input(input: &str) -> Game {
    let x_size = input.lines().nth(0).unwrap().chars().count();
    let y_size = input.lines().count();
    let mut map = vec![vec![Space::Wall; y_size]; x_size];
    let mut units = Vec::new();

    for (y, line) in input.trim().lines().enumerate() {
	for (x, ch) in line.chars().enumerate() {
	    map[y][x] = match ch {
		'#' => Space::Wall,
		'.' => Space::Empty,
		'G' => {
		    units.push(Unit {
			x: x,
			y: y,
			unit_type: UnitType::Goblin,
			hitpoints: 200,
			attack_power: 3,
			dead: false,
		    });
		    Space::Unit(units.len() - 1)
		}
		'E' => {
		    units.push(Unit {
			x: x,
			y: y,
			unit_type: UnitType::Elf,
			hitpoints: 200,
			attack_power: 3,
			dead: false,
		    });
		    Space::Unit(units.len() - 1)
		}
		_ => panic!("invalid input!"),
	    }
	}
    }

    Game {
	map: map,
	units: units,
	round: 0,
    }
}

fn part1(mut game: Game) -> u32 {
    //println!("{}", game);
    while !game.is_over() {
	game.perform_round()
    }
    //println!("{}", game);
    game.outcome()
}

fn part2(game: Game) -> u32 {
    println!("{}", game);
    for attack_power in 4.. {
	let mut g = game.clone();
	g.set_elf_attack_power(attack_power);
	while !(g.is_over() || g.elf_died()) {
	    g.perform_round()
	}
	if !g.elf_died() {
	    println!("{}", g);
	    println!("attack_power: {}", attack_power);
	    return g.outcome();
	}
    }
    unreachable!();
}

fn main() {
    let game = parse_input(include_str!("input.txt"));
    println!("part1: {}", part1(game.clone()));
    println!("part2: {}", part2(game));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
	let game = parse_input(include_str!("example1.txt"));
	assert_eq!(part1(game), 36334);
    }

    #[test]
    fn part1_test2() {
	let game = parse_input(include_str!("example2.txt"));
	assert_eq!(part1(game), 39514);
    }

    #[test]
    fn part1_test3() {
	let game = parse_input(include_str!("example3.txt"));
	assert_eq!(part1(game), 27755);
    }

    #[test]
    fn part1_test4() {
	let game = parse_input(include_str!("example4.txt"));
	assert_eq!(part1(game), 28944);
    }

    #[test]
    fn part1_test5() {
	let game = parse_input(include_str!("example5.txt"));
	assert_eq!(part1(game), 18740);
    }

    #[test]
    fn part2_test1() {
	let game = parse_input(include_str!("example0.txt"));
	assert_eq!(part2(game), 4988);
    }

    #[test]
    fn part2_test2() {
	let game = parse_input(include_str!("example2.txt"));
	assert_eq!(part2(game), 31284);
    }

    #[test]
    fn part2_test3() {
	let game = parse_input(include_str!("example3.txt"));
	assert_eq!(part2(game), 3478);
    }

    #[test]
    fn part2_test4() {
	let game = parse_input(include_str!("example4.txt"));
	assert_eq!(part2(game), 6474);
    }

    #[test]
    fn part2_test5() {
	let game = parse_input(include_str!("example5.txt"));
	assert_eq!(part2(game), 1140);
    }
}
