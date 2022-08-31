use lazy_static::lazy_static;
use onig::*;
use std::collections::HashMap;

pub mod regexes;




lazy_static! {
	// HTML attributes which its values will contain classes/ids
	static ref ATTRIBUTES_WHITELIST: HashMap<String, String> = HashMap::from([
		// div id="foo"
		(String::from("id"), String::from("id")),

		// class="foo bar baz"
		(String::from("class"), String::from("class")),

		// <div id="foo"></div>
		// <div aria-describedby="foo"></div>
		(String::from("aria-describedby"), String::from("id")),

		// <div id="foo"></div>
		// <div aria-labelledby="foo"></div>
		(String::from("aria-labelledby"), String::from("id")),

		// <input id="foo">
		// <label for="foo"></label>
		(String::from("for"), String::from("id")),

		// <form id="foo">
		// <input form="foo">
		(String::from("form"), String::from("id")),

		// <th id="foo"></th>
		// <td headers="foo"></td>
		(String::from("headers"), String::from("id")),

		// <div id="foo"></div>
		// <div itemref="foo bar"></div>
		// <div id="bar"></div>
		(String::from("itemref"), String::from("id")),

		// <input list="foo">
		// <datalist id="foo"></datalist>
		(String::from("list"), String::from("id")),

		// <a href="/#foo"></a>
		(String::from("href"), String::from("anchor")),
	]);
}




pub fn from_css(
	file_string: &mut String,
	selectors: &mut HashMap<String, String>,
	index: &mut usize,
	alphabet: &[char],
) {
	process_css(
		file_string,
		selectors,
		index,
		alphabet
	)
}

pub fn from_html(
	file_string: &mut String,
	selectors: &mut HashMap<String, String>,
	index: &mut usize,
	alphabet: &[char],
) {
	process_html(
		file_string,
		selectors,
		index,
		alphabet
	);
}

pub fn from_js(
	file_string: &mut String,
	selectors: &mut HashMap<String, String>,
	index: &mut usize,
	alphabet: &[char],
) {
	process_js(
		file_string,
		selectors,
		index,
		alphabet
	);
}




/// Fetch encoded selector from selectors hashmap.
/// If selector is new and unique, generate one for it
/// and add it to selectors.
fn get_encoded_selector(
	selector: &str,
	selectors: &mut HashMap<String, String>,
	index: &mut usize,
	alphabet: &[char],
) -> String {
	match selectors.contains_key(selector) {
		true => {
			selectors
				.get_key_value(selector)
				.unwrap().1.to_string()
		},

		false => {
			let encoded_selector: String = encode_selector::to_radix(index, alphabet);
			*index += 1;

			selectors.insert(
				selector.to_owned(),
				encoded_selector.clone()
			);

			encoded_selector
		}
	}
}

fn process_css(
	file_string: &mut String,
	selectors: &mut HashMap<String, String>,
	index: &mut usize,
	alphabet: &[char],
) {
	process_css_selectors(
		file_string,
		selectors,
		index,
		alphabet
	);

	process_css_attributes(
		file_string,
		selectors,
		index,
		alphabet
	);

	process_prefixed_selectors(
		file_string,
		selectors,
		index,
		alphabet
	);
}

/// Process HTML.
fn process_html(
	file_string: &mut String,
	selectors: &mut HashMap<String, String>,
	index: &mut usize,
	alphabet: &[char],
) {
	process_html_attributes(
		file_string,
		selectors,
		index,
		alphabet
	);

	// Processing any embedded scripts
	// Create subset string(s) to process <script> embeds
	*file_string = regexes::HTML_SCRIPT_ELEMENT.replace_all(
		file_string,
		|capture: &Captures| {
			let mut embedded_script = capture.at(2).unwrap().to_string();

			process_js(
				&mut embedded_script,
				selectors,
				index,
				alphabet
			);

			format!(
				"{tag_open}{script}{tag_close}",
				tag_open = capture.at(1).unwrap(),
				script = embedded_script,
				tag_close = capture.at(3).unwrap()
			)
		}
	);

	// Processing any embedded styles
	// Create subset string(s) to process <style> embeds
	*file_string = regexes::HTML_STYLE_ELEMENT.replace_all(
		file_string,
		|capture: &Captures| {
			let mut embedded_style = capture.at(2).unwrap().to_string();

			process_css(
				&mut embedded_style,
				selectors,
				index,
				alphabet
			);

			format!(
				"{tag_open}{styles}{tag_close}",
				tag_open = capture.at(1).unwrap(),
				styles = embedded_style,
				tag_close = capture.at(3).unwrap()
			)
		}
	);

	process_prefixed_selectors(
		file_string,
		selectors,
		index,
		alphabet
	);
}

