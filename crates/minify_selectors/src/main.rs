extern crate globwalk;

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

	// #[structopt(short = "o", long = "output")]
	// output: String,
}




fn main() {
	let stopwatch = Instant::now();
	let args = Cli::from_args();

	let mut input_glob = String::from(&args.source);
	// let mut output_glob = String::from(&args.output);
	// Set of selectors with its assigned base62 name
	let mut selectors: HashMap<String, String> = HashMap::new();
	// Counter of unique selectors
	let mut selector_counter: u32 = 0;


	// If glob string is for a directory,
	// append
	if is_dir(&input_glob) {
		if input_glob.ends_with("/") {
			input_glob.push_str("**/*.{css,html,js}");
		}
		else if input_glob.ends_with("/*") {
			input_glob.push_str(".{css,html,js}");
		}
		else {
			input_glob.push_str("/**/*.{css,html,js}");
		}
	}

	for entry in globwalk::glob(&input_glob).unwrap() {
		match entry {
			Ok(file) => {
				println!(
					"{}",
					process_file(
						file.path(),
						&mut selectors,
						&mut selector_counter
					)
				);
				println!("");
				// TODO:
				// fs::create_dir_all("examples/dist/");

				// fs::write(
				// 	format!(
				// 		"{path}{file_name}",
				// 		path = "examples/dist/",
				// 		file_name = file.file_name().and_then(OsStr::to_str).unwrap()
				// 	),
				// 	file_contents
				// );
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
	index: &mut u32
) -> String {
	let file = Path::new(file_path);
	let file_extension = file.extension().and_then(OsStr::to_str);
	let mut file_contents = fs::read_to_string(file_path).unwrap();

	match file_extension {
		Some("css") => {
			println!(
				"Processing CSS file: {}",
				file_path.display()
			);
			file_contents = parse_selectors::from_css(
				&mut file_contents,
				selectors,
				index
			);
		},
		Some("html") => {
			println!(
				"Processing HTML file: {}",
				file_path.display()
			);
			file_contents = parse_selectors::from_html(
				&mut file_contents,
				selectors,
				index
			);
		},
		Some("js") => {
			println!(
				"Processing JS file: {}",
				file_path.display()
			);
			// file_contents = parse_selectors::from_html(
			// 	&mut file_contents,
			// 	selectors,
			// 	index
			// );
		},
		_ => {},
	}

	return file_contents;
}

fn is_dir(glob: &String) -> bool {
	return fs::metadata(glob)
		.unwrap()
		.is_dir();
}