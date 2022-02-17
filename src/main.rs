extern crate globwalk;

// use bs62::*;
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




#[derive(StructOpt)]
struct Cli {
	#[structopt(short = "i", long = "input")]
	source: String,
}

lazy_static! {
	static ref SELECTORS_IN_CSS: Regex = Regex::new(
		r"(?x)
			([\#\.])
			(
				(?>[A-Za-z\_]|\-[A-Za-z\_])
				[\w\-\_]*+
			)
			(?=
				\s*+
				[\{\#\.\,\:\>\[\+\~]
			)
		"
	).unwrap();

	// FIXME
	static ref SELECTORS_IN_JS: Regex = Regex::new(
		r##"(?x)
			()
		"##
	).unwrap();

	// FIXME
	static ref SELECTORS_IN_HTML: Regex = Regex::new(
		r##"(?x)
			()
		"##
	).unwrap();

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
	let file_extension = Path::new(file)
		.extension()
		.and_then(OsStr::to_str)
		.unwrap();
	let mut file_contents = fs::read_to_string(file).unwrap();

	println!(
		"Processing {} file: {}",
		file_extension.to_ascii_uppercase(),
		file.display()
	);

	if file_extension == "css" {
		for item in SELECTORS_IN_CSS.captures_iter(&file_contents) {
			if !selectors.contains_key(&item.at(0).unwrap().to_owned()) {
				*index += 1;

				selectors.insert(
					item.at(0).unwrap().to_owned(),
					generate_selector(index)
				);
			}
		}

		for item in selectors {
			println!("{:?}", item);
		}
	}
}


fn generate_selector(position: &u32) -> String {
	const BASE: u32 = 62;
	const OFFSET: u32 = 10;
	const SUBSET: u32 = 52;

	let index: u32 = position - 1;
	let mut assigned_index: u32 = 0;
	let mut exponent: u32 = 0;
	let mut carry: u32 = 0;

	while index >= SUBSET * u32::pow(BASE, exponent) + carry {
		carry += SUBSET * u32::pow(BASE, exponent);
		exponent += 1;
	}

	assigned_index += OFFSET * u32::pow(BASE, exponent) - carry + index;

	return bs62::encode_num(&assigned_index);
}
