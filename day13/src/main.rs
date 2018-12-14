use std::collections::HashSet;
#[derive(Debug, PartialEq, Clone, Copy)]
enum TurnSpace {
    LeftDown,
    LeftUp,
    RightDown,
    RightUp,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Space {
    Empty,
    LeftRight,
    UpDown,
    Turn(TurnSpace),
    Intersection,
}

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord, Copy)]
enum Turn {
    Left,
    Straight,
    Right,
}

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
struct Cart {
    x: usize,
    y: usize,
    dir: Direction,
    turn: Turn
}

impl Cart {
    fn turn_toggle(&mut self) {
	self.turn = match self.turn {
	    Turn::Left => Turn::Straight,
	    Turn::Straight => Turn::Right,
	    Turn::Right => Turn::Left,
	}
    }

    fn step(&mut self, dir: Direction) {
	self.dir = dir;
	match self.dir {
	    Direction::Up => self.y -= 1,
	    Direction::Right => self.x += 1,
	    Direction::Left => self.x -= 1,
	    Direction::Down => self.y += 1,
	}
    }
}

fn parse_input(input: &str) -> (Vec<Vec<Space>>, Vec<Cart>) {
    let x_size = input.lines().nth(0).unwrap().chars().count();
    let y_size = input.lines().count();
    let mut track = vec![vec![Space::Empty; y_size]; x_size];
    let mut carts = Vec::new();

    for (y, line) in input.lines().enumerate() {
	for (x, ch) in line.chars().enumerate() {
	    assert!(x < x_size);
	    track[x][y] = match ch {
		// Carts
		'^' => {
		    assert!(y > 0 && y < y_size - 1);
		    carts.push(Cart { x: x, y: y, dir: Direction::Up, turn: Turn::Left });
		    Space::UpDown
		}
		'>' => {
		    assert!(x > 0 && x < x_size - 1);
		    carts.push(Cart { x: x, y: y, dir: Direction::Right, turn: Turn::Left });
		    Space::LeftRight
		}
		'<' => {
		    assert!(x > 0 && x < x_size - 1);
		    carts.push(Cart { x: x, y: y, dir: Direction::Left, turn: Turn::Left });
		    Space::LeftRight
		}
		'v' => {
		    assert!(y > 0 && y < y_size - 1);
		    carts.push(Cart { x: x, y: y, dir: Direction::Down, turn: Turn::Left });
		    Space::UpDown
		}
		// Track pieces
		'+' => Space::Intersection,
		'-' => Space::LeftRight,
		'|' => Space::UpDown,
		'/' => {
		    if x > 0 {
			match track[x-1][y] {
			    Space::Intersection | Space::LeftRight => Space::Turn(TurnSpace::LeftUp),
			    _ => Space::Turn(TurnSpace::RightDown),
			}
		    } else if y > 0 {
			match track[x][y-1] {
			    Space::Intersection | Space::UpDown => Space::Turn(TurnSpace::LeftUp),
			    _ => Space::Turn(TurnSpace::RightDown),
			}
		    } else {
			Space::Turn(TurnSpace::RightDown)
		    }
		},
		'\\' => {
		    if x > 0 {
			match track[x-1][y] {
			    Space::Intersection | Space::LeftRight => Space::Turn(TurnSpace::LeftDown),
			    _ => Space::Turn(TurnSpace::RightUp),
			}
		    } else if y > 0 {
			match track[x][y-1] {
			    Space::Intersection | Space::UpDown => Space::Turn(TurnSpace::RightUp),
			    _ => Space::Turn(TurnSpace::LeftDown),
			}
		    } else {
			panic!("Upper left space is turning into obvlivion!");
		    }
		}
		' ' => Space::Empty,
		_ => panic!("Invalid character! {} at ({}, {})", ch, x, y),
	    };
	}
    }
    (track, carts)
}

