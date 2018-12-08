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

    println!("dep_graph: {:?}", dep_graph);

    let dep_free = pairs.iter().map(|pair| pair.0)
	                .chain(pairs.iter().map(|pair| pair.1))
			.filter(|step| !dep_graph.contains_key(step))
		    	.collect::<HashSet<char>>();

    println!("deps_free: {:?}", dep_free);

    let mut heap = BinaryHeap::new();

    for step in dep_free {
	heap.push(Step(step));
    }

    let mut accum = String::new();
    loop {
	if let Some(s) = heap.pop() {
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
	} else {
	    break;
	}
    }
    accum
}

fn main() {
    let input = include_str!("input.txt");
    let pairs = parse_input(&input);

    println!("part1: {}", part1(&pairs));
}
