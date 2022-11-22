pub mod regexes;
pub mod whitelist;

use minify_selectors_utils::*;
use onig::*;

use crate::markup::regexes as markup_regex;
use crate::markup::whitelist::*;




/// Process HTML attributes.
pub fn process_html_attributes(
	file_string: &mut String,
	selectors: &mut Selectors,
	config: &Config,
) {
	*file_string = markup_regex::HTML_ATTRIBUTES.replace_all(file_string, |capture: &Captures| {
		// Matched string is a <code>/<script>/<style> element
		// or a HTML comment.
		if capture.at(1).is_none() {
			return match capture.at(0).unwrap().starts_with("<code") {
				// <script>/<style> element or HTML comment, leave as is.
				false => capture.at(0).unwrap().to_string(),

				// <code> element.
				// Check for attributes to encode on the opening tag.
				true => {
					let code_element = capture
						.at(0)
						.unwrap()
						.strip_prefix("<code")
						.unwrap()
						.split_once('>')
						.unwrap();

					let mut code_tag_attributes = code_element.0.to_string();

					process_html_attributes(&mut code_tag_attributes, selectors, config);

					format!(
						"<code{attributes}>{inner_html}",
						attributes = code_tag_attributes,
						inner_html = code_element.1,
					)
				},
			};
		}

		let attribute_name: &str = capture.at(1).unwrap();
		let attribute_quote: &str = capture.at(3).unwrap_or("");
		let mut attribute_value: String = unescape_html_chars(capture.at(4).unwrap());

		// Attributes whitelist of which its values should be processed.
		match ATTRIBUTES_WHITELIST.contains_key(&attribute_name.to_ascii_lowercase()) {
			true => {
				// Work out if value(s) are classes, IDs or selectors.
				let attribute_type_designation: &str = ATTRIBUTES_WHITELIST
					.get(&attribute_name.to_ascii_lowercase())
					.unwrap();

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
						super::process_css(&mut attribute_value, selectors, config);
					},

					"style" => {
						super::process_css_functions(&mut attribute_value, selectors, config);
					},

					"script" => {
						super::process_js(&mut attribute_value, selectors, config);
					},

					"anchor" => {
						super::process_anchor_links(&mut attribute_value, selectors, config);
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
			},

			// Attribute does not contain classes and/or IDs.
			// Leave it as is.
			false => {
				return capture.at(0).unwrap().to_string();
			},
		}
	});
}

// Process embedded scripts in HTML.
pub fn process_html_scripts(
	file_string: &mut String,
	selectors: &mut Selectors,
	config: &Config,
) {
	// Create subset string(s) to process each <script> embed.
	*file_string =
		markup_regex::HTML_SCRIPT_ELEMENT.replace_all(file_string, |capture: &Captures| {
			let mut embedded_script = capture.at(2).unwrap().to_string();
			super::process_js(&mut embedded_script, selectors, config);

			format!(
				"{tag_open}{script}{tag_close}",
				tag_open = capture.at(1).unwrap(),
				script = embedded_script,
				tag_close = capture.at(3).unwrap()
			)
		});
}

// Processing embedded styles in HTML.
pub fn process_html_styles(
	file_string: &mut String,
	selectors: &mut Selectors,
	config: &Config,
) {
	// Create subset string(s) to process each <style> embed.
	*file_string =
		markup_regex::HTML_STYLE_ELEMENT.replace_all(file_string, |capture: &Captures| {
			let mut embedded_style = capture.at(2).unwrap().to_string();
			super::process_css(&mut embedded_style, selectors, config);

			format!(
				"{tag_open}{styles}{tag_close}",
				tag_open = capture.at(1).unwrap(),
				styles = embedded_style,
				tag_close = capture.at(3).unwrap(),
			)
		});
}

/// Process HTML.
pub fn process_html(
	file_string: &mut String,
	selectors: &mut Selectors,
	config: &Config,
) {
	process_html_attributes(file_string, selectors, config);
	process_html_scripts(file_string, selectors, config);
	process_html_styles(file_string, selectors, config);
	super::process_prefixed_selectors(file_string, selectors, config);
}

// Convert any escaped chars in HTML substring to UTF8 char.
pub fn unescape_html_chars(substring: &str) -> String {
	let mut unescaped = substring.to_string();

	if markup_regex::ESCAPED_HTML_CHARS.find(substring).is_none() {
		return unescaped;
	}

	unescaped = markup_regex::ESCAPED_HTML_CHARS.replace_all(&unescaped, |capture: &Captures| {
		String::from(
			if capture.at(1).is_some() {
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
				.unwrap()
			} else if capture.at(2).is_some() {
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
				.unwrap()
			// } else if capture.at(3).is_some() {
				// TODO: named character references
				// todo!();
			} else {
				panic!("Not any of the known capture groups");
			},
		)
	});

	unescaped
}