/// Process Javascript.
fn process_js(
	file_string: &mut String,
	selectors: &mut HashMap<String, String>,
	index: &mut usize,
	alphabet: &[char],
) {
	process_js_arguments(
		file_string,
		selectors,
		index,
		alphabet,
	);

	process_js_properties(
		file_string,
		selectors,
		index,
		alphabet,
	);

	process_prefixed_selectors(
		file_string,
		selectors,
		index,
		alphabet
	);
}




/// Process minify-selectors specific prefixed selectors.
fn process_prefixed_selectors(
	file_string: &mut String,
	selectors: &mut HashMap<String, String>,
	index: &mut usize,
	alphabet: &[char],
) {
	*file_string = regexes::PREFIXED_SELECTORS.replace_all(
		file_string,
		|capture: &Captures| {
			let mut placeholder_value = capture.at(3).unwrap().trim().to_string();

			match capture.at(2) {
				Some("class") => {
					placeholder_value = get_encoded_selector(
						&format!(".{}", placeholder_value),
						selectors,
						index,
						alphabet
					);
				},

				Some("id") => {
					placeholder_value = get_encoded_selector(
						&format!("#{}", placeholder_value),
						selectors,
						index,
						alphabet
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

				// No context provided, meaning
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
							index,
							alphabet
						)
					);
				},
			}

			placeholder_value
		}
	);
}

/// Process classes and IDs in CSS file/embed or as a
/// CSS selector string.
fn process_css_selectors(
	file_string: &mut String,
	selectors: &mut HashMap<String, String>,
	index: &mut usize,
	alphabet: &[char],
) {
	*file_string = regexes::CSS_SELECTORS.replace_all(
		file_string,
		|capture: &Captures| {
			// Check that capture group 2 exists,
			// i.e. matched to a class/id name and not an attribute selector,
			// rule block or comments — which does not have this group.
			if capture.at(2).is_some() {
				return format!(
					"{prefix}{identifier}",
					prefix = &capture.at(1).unwrap(),
					identifier = get_encoded_selector(
						capture.at(0).unwrap(),
						selectors,
						index,
						alphabet
					)
				);
			}
			// Matched to an attribute selector, rule block or comment.
			// Leave it as is.
			capture.at(0).unwrap().to_owned()
		}
	);
}

