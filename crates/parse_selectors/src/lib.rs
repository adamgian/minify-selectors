pub mod markup;
pub mod regexes;
pub mod script;
pub mod style;

use markup::*;
use minify_selectors_utils::*;
use onig::*;
use script::*;
use style::*;




pub fn from_css(
	file_string: &mut String,
	selectors: &mut Selectors,
	config: &Config,
) {
	process_css(file_string, selectors, config);
}

pub fn from_html(
	file_string: &mut String,
	selectors: &mut Selectors,
	config: &Config,
) {
	process_html(
		file_string,
		selectors,
		config,
		Some(SelectorUsage::Identifier),
	);
}

pub fn from_js(
	file_string: &mut String,
	selectors: &mut Selectors,
	config: &Config,
) {
	process_js(file_string, selectors, config);
}




pub fn add_selector_to_map(
	selector: &str,
	selectors: &mut Selectors,
	usage: Option<SelectorUsage>,
) {
	selectors.add(selector.to_owned(), usage);
}

/// Fetch replacement encoded selector from selectors hashmap.
// In the event selector has been removed from the mapping because
// it is deemed not worth encoding, leave it as is by returning
// the selector right back.
pub fn get_encoded_selector(
	selector: &str,
	selectors: &mut Selectors,
) -> Option<String> {
	if let Some(encoded_selector) = selectors.map.get(selector) {
		if encoded_selector.replacement.is_some() {
			encoded_selector.replacement.clone()
		} else {
			None
		}
	} else {
		Some(selector.to_owned())
	}
}

/// Returns an iterator of function arguments.
pub fn get_function_arguments<'r, 't>(string: &'t str) -> FindCaptures<'r, 't> {
	regexes::STRING_DELIMITED_BY_COMMA.captures_iter(string)
}

/// Checks if a (minify-selector specific) prefixed selector
/// is used in the given string snippet.
pub fn is_prefixed_selector(string: &str) -> bool {
	regexes::PREFIXED_SELECTORS.find(string).is_some()
}




/// Process minify-selectors specific prefixed selectors.
pub fn process_prefixed_selectors(
	file_string: &mut String,
	selectors: &mut Selectors,
	config: &Config,
) {
	if config.current_step == ProcessingSteps::WritingToFiles {
		handle_file_write(file_string, selectors);
	} else {
		handle_file_read(file_string, selectors);
	}

	fn handle_file_read(
		file_string: &str,
		selectors: &mut Selectors,
	) {
		for capture in regexes::PREFIXED_SELECTORS.captures_iter(file_string) {
			// "#__ignore--foo", ".__ignore--bar" or "__ignore--baz"
			// Note: no need to add a selector that has been marked as ignore
			// to selectors map.
			if capture.at(2) == Some("ignore") {
				continue;
			}

			let mut indentifier = capture.at(3).unwrap().trim().to_string();

			match capture.at(2) {
				// "__class--foo"
				Some("class") => indentifier = format!(".{}", indentifier),
				// "__id--foo"
				Some("id") => indentifier = format!("#{}", indentifier),
				// "#__--foo" or ".__--bar"
				Some(&_) | None => {
					indentifier = format!(
						"{prefix}{name}",
						prefix = capture.at(1).unwrap(),
						name = indentifier,
					)
				},
			}

			add_selector_to_map(&indentifier, selectors, Some(SelectorUsage::Prefix));
		}
	}

	fn handle_file_write(
		file_string: &mut String,
		selectors: &mut Selectors,
	) {
		*file_string =
			regexes::PREFIXED_SELECTORS.replace_all(file_string, |capture: &Captures| {
				let mut placeholder_value = capture.at(3).unwrap().trim().to_string();

				match capture.at(2) {
					// "__class--foo"
					Some("class") => {
						placeholder_value =
							get_encoded_selector(&format!(".{}", placeholder_value), selectors)
								.unwrap_or(placeholder_value);
					},
					// "__id--foo"
					Some("id") => {
						placeholder_value =
							get_encoded_selector(&format!("#{}", placeholder_value), selectors)
								.unwrap_or(placeholder_value);
					},
					// "#__ignore--foo", ".__ignore--bar" or "__ignore--baz"
					Some("ignore") => {
						placeholder_value = format!(
							"{prefix}{name}",
							prefix = capture.at(1).unwrap_or(""),
							name = placeholder_value,
						);
					},
					// "#__--foo" or ".__--bar"
					Some(&_) | None => {
						placeholder_value = format!(
							"{prefix}{name}",
							prefix = capture.at(1).unwrap(),
							name = get_encoded_selector(
								&format!(
									"{prefix}{name}",
									prefix = capture.at(1).unwrap(),
									name = placeholder_value,
								),
								selectors,
							)
							.unwrap_or(placeholder_value)
						);
					},
				}

				placeholder_value
			});
	}
}


