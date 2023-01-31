use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use std::sync::RwLock;
use std::time::Instant;

use minify_selectors_utils::*;
use rayon::prelude::*;
use walkdir::WalkDir;




fn main() {
	std::process::exit(match minify_selectors() {
		Ok(_) => 0,
		Err(error) => {
			eprintln!("minify-selectors has encounted an error: {error:?}");
			1
		},
	});
}

fn minify_selectors() -> Result<(), Box<dyn Error>> {
	let stopwatch = Instant::now();
	let mut config = Config::new();
	let mut selectors = Selectors::new();

	// Multi-step process (stage 1/3):
	// Read files and note down selectors and their occurrences.
	process_files(&mut selectors, &config)?;

	// Multi-step process (stage 2/3):
	// Process selectors list and encode into a minified identifier.
	config.current_step = ProcessingSteps::EncodingSelectors;
	if !config.disable_sort {
		selectors.sort_by_frequency();
	}
	selectors.process(&mut config);

	// Multi-step process (stage 3/3):
	// Open files again and subsituite encoded selectors in place.
	config.current_step = ProcessingSteps::WritingToFiles;
	process_files(&mut selectors, &config)?;

	println!("minify-selectors finished in: {:.2?}", stopwatch.elapsed());

	Ok(())
}

///
fn process_files(
	selectors: &mut Selectors,
	config: &Config,
) -> Result<(), std::io::Error> {
	if config.parallel {
		let selectors_lock = Arc::new(RwLock::new(selectors));
		let files = WalkDir::new(&config.source)
			.into_iter()
			.filter_map(|e| e.ok())
			.filter(is_processable)
			.collect::<Vec<walkdir::DirEntry>>();

		files.into_par_iter().for_each(|entry| {
			handle_par_file(&selectors_lock, config, &entry)
				.expect("There was an issue processing this file.");
		});
	} else {
		for entry in WalkDir::new(&config.source)
			.into_iter()
			.filter_map(|e| e.ok())
			.filter(is_processable)
		{
			handle_file(selectors, config, &entry)?;
		}
	}

	fn is_processable(item: &walkdir::DirEntry) -> bool {
		// Check that current path is a file
		if !item.path().is_file() {
			return false;
		};
		// Finally, check file has a extension that can be processed
		matches!(
			item.path().extension().and_then(OsStr::to_str),
			Some("css") | Some("html") | Some("js") | Some("svg")
		)
	}

	fn handle_file(
		selectors: &mut Selectors,
		config: &Config,
		file: &walkdir::DirEntry,
	) -> Result<(), std::io::Error> {
		if config.current_step == ProcessingSteps::ReadingFromFiles {
			analyse_file(file.path(), selectors, config)?;
		} else {
			write_to_file(file.path(), selectors, config)?;
		}
		Ok(())
	}

	fn handle_par_file(
		selectors: &RwLock<&mut Selectors>,
		config: &Config,
		file: &walkdir::DirEntry,
	) -> Result<(), std::io::Error> {
		if config.current_step == ProcessingSteps::ReadingFromFiles {
			let mut selectors_in_file = Selectors::new();
			analyse_file(file.path(), &mut selectors_in_file, config)?;
			selectors.write().unwrap().merge(selectors_in_file);
		} else {
			write_to_file(file.path(), &selectors.read().unwrap(), config)?;
		}
		Ok(())
	}

	Ok(())
}

fn analyse_file(
	file_path: &Path,
	selectors: &mut Selectors,
	config: &Config,
) -> Result<(), std::io::Error> {
	let mut file_contents = fs::read_to_string(file_path)?;
	println!("Reading file: {}", file_path.display());

	match file_path.extension().and_then(OsStr::to_str) {
		Some("css") => parse_selectors::read_from_css(&mut file_contents, selectors, config),
		Some("html") | Some("svg") => {
			parse_selectors::read_from_html(&mut file_contents, selectors, config)
		},
		Some("js") => parse_selectors::read_from_js(&mut file_contents, selectors, config),
		_ => (),
	}
	Ok(())
}

fn write_to_file(
	file_path: &Path,
	selectors: &Selectors,
	config: &Config,
) -> Result<(), std::io::Error> {
	let mut file_contents = fs::read_to_string(file_path)?;
	println!("Processing file: {}", file_path.display());

	match file_path.extension().and_then(OsStr::to_str) {
		Some("css") => parse_selectors::write_to_css(&mut file_contents, selectors, config),
		Some("html") | Some("svg") => {
			parse_selectors::write_to_html(&mut file_contents, selectors, config)
		},
		Some("js") => parse_selectors::write_to_js(&mut file_contents, selectors, config),
		_ => (),
	}

	let output_path = match &config.source.is_dir() {
		// Remove given source directory to make each
		// matched file relative to the output directory.
		true => {
			config
				.output
				.join(file_path.strip_prefix(&config.source).unwrap())
		},
		// Or if input path was to a file, append only
		// the file name to the given output directory
		false => config.output.join(file_path.file_name().unwrap()),
	};

	// Making sure output directory exists or is
	// created before writing file.
	if let Some(dir_only) = &output_path.parent() {
		fs::create_dir_all(dir_only)?;
	};

	fs::write(output_path, file_contents)?;

	Ok(())
}