// Process CSS attribute selectors.
fn process_css_attributes(
	file_string: &mut String,
	selectors: &mut HashMap<String, String>,
	index: &mut usize,
	alphabet: &[char],
) {
	*file_string = regexes::CSS_ATTRIBUTES.replace_all(
		file_string,
		|capture: &Captures| {
			// Check that capture group 2 exists, if it doesn't it is
			// matched to an attribute selector, rule block or comment.
			// Leave it as is.
			if capture.at(2).is_none() {
				return capture.at(0).unwrap().to_owned();
			}

			let attribute_name: &str = capture.at(1).unwrap();
			let attribute_quote_type: &str = capture.at(3).unwrap_or("");
			let attribute_flag: &str = capture.at(5).unwrap_or("");
			let mut attribute_value = capture.at(4).unwrap().to_string();


			if ATTRIBUTES_WHITELIST.contains_key(attribute_name) {
				// Do not process attribute selector if case-insensitive
				// flag has been set.
				if attribute_flag.to_lowercase().ends_with('i') {
					return capture.at(0).unwrap().to_string();
				}

				// Work out if value(s) are classes, ids or selectors.
				let attribute_type_designation: &str = ATTRIBUTES_WHITELIST
					.get(attribute_name)
					.unwrap();

				match attribute_type_designation {
					"id" | "class" => {
						process_string_of_tokens(
							&mut attribute_value,
							selectors,
							index,
							alphabet,
							attribute_type_designation
						);
					},
					"selector" => {
						process_css(
							&mut attribute_value,
							selectors,
							index,
							alphabet
						);
					},
					_ => {},
				}
			} else {
				// Attribute does not contain classes and/or ids.
				// Leave it as is.
				return capture.at(0).unwrap().to_string();
			}

			format!(
				"[{attribute}{operator}{quote}{value}{quote}{flag}]",
				attribute = attribute_name,
				operator = capture.at(2).unwrap(),
				quote = attribute_quote_type,
				value = attribute_value,
				flag = attribute_flag,
			)
		}
	);
}

/// Process HTML attributes.
fn process_html_attributes(
	file_string: &mut String,
	selectors: &mut HashMap<String, String>,
	index: &mut usize,
	alphabet: &[char],
) {
	*file_string = regexes::HTML_ATTRIBUTES.replace_all(
		file_string,
		|capture: &Captures| {
			// Matched string is a <code>/<script>/<style> element
			// or a HTML comment.
			if capture.at(1).is_none() {
				return match capture.at(0).unwrap().starts_with("<code") {
					// <script>/<style> element or HTML comment, leave as is.
					false => capture.at(0).unwrap().to_string(),

					// <code> element.
					// Check for attributes to encode on the opening tag.
					true => {
						let code_element = capture.at(0)
							.unwrap()
							.strip_prefix("<code")
							.unwrap()
							.split_once('>')
							.unwrap();

						let mut code_tag_attributes = code_element.0.to_string();

						process_html_attributes(
							&mut code_tag_attributes,
							selectors,
							index,
							alphabet,
						);

						format!(
							"<code{attributes}>{inner_html}",
							attributes = code_tag_attributes,
							inner_html = code_element.1,
						)
					},
				}
			}

			let attribute_name: &str = capture.at(1).unwrap();
			let attribute_quote: &str = capture.at(4).unwrap_or("");
			let mut attribute_value: String = capture.at(3).unwrap().to_string();

			// Attributes whitelist of which its
			// values should be processed.
			match ATTRIBUTES_WHITELIST.contains_key(&attribute_name.to_ascii_lowercase()) {
				true => {
					// Work out if value(s) are classes, ids or selectors.
					let attribute_type_designation: &str = ATTRIBUTES_WHITELIST
						.get(capture.at(1).unwrap())
						.unwrap();

					// attribute_value will need to be cleaned up, as 'regexes::HTML_ATTRIBUTES'
					// regex will capture the opening quote if it has been used.
					if !attribute_quote.is_empty() {
						attribute_value = attribute_value
							.strip_prefix(attribute_quote)
							.unwrap()
							.to_string();
					}

					match attribute_type_designation {
						"id" | "class" => {
							process_string_of_tokens(
								&mut attribute_value,
								selectors,
								index,
								alphabet,
								attribute_type_designation
							);
						},

						"selector" => {
							process_css(
								&mut attribute_value,
								selectors,
								index,
								alphabet
							);
						},

						"anchor" => {
							process_anchor_links(
								&mut attribute_value,
								selectors,
								index,
								alphabet
							);
						},

						_ => {
							return capture.at(0).unwrap().to_string();
						},
					}

					return format!(
						"{attribute}{join}{quote}{value}{quote}",
						attribute = attribute_name,
						join = capture.at(2).unwrap(),
						value = attribute_value,
						quote = attribute_quote,
					);
				},

				// Attribute does not contain classes and/or ids.
				// Leave it as is.
				false => {
					return capture.at(0).unwrap().to_string();
				},
			}

		}
	);
}

