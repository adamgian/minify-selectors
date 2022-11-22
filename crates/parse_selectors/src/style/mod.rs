pub mod regexes;

use minify_selectors_utils::*;
use onig::*;
use crate::markup::whitelist::*;
use crate::style::regexes as style_regex;




pub fn process_css(
	file_string: &mut String,
	selectors: &mut Selectors,
	config: &Config,
) {
	process_css_selectors(file_string, selectors, config);
	process_css_attributes(file_string, selectors, config);
	process_css_functions(file_string, selectors, config);
	super::process_prefixed_selectors(file_string, selectors, config);
}

/// Process classes and IDs in CSS file/embed or as a
/// CSS selector string.
pub fn process_css_selectors(
	file_string: &mut String,
	selectors: &mut Selectors,
	config: &Config,
) {
	*file_string = style_regex::CSS_SELECTORS.replace_all(file_string, |capture: &Captures| {
		// Check that capture group 2 exists,
		// i.e. matched to a class/id name — and not an attribute selector,
		// rule block, @import, or comment — which does not have this group.
		if capture.at(2).is_some() {
			return format!(
				"{prefix}{identifier}",
				prefix = &capture.at(1).unwrap(),
				identifier = super::get_encoded_selector(
					&unescape_css_chars(capture.at(0).unwrap()),
					selectors,
					config,
				),
			);
		}
		// Matched to an attribute selector, rule block, @import or comment.
		// Leave it as is.
		capture.at(0).unwrap().to_owned()
	});
}

// Process CSS attribute selectors.
pub fn process_css_attributes(
	file_string: &mut String,
	selectors: &mut Selectors,
	config: &Config,
) {
	*file_string = style_regex::CSS_ATTRIBUTES.replace_all(file_string, |capture: &Captures| {
		// Check that capture group 2 exists — if it doesn't, it is matched
		// to an incomplete attribute selector (no value), rule block or comment.
		// Leave it as is.
		if capture.at(2).is_none() {
			return capture.at(0).unwrap().to_owned();
		}

		let attribute_name: String = unescape_css_chars(capture.at(1).unwrap());
		let attribute_quote_type: &str = capture.at(3).unwrap_or("");
		let attribute_flag: &str = capture.at(6).unwrap_or("");
		let mut attribute_value: String = capture.at(4).unwrap().to_string();

		if ATTRIBUTES_WHITELIST.contains_key(&attribute_name) {
			// Do not process attribute selector if case-insensitive
			// flag has been set.
			if attribute_flag.to_lowercase().contains('i') {
				return capture.at(0).unwrap().to_string();
			}

			// Work out if value(s) are classes, IDs or selectors.
			let attribute_type_designation: &str =
				ATTRIBUTES_WHITELIST.get(&attribute_name).unwrap();

			match attribute_type_designation {
				"id" | "class" => {
					super::process_string_of_tokens(
						&mut attribute_value,
						selectors,
						config,
						attribute_type_designation,
					);
				},
				"selector" => {
					attribute_value = unescape_css_chars(&attribute_value);
					process_css(&mut attribute_value, selectors, config);
				},
				"anchor" => {
					attribute_value = unescape_css_chars(&attribute_value);
					super::process_anchor_links(&mut attribute_value, selectors, config);
				},
				_ => {},
			}
		} else {
			// Attribute does not contain classes and/or IDs.
			// Leave it as is.
			return capture.at(0).unwrap().to_string();
		}

		format!(
			"[{attribute}{operator}{quote}{value}{quote}{space}{flag}]",
			attribute = capture.at(1).unwrap(),
			operator = capture.at(2).unwrap(),
			quote = attribute_quote_type,
			value = attribute_value,
			space = capture.at(5).unwrap(),
			flag = attribute_flag,
		)
	});
}

// Process CSS functions.
pub fn process_css_functions(
	file_string: &mut String,
	selectors: &mut Selectors,
	config: &Config,
) {
	*file_string = style_regex::CSS_FUNCTIONS.replace_all(file_string, |capture: &Captures| {
		// Check that capture group 4 (argument) exists
		if capture.at(4).is_none() {
			return capture.at(0).unwrap().to_owned();
		}

		let function_name: &str = capture.at(1).unwrap();
		let mut function_argument = capture.at(4).unwrap().to_string();

		// For now, only url is needed to be processed
		// match function_name {
		// 	"url" => {
		super::process_anchor_links(&mut function_argument, selectors, config);
		// 	},
		// 	_ => {},
		// }

		format!(
			"{function}{join}{quote}{argument}",
			function = function_name,
			join = capture.at(2).unwrap(),
			quote = capture.at(3).unwrap(),
			argument = function_argument,
		)
	});
}


// Convert any escaped chars in CSS selector string to UTF8 char.
pub fn unescape_css_chars(selector_string: &str) -> String {
	let mut unescaped = selector_string.to_string();

	if style_regex::ESCAPED_CSS_CHARS
		.find(selector_string)
		.is_none()
	{
		return unescaped;
	}

	unescaped = style_regex::ESCAPED_CSS_CHARS.replace_all(&unescaped, |capture: &Captures| {
		String::from(
			if capture.at(1).is_some() {
				// Unicode code point, remove trailing whitespace (if any)
				// and convert hex codepoint to UTF8 character.
				char::from_u32(
					u32::from_str_radix(
						capture
							.at(1)
							.unwrap()
							.strip_prefix('\\')
							.unwrap()
							.trim_end_matches(' '),
						16,
					)
					.unwrap(),
				)
				.unwrap()
			} else if capture.at(2).is_some() {
				// Escaped single character, only need to disregard the leading blackslash.
				capture.at(2).unwrap().chars().nth(1).unwrap()
			} else {
				panic!("Not any of the known capture groups");
			},
		)
	});

	unescaped
}
