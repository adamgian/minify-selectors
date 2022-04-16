extern crate globwalk;

use clap::Parser;
use std::{
	collections::HashMap,
	ffi::OsStr,
	fs,
	path::Path,
	path::PathBuf,
	time::Instant,
};

use parse_selectors;



/// Post-processor that minifies classes and IDs in CSS, HTML and JS files.
#[derive(Parser, Debug)]
struct Cli {
	/// Directory or file to process
	#[clap(short = 'i', long = "input")]
	source: String,

	/// Output directory to save file(s)
	#[clap(short = 'o', long = "output")]
	output: String,
}




fn main() {
	let stopwatch = Instant::now();

	// Set of selectors with its assigned base62 name
	let mut selectors: HashMap<String, String> = HashMap::new();
	// Counter of unique selectors
	let mut selector_counter: u16 = 0;

	let args = Cli::parse();
	let mut source_dir = PathBuf::from(&args.source);
	let mut source_glob = String::from(&args.source);
	let output_dir = String::from(&args.output);

	// If glob string is for a directory, append
	// glob pattern to search for CSS, HTML and JS files.
	if source_dir.is_dir() {
		if source_glob.ends_with("/") {
			source_glob.push_str("**/*.{css,html,js}");
		}
		else {
			source_glob.push_str("/**/*.{css,html,js}");
		}
	}
	else if source_glob.ends_with("/*") {
		source_dir = source_dir.parent().unwrap().to_path_buf();
		source_glob.push_str(".{css,html,js}");
	}

	// globwalk doesn't handle relative globs starting with "./".
	// https://github.com/Gilnaa/globwalk/issues/28
	if source_glob.starts_with("./") {
		source_glob = source_glob.strip_prefix("./").unwrap().to_string();
	}

	// Force all relative paths to start with "./"
	if source_dir.is_relative() & !source_dir.starts_with("./") {
		source_dir = Path::new("./").join(source_dir);
	}

	for entry in globwalk::glob(&source_glob).unwrap() {
		match entry {
			Ok(file) => {
				let file_path = Path::new(file.path());
				let mut output_file = PathBuf::new();

				// Remove given source directory to make each
				// match file relative to the output directory.
				if source_dir.is_dir() {
					output_file = PathBuf::from(&output_dir)
						.join(
							file_path
								.strip_prefix(&source_dir)
								.unwrap()
						);
				}
				// Or if input path was to a file, append only
				// the file name to the given output directory
				else {
					output_file = PathBuf::from(&output_dir)
						.join(
							file_path
								.file_name()
								.unwrap()
						);
				}

				// Making sure directories exists or are created
				// before writing file.
				if let Some(dir_only) = output_file.parent() {
					fs::create_dir_all(dir_only);
				};

				fs::write(
					output_file,
					process_file(
						file_path,
						&mut selectors,
						&mut selector_counter
					)
				);
			},

			Err(error) => println!("{:?}", error),
		}
	}

	println!(
		"minify-selectors finished in: {:.2?}",
		stopwatch.elapsed()
	);
}

fn process_file(
	file_path: &Path,
	selectors: &mut HashMap<String, String>,
	index: &mut u16
) -> String {
	let file_extension = file_path.extension().and_then(OsStr::to_str).unwrap();
	let mut file_contents = fs::read_to_string(file_path).unwrap();

	println!(
		"Processing file: {}",
		file_path.display()
	);

	match file_extension {
		"css" => {
			file_contents = parse_selectors::from_css(
				&mut file_contents,
				selectors,
				index
			);
		},
		"html" => {
			file_contents = parse_selectors::from_html(
				&mut file_contents,
				selectors,
				index
			);
		},
		"js" => {
			file_contents = parse_selectors::from_js(
				&mut file_contents,
				selectors,
				index
			);
		},
		_ => {}
	}

	return file_contents;
}
