pub mod regexes;

use minify_selectors_utils::*;
use onig::*;

use crate::markup::whitelist::*;
use crate::markup::*;
use crate::script::regexes as script_regex;
use crate::style::*;




/// Process Javascript.
pub fn process_js(
	file_string: &mut String,
	selectors: &mut Selectors,
	config: &Config,
) {
	process_js_arguments(file_string, selectors, config);
	process_js_properties(file_string, selectors, config);
	super::process_prefixed_selectors(file_string, selectors, config);
}

/// Process JS function arguments.
pub fn process_js_arguments(
	file_string: &mut String,
	selectors: &mut Selectors,
	config: &Config,
) {
	*file_string = script_regex::JS_ARGUMENTS.replace_all(file_string, |capture: &Captures| {
		// Matched string is a multiline or single line comment
		// i.e. it does not have any further capture groups
		if capture.at(1).is_none() {
			return capture.at(0).unwrap().to_string();
		}

		let mut replacement_args: String = unescape_js_chars(capture.at(3).unwrap());
		let mut function = capture.at(1).unwrap().to_owned();
		function.retain(|c| !c.is_whitespace());

		// Work out function call and its argument pattern:
		match function.as_str() {
			// Takes one argument, an CSS selector string.
			".querySelector" | ".querySelectorAll" | ".closest" | ".matches" => {
				// FIXME: rudimentary way to check if arg is a immediate
				// string value rather than some expression.
				let quote_type: &str = match replacement_args.chars().next() {
					Some('\'') => "'",
					Some('"') => "\"",
					Some('`') => "`",
					_ => "",
				};

				// Remove additional backslash in JS selector strings.
				replacement_args = replacement_args.replace("\\\\", "\\");

				if !quote_type.is_empty() {
					super::process_css(&mut replacement_args, selectors, config);
				}
			},

			// Takes one argument, a string of classes (no period prefixed)
			// separated by spaces (if more than one) —
			".getElementsByClassName" => {
				// Checking that argument is a string
				if capture.at(4).is_some() {
					super::process_string_of_tokens(
						&mut replacement_args,
						selectors,
						config,
						"class",
					);
				}
				// TODO: handle expressions?
			},

			// Takes one argument, an ID (no hash prefixed).
			".getElementById" => {
				// Checking that argument is a string
				if capture.at(4).is_some() {
					super::process_string_of_tokens(&mut replacement_args, selectors, config, "id");
				};
				// TODO: handle expressions?
			},

			// Takes two arguments: attribute name and value,
			// process value if attribute is whitelisted.
			".setAttribute" => {
				// Go over the (two) function arguments
				let mut function_args = super::get_function_arguments(&replacement_args);

				// Check first arg in function, without the string delimiters
				// and then trimming any whitespace off ends.
				let attribute_name: &str = function_args.next().unwrap().at(3).unwrap_or("").trim();

				// Check first argument is an known attribute which its value will have
				// classses or an id. If it is not, leave value as is (second argument).
				if ATTRIBUTES_WHITELIST.contains_key(attribute_name) {
					if let Some(attribute_value) = function_args.next() {
						if attribute_value.at(3).is_some() {
							let mut replacement_value = attribute_value.at(3).unwrap().to_string();
							let attribute_type_designation: &str =
								ATTRIBUTES_WHITELIST.get(attribute_name).unwrap();


							match attribute_type_designation {
								"id" | "class" => {
									super::process_string_of_tokens(
										&mut replacement_value,
										selectors,
										config,
										attribute_type_designation,
									);
								},

								"selector" => {
									super::process_css(&mut replacement_value, selectors, config);
								},

								"style" => {
									process_css_functions(
										&mut replacement_value,
										selectors,
										config,
									);
								},

								"script" => {
									process_js(&mut replacement_value, selectors, config);
								},

								"anchor" => {
									super::process_anchor_links(
										&mut replacement_value,
										selectors,
										config,
									);
								},

								_ => return replacement_value,
							};

							replacement_args = replacement_args
								.replace(attribute_value.at(3).unwrap(), &replacement_value);
						}
					}
				}
			},

			// Takes two arguments: position and html,
			// we are only interested in the latter argument.
			".insertAdjacentHTML" => {
				if let Some(html) = super::get_function_arguments(&replacement_args).nth(1) {
					// Third capture group, which should be just the string (without the
					// delimeters).
					if html.at(3).is_some() {
						let mut replacement_html = html.at(3).unwrap().to_string();

						match html.at(3).unwrap().contains("</body>") {
							true => super::process_html(&mut replacement_html, selectors, config),
							false => {
								process_html_attributes(&mut replacement_html, selectors, config)
							},
						};

						replacement_args =
							replacement_args.replace(html.at(3).unwrap(), &replacement_html);
					}
				}
			},

			// Takes either only one argument or up to two arguments:
			// we are only ever interested in argument number 1.
			"window.open" | "window.location.assign" | "window.location.replace" => {
				if let Some(link) = super::get_function_arguments(&replacement_args).next() {
					let mut replacement_link = link.at(0).unwrap().to_string();
					super::process_anchor_links(&mut replacement_link, selectors, config);
					replacement_args =
						replacement_args.replace(link.at(0).unwrap(), &replacement_link);
				}
			},

			// Takes two or three arguments, the final argument which
			// is an optional URL is the one that we are interested in.
			"history.pushState" | "history.replaceState" => {
				if let Some(link) = super::get_function_arguments(&replacement_args).nth(2) {
					let mut replacement_link = link.at(0).unwrap().to_string();
					super::process_anchor_links(&mut replacement_link, selectors, config);
					replacement_args =
						replacement_args.replace(link.at(0).unwrap(), &replacement_link);
				}
			},

			// Takes one or more arguments, each argument is for
			// an individual class name (no period prefix).
			".classList.add"
			| ".classList.contains"
			| ".classList.remove"
			| ".classList.replace"
			| ".classList.toggle" => {
				super::process_string_of_arguments(
					&mut replacement_args,
					selectors,
					config,
					"class",
				);
			},

			_ => {},
		}

		format!(
			"{function}{join}{arguments}",
			function = capture.at(1).unwrap(),
			join = capture.at(2).unwrap(),
			arguments = replacement_args
		)
	});
}

