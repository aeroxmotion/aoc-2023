use std::collections::HashMap;

pub fn decode_digit(line: &str, i: usize) -> u8 {
	let digits = HashMap::from([
		("one", 1),
		("two", 2),
		("six", 6),
		("four", 4),
		("five", 5),
		("nine", 9),
		("three", 3),
		("seven", 7),
		("eight", 8),
	]);

	for delta in 2..=4 {
		if i >= delta {
			let digit = digits.get(&line[(i - delta)..=i]).unwrap_or(&0);

			if digit != &0 {
				return *digit;
			}
		}
	}

	return 0;
}

pub fn day1(input: &str, decode: bool) -> i32 {
	let mut sum = 0;

	for line in input.lines() {
		let mut first = 0;
		let mut last = 0;

		for (i, byte) in line.as_bytes().iter().enumerate() {
			let digit = if byte >= &('0' as u8) && byte <= &('9' as u8) {
				*byte - '0' as u8
			} else if decode {
				decode_digit(line, i)
			} else {
				0
			};

			if digit != 0 {
				if first == 0 {
					first = digit;
				}

				last = digit;
			}
		}

		sum += first as i32 * 10 + last as i32;
	}

	sum
}

mod tests {
	#[tokio::test]
	async fn it_should_work() {
		let input = crate::input::read_input(1).await;

		assert_eq!(super::day1(&input, false), 56_108);
		assert_eq!(super::day1(&input, true), 55_652);
	}
}
