#[macro_use]
extern crate failure;
#[macro_use]
extern crate nom;

use failure::Error;
use nom::types::CompleteStr as Input;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Claim {
    claim_id : usize,
    from_left : usize,
    from_top : usize,
    width : usize,
    height : usize,
}

impl FromStr for Claim {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
	named!(integer(Input) -> usize, map_res!(nom::digit, |d: Input| d.parse()));

	named!(claim_parser(Input) -> Claim,
	  do_parse!(
	                               tag!("#") >>
	    claim_id:  ws!(integer) >> tag!("@") >>
	    from_left: ws!(integer) >> tag!(",") >>
	    from_top:  ws!(integer) >> tag!(":") >>
	    width:     ws!(integer) >> tag!("x") >>
	    height:    ws!(integer) >>
	    (Claim { claim_id, from_left, from_top, width, height })
	  )
	);

	claim_parser(Input(s)).map(|(_s, c)| c).map_err(|e| format_err!("{}", e))
    }
}

fn parse_input(input: &str) -> Result<Vec<Claim>, Error> {
    input.split('\n')
	 .filter(|line| !line.is_empty())
	 .map(str::parse::<Claim>)
 	 .collect::<Result<Vec<_>, Error>>()
}

fn part1(claims: &Vec<Claim>) -> usize {
    let mut grid = [[0u8; 1000]; 1000];
    for claim in claims {
	for x in 0..claim.width {
	    for y in 0..claim.height {
		grid[claim.from_left + x][claim.from_top + y] += 1;
	    }
	}
    }
    (0..1000).fold(0, |accum1, x| {
	accum1 + (0..1000).fold(0, |accum2, y| {
	    if grid[x][y] > 1 {
		accum2 + 1
	    } else {
		accum2
	    }
	})
    })
}

fn part2(claims: &Vec<Claim>) -> Option<usize> {
    let mut grid = [[0u8; 1000]; 1000];
    for claim in claims {
	for x in 0..claim.width {
	    for y in 0..claim.height {
		grid[claim.from_left + x][claim.from_top + y] += 1;
	    }
	}
    }
    claims.iter().find_map(|claim| {
	let mut xs = (0..claim.width).map(|w| w + claim.from_left);
	let ys = (0..claim.height).map(|h| h + claim.from_top);
	if xs.all(|x| { ys.clone().all(|y| grid[x][y] == 1)}) {
	    Some(claim.claim_id)
	} else {
	    None
	}
    })
}


fn main() {
    let input = include_str!("input.txt");
    let claims = parse_input(input).unwrap();
    println!("part1: {}", part1(&claims));
    println!("part2: {:?}", part2(&claims));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
	let input = "#1 @ 1,3: 4x4 //\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2";
        assert_eq!(
	    vec![
		Claim{claim_id: 1, from_left: 1, from_top: 3, width: 4, height: 4},
		Claim{claim_id: 2, from_left: 3, from_top: 1, width: 4, height: 4},
		Claim{claim_id: 3, from_left: 5, from_top: 5, width: 2, height: 2},
	    ],
	    parse_input(&input).unwrap()
	);
    }

    #[test]
    fn part1_test() {
	let input = "#1 @ 1,3: 4x4 //\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2";
	let claims = parse_input(&input).unwrap();
	assert_eq!(part1(&claims), 4);
    }

    #[test]
    fn part2_test() {
	let input = "#1 @ 1,3: 4x4 //\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2";
	let claims = parse_input(&input).unwrap();
	assert_eq!(part2(&claims), Some(3));
    }
}
