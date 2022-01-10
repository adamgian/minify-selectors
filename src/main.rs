extern crate globwalk;

use regex::Regex;
use std::env;
use std::fs;
use structopt::StructOpt;


#[derive(StructOpt)]
struct Cli {
	#[structopt(short = "i", long = "input")]
	source: String,
}


fn main() {
	let args = Cli::from_args();
	let is_glob_a_directory = !Regex::new(r"\.\{?(\w{2,4},?)+\}?$")
		.unwrap()
		.is_match(&args.source);
	let mut glob_string = String::from(&args.source);

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
			Ok(file) => println!("{:?}", file.path()),
			Err(e) => println!("{:?}", e),
		}
	}
}
