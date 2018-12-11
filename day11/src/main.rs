fn part1(serial: isize) -> (usize, usize) {
    // Compute grid.
    let mut grid = vec![vec![0; 300]; 300];

    for x in 0..300 {
	for y in 0..300 {
	    let rack_id : isize = x + 11;
	    let power_level = ((rack_id * (y + 1)) + serial) * rack_id;
	    let power_level = (power_level / 100) % 10;
	    grid[x as usize][y as usize] = power_level - 5;
	}
    }

    // Compute value for each point in the grid.
    let mut power_max = 0;
    let mut x_max = 0;
    let mut y_max = 0;
    for x in 0..297 {
	for y in 0..297 {
	   let power  = (0..3).fold(0, |accum, x2|  {
		accum + (0..3).fold(0, |accum2, y2| {
		    accum2 + grid[x + x2][y + y2]
		})
	   });
	   if power > power_max {
	       power_max = power;
	       x_max = x;
	       y_max = y;
	   }
	}
    }
    (x_max + 1, y_max + 1)
}

fn part2(serial: isize) -> (usize, usize, usize) {
    // Compute grid.
    let mut grid = vec![vec![0; 300]; 300];

    for x in 0..300 {
	for y in 0..300 {
	    let rack_id : isize = x + 11;
	    let power_level = ((rack_id * (y + 1)) + serial) * rack_id;
	    let power_level = (power_level / 100) % 10;
	    grid[x as usize][y as usize] = power_level - 5;
	}
    }

    // Compute value for each point in the grid.
    let mut power_max = 0;
    let mut x_max = 0;
    let mut y_max = 0;
    let mut size_max = 0;
    for size in 1..=300 {
	for x in 0..(300 - size) {
	    for y in 0..(300 - size) {
	       let power  = (0..size).fold(0, |accum, x2|  {
		    accum + (0..size).fold(0, |accum2, y2| {
			accum2 + grid[x + x2][y + y2]
		    })
	       });
	       if power > power_max {
		   eprintln!("power_max: {} size: {}, x: {}, y: {}", power_max, size, x, y);
		   power_max = power;
		   x_max = x;
		   y_max = y;
		   size_max = size;
	       }
	    }
	}
    }
    (x_max + 1, y_max + 1, size_max)
}

fn main() {
    println!("part1: {:?}", part1(9435));
    println!("part2: {:?}", part2(9435));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
	assert_eq!(part1(18), (33, 45));
	assert_eq!(part1(42), (21, 61));
    }

    #[test]
    fn part2_test() {
	assert_eq!(part2(18), (90, 269, 16));
	assert_eq!(part2(42), (232, 251, 12));
    }
}
