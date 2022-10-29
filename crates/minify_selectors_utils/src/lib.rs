use std::collections::HashMap;
use std::path::PathBuf;

use clap::Parser;




/// Post-processor that minifies classes and IDs in CSS, HTML, JS and SVG files.
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
pub struct Cli {
	/// Directory or file to process
	#[clap(short = 'i', long)]
	source: String,

	/// Output directory to save file(s)
	#[clap(short = 'o', long)]
	output: String,

	/// Index to start encoding from
	#[clap(long = "start-index")]
	start_index: Option<usize>,

	/// Sequence of characters to use when encoding
	#[clap(long)]
	alphabet: Option<String>,
}


#[derive(Debug)]
pub struct Config {
	pub source: PathBuf,
	pub output: PathBuf,
	pub alphabet: Vec<char>,
	pub start_index: usize,
}

impl Config {
	pub fn new() -> Self {
		let args = Cli::parse();

		Self {
			source: PathBuf::from(&args.source),
			output: PathBuf::from(&args.output),
			alphabet: encode_selector::into_alphabet_set(match &args.alphabet {
				Some(alphabet) => alphabet,
				None => "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ",
			}),
			start_index: match &args.start_index {
				Some(index) => *index,
				None => 0,
			},
		}
	}
}

impl Default for Config {
	fn default() -> Self {
		Self::new()
	}
}


#[derive(Debug)]
pub struct Selectors {
	pub map: HashMap<String, String>,
	pub class_index: usize,
	pub id_index: usize,
}

impl Selectors {
	pub fn contains(
		&self,
		selector: &str,
	) -> bool {
		self.map.contains_key(selector)
	}

	// Note: assumes that key exists, should check first with contains().
	pub fn get(
		&self,
		selector: &str,
	) -> String {
		self.map.get_key_value(selector).unwrap().1.to_string()
	}

	pub fn new(starting_index: usize) -> Self {
		Self {
			map: HashMap::new(),
			class_index: starting_index,
			id_index: starting_index,
		}
	}

	// Note: assumes that this key is unique,
	// should check first with contains().
	pub fn add(
		&mut self,
		key: String,
		value: String,
	) {
		self.map.insert(key.clone(), value);
		match key.chars().next() {
			Some('.') => self.class_index += 1,
			// Some('#') |
			_ => self.id_index += 1,
		}
	}
}