fn part1(track: &Vec<Vec<Space>>, carts: &Vec<Cart>) -> (usize, usize) {
    let mut carts = carts.clone();
    loop {
	carts.sort();
	for i in 0..carts.len() {
	    let cart = &mut carts[i];
	    match track[cart.x][cart.y] {
		Space::LeftRight => {
		    match cart.dir {
			Direction::Left | Direction::Right => cart.step(cart.dir),
			_ => panic!("Cart facing wrong direction"),
		    }
		},
		Space::UpDown => {
		    match cart.dir {
			Direction::Up | Direction::Down => cart.step(cart.dir),
			_ => panic!("Cart facing wrong direction"),
		    }
		}
		Space::Turn(ref turn) => {
		    let new_dir = match (turn, cart.dir) {
			(TurnSpace::LeftDown, Direction::Right) => Direction::Down,
			(TurnSpace::LeftDown, Direction::Up) => Direction::Left,
			(TurnSpace::LeftUp, Direction::Right) => Direction::Up,
			(TurnSpace::LeftUp, Direction::Down) => Direction::Left,
			(TurnSpace::RightDown, Direction::Left) => Direction::Down,
			(TurnSpace::RightDown, Direction::Up) => Direction::Right,
			(TurnSpace::RightUp, Direction::Down) => Direction::Right,
			(TurnSpace::RightUp, Direction::Left) => Direction::Up,
			_ => panic!("Facing incorrect direction!"),
		    };
		    cart.step(new_dir);
		}
		Space::Intersection => {
		    let new_dir = match (cart.turn, cart.dir) {
			(Turn::Left, Direction::Right) => Direction::Up,
			(Turn::Left, Direction::Down) => Direction::Right,
			(Turn::Left, Direction::Left) => Direction::Down,
			(Turn::Left, Direction::Up) => Direction::Left,
			(Turn::Straight, _) => cart.dir,
			(Turn::Right, Direction::Right) => Direction::Down,
			(Turn::Right, Direction::Down) => Direction::Left,
			(Turn::Right, Direction::Left) => Direction::Up,
			(Turn::Right, Direction::Up) => Direction::Right,
		    };
		    cart.step(new_dir);
		    cart.turn_toggle();
		}
		Space::Empty => panic!("Cart landed off the track!"),
	    }
	    let cart = &carts[i];
	    if carts.iter().filter(|&c| c != cart).any(|c| c.x == cart.x && c.y == cart.y) {
		return (cart.x, cart.y);
	    }
	}
    }
}

fn part2(track: &Vec<Vec<Space>>, carts: &Vec<Cart>) -> (usize, usize) {
    let mut carts = carts.clone();
    loop {
	carts.sort();
	let mut crashed = HashSet::new();
	for i in 0..carts.len() {
	    let cart = &mut carts[i];
	    match track[cart.x][cart.y] {
		Space::LeftRight => {
		    match cart.dir {
			Direction::Left | Direction::Right => cart.step(cart.dir),
			_ => panic!("Cart facing wrong direction(left-right) {:?}", cart.dir),
		    }
		},
		Space::UpDown => {
		    match cart.dir {
			Direction::Up | Direction::Down => cart.step(cart.dir),
			_ => panic!("Cart facing wrong direction(up-down) {:?}", cart.dir),
		    }
		}
		Space::Turn(ref turn) => {
		    let new_dir = match (turn, cart.dir) {
			(TurnSpace::LeftDown, Direction::Right) => Direction::Down,
			(TurnSpace::LeftDown, Direction::Up) => Direction::Left,
			(TurnSpace::LeftUp, Direction::Right) => Direction::Up,
			(TurnSpace::LeftUp, Direction::Down) => Direction::Left,
			(TurnSpace::RightDown, Direction::Left) => Direction::Down,
			(TurnSpace::RightDown, Direction::Up) => Direction::Right,
			(TurnSpace::RightUp, Direction::Down) => Direction::Right,
			(TurnSpace::RightUp, Direction::Left) => Direction::Up,
			_ => panic!("Cart facing wrong direction {:?} {:?} {},{}", turn, cart.dir, cart.x, cart.y),
		    };
		    cart.step(new_dir);
		}
		Space::Intersection => {
		    let new_dir = match (cart.turn, cart.dir) {
			(Turn::Left, Direction::Right) => Direction::Up,
			(Turn::Left, Direction::Down) => Direction::Right,
			(Turn::Left, Direction::Left) => Direction::Down,
			(Turn::Left, Direction::Up) => Direction::Left,
			(Turn::Straight, _) => cart.dir,
			(Turn::Right, Direction::Right) => Direction::Down,
			(Turn::Right, Direction::Down) => Direction::Left,
			(Turn::Right, Direction::Left) => Direction::Up,
			(Turn::Right, Direction::Up) => Direction::Right,
		    };
		    cart.step(new_dir);
		    cart.turn_toggle();
		}
		Space::Empty => panic!("Cart landed off the track!"),
	    }
	    let cart = &carts[i];
	    carts.iter()
		 .enumerate()
		 .filter(|(_, c)| *c != cart)
		 //.filter(|(j, _)| !crashed.contains(j))
		 .filter(|(_, c)| c.x == cart.x && c.y == cart.y)
		 .for_each(|(j, _)| {
		     crashed.insert(i);
		     crashed.insert(j);
	    });
	}
	carts = carts.into_iter().enumerate().filter(|(i, _)| !crashed.contains(i)).map(|(_, c)| c).collect();
	if carts.len() == 1 {
	    return (carts[0].x, carts[0].y)
	}
    }
}

fn main() {
    let (track, carts) = parse_input(include_str!("input.txt"));
    println!("part1: {:?}", part1(&track, &carts));
    println!("part2: {:?}", part2(&track, &carts));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
	let (track, carts) = parse_input(include_str!("sample.txt"));
	assert_eq!(part1(&track, &carts), (7, 3));
    }

    #[test]
    fn part2_test() {
	let (track, carts) = parse_input(include_str!("sample2.txt"));
	assert_eq!(part2(&track, &carts), (6, 4));
    }
}
