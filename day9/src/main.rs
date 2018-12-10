use std::collections::VecDeque;

fn part1(player_count: usize, last_marble: usize) -> usize {
    let mut scores = vec![0; player_count];

    let mut circle = VecDeque::with_capacity(last_marble + 1);
    circle.push_back(0);
    for marble in 1..=last_marble {
	if marble % 23 == 0 {
	    for _ in 0..7 {
		let tmp = circle.pop_back().unwrap();
		circle.push_front(tmp);
	    }
	    let points = marble + circle.pop_front().unwrap();
	    scores[(marble - 1) % player_count] += points;
	} else {
	    for _ in 0..2 {
		let tmp = circle.pop_front().unwrap();
		circle.push_back(tmp);
	    }
	    circle.push_front(marble);
	}
    }
    *scores.iter().max().unwrap()
}

fn main() {
    println!("part1: {}", part1(405, 70953));
    println!("part1: {}", part1(405, 7095300));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(part1(9, 25), 32);
        assert_eq!(part1(10, 1618), 8317);
        assert_eq!(part1(13, 7999), 146373);
        assert_eq!(part1(17, 1104), 2764);
        assert_eq!(part1(21, 6111), 54718);
        assert_eq!(part1(30, 5807), 37305);
    }
}
