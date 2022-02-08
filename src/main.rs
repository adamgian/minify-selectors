extern crate globwalk;

use bs62::*;
use onig::*;
use std::collections::HashMap;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::time::Instant;
use structopt::StructOpt;




#[derive(StructOpt)]
struct Cli {
	#[structopt(short = "i", long = "input")]
	source: String,
}


fn main() {
	let stopwatch = Instant::now();
	let args = Cli::from_args();
	let is_glob_a_directory = !Regex::new(r"\.\{?(\w{2,4},?)+\}?$")
		.unwrap()
		.is_match(&args.source);
	let mut glob_string = String::from(&args.source);

	// Counter of unique selectors
	let mut selector_counter: u32 = 0;


	// If glob string is for a directory
	if is_glob_a_directory {
		if glob_string.ends_with("/") {
			glob_string.push_str("**/*.{css,html,js}");
		}
		else if glob_string.ends_with("/*") {
			glob_string.push_str(".{css,html,js}");
		}
		else {
			glob_string.push_str("/**/*.{css,html,js}");
		}
	}

	for entry in globwalk::glob(&glob_string).unwrap() {
		match entry {
			Ok(file) => process_file(file.path(), &mut selector_counter),
			Err(e) => println!("{:?}", e),
		}
	}

	println!(
		"minify-selectors finished in: {:.2?}",
		stopwatch.elapsed()
	);
}

fn process_file(file: &Path, index: &mut u32) {
	let file_extension = Path::new(file)
		.extension()
		.and_then(OsStr::to_str)
		.unwrap();
	let mut file_contents = fs::read_to_string(file).unwrap();
	let selector = Regex::new(r"(?<=\#|\.)(?>[A-Za-z\_]{1}|\-[A-Za-z\_]{2})[\w\-\_]*(?=\s*[\{\#\.\,\:\>\[\+\~])").unwrap();
	let mut selectors: HashMap<&str, String> = HashMap::new();

	println!(
		"Processing {} file: {}",
		file_extension.to_ascii_uppercase(),
		file.display()
	);

	if file_extension == "css" {
		for item in selector.captures_iter(&file_contents) {
			if !selectors.contains_key(item.at(0).unwrap()) {
				// todo: increment counter that avoids generating
				// base62 strings that start with a numeral
				*index += 1;

				selectors.insert(
					item.at(0).unwrap(),
					generate_id(index)
				);

				println!("{:?}", index);
			}
		}

		for item in selectors {
			println!("{:?}", item);
		}
	}
}


fn generate_id(index: &u32) -> String {
	return bs62::encode_num(index);
}
