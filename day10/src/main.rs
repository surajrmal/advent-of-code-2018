#[macro_use]
extern crate failure;
#[macro_use]
extern crate nom;

use failure::Error;
use nom::types::CompleteStr as Input;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point(isize, isize);

#[derive(Debug, Clone, PartialEq)]
struct Velocity(isize, isize);

#[derive(Debug, Clone, PartialEq)]
struct Node {
    position : Point,
    velocity : Velocity,
}

impl Node {
    fn apply_round(&mut self) {
	self.position.0 += self.velocity.0;
	self.position.1 += self.velocity.1;
    }
}

impl FromStr for Node {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        named!(integer(Input) -> isize, map!(
                pair!(
                    map!(opt!(tag!("-")),
                        |sign| if sign.is_some() { -1_isize } else { 1_isize }),
                    map_res!(nom::digit, |d: Input| d.parse())),
                |(sign, val): (isize, isize)| -> isize { sign * val }));

	named!(point_parser(Input) -> Point,
	  do_parse!(
	    x: ws!(integer) >> tag!(",") >>
	    y: ws!(integer) >>
	    (Point(x, y))
	  )
	);

	named!(velocity_parser(Input) -> Velocity,
	  do_parse!(
	    x: ws!(integer) >> tag!(",") >>
	    y: ws!(integer) >>
	    (Velocity(x, y))
	  )
	);

	named!(node_parser(Input) -> Node,
	  do_parse!(
	    tag!("position=<") >>
	    position: point_parser >> tag!("> velocity=<") >>
	    velocity: velocity_parser >> tag!(">") >>
	    (Node { position, velocity })
	  )
	);

	node_parser(Input(s)).map(|(_s, c)| c).map_err(|e| format_err!("{}", e))
    }
}

fn parse_input(input: &str) -> Result<Vec<Node>, Error> {
    input.split('\n')
	 .filter(|line| !line.is_empty())
	 .map(str::parse::<Node>)
 	 .collect::<Result<Vec<_>, Error>>()
}


fn part1(nodes: &Vec<Node>, rounds: usize) {
    let mut nodes = nodes.clone();
    for i in 0..rounds {
	// Extract all of the points.
	let points = nodes.iter().map(|node| node.position.clone()).collect::<HashSet<_>>();

	// Offset points so min is (0, 0).
	let minx = points.iter().map(|p| p.0).min().unwrap();
	let miny = points.iter().map(|p| p.1).min().unwrap();
	let points = points.iter().map(|p| Point(p.0 - minx, p.1 - miny)).collect::<Vec<_>>();

	// Find max size.
	let maxx = points.iter().map(|p| p.0).max().unwrap();
	let maxy = points.iter().map(|p| p.1).max().unwrap();

	if maxx < 70 {
	    println! ("\nRound {}:", i);
	    let mut grid = vec![vec!['.'; maxy as usize + 1]; maxx as usize + 1];
	    for point in points {
		grid[point.0 as usize][point.1 as usize] = '#';
	    }
	    for y in 0..=maxy {
		for x in 0..=maxx {
		    print!("{}", grid[x as usize][y as usize]);
		}
		println!(".");
	    }
	}

	nodes.iter_mut().for_each(|node| node.apply_round());
    }
}

fn main() {
    let input = include_str!("input.txt");
    let nodes = parse_input(input).unwrap();
    part1(&nodes, 100000);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
	let input = include_str!("sample.txt");
        assert_eq!(
	    Node { position: Point(9, 1), velocity: Velocity(0, 2) },
	    parse_input(&input).unwrap()[0]
	);
    }
}