/// Process string with tokens delimited by whitespaces.
///
/// Notes:
///  - As regexes::STRING_DELIMITED_BY_SPACE regex is simple - only grouping non
///    whitespace characters together - any quote delimiters will need to be
///    trimmed and added back on afterwards.
///  - context is neseccary in order to determine what the token(s) should be
///    processed as (e.g. class or id).
pub fn process_string_of_tokens(
	string: &mut String,
	selectors: &mut Selectors,
	config: &Config,
	context: &str,
	usage: Option<SelectorUsage>,
) {
	let prefix: &str = match context {
		"class" => ".",
		"id" => "#",
		_ => "",
	};

	// Handle strings that have quote delimiters included.
	let quote_type: &str = match string.chars().next() {
		Some('\'') if string.len() >= 2 => "'",
		Some('"') if string.len() >= 2 => "\"",
		Some('`') if string.len() >= 2 => "`",
		_ => "",
	};

	// Trim quotes (if any) from value capture group.
	if !quote_type.is_empty() {
		string.pop();
		string.remove(0);
	}

	if config.current_step == ProcessingSteps::WritingToFiles {
		handle_file_write(string, prefix, quote_type, selectors);
	} else {
		handle_file_read(string, prefix, usage, selectors);
	}

	fn handle_file_read(
		string: &str,
		prefix: &str,
		usage: Option<SelectorUsage>,
		selectors: &mut Selectors,
	) {
		for capture in regexes::STRING_DELIMITED_BY_SPACE.captures_iter(string) {
			// Check if token has a minify-selectors specific prefix,
			// It should be handled with process_prefixed_selectors().
			if !is_prefixed_selector(capture.at(0).unwrap()) {
				add_selector_to_map(
					&format!(
						"{prefix}{token}",
						prefix = prefix,
						token = unescape_css_chars(capture.at(1).unwrap()),
					),
					selectors,
					usage,
				);
			}
		}
	}

	fn handle_file_write(
		string: &mut String,
		prefix: &str,
		quote: &str,
		selectors: &mut Selectors,
	) {
		*string = format!(
			"{quote}{tokens}{quote}",
			tokens =
				regexes::STRING_DELIMITED_BY_SPACE.replace_all(string, |capture: &Captures| {
					// Check if token has a minify-selectors specific prefix,
					// It should be handled with process_prefixed_selectors().
					if is_prefixed_selector(capture.at(0).unwrap()) {
						return capture.at(0).unwrap().to_string();
					}

					get_encoded_selector(
						&format!(
							"{prefix}{token}",
							prefix = prefix,
							token = unescape_css_chars(capture.at(1).unwrap())
						),
						selectors,
					)
					.unwrap_or_else(|| capture.at(1).unwrap().to_string())
				}),
			quote = quote,
		);
	}
}

