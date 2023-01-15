use std::path::PathBuf;

use clap::Parser;
use indexmap::IndexMap;




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




#[derive(Clone, Debug)]
pub struct Config {
	pub source: PathBuf,
	pub output: PathBuf,
	pub alphabet: (Vec<char>, Vec<usize>),
	pub start_index: usize,
	pub current_step: ProcessingSteps,
}

#[derive(Clone, Debug, PartialEq)]
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
	pub replacement: Option<String>,
	pub counter: usize,
	pub selector_string_counter: usize,
	pub identifier_counter: usize,
	pub anchor_counter: usize,
	pub style_counter: usize,
	pub script_counter: usize,
	pub prefix_counter: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SelectorType {
	Class,
	Id,
	Undefined,
}

#[derive(Clone, Copy, Debug)]
pub enum SelectorUsage {
	Identifier,
	Selector,
	Anchor,
	Style,
	Script,
	Prefix,
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
			SelectorUsage::Prefix => self.prefix_counter += 1,
		}
	}

	pub fn sum(
		&mut self,
		incoming: Selector,
	) {
		self.counter += incoming.counter;
		self.identifier_counter += incoming.identifier_counter;
		self.selector_string_counter += incoming.selector_string_counter;
		self.anchor_counter += incoming.anchor_counter;
		self.style_counter += incoming.style_counter;
		self.script_counter += incoming.script_counter;
		self.prefix_counter += incoming.prefix_counter;
	}

	pub fn set_replacement(
		&mut self,
		replacement: String,
	) {
		self.replacement = Some(replacement);
	}
}

impl Default for Selector {
	fn default() -> Self {
		Self {
			r#type: SelectorType::Undefined,
			replacement: None,
			counter: 0,
			identifier_counter: 0,
			selector_string_counter: 0,
			anchor_counter: 0,
			style_counter: 0,
			script_counter: 0,
			prefix_counter: 0,
		}
	}
}




#[derive(Debug)]
pub struct Selectors {
	pub map: IndexMap<String, Selector>,
	pub class_counter: usize,
	pub id_counter: usize,
}

impl Selectors {
	pub fn new() -> Self {
		Self {
			map: IndexMap::new(),
			class_counter: 0,
			id_counter: 0,
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

	pub fn process(
		&mut self,
		config: &mut Config,
	) {
		for val in self.map.values_mut() {
			// Quick way to check if selectors are only being used in HTML
			// attributes and no where else. Skip generating a replacement.
			if val.identifier_counter == val.counter {
				continue;
			}

			val.set_replacement(encode_selector::to_radix(
				match val.r#type {
					SelectorType::Class => &self.class_counter,
					SelectorType::Id => &self.id_counter,
					SelectorType::Undefined => {
						panic!("Trying to encode a selector with undefined type.")
					},
				},
				&config.alphabet,
			));

			if val.r#type == SelectorType::Class {
				self.class_counter += 1;
			} else if val.r#type == SelectorType::Id {
				self.id_counter += 1;
			}
		}
	}
}

impl Default for Selectors {
	fn default() -> Self {
		Self::new()
	}
}
