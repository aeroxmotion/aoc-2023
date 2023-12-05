use std::collections::{HashMap, HashSet};

pub fn day4a(input: &str) -> i32 {
	let mut points = 0;

	for line in input.lines() {
		let mut next_points = 0;
		let mut matches = HashSet::new();
		let mut parts = line.split(":");

		parts.next();

		for num in parts.next().unwrap().split(" ") {
			if num.is_empty() || num == "|" {
				continue;
			}

			if matches.contains(num) {
				next_points = 1.max(next_points * 2);
			} else {
				matches.insert(num);
			}
		}

		points += next_points;
	}

	return points;
}

fn parse_card_num(input: &str) -> i32 {
	let mut n = String::new();
	let bytes = input.as_bytes();
	let mut i = input.len() - 1;

	loop {
		let char = bytes[i] as char;

		if char < '0' || char > '9' {
			break;
		}

		n.insert(0, char);
		i -= 1;
	}

	n.parse().unwrap()
}

pub fn day4b(input: &str) -> i32 {
	let mut total = 0;
	let mut copies: HashMap<i32, i32> = HashMap::new();

	for line in input.lines() {
		let mut winning = 0;
		let mut matches = HashSet::new();
		let mut parts = line.split(":");

		let card_n = parse_card_num(parts.next().unwrap().trim());
		let card_copies = copies.get(&card_n).unwrap_or(&0) + 1;

		total += card_copies;

		for num in parts.next().unwrap().split(" ") {
			if num.is_empty() || num == "|" {
				continue;
			}

			if matches.contains(num) {
				winning += 1;
			} else {
				matches.insert(num);
			}
		}

		for i in 1..=winning {
			copies
				.entry(card_n + i)
				.and_modify(|v| *v += card_copies)
				.or_insert(card_copies);
		}
	}

	return total;
}

mod tests {
	#[tokio::test]
	async fn it_should_work() {
		let input = crate::input::read_input(4).await;

		assert_eq!(super::day4a(&input), 22_193);
		assert_eq!(super::day4b(&input), 5_625_994);
	}
}
