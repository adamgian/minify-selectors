pub mod regexes;

use minify_selectors_utils::*;
use onig::*;

use crate::markup::html_attributes::*;
use crate::style::regexes as style_regex;




pub fn analyse_css(
	file_string: &mut str,
	selectors: &mut Selectors,
	config: &Config,
) {
	analyse_css_selectors(file_string, selectors);
	analyse_css_attributes(file_string, selectors, config);
	analyse_css_functions(file_string, selectors);
	super::analyse_prefixed_selectors(file_string, selectors);
}

pub fn rewrite_css(
	file_string: &mut String,
	selectors: &Selectors,
	config: &Config,
) {
	rewrite_css_selectors(file_string, selectors);
	rewrite_css_attributes(file_string, selectors, config);
	rewrite_css_functions(file_string, selectors);
	super::rewrite_prefixed_selectors(file_string, selectors);
}

/// Analyse classes and IDs in CSS file/embed or as a
/// CSS selector string.
pub fn analyse_css_selectors(
	file_string: &mut str,
	selectors: &mut Selectors,
) {
	for capture in style_regex::CSS_SELECTORS.captures_iter(file_string) {
		// Check that capture group 2 exists,
		// i.e. matched to a class/id name — and not an attribute selector,
		// rule block, @import, or comment — which does not have this group.
		if capture.at(2).is_some() {
			super::add_selector_to_map(
				&format!(
					"{prefix}{identifier}",
					prefix = &capture.at(1).unwrap(),
					identifier = &unescape_css_chars(capture.at(2).unwrap()),
				),
				selectors,
				Some(SelectorUsage::Style),
			);
		}
	}
}

/// Rewrite classes and IDs in CSS file/embed or as a
/// CSS selector string.
pub fn rewrite_css_selectors(
	file_string: &mut String,
	selectors: &Selectors,
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
				)
				.unwrap_or_else(|| capture.at(0).unwrap().to_string()),
			);
		}
		// Matched to an attribute selector, rule block, @import or comment.
		// Leave it as is.
		capture.at(0).unwrap().to_owned()
	});
}

// Analyse CSS attribute selectors.
pub fn analyse_css_attributes(
	file_string: &mut str,
	selectors: &mut Selectors,
	config: &Config,
) {
	for capture in style_regex::CSS_ATTRIBUTES.captures_iter(file_string) {
		// Check that capture group 2 exists — if it doesn't, it is matched
		// to an incomplete attribute selector (no value), rule block or comment.
		// Leave it as is.
		if capture.at(2).is_none() {
			continue;
		}

		let attribute_name: String = unescape_css_chars(capture.at(1).unwrap());
		let attribute_flag: &str = capture.at(6).unwrap_or("");
		let mut attribute_value: String = capture.at(4).unwrap().to_string();

		if !WHITELIST.get().unwrap().contains_key(&attribute_name) {
			continue;
		}

		// Do not process attribute selector if case-insensitive
		// flag has been set.
		if attribute_flag.to_lowercase().contains('i') {
			continue;
		}

		// Work out if value(s) are classes, IDs or selectors.
		let attribute_type_designation: &str =
			WHITELIST.get().unwrap().get(&attribute_name).unwrap();

		match attribute_type_designation {
			"id" | "class" => {
				super::analyse_string_of_tokens(
					&mut attribute_value,
					selectors,
					attribute_type_designation,
					Some(SelectorUsage::Style),
				);
			},
			"selector" => {
				attribute_value = unescape_css_chars(&attribute_value);
				analyse_css(&mut attribute_value, selectors, config);
			},
			"anchor" => {
				attribute_value = unescape_css_chars(&attribute_value);
				super::analyse_anchor_links(&mut attribute_value, selectors);
			},
			_ => {},
		}
	}
}

// Rewrite CSS attribute selectors.
pub fn rewrite_css_attributes(
	file_string: &mut String,
	selectors: &Selectors,
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

		// Attribute does not contain classes and/or IDs. Leave it as is.
		if !WHITELIST.get().unwrap().contains_key(&attribute_name) {
			return capture.at(0).unwrap().to_string();
		}

		// Do not process attribute selector if case-insensitive
		// flag has been set.
		if attribute_flag.to_lowercase().contains('i') {
			return capture.at(0).unwrap().to_string();
		}

		// Work out if value(s) are classes, IDs or selectors.
		let attribute_type_designation: &str =
			WHITELIST.get().unwrap().get(&attribute_name).unwrap();

		match attribute_type_designation {
			"id" | "class" => {
				super::rewrite_string_of_tokens(
					&mut attribute_value,
					selectors,
					attribute_type_designation,
				);
			},
			"selector" => {
				attribute_value = unescape_css_chars(&attribute_value);
				rewrite_css(&mut attribute_value, selectors, config);
			},
			"anchor" => {
				attribute_value = unescape_css_chars(&attribute_value);
				super::rewrite_anchor_links(&mut attribute_value, selectors);
			},
			_ => {},
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

// Analyse CSS functions.
pub fn analyse_css_functions(
	file_string: &mut str,
	selectors: &mut Selectors,
) {
	for capture in style_regex::CSS_FUNCTIONS.captures_iter(file_string) {
		// Check that capture group 4 (argument) exists
		if capture.at(4).is_none() {
			continue;
		}

		// let function_name: &str = capture.at(1).unwrap();
		let mut function_argument = capture.at(4).unwrap().to_string();

		// For now, only url is needed to be processed
		super::analyse_anchor_links(&mut function_argument, selectors);
	}
}

// Rewrite CSS functions.
pub fn rewrite_css_functions(
	file_string: &mut String,
	selectors: &Selectors,
) {
	*file_string = style_regex::CSS_FUNCTIONS.replace_all(file_string, |capture: &Captures| {
		// Check that capture group 4 (argument) exists
		if capture.at(4).is_none() {
			return capture.at(0).unwrap().to_owned();
		}

		let function_name: &str = capture.at(1).unwrap();
		let mut function_argument = capture.at(4).unwrap().to_string();

		// For now, only url is needed to be processed
		super::rewrite_anchor_links(&mut function_argument, selectors);

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
