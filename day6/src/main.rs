#[macro_use]
extern crate failure;
extern crate rayon;

use {
    failure::Error,
    rayon::prelude::*,
    std::{
        cmp::Ordering,
        collections::{HashMap, HashSet},
	str::FromStr,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point {
    x : i32,
    y : i32,
}

impl Point {
    fn dist(&self, other: &Point) -> i32 {
	i32::abs(self.x - other.x) + i32::abs(self.y - other.y)
    }
}

impl FromStr for Point {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
	let s = s.split(", ")
	         .map(|num| str::parse::<i32>(num))
		 .collect::<Result<Vec<_>, std::num::ParseIntError>>()?;
	if s.len() != 2 {
	    return Err(format_err!("invalid string: {:?}", s));
	}
	Ok(Point {
	    x: s[0],
	    y: s[1],
	})
    }
}

fn parse_input(input: &str) -> Vec<Point> {
    input.lines()
	 .filter(|line| !line.is_empty())
	 .map(|line| str::parse::<Point>(line))
	 .collect::<Result<Vec<_>, Error>>()
	 .unwrap()
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
struct PointAndDist(i32, Point);

impl Ord for PointAndDist {
    fn cmp(&self, other: &PointAndDist) -> Ordering {
        self.0.cmp(&other.0)
    }
}

// Returns None if tie.
fn find_best(current: &Point, points: &Vec<Point>) -> Option<Point> {
    let mut points = points.into_iter().map(|point| {
	PointAndDist(current.dist(point), point.clone())
    }).collect::<Vec<_>>();
    points.sort();
    if points[0].0 == points[1].0 {
	return None
    }
    Some(points[0].1.clone())
}

fn part1(points: &Vec<Point>) -> i32 {
    // Find boundaries.
    let minx = points.iter().map(|p| p.x).min().unwrap();
    let miny = points.iter().map(|p| p.y).min().unwrap();
    let maxx = points.iter().map(|p| p.x).max().unwrap();
    let maxy = points.iter().map(|p| p.y).max().unwrap();

    let mut count = HashMap::new();
    // Find winners for each point in subspace.
    for x in minx..=maxx {
	for y in miny..=maxy {
	    let current = Point { x: x, y: y };
	    if let Some(best_point) = find_best(&current, points) {
		let entry = count.entry(best_point).or_insert(0);
		*entry += 1;
	    }
	}
    }
    // Find ones who will be infinite.
    let mut infinite = HashSet::new();
    let mut check_infinite = |x, y| {
	let current = Point { x: x, y: y };
	if let Some(best_point) = find_best(&current, points) {
	    infinite.insert(best_point);
	}
    };
    for x in (minx-1)..=(maxx+1) {
	check_infinite(x as i32, miny - 1);
	check_infinite(x as i32, maxy + 1);
    }
    for y in (miny-1)..=(maxy+1) {
	check_infinite(minx - 1, y as i32);
	check_infinite(maxx + 1, y as i32);
    }
    // Remove infinite regions and find max.
    *count.iter()
	  .filter(|(point, _)| !infinite.contains(point))
	  .map(|(_, count)| count)
	  .max()
	  .unwrap()
}

fn part2(points: &Vec<Point>, less: i32) -> usize {
    // Find boundaries.
    let minx = points.iter().map(|p| p.x).min().unwrap();
    let miny = points.iter().map(|p| p.y).min().unwrap();
    let maxx = points.iter().map(|p| p.x).max().unwrap();
    let maxy = points.iter().map(|p| p.y).max().unwrap();

    // Check each point in search space.
    let xs = (minx..=maxx).collect::<Vec<_>>();
    xs.par_iter().map(|&x| {
	(miny..=maxy).filter(|&y| {
	    let current = Point { x: x, y: y };
	    points.iter().map(|point| {
		current.dist(point)
	    }).sum::<i32>() < less
	}).count()
    }).sum()
}

fn main() {
    let input = include_str!("input.txt").trim();
    let points = parse_input(input);
    println!("part1: {:?}", part1(&points));
    println!("part2: {:?}", part2(&points, 10000));
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
	let input = "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";
	let points = parse_input(input);
	assert_eq!(part1(&points), 17);
    }

    #[test]
    fn part2_test() {
	let input = "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";
	let points = parse_input(input);
	assert_eq!(part2(&points, 32), 16);
    }
}