/// Process JS property operation values.
pub fn process_js_properties(
	file_string: &mut String,
	selectors: &mut Selectors,
	config: &Config,
) {
	*file_string = script_regex::JS_PROPERTIES.replace_all(file_string, |capture: &Captures| {
		// Matched string is a multiline or single line comment
		// i.e. it does not have any further capture groups
		if capture.at(1).is_none() {
			return capture.at(0).unwrap().to_string();
		}

		let mut property_value: String = unescape_js_chars(capture.at(4).unwrap());
		let property_name: &str = capture.at(1).unwrap();

		match property_name {
			".id" => {
				super::process_string_of_tokens(&mut property_value, selectors, config, "id");
			},
			".className" => {
				super::process_string_of_tokens(&mut property_value, selectors, config, "class");
			},
			".innerHTML" | ".outerHTML" => {
				if property_value.contains("</body>") {
					super::process_html(&mut property_value, selectors, config);
				} else {
					process_html_attributes(&mut property_value, selectors, config);
				}
			},
			"window.location.hash" | "window.location.href" | "window.location" => {
				super::process_anchor_links(&mut property_value, selectors, config);
			},
			_ if property_name.starts_with(".classList") => {
				super::process_string_of_tokens(&mut property_value, selectors, config, "class");
			},
			_ => {},
		}

		format!(
			"{name}{operator}{value}",
			name = property_name,
			operator = capture.at(3).unwrap(),
			value = property_value,
		)
	});
}

// Converts any escaped chars in JS substring to UTF8 char.
pub fn unescape_js_chars(js_string: &str) -> String {
	let mut unescaped = js_string.to_string();

	if script_regex::ESCAPED_JS_CHARS.find(js_string).is_none() {
		return unescaped;
	}

	unescaped = script_regex::ESCAPED_JS_CHARS.replace_all(&unescaped, |capture: &Captures| {
		String::from(
			if capture.at(1).is_some() {
				char::from_u32(
					u32::from_str_radix(capture.at(1).unwrap().strip_prefix('%').unwrap(), 16)
						.unwrap(),
				)
				.unwrap()
			} else if capture.at(2).is_some() {
				char::from_u32(
					u32::from_str_radix(capture.at(2).unwrap().strip_prefix("\\x").unwrap(), 16)
						.unwrap(),
				)
				.unwrap()
			} else if capture.at(3).is_some() {
				char::from_u32(
					u32::from_str_radix(capture.at(3).unwrap().strip_prefix("\\u").unwrap(), 16)
						.unwrap(),
				)
				.unwrap()
			} else if capture.at(4).is_some() {
				char::from_u32(
					u32::from_str_radix(
						capture
							.at(4)
							.unwrap()
							.strip_prefix("\\u{")
							.unwrap()
							.strip_suffix('}')
							.unwrap(),
						16,
					)
					.unwrap(),
				)
				.unwrap()
			} else {
				panic!("Not any of the known capture groups");
			},
		)
	});

	unescaped
}