/// Process JS function arguments.
fn process_js_arguments(
	file_string: &mut String,
	selectors: &mut HashMap<String, String>,
	index: &mut usize,
	alphabet: &[char],
) {
	*file_string = regexes::JS_ARGUMENTS.replace_all(
		file_string,
		|capture: &Captures| {
			// Matched string is a multiline or single line comment
			// i.e. it does not have any further capture groups
			if capture.at(1).is_none() {
				return capture.at(0).unwrap().to_string();
			}

			let mut replacement_args: String = capture.at(3).unwrap().to_string();
			let mut function = capture.at(1).unwrap().to_owned();
			function.retain(|c| !c.is_whitespace());

			// Work out function call and its argument pattern:
			match function.as_str() {
				// Takes one argument, an CSS selector string.
				".querySelector" | ".querySelectorAll" | ".closest" => {
					// FIXME: rudimentary way to check if arg is a immediate
					// string value rather than some expression.
					let quote_type: &str = match replacement_args.chars().next(){
						Some('\'') => "'",
						Some('"') => "\"",
						Some('`') => "`",
						_ => "",
					};

					if !quote_type.is_empty() {
						process_css(
							&mut replacement_args,
							selectors,
							index,
							alphabet
						);
					}
				},

				// Takes one argument, a string of classes (no period prefixed)
				// separated by spaces (if more than one) —
				".getElementsByClassName" => {
					// Checking that argument is a string
					if capture.at(4).is_some() {
						process_string_of_tokens(
							&mut replacement_args,
							selectors,
							index,
							alphabet,
							"class"
						);
					}
					// TODO: handle expressions?
				},

				// Takes one argument, an ID (no hash prefixed).
				".getElementById" => {
					// Checking that argument is a string
					if capture.at(4).is_some() {
						process_string_of_tokens(
							&mut replacement_args,
							selectors,
							index,
							alphabet,
							"id"
						);
					};
					// TODO: handle expressions?
				},

				// Takes two arguments: attribute name and value,
				// process value if attribute is whitelisted.
				".setAttribute" => {
					// Go over the (two) function arguments
					let mut function_args = get_function_arguments(&replacement_args);

					// Check first arg in function, without the string delimiters
					// and then trimming any whitespace off ends.
					let attribute_name: &str = function_args
						.next()
						.unwrap()
						.at(2)
						.unwrap_or("")
						.trim();

					// Check first argument is an known attribute which its value will have
					// classses or an id. If it is not, leave value as is (second argument).
					if ATTRIBUTES_WHITELIST.contains_key(attribute_name) {

						if let Some(attribute_value) = function_args.next() {
							if attribute_value.at(2).is_some() {
								let mut replacement_value = attribute_value.at(2).unwrap().to_string();
								let attribute_type_designation: &str = ATTRIBUTES_WHITELIST
									.get(attribute_name)
									.unwrap();


								match attribute_type_designation {
									"id" | "class" => {
										process_string_of_tokens(
											&mut replacement_value,
											selectors,
											index,
											alphabet,
											attribute_type_designation
										);
									},

									"selector" => {
										process_css(
											&mut replacement_value,
											selectors,
											index,
											alphabet
										);
									},

									_ => {
										return replacement_value;
									},
								};

								replacement_args = replacement_args.replace(
									&attribute_value.at(2).unwrap(),
									&replacement_value,
								);
							}
						}

					}
				},

				// Takes two arguments: position and html,
				// we are only interested in the latter argument.
				".insertAdjacentHTML" => {
					if let Some(html) = get_function_arguments(&replacement_args).nth(1) {
						// Second capture group, which should be the string (without the delimeters).
						if html.at(2).is_some() {
							let mut replacement_html = html.at(2).unwrap().to_string();

							match html.at(2).unwrap().contains("</body>") {
								true => process_html(
									&mut replacement_html,
									selectors,
									index,
									alphabet
								),
								false => process_html_attributes(
									&mut replacement_html,
									selectors,
									index,
									alphabet
								),
							};

							replacement_args = replacement_args.replace(
								&html.at(2).unwrap(),
								&replacement_html,
							);
						}
					}
				},

				// Takes either only one argument or up to two arguments:
				// we are only ever interested in argument number 1.
				"window.open" | "window.location.assign" | "window.location.replace" => {
					if let Some(link) = get_function_arguments(&replacement_args).next() {
						let mut replacement_link = link.at(0).unwrap().to_string();

						process_anchor_links(
							&mut replacement_link,
							selectors,
							index,
							alphabet
						);

						replacement_args = replacement_args.replace(
							&link.at(0).unwrap(),
							&replacement_link
						);
					}
				},

				// Takes two or three arguments, the final argument which
				// is an optional URL is the one that we are interested in.
				"history.pushState" | "history.replaceState" => {
					if let Some(link) = get_function_arguments(&replacement_args).nth(2) {
						let mut replacement_link = link.at(0).unwrap().to_string();
						process_anchor_links(
							&mut replacement_link,
							selectors,
							index,
							alphabet
						);
						replacement_args = replacement_args.replace(
							link.at(0).unwrap(),
							&replacement_link,
						);
					}
				},

				// Takes one or more arguments, each argument is for
				// an individual class name (no period prefix).
				".classList.add" | ".classList.contains" | ".classList.remove"
				| ".classList.replace" | ".classList.toggle"  => {
					process_string_of_arguments(
						&mut replacement_args,
						selectors,
						index,
						alphabet,
						"class"
					);
				},

				_ => {},
			}

			return format!(
				"{function}{join}{arguments}",
				function = capture.at(1).unwrap(),
				join = capture.at(2).unwrap(),
				arguments = replacement_args
			);
		}
	);
}

