use std::collections::HashMap;
use std::path::PathBuf;

use clap::Parser;




/// Post-processor that minifies classes and IDs in CSS, HTML, JS and SVG files.
#[derive(Parser, Debug)]
#[clap(
	name = "minify-selectors",
	version,
	long_about = None,
)]
pub struct Cli {
	/// Directory or file to process
	#[clap(short = 'i', long = "input")]
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
	pub alphabet: (Vec<char>, Vec<usize>),
	pub start_index: usize,
	pub current_step: ProcessingSteps,
}

#[derive(Clone, Debug)]
pub enum ProcessingSteps {
	ReadingFromFiles,
	EncodingSelectors,
	WritingToFiles,
}

impl Config {
	pub fn new() -> Self {
		let args = Cli::parse();
		let mut config: Config = Default::default();

		if let Some(alphabet) = args.alphabet {
			config.alphabet = encode_selector::into_alphabet_set(&alphabet);
		}
		if let Some(index) = args.start_index {
			config.start_index = index;
		}

		config.source = PathBuf::from(&args.source);
		config.output = PathBuf::from(&args.output);

		config
	}
}

impl Default for Config {
	fn default() -> Self {
		Self {
			source: PathBuf::from(""),
			output: PathBuf::from(""),
			alphabet: encode_selector::into_alphabet_set(
				"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ",
			),
			start_index: 0,
			current_step: ProcessingSteps::ReadingFromFiles,
		}
	}
}




/// Metadata for selector
#[derive(Clone, Debug)]
pub struct Selector {
	pub r#type: SelectorType,
	pub counter: usize,
	pub selector_string_counter: usize,
	pub identifier_counter: usize,
	pub anchor_counter: usize,
	pub style_counter: usize,
	pub script_counter: usize,
}

#[derive(Clone, Debug)]
pub enum SelectorType {
	Class,
	Id,
	Undefined,
}

#[derive(Clone, Debug)]
pub enum SelectorUsage {
	Identifier,
	Selector,
	Anchor,
	Style,
	Script,
}

impl Selector {
	pub fn new(selector: &str) -> Self {
		Self {
			r#type: match selector.chars().next() {
				Some('.') => SelectorType::Class,
				Some('#') => SelectorType::Id,
				_ => panic!("Missing or unknown selector type"),
			},
			..Default::default()
		}
	}

	pub fn count(
		&mut self,
		usage: SelectorUsage,
	) {
		self.counter += 1;
		match usage {
			SelectorUsage::Identifier => self.identifier_counter += 1,
			SelectorUsage::Selector => self.selector_string_counter += 1,
			SelectorUsage::Anchor => self.anchor_counter += 1,
			SelectorUsage::Style => self.style_counter += 1,
			SelectorUsage::Script => self.script_counter += 1,
		}
	}

	pub fn sum(&mut self, incoming: Selector) {
		self.counter += incoming.counter;
		self.identifier_counter += incoming.identifier_counter;
		self.selector_string_counter += incoming.selector_string_counter;
		self.anchor_counter += incoming.anchor_counter;
		self.style_counter += incoming.style_counter;
		self.script_counter += incoming.script_counter;
	}
}

impl Default for Selector {
	fn default() -> Self {
		Self {
			r#type: SelectorType::Undefined,
			counter: 0,
			identifier_counter: 0,
			selector_string_counter: 0,
			anchor_counter: 0,
			style_counter: 0,
			script_counter: 0,
		}
	}
}




#[derive(Debug)]
pub struct Selectors {
	pub map: HashMap<String, Selector>,
}

impl Selectors {
	pub fn new() -> Self {
		Self {
			map: HashMap::new(),
		}
	}

	pub fn add(
		&mut self,
		selector: String,
		usage: SelectorUsage,
	) {
		// Create map entry if it does not yet exist
		if !self.map.contains_key(&selector) {
			self.map.insert(selector.clone(), Selector::new(&selector));
		}
		self.map.get_mut(&selector).unwrap().count(usage);
	}

	pub fn merge(
		&mut self,
		incoming: Selectors,
	) {
		for (key, val) in incoming.map {
			if self.map.contains_key(&key) {
				self.map.get_mut(&key).unwrap().sum(val);
			} else {
				self.map.insert(key.clone(), val.clone());
			}
		}
	}
}
