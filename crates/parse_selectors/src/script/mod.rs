pub mod regexes;

use minify_selectors_utils::*;
use onig::*;

use crate::markup::html_attributes::WHITELIST;
use crate::markup::*;
use crate::script::regexes as script_regex;
use crate::style::*;




/// Analyse JS.
pub fn analyse_js(
	file_string: &mut str,
	selectors: &mut Selectors,
	config: &Config,
) {
	analyse_js_arguments(file_string, selectors, config);
	analyse_js_properties(file_string, selectors, config);
	super::analyse_prefixed_selectors(file_string, selectors);
}

/// Process JS.
pub fn rewrite_js(
	file_string: &mut String,
	selectors: &Selectors,
	config: &Config,
) {
	rewrite_js_arguments(file_string, selectors, config);
	rewrite_js_properties(file_string, selectors, config);
	super::rewrite_prefixed_selectors(file_string, selectors);
}

/// Analyse JS function arguments.
pub fn analyse_js_arguments(
	file_string: &mut str,
	selectors: &mut Selectors,
	config: &Config,
) {
	for capture in script_regex::JS_ARGUMENTS.captures_iter(file_string) {
		// Matched string is a multiline or single line comment
		// i.e. it does not have any further capture groups
		if capture.at(1).is_none() {
			continue;
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

				// Remove any additional backslash in JS selector strings.
				replacement_args = replacement_args.replace("\\\\", "\\");

				if !quote_type.is_empty() {
					super::analyse_css(&mut replacement_args, selectors, config);
				}
			},

			// Takes one argument, a string of classes (no period prefixed)
			// separated by spaces (if more than one) —
			".getElementsByClassName" => {
				// Checking that argument is a string
				if capture.at(4).is_some() {
					super::analyse_string_of_tokens(
						&mut replacement_args,
						selectors,
						"class",
						Some(SelectorUsage::Script),
					);
				}
				// TODO: handle expressions?
			},

			// Takes one argument, an ID (no hash prefixed).
			".getElementById" => {
				// Checking that argument is a string
				if capture.at(4).is_some() {
					super::analyse_string_of_tokens(
						&mut replacement_args,
						selectors,
						"id",
						Some(SelectorUsage::Script),
					);
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
				if WHITELIST.get().unwrap().contains_key(attribute_name) {
					if let Some(attribute_value) = function_args.next() {
						if attribute_value.at(3).is_some() {
							let mut replacement_value = attribute_value.at(3).unwrap().to_string();
							let attribute_type_designation: &str =
								WHITELIST.get().unwrap().get(attribute_name).unwrap();


							match attribute_type_designation {
								"id" | "class" => {
									super::analyse_string_of_tokens(
										&mut replacement_value,
										selectors,
										attribute_type_designation,
										Some(SelectorUsage::Script),
									);
								},

								"selector" => {
									super::analyse_css(&mut replacement_value, selectors, config);
								},

								"style" => {
									analyse_css_functions(&mut replacement_value, selectors);
								},

								"script" => {
									analyse_js(&mut replacement_value, selectors, config);
								},

								"anchor" => {
									super::analyse_anchor_links(&mut replacement_value, selectors);
								},

								_ => continue,
							};
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
							true => {
								super::analyse_html(
									&mut replacement_html,
									selectors,
									config,
									Some(SelectorUsage::Script),
								)
							},
							false => {
								analyse_html_attributes(
									&mut replacement_html,
									selectors,
									config,
									Some(SelectorUsage::Script),
								)
							},
						};
					}
				}
			},

			// Takes either only one argument or up to two arguments:
			// we are only ever interested in argument number 1.
			"window.open" | "window.location.assign" | "window.location.replace" => {
				if let Some(link) = super::get_function_arguments(&replacement_args).next() {
					let mut replacement_link = link.at(0).unwrap().to_string();
					super::analyse_anchor_links(&mut replacement_link, selectors);
				}
			},

			// Takes two or three arguments, the final argument which
			// is an optional URL is the one that we are interested in.
			"history.pushState" | "history.replaceState" => {
				if let Some(link) = super::get_function_arguments(&replacement_args).nth(2) {
					let mut replacement_link = link.at(0).unwrap().to_string();
					super::analyse_anchor_links(&mut replacement_link, selectors);
				}
			},

			// Takes one or more arguments, each argument is for
			// an individual class name (no period prefix).
			".classList.add"
			| ".classList.contains"
			| ".classList.remove"
			| ".classList.replace"
			| ".classList.toggle" => {
				super::analyse_string_of_arguments(
					&mut replacement_args,
					selectors,
					"class",
					Some(SelectorUsage::Script),
				);
			},

			_ => {},
		}
	}
}