/// Process JS property operation values.
fn process_js_properties(
	file_string: &mut String,
	selectors: &mut HashMap<String, String>,
	index: &mut usize,
	alphabet: &[char],
) {
	*file_string = regexes::JS_PROPERTIES.replace_all(
		file_string,
		|capture: &Captures| {
			// Matched string is a multiline or single line comment
			// i.e. it does not have any further capture groups
			if capture.at(1).is_none() {
				return capture.at(0).unwrap().to_string();
			}

			let mut property_value: String = capture.at(3).unwrap().to_string();
			let property_name: &str = capture.at(1).unwrap();

			match property_name {
				".className" => {
					process_string_of_tokens(
						&mut property_value,
						selectors,
						index,
						alphabet,
						"class"
					);
				},
				".innerHTML" | ".outerHTML" => {
					if property_value.contains("</body>") {
						process_html(
							&mut property_value,
							selectors,
							index,
							alphabet
						);
					} else {
						process_html_attributes(
							&mut property_value,
							selectors,
							index,
							alphabet
						);
					}
				},
				"window.location.hash"
				| "window.location.href"
				| "window.location" => {
					process_anchor_links(
						&mut property_value,
						selectors,
						index,
						alphabet
					);
				},
				_ => {},
			}

			format!(
				"{name}{operator}{value}",
				name = property_name,
				operator = capture.at(2).unwrap(),
				value = property_value,
			)
		}
	);
}

