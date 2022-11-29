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
	process_html(file_string, selectors, config);
}

pub fn from_js(
	file_string: &mut String,
	selectors: &mut Selectors,
	config: &Config,
) {
	process_js(file_string, selectors, config);
}




/// Fetch encoded selector from selectors hashmap.
/// If selector is new and unique, generate one for it
/// and add it to selectors.
pub fn get_encoded_selector(
	selector: &str,
	selectors: &mut Selectors,
	config: &Config,
) -> String {
	match selectors.contains(selector) {
		true => selectors.get(selector),

		false => {
			let encoded_selector: String = encode_selector::to_radix(
				match selector.chars().next() {
					Some('.') => &selectors.class_index,
					Some('#') => &selectors.id_index,
					_ => panic!("Missing or unknown selector type"),
				},
				&config.alphabet,
			);
			selectors.add(selector.to_owned(), encoded_selector.clone());
			encoded_selector
		},
	}
}

/// Returns an iterator of function arguments.
pub fn get_function_arguments<'r, 't>(string: &'t str) -> FindCaptures<'r, 't> {
	regexes::STRING_DELIMITED_BY_COMMA.captures_iter(string)
}

/// Process minify-selectors specific prefixed selectors.
pub fn process_prefixed_selectors(
	file_string: &mut String,
	selectors: &mut Selectors,
	config: &Config,
) {
	*file_string = regexes::PREFIXED_SELECTORS.replace_all(file_string, |capture: &Captures| {
		let mut placeholder_value = capture.at(3).unwrap().trim().to_string();

		match capture.at(2) {
			#[rustfmt::skip]
			// "__class--foo"
			Some("class") => {
				placeholder_value = get_encoded_selector(
					&format!(".{}", placeholder_value),
					selectors,
					config
				);
			},

			#[rustfmt::skip]
			// "__id--foo"
			Some("id") => {
				placeholder_value = get_encoded_selector(
					&format!("#{}", placeholder_value),
					selectors,
					config
				);
			},

			// Prefix (if any # or .) prefixed to value capture group
			// will replace the entire match.
			Some("ignore") => {
				placeholder_value = format!(
					"{prefix}{name}",
					prefix = capture.at(1).unwrap_or(""),
					name = placeholder_value
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
						config
					)
				);
			},
		}

		placeholder_value
	});
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

	*string = format!(
		"{quote}{tokens}{quote}",
		tokens = regexes::STRING_DELIMITED_BY_SPACE.replace_all(string, |capture: &Captures| {
			// Check if token has a minify-selectors specific prefix,
			// It should be handled with process_prefixed_selectors().
			if regexes::PREFIXED_SELECTORS
				.find(capture.at(0).unwrap())
				.is_some()
			{
				return capture.at(0).unwrap().to_string();
			}

			get_encoded_selector(
				&format!(
					"{prefix}{token}",
					prefix = prefix,
					token = unescape_css_chars(capture.at(1).unwrap())
				),
				selectors,
				config,
			)
		}),
		quote = quote_type,
	);
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
) {
	let prefix: &str = match context {
		"class" => ".",
		"id" => "#",
		_ => "",
	};

	*string = regexes::STRING_DELIMITED_BY_COMMA.replace_all(string, |capture: &Captures| {
		// Check if argument has a minify-selectors specific prefix,
		// It should be handled with process_prefixed_selectors().
		if regexes::PREFIXED_SELECTORS
			.find(capture.at(0).unwrap())
			.is_some()
		{
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
					config,
				),
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
					config
				),
			)
		}),
		quote = quote_type,
	);
}
