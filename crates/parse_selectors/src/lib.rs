pub mod markup;
pub mod regexes;
pub mod script;
pub mod style;

use markup::*;
use minify_selectors_utils::*;
use onig::*;
use script::*;
use style::*;




pub fn read_from_css(
	file_string: &mut str,
	selectors: &mut Selectors,
	config: &Config,
) {
	markup::html_attributes::init(&config.custom_attributes);
	analyse_css(file_string, selectors, config);
}

pub fn write_to_css(
	file_string: &mut String,
	selectors: &Selectors,
	config: &Config,
) {
	rewrite_css(file_string, selectors, config);
}

pub fn read_from_html(
	file_string: &mut str,
	selectors: &mut Selectors,
	config: &Config,
) {
	markup::html_attributes::init(&config.custom_attributes);
	analyse_html(file_string, selectors, config, None);
}

pub fn write_to_html(
	file_string: &mut String,
	selectors: &Selectors,
	config: &Config,
) {
	rewrite_html(file_string, selectors, config);
}

pub fn read_from_js(
	file_string: &mut str,
	selectors: &mut Selectors,
	config: &Config,
) {
	markup::html_attributes::init(&config.custom_attributes);
	analyse_js(file_string, selectors, config);
}

pub fn write_to_js(
	file_string: &mut String,
	selectors: &Selectors,
	config: &Config,
) {
	rewrite_js(file_string, selectors, config);
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
	selectors: &Selectors,
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
pub fn get_function_arguments(string: &str) -> FindCaptures {
	regexes::STRING_DELIMITED_BY_COMMA.captures_iter(string)
}

/// Checks if a (minify-selector specific) prefixed selector
/// is used in the given string snippet.
pub fn is_prefixed_selector(string: &str) -> bool {
	regexes::PREFIXED_SELECTORS.find(string).is_some()
}




/// Analyse minify-selectors specific prefixed selectors.
pub fn analyse_prefixed_selectors(
	file_string: &mut str,
	selectors: &mut Selectors,
) {
	for capture in regexes::PREFIXED_SELECTORS.captures_iter(file_string) {
		// "#__ignore--foo", ".__ignore--bar" or "__ignore--baz"
		// Note: no need to add a selector that has been marked as ignore
		// to selectors map.
		if capture.at(2) == Some("ignore") {
			continue;
		}

		let mut identifier = unescape_css_chars(capture.at(3).unwrap().trim());

		match capture.at(2) {
			// "__class--foo"
			Some("class") => identifier = format!(".{identifier}"),
			// "__id--foo"
			Some("id") => identifier = format!("#{identifier}"),
			// "#__--foo" or ".__--bar"
			Some(&_) | None => {
				identifier = format!(
					"{prefix}{name}",
					prefix = capture.at(1).unwrap(),
					name = identifier,
				)
			},
		}

		add_selector_to_map(&identifier, selectors, Some(SelectorUsage::Prefix));
	}
}

/// Rewrite minify-selectors specific prefixed selectors.
pub fn rewrite_prefixed_selectors(
	file_string: &mut String,
	selectors: &Selectors,
) {
	*file_string = regexes::PREFIXED_SELECTORS.replace_all(file_string, |capture: &Captures| {
		let mut identifier = unescape_css_chars(capture.at(3).unwrap().trim());

		match capture.at(2) {
			// "__class--foo"
			Some("class") => {
				identifier = get_encoded_selector(&format!(".{identifier}"), selectors)
					.unwrap_or(identifier);
			},
			// "__id--foo"
			Some("id") => {
				identifier = get_encoded_selector(&format!("#{identifier}"), selectors)
					.unwrap_or(identifier);
			},
			// "#__ignore--foo", ".__ignore--bar" or "__ignore--baz"
			Some("ignore") => {
				identifier = format!(
					"{prefix}{name}",
					prefix = capture.at(1).unwrap_or(""),
					name = identifier,
				);
			},
			// "#__--foo" or ".__--bar"
			Some(&_) | None => {
				identifier = format!(
					"{prefix}{name}",
					prefix = capture.at(1).unwrap(),
					name = get_encoded_selector(
						&format!(
							"{prefix}{name}",
							prefix = capture.at(1).unwrap(),
							name = identifier,
						),
						selectors,
					)
					.unwrap_or(identifier)
				);
			},
		}

		identifier
	});
}

/// Analyse string with tokens delimited by whitespaces.
///
/// Notes:
///  - As regexes::STRING_DELIMITED_BY_SPACE regex is simple - only grouping non
///    whitespace characters together - any quote delimiters will need to be
///    trimmed and added back on afterwards.
///  - context is necessary in order to determine what the token(s) should be
///    processed as (e.g. class or id).
pub fn analyse_string_of_tokens(
	string: &mut String,
	selectors: &mut Selectors,
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

	for capture in regexes::STRING_DELIMITED_BY_SPACE.captures_iter(string) {
		// Check if token has a minify-selectors specific prefix,
		// It should be handled with parse_prefixed_selectors().
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

/// Rewrite string with tokens delimited by whitespaces.
///
/// Notes:
///  - As regexes::STRING_DELIMITED_BY_SPACE regex is simple - only grouping non
///    whitespace characters together - any quote delimiters will need to be
///    trimmed and added back on afterwards.
///  - context is necessary in order to determine what the token(s) should be
///    processed as (e.g. class or id).
pub fn rewrite_string_of_tokens(
	string: &mut String,
	selectors: &Selectors,
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
			// It should be handled with parse_prefixed_selectors().
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
		quote = quote_type,
	);
}

/// Analyse function arguments, delimited by commas.
///
/// Notes:
///  - context is necessary in order to determine what the token(s) should be
///    processed as (e.g. class or id).
pub fn analyse_string_of_arguments(
	string: &mut str,
	selectors: &mut Selectors,
	context: &str,
	usage: Option<SelectorUsage>,
) {
	let prefix: &str = match context {
		"class" => ".",
		"id" => "#",
		_ => "",
	};

	for capture in regexes::STRING_DELIMITED_BY_COMMA.captures_iter(string) {
		// Check if argument has a minify-selectors specific prefix,
		// It should be handled with parse_prefixed_selectors().
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

/// Rewrite function arguments, delimited by commas.
///
/// Notes:
///  - context is necessary in order to determine what the token(s) should be
///    processed as (e.g. class or id).
pub fn rewrite_string_of_arguments(
	string: &mut String,
	selectors: &Selectors,
	context: &str,
) {
	let prefix: &str = match context {
		"class" => ".",
		"id" => "#",
		_ => "",
	};

	*string = regexes::STRING_DELIMITED_BY_COMMA.replace_all(string, |capture: &Captures| {
		// Check if argument has a minify-selectors specific prefix,
		// It should be handled with parse_prefixed_selectors().
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

// Analyse target IDs in anchor link URLs.
pub fn analyse_anchor_links(
	string: &mut String,
	selectors: &mut Selectors,
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

// Rewrite target IDs in anchor link URLs.
pub fn rewrite_anchor_links(
	string: &mut String,
	selectors: &Selectors,
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
				target_id =
					get_encoded_selector(&unescape_js_chars(capture.at(2).unwrap()), selectors,)
						.unwrap_or_else(|| capture.at(2).unwrap().to_string()),
			)
		}),
		quote = quote_type,
	);
}
