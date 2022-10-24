use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::time::Instant;

use minify_selectors_utils::*;
use rayon::prelude::*;
use walkdir::WalkDir;




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

	// TODO: setup rayon
	// let entries: Vec<PathBuf> = WalkDir::new(&config.source)
	// 	.into_iter()
	// 	.filter_map(|e| e.ok())
	// 	.filter(|e| is_processable(e))
	// 	.map(|e| e.path().to_owned())
	// 	.collect();

	// entries.par_iter().for_each(|entry| {
	// 	println!("entries: {}", entry.display());
	// });

	for entry in WalkDir::new(&config.source)
		.into_iter()
		.filter_map(|e| e.ok())
		.filter(|e| is_processable(e))
	{
		let output_path = match &config.source.is_dir() {
			// Remove given source directory to make each
			// matched file relative to the output directory.
			true => {
				config
					.output
					.join(entry.path().strip_prefix(&config.source).unwrap())
			},
			// Or if input path was to a file, append only
			// the file name to the given output directory
			false => config.output.join(entry.path().file_name().unwrap()),
		};

		// Making sure output directory exists or is
		// created before writing file.
		if let Some(dir_only) = &output_path.parent() {
			fs::create_dir_all(dir_only)?;
		};

		fs::write(
			output_path,
			process_file(entry.path(), &mut selectors, &config)?,
		)?;
	}

	println!("minify-selectors finished in: {:.2?}", stopwatch.elapsed());

	Ok(())
}

fn process_file(
	file_path: &Path,
	selectors: &mut Selectors,
	config: &Config,
) -> Result<String, std::io::Error> {
	let mut file_contents = fs::read_to_string(file_path)?;
	println!("Processing file: {}", file_path.display());

	match file_path.extension().and_then(OsStr::to_str) {
		Some("css") => parse_selectors::from_css(&mut file_contents, selectors, config),
		Some("html") | Some("svg") => {
			parse_selectors::from_html(&mut file_contents, selectors, config)
		},
		Some("js") => parse_selectors::from_js(&mut file_contents, selectors, config),
		_ => (),
	}

	Ok(file_contents)
}

fn is_processable(entry: &walkdir::DirEntry) -> bool {
	// Check that current path is a file
	if !entry.path().is_file() {
		return false;
	};

	// Finally, check file has a extension that can be processed
	matches!(
		entry.path().extension().and_then(OsStr::to_str),
		Some("css") | Some("html") | Some("js") | Some("svg")
	)
}