/// Process function arguments, delimited by commas.
///
/// Notes:
///  - context is neseccary in order to determine what the token(s) should be
///    processed as (e.g. class or id).
pub fn process_string_of_arguments(
	string: &mut String,
	selectors: &mut Selectors,
	config: &Config,
	context: &str,
	usage: Option<SelectorUsage>,
) {
	let prefix: &str = match context {
		"class" => ".",
		"id" => "#",
		_ => "",
	};

	if config.current_step == ProcessingSteps::WritingToFiles {
		handle_file_write(string, prefix, selectors);
	} else {
		handle_file_read(string, prefix, usage, selectors);
	}

	fn handle_file_read(
		string: &str,
		prefix: &str,
		usage: Option<SelectorUsage>,
		selectors: &mut Selectors,
	) {
		for capture in regexes::STRING_DELIMITED_BY_COMMA.captures_iter(string) {
			// Check if argument has a minify-selectors specific prefix,
			// It should be handled with process_prefixed_selectors().
			if is_prefixed_selector(capture.at(0).unwrap()) {
				continue;
			}

			// String argument
			if capture.at(3).is_some() {
				add_selector_to_map(
					&format!(
						"{prefix}{token}",
						prefix = prefix,
						token = capture.at(3).unwrap(),
					),
					selectors,
					usage,
				);
			}
		}
	}

	fn handle_file_write(
		string: &mut String,
		prefix: &str,
		selectors: &mut Selectors,
	) {
		*string = regexes::STRING_DELIMITED_BY_COMMA.replace_all(string, |capture: &Captures| {
			// Check if argument has a minify-selectors specific prefix,
			// It should be handled with process_prefixed_selectors().
			if is_prefixed_selector(capture.at(0).unwrap()) {
				return capture.at(0).unwrap().to_string();
			}

			// Check if argument is a string, variable/expression or object/array.
			//   - 1: simple string argument (token string and delimiters)
			//       - 2: token delimiter
			//       - 3: token string
			//   - 4: variable or expression argument
			//   - 5: object argument
			//   - 6: array argument
			if capture.at(3).is_some() {
				format!(
					"{quote}{argument}{quote}",
					argument = get_encoded_selector(
						&format!(
							"{prefix}{token}",
							prefix = prefix,
							token = capture.at(3).unwrap(),
						),
						selectors,
					)
					.unwrap_or_else(|| capture.at(3).unwrap().to_string()),
					quote = capture.at(2).unwrap(),
				)
			// TODO:
			//} else if capture.at(4).is_some() {
			//	return capture.at(0).unwrap().to_string();
			} else {
				// Capture group 5 (<object>) or 6 (<array>) .is_some() evaluates to true
				// or another case. Either way nothing needs to be done to this argument.
				return capture.at(0).unwrap().to_string();
			}
		});
	}
}

// Process target IDs in anchor link URLs.
pub fn process_anchor_links(
	string: &mut String,
	selectors: &mut Selectors,
	config: &Config,
) {
	// Handle strings that have quote delimiters included.
	let quote_type: &str = match string.chars().next() {
		Some('\'') => "'",
		Some('"') => "\"",
		Some('`') => "`",
		_ => "",
	};

	// Trim quotes (if any).
	if !quote_type.is_empty() {
		string.pop();
		string.remove(0);
	}

	if config.current_step == ProcessingSteps::WritingToFiles {
		handle_file_write(string, quote_type, selectors);
	} else {
		handle_file_read(string, selectors);
	}

	fn handle_file_read(
		string: &str,
		selectors: &mut Selectors,
	) {
		for capture in regexes::INTERNAL_ANCHOR_TARGET_ID.captures_iter(string) {
			if capture.at(1).is_none() {
				continue;
			}

			add_selector_to_map(
				&unescape_js_chars(capture.at(2).unwrap()),
				selectors,
				Some(SelectorUsage::Anchor),
			);
		}
	}

	fn handle_file_write(
		string: &mut String,
		quote: &str,
		selectors: &mut Selectors,
	) {
		*string = format!(
			"{quote}{url}{quote}",
			url = regexes::INTERNAL_ANCHOR_TARGET_ID.replace(string, |capture: &Captures| {
				if capture.at(1).is_none() {
					return capture.at(0).unwrap().to_string();
				}

				format!(
					"{url}#{target_id}",
					url = capture.at(1).unwrap_or(""),
					target_id = get_encoded_selector(
						&unescape_js_chars(capture.at(2).unwrap()),
						selectors,
					)
					.unwrap_or_else(|| capture.at(2).unwrap().to_string()),
				)
			}),
			quote = quote,
		);
	}
}
