pub mod html_attributes;
pub mod named_char_refs;
pub mod regexes;

use minify_selectors_utils::*;
use onig::*;

use crate::markup::html_attributes::WHITELIST;
use crate::markup::named_char_refs::ENTITIES;
use crate::markup::regexes as markup_regex;




/// Analyse HTML.
pub fn analyse_html(
	file_string: &mut str,
	selectors: &mut Selectors,
	config: &Config,
	usage: Option<SelectorUsage>,
) {
	analyse_html_attributes(file_string, selectors, config, usage);
	analyse_html_scripts(file_string, selectors, config);
	analyse_html_styles(file_string, selectors, config);
	super::analyse_prefixed_selectors(file_string, selectors);
}

/// Rewrite HTML.
pub fn rewrite_html(
	file_string: &mut String,
	selectors: &Selectors,
	config: &Config,
) {
	rewrite_html_attributes(file_string, selectors, config);
	rewrite_html_scripts(file_string, selectors, config);
	rewrite_html_styles(file_string, selectors, config);
	super::rewrite_prefixed_selectors(file_string, selectors);
}

/// Analyse HTML attributes.
pub fn analyse_html_attributes(
	file_string: &mut str,
	selectors: &mut Selectors,
	config: &Config,
	usage: Option<SelectorUsage>,
) {
	for capture in markup_regex::HTML_ATTRIBUTES.captures_iter(file_string) {
		// Matched string is a <code>/<script>/<style> element or a HTML comment.
		if capture.at(1).is_none() && capture.at(0).unwrap().starts_with("<code") {
			// Check for attributes to encode on the opening tag.
			let code_element = capture
				.at(0)
				.unwrap()
				.strip_prefix("<code")
				.unwrap()
				.split_once('>')
				.unwrap();
			let mut code_tag_attributes = code_element.0.to_string();
			analyse_html_attributes(&mut code_tag_attributes, selectors, config, usage);
			continue;
		}

		// Matched string is a <script>/<style> element or HTML comment, leave as is.
		if capture.at(1).is_none() && !capture.at(0).unwrap().starts_with("<code") {
			continue;
		}

		// Attribute does not contain classes and/or IDs.
		// Leave it as is.
		if !WHITELIST
			.get()
			.unwrap()
			.contains_key(&capture.at(1).unwrap().to_ascii_lowercase())
		{
			continue;
		}

		let attribute_name: &str = capture.at(1).unwrap();
		let mut attribute_value: String = unescape_html_chars(capture.at(4).unwrap());

		// Work out if value(s) are classes, IDs, selectors, etc.
		let attribute_type_designation: &str = WHITELIST
			.get()
			.unwrap()
			.get(&attribute_name.to_ascii_lowercase())
			.unwrap();

		match attribute_type_designation {
			"id" | "class" => {
				super::analyse_string_of_tokens(
					&mut attribute_value,
					selectors,
					attribute_type_designation,
					if usage.is_none() {
						Some(
							if attribute_type_designation == "id" {
								SelectorUsage::MarkupId
							} else {
								SelectorUsage::MarkupClass
							},
						)
					} else {
						usage
					},
				);
			},

			"selector" => {
				super::analyse_css(&mut attribute_value, selectors, config);
			},

			"style" => {
				super::analyse_css_functions(&mut attribute_value, selectors);
			},

			"script" => {
				super::analyse_js(&mut attribute_value, selectors, config);
			},

			"anchor" => {
				super::analyse_anchor_links(&mut attribute_value, selectors);
			},

			_ => continue,
		}
	}
}

/// Rewrite HTML attributes.
pub fn rewrite_html_attributes(
	file_string: &mut String,
	selectors: &Selectors,
	config: &Config,
) {
	*file_string = markup_regex::HTML_ATTRIBUTES.replace_all(file_string, |capture: &Captures| {
		// Matched string is a <code>/<script>/<style> element or a HTML comment.
		if capture.at(1).is_none() && capture.at(0).unwrap().starts_with("<code") {
			// Check for attributes to encode on the opening tag.
			let code_element = capture
				.at(0)
				.unwrap()
				.strip_prefix("<code")
				.unwrap()
				.split_once('>')
				.unwrap();

			let mut code_tag_attributes = code_element.0.to_string();

			rewrite_html_attributes(&mut code_tag_attributes, selectors, config);

			return format!(
				"<code{attributes}>{inner_html}",
				attributes = code_tag_attributes,
				inner_html = code_element.1,
			);
		}

		// Matched string is a <script>/<style> element or HTML comment, leave as is.
		if capture.at(1).is_none() && !capture.at(0).unwrap().starts_with("<code") {
			return capture.at(0).unwrap().to_string();
		}

		// Attribute does not contain classes and/or IDs.
		// Leave it as is.
		if !WHITELIST
			.get()
			.unwrap()
			.contains_key(&capture.at(1).unwrap().to_ascii_lowercase())
		{
			return capture.at(0).unwrap().to_string();
		}

		let attribute_name: &str = capture.at(1).unwrap();
		let attribute_quote: &str = capture.at(3).unwrap_or("");
		let mut attribute_value: String = unescape_html_chars(capture.at(4).unwrap());

		// Work out if value(s) are classes, IDs, selectors, etc.
		let attribute_type_designation: &str = WHITELIST
			.get()
			.unwrap()
			.get(&attribute_name.to_ascii_lowercase())
			.unwrap();

		match attribute_type_designation {
			"id" | "class" => {
				super::rewrite_string_of_tokens(
					&mut attribute_value,
					selectors,
					attribute_type_designation,
				);
			},

			"selector" => {
				super::rewrite_css(&mut attribute_value, selectors, config);
			},

			"style" => {
				super::rewrite_css_functions(&mut attribute_value, selectors);
			},

			"script" => {
				super::rewrite_js(&mut attribute_value, selectors, config);
			},

			"anchor" => {
				super::rewrite_anchor_links(&mut attribute_value, selectors);
			},

			_ => {
				return capture.at(0).unwrap().to_string();
			},
		}

		format!(
			"{attribute}{join}{quote}{value}",
			attribute = attribute_name,
			join = capture.at(2).unwrap(),
			value = attribute_value,
			quote = attribute_quote,
		)
	});
}

