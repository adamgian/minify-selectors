extern crate globwalk;

use bs62::*;
use onig::*;
use std::collections::HashMap;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::time::Instant;
// use std::rc::Rc;
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
	// let mut selectors: Rc<HashSet<String>> = Rc::new(HashSet::new());

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
			Ok(file) => process_file(file.path()),
			Err(e) => println!("{:?}", e),
		}
	}

    println!("minify-selectors finished in: {:.2?}", stopwatch.elapsed());
}

fn process_file(file: &Path) {
	let file_extension = Path::new(file)
		.extension()
		.and_then(OsStr::to_str)
		.unwrap();
	let mut file_contents = fs::read_to_string(file).unwrap();
	let selector = Regex::new(r"(?<=\#|\.)(?>[A-Za-z\_]{1}|\-[A-Za-z\_]{2})[\w\-\_]*(?=\s*[\{\#\.\,\:\>\[\+\~])").unwrap();
	let mut selectors: HashMap<&str, &str> = HashMap::new();

	println!(
		"Processing {} file: {}",
		file_extension.to_ascii_uppercase(),
		file.display()
	);

	if file_extension == "css" {
		for item in selector.captures_iter(&file_contents) {
			if !selectors.contains_key(item.at(0).unwrap()) {
				selectors.insert(
					item.at(0).unwrap(),
					"todo"
				);
			}
		}

		for item in selectors {
			println!("{:?}", item);
		}
	}
}