/// Process string with tokens delimited by whitespaces.
///
/// Notes:
///  - As regexes::STRING_DELIMITED_BY_SPACE regex is simple - only
///    grouping non whitespace characters together - any quote
///    delimiters will need to be trimmed and added back on
///    afterwards.
///  - context is neseccary in order to determine what the
///    token(s) should be processed as (e.g. class or id).
fn process_string_of_tokens(
	string: &mut String,
	selectors: &mut HashMap<String, String>,
	index: &mut usize,
	alphabet: &[char],
	context: &str,
) {
	let prefix: &str = match context {
		"class" => ".",
		"id" => "#",
		_ => "",
	};

	// Handle strings that have quote delimiters included.
	let quote_type: &str = match string.chars().next(){
		Some('\'') => "'",
		Some('"') => "\"",
		Some('`') => "`",
		_ => "",
	};

	// Trim quotes (if any) from value capture group.
	if !quote_type.is_empty() {
		string.pop();
		string.remove(0);
	}

	*string = format!(
		"{quote}{tokens}{quote}",
		tokens = regexes::STRING_DELIMITED_BY_SPACE.replace_all(
			string,
			|capture: &Captures| {
				// Check if token has a minify-selectors specific prefix,
				// It should be handled with process_prefixed_selectors().
				if regexes::PREFIXED_SELECTORS.find(capture.at(0).unwrap()).is_some() {
					return capture.at(0).unwrap().to_string();
				}

				return get_encoded_selector(
					&format!(
						"{prefix}{token}",
						prefix = prefix,
						token = &capture.at(1).unwrap()
					),
					selectors,
					index,
					alphabet
				);
			}
		),
		quote = quote_type,
	);
}

/// Returns an iterator of function arguments.
fn get_function_arguments<'r, 't> (
	string: &'t str
) -> FindCaptures<'r, 't> {
	regexes::STRING_DELIMITED_BY_COMMA.captures_iter(string)
}

/// Process function arguments, delimited by commas.
///
/// Notes:
///  - context is neseccary in order to determine what the
///    token(s) should be processed as (e.g. class or id).
fn process_string_of_arguments(
	string: &mut String,
	selectors: &mut HashMap<String, String>,
	index: &mut usize,
	alphabet: &[char],
	context: &str,
) {
	let prefix: &str = match context {
		"class" => ".",
		"id" => "#",
		_ => "",
	};

	*string = regexes::STRING_DELIMITED_BY_COMMA.replace_all(
		string,
		|capture: &Captures| {
			// Check if argument has a minify-selectors specific prefix,
			// It should be handled with process_prefixed_selectors().
			if regexes::PREFIXED_SELECTORS.find(capture.at(0).unwrap()).is_some() {
				return capture.at(0).unwrap().to_string();
			}

			// Check if argument is a string, variable/expression or object/array.
			//   - 1: simple string argument (with delimiters)
			//   - 2: simple string argument (without delimiters)
			//   - 3: variable or expression argument
			//   - 4: object argument
			//   - 5: array argument
			if capture.at(2).is_some() {
				let mut token:String = capture.at(1).unwrap().to_string();

				// Get quote delimiters from argument.
				let quote_type: &str = match capture.at(1).unwrap().chars().next(){
					Some('\'') => "'",
					Some('"') => "\"",
					Some('`') => "`",
					_ => "",
				};

				// Trim quotes from argument.
				token.pop();
				token.remove(0);

				return format!(
					"{quote}{argument}{quote}",
					argument = get_encoded_selector(
						&format!(
							"{prefix}{token}",
							prefix = prefix,
							token = token
						),
						selectors,
						index,
						alphabet
					),
					quote = quote_type,
				);
			} else if capture.at(3).is_some() {
				// TODO:
				return capture.at(0).unwrap().to_string();
			} else {
				// Capture group 4 (<object>) or 5 (<array>) .is_some() evaluates to true
				// or another case. Either way nothing needs to be done to this argument.
				return capture.at(0).unwrap().to_string();
			}
		}
	);
}

// Process target IDs in anchor link URLs.
fn process_anchor_links(
	string: &mut String,
	selectors: &mut HashMap<String, String>,
	index: &mut usize,
	alphabet: &[char],
) {
	// Handle strings that have quote delimiters included.
	let quote_type: &str = match string.chars().next(){
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
		url = regexes::INTERNAL_ANCHOR_TARGET_ID.replace(
			string,
			|capture: &Captures| {
				if capture.at(1).is_none() {
					return capture.at(0).unwrap().to_string();
				}

				format!(
					"{url}#{target_id}",
					url = capture.at(1).unwrap_or(""),
					target_id = get_encoded_selector(
						capture.at(2).unwrap(),
						selectors,
						index,
						alphabet
					),
				)
			}
		),
		quote = quote_type,
	);
}
