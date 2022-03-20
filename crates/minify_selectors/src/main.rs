extern crate globwalk;

use lazy_static::lazy_static;
use onig::*;
use std::{
	collections::HashMap,
	ffi::OsStr,
	fs,
	path::Path,
	time::Instant,
};
use structopt::StructOpt;

use parse_selectors;




#[derive(StructOpt)]
struct Cli {
	#[structopt(short = "i", long = "input")]
	source: String,
}




lazy_static! {
	static ref IS_GLOB_A_DIRECTORY: Regex = Regex::new(
		r"\.\{?(\w{2,4},?)+\}?$"
	).unwrap();
}




fn main() {
	let stopwatch = Instant::now();
	let args = Cli::from_args();

	let mut glob_string = String::from(&args.source);
	// Set of selectors with its assigned base62 name
	let mut selectors: HashMap<String, String> = HashMap::new();
	// Counter of unique selectors
	let mut selector_counter: u32 = 0;


	// If glob string is for a directory
	if !IS_GLOB_A_DIRECTORY.is_match(&glob_string) {
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
			Ok(file) => process_file(
				file.path(),
				&mut selectors,
				&mut selector_counter
			),
			Err(e) => println!("{:?}", e),
		}
	}

	println!(
		"minify-selectors finished in: {:.2?}",
		stopwatch.elapsed()
	);
}

fn process_file(
	file: &Path,
	selectors: &mut HashMap<String, String>,
	index: &mut u32
) {
	let file_extension = Path::new(file).extension().and_then(OsStr::to_str);
	let mut file_contents = fs::read_to_string(file).unwrap();

	println!(
		"Processing {} file: {}",
		file_extension.unwrap().to_ascii_uppercase(),
		file.display()
	);

	match file_extension {
		Some("css") => {
			file_contents = parse_selectors::from_css(
				&mut file_contents,
				selectors,
				index
			);
		},
		Some("html") => {
			// file_contents = parse_selectors::from_html(
			// 	&mut file_contents,
			// 	selectors,
			// 	index
			// );
		},
		Some("js") => {
			// file_contents = parse_selectors::from_html(
			// 	&mut file_contents,
			// 	selectors,
			// 	index
			// );
		},
		_ => {},
	}

	// println!("{}", file_contents);
	// println!("");
}