// Analyse embedded scripts in HTML.
pub fn analyse_html_scripts(
	file_string: &mut str,
	selectors: &mut Selectors,
	config: &Config,
) {
	for capture in markup_regex::HTML_SCRIPT_ELEMENT.captures_iter(file_string) {
		let mut embedded_script = capture.at(2).unwrap().to_string();
		super::analyse_js(&mut embedded_script, selectors, config);
	}
}

// Rewrite embedded scripts in HTML.
pub fn rewrite_html_scripts(
	file_string: &mut String,
	selectors: &Selectors,
	config: &Config,
) {
	*file_string =
		markup_regex::HTML_SCRIPT_ELEMENT.replace_all(file_string, |capture: &Captures| {
			let mut embedded_script = capture.at(2).unwrap().to_string();
			super::rewrite_js(&mut embedded_script, selectors, config);

			format!(
				"{tag_open}{script}{tag_close}",
				tag_open = capture.at(1).unwrap(),
				script = embedded_script,
				tag_close = capture.at(3).unwrap()
			)
		});
}

// Analyse embedded styles in HTML.
pub fn analyse_html_styles(
	file_string: &mut str,
	selectors: &mut Selectors,
	config: &Config,
) {
	for capture in markup_regex::HTML_STYLE_ELEMENT.captures_iter(file_string) {
		let mut embedded_style = capture.at(2).unwrap().to_string();
		super::analyse_css(&mut embedded_style, selectors, config);
	}
}

// Rewrite embedded styles in HTML.
pub fn rewrite_html_styles(
	file_string: &mut String,
	selectors: &Selectors,
	config: &Config,
) {
	*file_string =
		markup_regex::HTML_STYLE_ELEMENT.replace_all(file_string, |capture: &Captures| {
			let mut embedded_style = capture.at(2).unwrap().to_string();
			super::rewrite_css(&mut embedded_style, selectors, config);

			format!(
				"{tag_open}{styles}{tag_close}",
				tag_open = capture.at(1).unwrap(),
				styles = embedded_style,
				tag_close = capture.at(3).unwrap(),
			)
		});
}

// Convert any escaped chars in HTML substring to UTF8 char.
pub fn unescape_html_chars(substring: &str) -> String {
	let mut unescaped = substring.to_string();

	if markup_regex::ESCAPED_HTML_CHARS.find(substring).is_none() {
		return unescaped;
	}

	unescaped = markup_regex::ESCAPED_HTML_CHARS.replace_all(&unescaped, |capture: &Captures| {
		if capture.at(1).is_some() {
			return String::from(
				char::from_u32(
					u32::from_str_radix(
						capture
							.at(1)
							.unwrap()
							.strip_prefix("&#x")
							.unwrap()
							.strip_suffix(';')
							.unwrap(),
						16,
					)
					.unwrap(),
				)
				.unwrap(),
			);
		} else if capture.at(2).is_some() {
			return String::from(
				char::from_u32(
					capture
						.at(2)
						.unwrap()
						.strip_prefix("&#")
						.unwrap()
						.strip_suffix(';')
						.unwrap()
						.parse::<u32>()
						.unwrap(),
				)
				.unwrap(),
			);
		} else if capture.at(3).is_some() {
			if !ENTITIES.contains_key(capture.at(3).unwrap()) {
				return capture
					.at(3)
					.unwrap()
					.to_string()
					.replace(';', "\\3B")
					.replace('&', "\\26");
			}
			return ENTITIES.get(capture.at(3).unwrap()).unwrap().to_string();
		}

		panic!("Not any of the known capture groups");
	});

	unescaped
}
