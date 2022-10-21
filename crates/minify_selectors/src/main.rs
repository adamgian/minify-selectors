extern crate globwalk;

use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::time::Instant;

use minify_selectors_utils::*;




fn main() {
	std::process::exit(match minify_selectors() {
		Ok(_) => 0,
		Err(error) => {
			eprintln!("minify-selectors has encounted an error: {:?}", error);
			1
		},
	});
}

fn minify_selectors() -> Result<(), Box<dyn Error>> {
	let stopwatch = Instant::now();
	let config = Config::new();
	let mut selectors = Selectors::new(config.start_index);

	let mut source_dir = PathBuf::from(&config.source);
	let mut source_glob = config.source.to_owned();
	let output_dir = config.output.clone();

	// If glob string is for a directory, append
	// glob pattern to search for CSS, HTML, JS and SVG files.
	if source_dir.is_dir() {
		if source_glob.ends_with('/') {
			source_glob.push_str("**/*.{css,html,js,svg}");
		} else {
			source_glob.push_str("/**/*.{css,html,js,svg}");
		}
	} else if source_glob.ends_with("/*") {
		source_dir = source_dir.parent().unwrap().to_path_buf();
		source_glob.push_str(".{css,html,js,svg}");
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

	for entry in globwalk::glob(&source_glob)? {
		if let Ok(entry) = entry {
			let source_path = Path::new(entry.path());

			// Making sure glob matched path is indeed a file to proceed.
			if source_path.is_file() {
				let output_path = match source_dir.is_dir() {
					// Remove given source directory to make each
					// matched file relative to the output directory.
					true => output_dir.join(source_path.strip_prefix(&source_dir).unwrap()),
					// Or if input path was to a file, append only
					// the file name to the given output directory
					false => output_dir.join(source_path.file_name().unwrap()),
				};

				// Making sure directories exists or are created
				// before writing file.
				if let Some(dir_only) = &output_path.parent() {
					fs::create_dir_all(dir_only)?;
				};

				fs::write(
					output_path,
					process_file(source_path, &mut selectors, &config)?,
				)?;
			}
		}
	}

	println!("minify-selectors finished in: {:.2?}", stopwatch.elapsed());

	Ok(())
}

fn process_file(
	file_path: &Path,
	selectors: &mut Selectors,
	config: &Config,
) -> Result<String, std::io::Error> {
	let file_extension = file_path.extension().and_then(OsStr::to_str).unwrap();
	let mut file_contents = fs::read_to_string(file_path)?;

	println!("Processing file: {}", file_path.display());

	match file_extension {
		"css" => parse_selectors::from_css(&mut file_contents, selectors, config),
		"html" | "svg" => parse_selectors::from_html(&mut file_contents, selectors, config),
		"js" => parse_selectors::from_js(&mut file_contents, selectors, config),
		_ => {},
	}

	Ok(file_contents)
}
