use std::cmp;

fn is_num(char: char) -> bool {
	char >= '0' && char <= '9'
}

fn adjacent_symbol(schematics: &Vec<&[u8]>, (i, j): (usize, usize)) -> Option<(usize, usize)> {
	for ix in cmp::max(0, i as i32 - 1)..=cmp::min(schematics.len() as i32 - 1, i as i32 + 1) {
		for jx in cmp::max(0, j as i32 - 1)..=cmp::min(schematics[i].len() as i32 - 1, j as i32 + 1)
		{
			let cell = schematics[ix as usize][jx as usize] as char;

			if cell != '.' && !is_num(cell) {
				return Some((ix as usize, jx as usize));
			}
		}
	}

	None
}

pub fn day3a(input: &str) -> i32 {
	let schematics: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();
	let mut sum = 0;
	let mut num = String::new();
	let mut adjacent = false;

	for (i, row) in schematics.iter().enumerate() {
		for (j, cell) in row.iter().enumerate() {
			let char = *cell as char;

			if is_num(char) {
				num.push(char);

				if !adjacent {
					adjacent = adjacent_symbol(&schematics, (i, j)).is_some();
				}
			} else if !num.is_empty() {
				if adjacent {
					sum += num.parse::<i32>().unwrap();
				}

				num.clear();
				adjacent = false;
			}
		}

		if !num.is_empty() {
			if adjacent {
				sum += num.parse::<i32>().unwrap();
			}

			num.clear();
			adjacent = false;
		}
	}

	sum
}

pub fn day3b(input: &str) -> i32 {
	let schematics: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();
	let mut sum = 0;
	let mut nums = vec![];
	let mut num = String::new();
	let mut adjacent = None;

	for (i, row) in schematics.iter().enumerate() {
		for (j, cell) in row.iter().enumerate() {
			let char = *cell as char;

			if is_num(char) {
				num.push(char);

				if adjacent.is_none() {
					adjacent = match adjacent_symbol(&schematics, (i, j)) {
						Some((i, j)) => {
							if schematics[i][j] as char == '*' {
								Some((i, j))
							} else {
								None
							}
						}
						_ => None,
					};
				}
			} else if !num.is_empty() {
				if let Some(coords) = adjacent {
					nums.push((coords, num.parse::<i32>().unwrap()));
				}

				num.clear();
				adjacent = None;
			}
		}

		if !num.is_empty() {
			if let Some(coords) = adjacent {
				nums.push((coords, num.parse::<i32>().unwrap()));
			}

			num.clear();
			adjacent = None;
		}
	}

	for i in 0..nums.len() {
		for j in (i + 1)..nums.len() {
			if nums[i].0 == nums[j].0 {
				sum += nums[i].1 * nums[j].1
			}
		}
	}

	sum
}

mod tests {
	#[tokio::test]
	async fn it_should_work() {
		let input = crate::input::read_input(3).await;

		assert_eq!(super::day3a(&input), 556_367);
		assert_eq!(super::day3b(&input), 89_471_771);
	}
}
