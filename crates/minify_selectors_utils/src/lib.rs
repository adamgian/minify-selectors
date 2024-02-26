use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

use clap::Parser;
use indexmap::IndexMap;
use serde::Deserialize;
use serde_json::Result;




/// Post-processor that minifies classes and IDs in CSS, HTML, JS and SVG files.
#[derive(Debug, Parser)]
#[clap(
	name = "minify-selectors",
	version,
	long_about = None,
)]
pub struct Cli {
	/// Path to minify-selectors config file
	#[clap(
		short = 'c',
		long,
		conflicts_with_all(["input", "output"]),
		required_unless_present_all(["input", "output"]),
	)]
	config: Option<String>,

	/// Directory to process from
	#[clap(short = 'i', long, requires("output"))]
	input: Option<String>,

	/// Output directory to save files to
	#[clap(short = 'o', long, requires("input"))]
	output: Option<String>,

	/// Index to start encoding from
	#[clap(long = "start-index")]
	start_index: Option<usize>,

	/// Sequence of characters to use when encoding
	#[clap(long)]
	alphabet: Option<String>,

	/// Run processing operations concurrently where possible
	#[clap(long)]
	parallel: Option<Option<bool>>,

	/// Reorder selectors by frequency before minifying
	#[clap(long)]
	sort: Option<Option<bool>>,

	/// Custom attributes that contain space-separated list of classes.
	#[clap(long = "custom-class-attribute", value_delimiter = ',')]
	custom_class_attribute: Option<Vec<String>>,

	/// Custom attributes that contain an ID (or space-separated list of IDs).
	#[clap(long = "custom-id-attribute", value_delimiter = ',')]
	custom_id_attribute: Option<Vec<String>>,

	/// Custom attributes that contain a selector string.
	#[clap(long = "custom-selector-attribute", value_delimiter = ',')]
	custom_selector_attribute: Option<Vec<String>>,

	/// Custom attributes that contain a URL.
	#[clap(long = "custom-anchor-attribute", value_delimiter = ',')]
	custom_anchor_attribute: Option<Vec<String>>,

	/// Custom attributes that contain CSS styles.
	#[clap(long = "custom-style-attribute", value_delimiter = ',')]
	custom_style_attribute: Option<Vec<String>>,

	/// Custom attributes that contain JS code.
	#[clap(long = "custom-script-attribute", value_delimiter = ',')]
	custom_script_attribute: Option<Vec<String>>,
}




#[derive(Clone, Debug)]
pub struct Config {
	pub input: PathBuf,
	pub output: PathBuf,
	pub alphabet: (Vec<char>, Vec<usize>),
	pub start_index: usize,
	pub current_step: ProcessingSteps,
	pub parallel: bool,
	pub sort: bool,
	pub custom_attributes: Vec<(String, String)>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum ProcessingSteps {
	#[default]
	ReadingFromFiles,
	EncodingSelectors,
	WritingToFiles,
}

impl Config {
	pub fn new() -> Self {
		let cli_args = Cli::parse();
		let mut config: Config = Default::default();
		let external_config: Option<ExternalConfig> = cli_args.config.clone().map(|config_path| {
			serde_json::from_str(
				&fs::read_to_string(config_path).expect("Could not find or open config file"),
			)
			.expect("Could not parse config file")
		});

		println!("{:?}", external_config);

		config.input = if cli_args.config.is_some() {
			PathBuf::from(&external_config.as_ref().unwrap().input)
		} else {
			PathBuf::from(&cli_args.input.unwrap())
		};
		config.output = if cli_args.config.is_some() {
			PathBuf::from(&external_config.as_ref().unwrap().output)
		} else {
			PathBuf::from(&cli_args.output.unwrap())
		};

		if external_config.is_some() {
			if let Some(alphabet) = &external_config.as_ref().unwrap().alphabet {
				config.alphabet = encode_selector::into_alphabet_set(&alphabet);
			}
		} else {
			if let Some(alphabet) = cli_args.alphabet {
				config.alphabet = encode_selector::into_alphabet_set(&alphabet);
			}
		}

		if external_config.is_some() {
			if let Some(index) = external_config.as_ref().unwrap().start_index {
				config.start_index = index;
			}
		} else {
			if let Some(index) = cli_args.start_index {
				config.start_index = index;
			}
		}

		if external_config.is_some() {
			if let Some(parallel) = external_config.as_ref().unwrap().parallel {
				config.parallel = parallel;
			}
		} else {
			config.parallel = match &cli_args.parallel {
				None => false,
				Some(None) => true,         // --parallel
				Some(Some(true)) => true,   // --parallel=true
				Some(Some(false)) => false, // --parallel=false
			};
		}

		if external_config.is_some() {
			if let Some(sort) = external_config.as_ref().unwrap().sort {
				config.sort = sort;
			}
		} else {
			config.sort = match &cli_args.sort {
				None => true,
				Some(None) => true,         // --sort
				Some(Some(true)) => true,   // --sort=true
				Some(Some(false)) => false, // --sort=false
			};
		}

		let mut custom_attributes: Vec<(String, String)> = vec![];

		if let Some(attributes) = &cli_args.custom_class_attribute {
			for name in attributes {
				custom_attributes.push((name.to_string(), "class".to_string()));
			}
		}
		if let Some(attributes) = &cli_args.custom_id_attribute {
			for name in attributes {
				custom_attributes.push((name.to_string(), "id".to_string()));
			}
		}
		if let Some(attributes) = &cli_args.custom_selector_attribute {
			for name in attributes {
				custom_attributes.push((name.to_string(), "selector".to_string()));
			}
		}
		if let Some(attributes) = &cli_args.custom_anchor_attribute {
			for name in attributes {
				custom_attributes.push((name.to_string(), "anchor".to_string()));
			}
		}
		if let Some(attributes) = &cli_args.custom_style_attribute {
			for name in attributes {
				custom_attributes.push((name.to_string(), "style".to_string()));
			}
		}
		if let Some(attributes) = &cli_args.custom_script_attribute {
			for name in attributes {
				custom_attributes.push((name.to_string(), "script".to_string()));
			}
		}
		config.custom_attributes = custom_attributes;

		config
	}
}

impl Default for Config {
	fn default() -> Self {
		Self {
			input: PathBuf::from(""),
			output: PathBuf::from(""),
			alphabet: encode_selector::into_alphabet_set(
				"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ",
			),
			start_index: 0,
			current_step: ProcessingSteps::ReadingFromFiles,
			parallel: false,
			sort: true,
			custom_attributes: vec![],
		}
	}
}




/// Dedicated struct to handle the external config file specific structure
#[allow(dead_code)]
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ExternalConfig {
	input: String,
	output: String,
	alphabet: Option<String>,
	// #[serde(rename = "startIndex")]
	start_index: Option<usize>,
	parallel: Option<bool>,
	sort: Option<bool>,
	custom_attributes: Option<Vec<(String, String)>>,
}




/// Metadata for selector
#[derive(Clone, Debug, Default)]
pub struct Selector {
	pub kind: Option<SelectorType>,
	pub replacement: Option<String>,
	pub counter: usize,
	pub markup_class_counter: usize,
	pub markup_id_counter: usize,
	pub selector_string_counter: usize,
	pub anchor_counter: usize,
	pub style_counter: usize,
	pub script_counter: usize,
	pub prefix_counter: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SelectorType {
	Class,
	Id,
}

#[derive(Clone, Copy, Debug)]
pub enum SelectorUsage {
	MarkupClass,
	MarkupId,
	SelectorString,
	Anchor,
	Style,
	Script,
	Prefix,
}

impl Selector {
	pub fn new(selector: &str) -> Self {
		Self {
			kind: match selector.chars().next() {
				Some('.') => Some(SelectorType::Class),
				Some('#') => Some(SelectorType::Id),
				_ => panic!("Missing or unknown selector type"),
			},
			..Default::default()
		}
	}

