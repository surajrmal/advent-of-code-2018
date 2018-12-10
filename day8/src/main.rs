fn parse_input(input: &str) -> Vec<u32> {
    input.trim().split(" ")
	 .map(str::parse::<u32>)
	 .collect::<Result<Vec<u32>, std::num::ParseIntError>>()
	 .unwrap()
}

fn part1(input: &[u32]) -> (u32, usize)  {
    assert!(input.len() >= 2);
    let child_nodes = input[0];
    let metadata_entries = input[1] as usize;
    let (child_checksum, first) = (0..child_nodes).fold((0, 2), |(checksum, first), _i| {
	let last = input.len() - metadata_entries;
	let (child_checksum, child_size) = part1(&input[first..last]);
	(checksum + child_checksum,  first + child_size)
    });
    assert!(input.len() >= first + metadata_entries);
    let checksum: u32 = input.iter().skip(first).take(metadata_entries).sum();
    (checksum + child_checksum, first + metadata_entries)
}

fn part2(input: &[u32]) -> (u32, usize)  {
    assert!(input.len() >= 2);
    let child_nodes = input[0];
    let metadata_entries = input[1] as usize;
    // Return metadata as value if no children.
    if child_nodes == 0 {
       let value = input.iter().skip(2).take(metadata_entries).sum();
       return (value, 2 + metadata_entries);
    }

    // Otherwwise compute it's children's values.
    let mut first = 2;
    let children = std::iter::once(0).chain((0..child_nodes).map(|_| {
	let last = input.len() - metadata_entries;
	let (child_value, child_size) = part2(&input[first..last]);
	first += child_size;
	child_value
    })).collect::<Vec<_>>();

    // And then compute it's own values.
    assert!(input.len() >= first + metadata_entries);
    let value = input.iter().skip(first).take(metadata_entries).fold(0, |sum, entry| {
	sum + children.get(*entry as usize).unwrap_or(&0)
    });
    (value, first + metadata_entries)
}

fn main() {
    let input = include_str!("input.txt");
    let input = parse_input(input);
    println!("part1: {}", part1(&input).0);
    println!("part2: {}", part2(&input).0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
	let input = parse_input("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2");
        assert_eq!(part1(&input), (138, input.len()));
    }

    #[test]
    fn part2_test() {
	let input = parse_input("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2");
        assert_eq!(part2(&input), (66, input.len()));
    }
}
