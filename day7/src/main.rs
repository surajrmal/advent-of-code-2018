#![feature(drain_filter)]

use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BinaryHeap;

#[derive(Debug)]
struct Pair(char, char);

#[derive(Copy, Clone, Eq, PartialEq)]
struct Step(char);

impl Ord for Step {
    fn cmp(&self, other: &Step) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.0.cmp(&self.0)
    }
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Step) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Step {
    fn to_secs(&self, offset: u32) -> u32 {
	(self.0 as u8) as u32 - 64 + offset
    }
}

fn parse_input(input: &str) -> Vec<Pair> {
    let re = Regex::new(r"Step (.) must be finished before step (.) can begin.").unwrap();
    input.lines().map(|line| {
	let cap = re.captures(line).unwrap();
	Pair(cap[1].chars().nth(0).unwrap(), cap[2].chars().nth(0).unwrap())
    }).collect::<Vec<_>>()
}

fn part1(pairs: &Vec<Pair>) -> String {
    let mut dep_graph = HashMap::new();
    for pair in pairs {
	let entry = dep_graph.entry(pair.1).or_insert(HashSet::new());
	entry.insert(pair.0);
    }

    let dep_free = pairs.iter().map(|pair| pair.0)
	                .chain(pairs.iter().map(|pair| pair.1))
			.filter(|step| !dep_graph.contains_key(step))
		    	.collect::<HashSet<char>>();

    let mut heap = BinaryHeap::new();

    for step in dep_free {
	heap.push(Step(step));
    }

    let mut accum = String::new();
    while let Some(s) = heap.pop() {
	accum.push(s.0);
	let dep_free = dep_graph.iter_mut().filter_map(|(&step, deps)| {
	    deps.remove(&s.0);
	    if deps.is_empty() {
		Some(step)
	    } else {
		None
	    }
	}).collect::<Vec<char>>();

	for step in dep_free {
	    dep_graph.remove(&step);
	    heap.push(Step(step));
	}
    }
    accum
}

fn part2(pairs: &Vec<Pair>, worker_count: usize, offset: u32) -> u32 {
    let mut dep_graph = HashMap::new();
    for pair in pairs {
	let entry = dep_graph.entry(pair.1).or_insert(HashSet::new());
	entry.insert(pair.0);
    }

    let dep_free = pairs.iter().map(|pair| pair.0)
	                .chain(pairs.iter().map(|pair| pair.1))
			.filter(|step| !dep_graph.contains_key(step))
		    	.collect::<HashSet<char>>();

    let mut heap = BinaryHeap::new();

    for step in dep_free {
	heap.push(Step(step));
    }

    let mut cycles = 0;
    let mut workers = Vec::new();
    loop {
	while workers.len() < worker_count {
	    if let Some(s) = heap.pop() {
		workers.push((s.to_secs(offset), s.0));
	    } else {
		break;
	    }
	}
	if workers.is_empty() {
	    break;
	}
	let ticks = workers.iter().min().unwrap().0;
	cycles += ticks;
	for (_, s) in workers.drain_filter(|(c, _)| {
	    *c -= ticks;
	    *c == 0
	}) {
	    let dep_free = dep_graph.iter_mut().filter_map(|(&step, deps)| {
		deps.remove(&s);
		if deps.is_empty() {
		    Some(step)
		} else {
		    None
		}
	    }).collect::<Vec<char>>();

	    for step in dep_free {
		dep_graph.remove(&step);
		heap.push(Step(step));
	    }
	}
    }
    cycles
}

fn main() {
    let input = include_str!("input.txt");
    let pairs = parse_input(&input);

    println!("part1: {}", part1(&pairs));
    println!("part1: {}", part2(&pairs, 5, 60));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
	let input = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";
	let pairs = parse_input(input);
	assert_eq!(part1(&pairs).as_str(), "CABDFE");
    }

    #[test]
    fn part2_test() {
	let input = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";
	let pairs = parse_input(input);
	assert_eq!(part2(&pairs, 2, 0), 15);
    }
}