/// Rewrite JS function arguments.
pub fn rewrite_js_arguments(
	file_string: &mut String,
	selectors: &Selectors,
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

				// Remove any additional backslash in JS selector strings.
				replacement_args = replacement_args.replace("\\\\", "\\");

				if !quote_type.is_empty() {
					super::rewrite_css(&mut replacement_args, selectors, config);
				}
			},

			// Takes one argument, a string of classes (no period prefixed)
			// separated by spaces (if more than one) —
			".getElementsByClassName" => {
				// Checking that argument is a string
				if capture.at(4).is_some() {
					super::rewrite_string_of_tokens(&mut replacement_args, selectors, "class");
				}
				// TODO: handle expressions?
			},

			// Takes one argument, an ID (no hash prefixed).
			".getElementById" => {
				// Checking that argument is a string
				if capture.at(4).is_some() {
					super::rewrite_string_of_tokens(&mut replacement_args, selectors, "id");
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
				if WHITELIST.get().unwrap().contains_key(attribute_name) {
					if let Some(attribute_value) = function_args.next() {
						if attribute_value.at(3).is_some() {
							let mut replacement_value = attribute_value.at(3).unwrap().to_string();
							let attribute_type_designation: &str =
								WHITELIST.get().unwrap().get(attribute_name).unwrap();


							match attribute_type_designation {
								"id" | "class" => {
									super::rewrite_string_of_tokens(
										&mut replacement_value,
										selectors,
										attribute_type_designation,
									);
								},

								"selector" => {
									super::rewrite_css(&mut replacement_value, selectors, config);
								},

								"style" => {
									rewrite_css_functions(&mut replacement_value, selectors);
								},

								"script" => {
									rewrite_js(&mut replacement_value, selectors, config);
								},

								"anchor" => {
									super::rewrite_anchor_links(&mut replacement_value, selectors);
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
							true => super::rewrite_html(&mut replacement_html, selectors, config),
							false => {
								rewrite_html_attributes(&mut replacement_html, selectors, config)
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
					super::rewrite_anchor_links(&mut replacement_link, selectors);
					replacement_args =
						replacement_args.replace(link.at(0).unwrap(), &replacement_link);
				}
			},

			// Takes two or three arguments, the final argument which
			// is an optional URL is the one that we are interested in.
			"history.pushState" | "history.replaceState" => {
				if let Some(link) = super::get_function_arguments(&replacement_args).nth(2) {
					let mut replacement_link = link.at(0).unwrap().to_string();
					super::rewrite_anchor_links(&mut replacement_link, selectors);
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
				super::rewrite_string_of_arguments(&mut replacement_args, selectors, "class");
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

/// Analyse JS property operation values.
pub fn analyse_js_properties(
	file_string: &mut str,
	selectors: &mut Selectors,
	config: &Config,
) {
	for capture in script_regex::JS_PROPERTIES.captures_iter(file_string) {
		// Matched string is a multiline or single line comment
		// i.e. it does not have any further capture groups
		if capture.at(1).is_none() {
			continue;
		}

		let mut property_value: String = unescape_js_chars(capture.at(4).unwrap());
		let property_name: &str = capture.at(1).unwrap();

		if property_name == ".innerHTML" || property_name == ".outerHTML" {
			if property_value.contains("</body>") {
				super::analyse_html(
					&mut property_value,
					selectors,
					config,
					Some(SelectorUsage::Script),
				);
			} else {
				analyse_html_attributes(
					&mut property_value,
					selectors,
					config,
					Some(SelectorUsage::Script),
				);
			}
		} else if property_name == "window.location"
			|| property_name == "window.location.href"
			|| property_name == "window.location.hash"
		{
			super::analyse_anchor_links(&mut property_value, selectors);
		} else if property_name == ".id" {
			super::analyse_string_of_tokens(
				&mut property_value,
				selectors,
				"id",
				Some(SelectorUsage::Script),
			);
		} else if property_name == ".className" || property_name.starts_with(".classList") {
			super::analyse_string_of_tokens(
				&mut property_value,
				selectors,
				"class",
				Some(SelectorUsage::Script),
			);
		}
	}
}

/// Rewrite JS property operation values.
pub fn rewrite_js_properties(
	file_string: &mut String,
	selectors: &Selectors,
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

		if property_name == ".innerHTML" || property_name == ".outerHTML" {
			if property_value.contains("</body>") {
				super::rewrite_html(&mut property_value, selectors, config);
			} else {
				rewrite_html_attributes(&mut property_value, selectors, config);
			}
		} else if property_name == "window.location"
			|| property_name == "window.location.href"
			|| property_name == "window.location.hash"
		{
			super::rewrite_anchor_links(&mut property_value, selectors);
		} else if property_name == ".id" {
			super::rewrite_string_of_tokens(&mut property_value, selectors, "id");
		} else if property_name == ".className" || property_name.starts_with(".classList") {
			super::rewrite_string_of_tokens(&mut property_value, selectors, "class");
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
