use std::{env, fs, io::Write};

use reqwest::header::COOKIE;

/// Tries to read input given the day from the cache.
/// Otherwise loads it from the web (and cache it).
pub async fn read_input(day: u8) -> String {
	let cache_filename = format!(".input_cache/day{}.txt", day);

	println!("Reading day{} from cache...", day);

	// Cache...
	match fs::read_to_string(&cache_filename) {
		Ok(input) => return input,
		Err(err) => println!("Skipped err: {}", err),
	}

	println!("Reading day{} from web...", day);

	// Web...
	let client = reqwest::Client::new();
	let response = client
		.get(format!("https://adventofcode.com/2023/day/{}/input", day))
		.header(COOKIE, format!("session={}", env::var("SESSION").unwrap()))
		.send()
		.await
		.unwrap();

	let input = response.text().await.unwrap();

	println!("Caching day{} input...", day);

	match fs::create_dir(".input_cache") {
		Err(err) => println!("Skipped err: {}", err),
		_ => {}
	}

	let mut file = fs::File::create(&cache_filename).unwrap();

	file.write_all(input.as_bytes()).unwrap();

	return input;
}