	pub fn count(
		&mut self,
		usage: Option<SelectorUsage>,
	) {
		if usage.is_some() {
			self.counter += 1;
		}
		match usage {
			Some(SelectorUsage::MarkupClass) => self.markup_class_counter += 1,
			Some(SelectorUsage::MarkupId) => self.markup_id_counter += 1,
			Some(SelectorUsage::SelectorString) => self.selector_string_counter += 1,
			Some(SelectorUsage::Anchor) => self.anchor_counter += 1,
			Some(SelectorUsage::Style) => self.style_counter += 1,
			Some(SelectorUsage::Script) => self.script_counter += 1,
			Some(SelectorUsage::Prefix) => self.prefix_counter += 1,
			None => {},
		}
	}

	pub fn sum(
		&mut self,
		incoming: Selector,
	) {
		self.counter += incoming.counter;
		self.markup_class_counter += incoming.markup_class_counter;
		self.markup_id_counter += incoming.markup_id_counter;
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




#[derive(Debug)]
pub struct Selectors {
	// Note: map key should not have any escaped characters,
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
		usage: Option<SelectorUsage>,
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
		let mut encoded_classes: HashSet<String> = HashSet::new();
		let mut skipped_classes: HashSet<String> = HashSet::new();
		let mut requires_recheck: bool;

		loop {
			requires_recheck = false;

			// Loop through selectors map and assign an encoded selector to each.
			for (key, value) in self.map.iter_mut() {
				// Skip generating a replacement if classes are only being used
				// in markup attributes and no where else.
				if value.markup_class_counter == value.counter
					&& !encoded_classes.contains(key.clone().strip_prefix('.').unwrap())
				{
					if !skipped_classes.contains(&key.clone()) {
						skipped_classes.insert(key.clone());
						requires_recheck = true;
					}
					continue;
				}

				// Conflicts with an encoded class with the skipped class name,
				// it cannot be skipped and will need to be encoded.
				if value.markup_class_counter == value.counter
					&& encoded_classes.contains(key.clone().strip_prefix('.').unwrap())
					&& skipped_classes.contains(&key.clone())
				{
					skipped_classes.remove(&key.clone());
					// Requests another loop over since there's a new encoded class being
					// added. Need to make sure any remaining skipped classes still do not clash.
					requires_recheck = true;
				}

				if value.replacement.is_some() {
					continue;
				}

				value.set_replacement(encode_selector::to_radix(
					match value.kind {
						Some(SelectorType::Class) => &self.class_counter,
						Some(SelectorType::Id) => &self.id_counter,
						None => {
							panic!("Trying to encode a selector with undefined type.");
						},
					},
					&config.alphabet,
				));

				if value.kind == Some(SelectorType::Class) {
					// Also keep track of encoded class name
					encoded_classes.insert(value.replacement.to_owned().unwrap());
					self.class_counter += 1;
				} else if value.kind == Some(SelectorType::Id) {
					self.id_counter += 1;
				}
			}

			if skipped_classes.is_empty() || !requires_recheck {
				break;
			}
		}
	}

	/// Reorder selectors map, by highest frenquency first.
	pub fn sort_by_frequency(&mut self) {
		self.map
			.sort_by(|_x_key, x_val, _y_key, y_val| y_val.counter.cmp(&x_val.counter));
	}
}

impl Default for Selectors {
	fn default() -> Self {
		Self::new()
	}
}
