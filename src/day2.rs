use std::{cmp, collections::HashMap};

pub fn day2a(input: &str) -> i32 {
	let config = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
	let mut sum = 0;

	'game: for line in input.lines() {
		let parts: Vec<&str> = line.split(":").collect();

		for set in parts[1].split(";") {
			for subset in set.split(",") {
				let subparts: Vec<&str> = subset.trim().split(" ").collect();
				let target = config.get(subparts[1]).unwrap();

				if target < &subparts[0].parse().unwrap() {
					continue 'game;
				}
			}
		}

		sum += parts[0].split(" ").collect::<Vec<&str>>()[1]
			.parse::<i32>()
			.unwrap();
	}

	sum
}

pub fn day2b(input: &str) -> i32 {
	let mut sum = 0;

	for line in input.lines() {
		let parts: Vec<&str> = line.split(":").collect();
		let mut max_config = HashMap::new();

		for set in parts[1].split(";") {
			for subset in set.split(",") {
				let subparts: Vec<&str> = subset.trim().split(" ").collect();
				let n: i32 = subparts[0].parse().unwrap();

				max_config
					.entry(subparts[1])
					.and_modify(|v| *v = cmp::max(*v, n))
					.or_insert(n);
			}
		}

		let mut power = 1;

		for (_, v) in max_config {
			power *= v;
		}

		sum += power;
	}

	sum
}

mod tests {
	#[tokio::test]
	async fn it_should_work() {
		let input = crate::input::read_input(2).await;

		assert_eq!(super::day2a(&input), 2_377);
		assert_eq!(super::day2b(&input), 71_220);
	}
}
