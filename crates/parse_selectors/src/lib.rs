use lazy_static::lazy_static;
use onig::*;
use std::collections::HashMap;




lazy_static! {
	// Extracts classes and IDs from selector rules in
	// stylesheets and embedded styles.
	//
	// See for reference: https://www.w3.org/TR/selectors-3/#grammar
	//
	// 1. Needs '#' or '.' to define in CSS an ID or class respectively.
	// 2. Next character after is '-', which is optional.
	// 3. Next character after is the 'nmstart' which is any of:
	//    a. underscore and lowercase/uppercase latin letters ([A-Za-z_]).
	//    b. anything else that is not ASCII ([^\0-\177]).
	//    c. escaped unicode number or character. Unicode numbers are 6 hex
	//        digits following the backslash. Unicode numbers can also be
	//        terminated earlier by by a space, newline, tab or form feed
	// 4. Finally after the mandatory 'nmstart' character, there are zero,
	//    one or many of 'nmchar' characters. 'nmchar's have exactly the
	//    same rules as 'nmstart' except for part a. — it is acceptable
	//    to have numerical digits and dashes as well (simplified down
	//    to [\w\-]).
	//
	// Caveats:
	// -  This regex in HTML files will match JS functions, objects, inner
	//    HTML, etc. — stuff it should not pick up. To circumvent this
	//    problem, this regex should only be run a subset of the HTML file
	//    string (i.e. content within <style></style>).
	// -  This regex will 'ignore'/blackout CSS blocks ({...}) in the sense
	//    that it will capture everything in the firstmost capture group
	//    and block the main regex portion from ever matching hex color
	//    values, units and the like.
	// -  This regex will 'ignore'/blackout attibutes selectors completely
	//    to avoid any false positives.
	// -  Multiline comments are 'ignored'/blacked out.
	static ref CSS_SELECTORS: Regex = Regex::new(
		r##"(?x)
			{
				[^{}]*
			}
			|
			\[
				\s*
					["']?.*?["']?
				\s*
			\]
			|
			\/\*[^*]*\*+(?>[^\/*][^*]*\*+)*\/
			|
			(?<type>[\#\.])
			(?<name>
				-?
				(?>
					[A-Za-z_]
					| [^\0-\177]
					| (?>
						\\[0-9A-Fa-f]{1,6}(?>\r\n|[ \n\r\t\f])?
						| \\[^\n\r\f0-9A-Fa-f]
					)
				)
				(?>
					[\w\-]
					| [^\0-\177]
					| (?>
						\\[0-9A-Fa-f]{1,6}(?>\r\n|[ \n\r\t\f])?
						| \\[^\n\r\f0-9A-Fa-f]
					)
				)*
			)
		"##
	).unwrap();

	// Extracts classes and IDs from a limited set of
	// attribute selectors. Attribute name must be 'class' or 'id'
	// and use the exact match operator.
	// i.e. [class="foo"][id="bar"]
	static ref CSS_ATTRIBUTES: Regex = Regex::new(
		r##"(?x)
			\/\*[^*]*\*+(?>[^\/*][^*]*\*+)*\/
			|
			\[\s*+
			(?<attribute>
				[^\f\n\t\ >"'|^$*~=]++
			)
			(?<operator>
				[~]?=
			)
			["']?
			(?<value>
				-?
				(?>
					[A-Za-z_]
					| [^\0-\177]
					| (?>
						\\[0-9A-Fa-f]{1,6}(?>\r\n|[ \n\r\t\f])?
						| \\[^\n\r\f0-9A-Fa-f]
					)
				)
				(?>
					[\w\-]
					| [^\0-\177]
					| (?>
						\\[0-9A-Fa-f]{1,6}(?>\r\n|[ \n\r\t\f])?
						| \\[^\n\r\f0-9A-Fa-f]
					)
				)*
			)
			(?<quote>
				["']
			)?
			\s*+
			(?<flag>
				[IiSs]
			)?
			\s*+\]
		"##
	).unwrap();

	// Extracts arguments from functions that take
	// classes, IDs or a CSS selector string.
	//
	// Objective is to capture a string of the function
	// input (between the parens) for further processing.
	static ref JS_ARGUMENTS: Regex = Regex::new(
		r##"(?x)
			\.
			(?<function>
				className
				| querySelectorAll
				| querySelector
				| closest
				| getElementById
				| getElementsByClassName
				| classList\s*+\.(?> add | remove | contains | replace | toggle )
				| setAttribute
			)
			(?<join>
				\(\s*+
				| \s*+[\=\+\!]++\s*+
			)
			(?>
				\s*+
				(?<arguments>
					(?:
						(?:
							`
							(?:
								[^`\\] | \\.
							)+
							[^)][`,]?\s*
						)
						| (?:
							"
							(?:
								[^"\\] | \\.
							)+
							[^)][",]?\s*
						)
						| (?:
							'
							(?:
								[^'\\] | \\.
							)+
							[^)][',]?\s*
						)
					)++
				)
			)
		"##
	).unwrap();

	// Extract instances of <style></style> from HTML files.
	static ref HTML_STYLE_ELEMENT: Regex = Regex::new(
		r##"(?x)
			(?<tag_open>
				<style[^>]*>
			)
			(?<styles>
				(?:.|\n|\r)*?
			)
			(?<tag_close>
				<\/style>
			)
		"##
	).unwrap();

	// Extract <body> from HTML files.
	static ref HTML_BODY: Regex = Regex::new(
		r##"(?x)
			(?<tag_open>
				<body[^>]*>
			)
			(?<styles>
				(?:.|\n|\r)*?
			)
			(?<tag_close>
				<\/body>
			)
		"##
	).unwrap();

	// Extracts all attributes with values from HTML.
	//
	// Will need additional processing to consider 'whitelisted'
	// attributes and separate out the values.
	//
	// See: https://www.w3.org/TR/2018/SPSD-html5-20180327/syntax.html#attributes-0
	//
	// 1. Attribute name - consists of one of more characters.
	//    - Cannot be a whitespace character, null, quotation ("),
	//        apostrophe ('), forward slash (/) or equals sign (=).
	//    - ASCII case insensitive
	//    - Character references:
	//        - Named: e.g. &copy;, &nbsp;
	//        - Decimal numeric: &#931;, &#0931;
	//        - Hexadecimal numeric: &#x3A3;, &#x03A3;, &#x3a3;
	// 2. Then optionally followed by an attribute value. An single equals
	//    sign is used to separate name from the value. Even though values
	//    are optional, we are only interested in attributes that have a
	//    value. Note: it is valid to have one or more whitespace chars
	//    on either side of the equals sign.
	// 3. Attibutes values cannot contain: <, >, `, or =. Additional
	//    rules as follows:
	//    - Unquoted value - cannot have: ", ' or be an empty string.
	//    - Single-quoted value, cannot contain any ' characters.
	//    - Double-quoted value, cannot contain any " characters.
	//    - Like names, values can have character references also.
	//    - If followed by another attribute or /, there must be at least
	//        a whitespace character before them.
	static ref HTML_ATTRIBUTES: Regex = Regex::new(
		r##"(?x)
			(?<attribute>
				[^\s\x00\/>"'=]+
			)
			(?<join>
				\s*=\s*
			)
			(?<value>
				[^\s\/<>"'=]+
				| "[^\/<>"=]+
				| '[^\/<>'=]+
			)
			(?<quote>["']?)
		"##
	).unwrap();

	// Extract tokens — seperated by whitespace(s).
	static ref STRING_DELIMITED_BY_SPACE: Regex = Regex::new(
		r##"(?x)
			(?<token>
				[^\s]++
			)
		"##
	).unwrap();

	// Extract function arguments.
	static ref STRING_DELIMITED_BY_COMMA: Regex = Regex::new(
		r##"(?x)
			["'`]
			(?<token>
				[^"'`]*+
			)
			["'`]
		"##
	).unwrap();

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
	]);
}




pub fn from_css(
	file_string: &mut str,
	selectors: &mut HashMap<String, String>,
	index: &mut usize,
	alphabet: &[char]
) -> String {
	return process_css(
		file_string,
		selectors,
		index,
		alphabet
	);
}

pub fn from_html(
	file_string: &mut str,
	selectors: &mut HashMap<String, String>,
	index: &mut usize,
	alphabet: &[char]
) -> String {
	return process_html(
		file_string,
		selectors,
		index,
		alphabet
	);
}

pub fn from_js(
	file_string: &mut str,
	selectors: &mut HashMap<String, String>,
	index: &mut usize,
	alphabet: &[char]
) -> String {
	return process_js(
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
	alphabet: &[char]
) -> String {
	match selectors.contains_key(selector) {
		true => {
			return selectors
				.get_key_value(selector)
				.unwrap().1.to_string();
		},

		false => {
			let encoded_selector: String = encode_selector::to_radix(index, alphabet);
			*index += 1;

			selectors.insert(
				selector.to_owned(),
				encoded_selector.clone()
			);

			return encoded_selector;
		}
	}
}

fn process_css(
	file_string: &mut str,
	selectors: &mut HashMap<String, String>,
	index: &mut usize,
	alphabet: &[char]
) -> String {
	let mut css: String = process_css_selectors(
		file_string,
		selectors,
		index,
		alphabet
	);

	css = process_css_attributes(
		&mut css,
		selectors,
		index,
		alphabet
	);

	return css;
}

/// Process HTML.
fn process_html(
	file_string: &mut str,
	selectors: &mut HashMap<String, String>,
	index: &mut usize,
	alphabet: &[char]
) -> String {
	// Initial step — go through <body> and parse attributes
	let mut html: String = HTML_BODY.replace_all(
		file_string,
		|capture: &Captures| {
			return format!(
				"{tag_open}{styles}{tag_close}",
				tag_open = capture.at(1).unwrap(),
				styles = process_html_attributes(
					&mut capture.at(2).unwrap().to_owned(),
					selectors,
					index,
					alphabet
				),
				tag_close = capture.at(3).unwrap()
			);
		}
	);

	// Processing any embedded styles
	// Create subset string(s) to process <style> embeds
	html = HTML_STYLE_ELEMENT.replace_all(
		&html,
		|capture: &Captures| {
			return format!(
				"{tag_open}{styles}{tag_close}",
				tag_open = capture.at(1).unwrap(),
				styles = process_css(
					&mut capture.at(2).unwrap().to_owned(),
					selectors,
					index,
					alphabet
				),
				tag_close = capture.at(3).unwrap()
			);
		}
	);

	// Processing any embedded js
	html = process_js(
		&mut html,
		selectors,
		index,
		alphabet
	);

	return html;
}

/// Process Javascript.
fn process_js(
	file_string: &mut str,
	selectors: &mut HashMap<String, String>,
	index: &mut usize,
	alphabet: &[char]
) -> String {
	return JS_ARGUMENTS.replace_all(
		file_string,
		|capture: &Captures| {
			let mut replacement_value: String = capture.at(3).unwrap().to_string();

			// Work out function call and its argument pattern:
			match capture.at(1).unwrap() {
				// Takes one argument, an CSS selector string.
				"querySelector" | "querySelectorAll" | "closest" => {
					replacement_value = process_css(
						&mut replacement_value,
						selectors,
						index,
						alphabet
					);
				},

				// Takes one argument, a string of classes (no period prefixed)
				// separated by spaces (if more than one) —
				"getElementsByClassName"
				// or property will be operated on with a string of classes.
				| "className" => {
					replacement_value = process_string_of_tokens(
						&mut replacement_value,
						selectors,
						index,
						alphabet,
						"class"
					);
				},

				// Takes one argument, an ID (no hash prefixed).
				"getElementById" => {
					replacement_value = process_string_of_tokens(
						&mut replacement_value,
						selectors,
						index,
						alphabet,
						"id"
					);
				},

				// Takes one or more arguments, each argument is for
				// an individual class name (no period prefixed).
				"classList.add"
				| "classList.replace"
				| "classList.remove"
				| "classList.contains"
				| "classList.toggle" => {
					replacement_value = process_string_of_arguments(
						&mut replacement_value,
						selectors,
						index,
						alphabet,
						"class"
					);
				},

				// Takes two arguments: attribute name and value,
				// process value if attribute is whitelisted.
				"setAttribute" => {
					let attribute_name: &str = STRING_DELIMITED_BY_COMMA
						.captures(&replacement_value)
						.unwrap()
						.at(0)
						.unwrap();

					match ATTRIBUTES_WHITELIST.contains_key(attribute_name) {
						true => {
							let attribute_type_designation: &str = ATTRIBUTES_WHITELIST
								.get(attribute_name)
								.unwrap();

							replacement_value = STRING_DELIMITED_BY_COMMA.replace_all(
								&replacement_value,
								|capture: &Captures| {
									let current_value: &str = capture.at(1).unwrap();

									if current_value == attribute_name {
										return current_value.to_string();
									}

									match attribute_type_designation {
										"id" | "class" => {
											return process_string_of_tokens(
												&mut current_value.to_string(),
												selectors,
												index,
												alphabet,
												attribute_type_designation
											);
										},

										"selector" => {
											return process_css(
												&mut current_value.to_string(),
												selectors,
												index,
												alphabet
											);
										},

										_ => {
											return current_value.to_string();
										}
									}
								}
							);
						},

						// Attribute does not contain classes and/or ids.
						// Leave it as is.
						false => {
							return capture.at(0).unwrap().to_string();
						},
					}
				},

				_ => {},
			}

			return format!(
				".{function}{join}{arguments}",
				function = capture.at(1).unwrap(),
				join = capture.at(2).unwrap(),
				arguments = replacement_value
			);
		}
	);
}




/// Process classes and IDs in CSS file/embed or as a
/// CSS selector string.
fn process_css_selectors(
	file_string: &mut str,
	selectors: &mut HashMap<String, String>,
	index: &mut usize,
	alphabet: &[char]
) -> String {
	return CSS_SELECTORS.replace_all(
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
			return capture.at(0).unwrap().to_owned();
		}
	);
}

// Process CSS attribute selectors.
fn process_css_attributes(
	file_string: &mut str,
	selectors: &mut HashMap<String, String>,
	index: &mut usize,
	alphabet: &[char]
) -> String {
	return CSS_ATTRIBUTES.replace_all(
		file_string,
		|capture: &Captures| {
			// Check that capture group 2 exists, if it doesn't it is
			// matched to an attribute selector, rule block or comment.
			// Leave it as is.
			if capture.at(2).is_none() {
				return capture.at(0).unwrap().to_owned();
			}

			let attribute_name: &str = capture.at(1).unwrap();
			let mut attribute_value: String = capture.at(3).unwrap().to_string();

			match ATTRIBUTES_WHITELIST.contains_key(attribute_name) {
				true => {
					// Do not process attribute selector if case-insensitive
					// flag has been set.
					if let Some("i") | Some("I") = capture.at(5) {
						return capture.at(0).unwrap().to_string();
					}

					// Work out if value(s) are classes, ids or selectors.
					let attribute_type_designation: &str = ATTRIBUTES_WHITELIST
						.get(capture.at(1).unwrap())
						.unwrap();

					match attribute_type_designation {
						"id" | "class" => {
							attribute_value = process_string_of_tokens(
								&mut attribute_value,
								selectors,
								index,
								alphabet,
								attribute_type_designation
							);
						},
						"selector" => {
							attribute_value = process_css(
								&mut attribute_value,
								selectors,
								index,
								alphabet
							);
						},
						_ => {},
					}

					return format!(
						"[{attribute}{operator}{quote}{value}{quote}{flag}]",
						attribute = attribute_name,
						operator = capture.at(2).unwrap(),
						quote = capture.at(4).unwrap_or(""),
						value = attribute_value,
						flag = match capture.at(5) {
							Some(f) => " ".to_string() + f,
							None => "".to_string(),
						},
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

/// Process HTML attributes.
fn process_html_attributes(
	file_string: &mut str,
	selectors: &mut HashMap<String, String>,
	index: &mut usize,
	alphabet: &[char]
) -> String {
	return HTML_ATTRIBUTES.replace_all(
		file_string,
		|capture: &Captures| {
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

					// attribute_value will need to be cleaned up, as 'HTML_ATTRIBUTES'
					// regex will capture the opening quote if it has been used.
					if !attribute_quote.is_empty() {
						attribute_value.remove(0);
					}

					match attribute_type_designation {
						"id" | "class" => {
							attribute_value = process_string_of_tokens(
								&mut attribute_value,
								selectors,
								index,
								alphabet,
								attribute_type_designation
							);
						},

						"selector" => {
							attribute_value = process_css(
								&mut attribute_value,
								selectors,
								index,
								alphabet
							);
						},

						_ => {}
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

///
///
/// selector_type
fn process_string_of_tokens(
	string: &mut str,
	selectors: &mut HashMap<String, String>,
	index: &mut usize,
	alphabet: &[char],
	selector_type: &str
) -> String {
	let prefix: &str = match selector_type {
		"class" => { "." },
		"id" => { "#" },
		_ => { "" },
	};

	// Handle strings that have quote delimiters included.
	let quote_type: &str = match string.chars().next(){
		Some('\'') => { "'" },
		Some('"') => { "\"" },
		Some('`') => { "`" },
		_ => { "" },
	};

	return format!(
		"{quote}{tokens}{quote}",
		tokens = STRING_DELIMITED_BY_SPACE.replace_all(
			string,
			|capture: &Captures| {
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

///
///
/// selector_type
fn process_string_of_arguments(
	string: &mut str,
	selectors: &mut HashMap<String, String>,
	index: &mut usize,
	alphabet: &[char],
	selector_type: &str
) -> String {
	let prefix: &str = match selector_type {
		"class" => { "." },
		"id" => { "#" },
		_ => { "" },
	};

	return STRING_DELIMITED_BY_COMMA.replace_all(
		string,
		|capture: &Captures| {
			// Need to put quote delimiters back around the argument value
			let quote_type: &str = match capture.at(0).unwrap().chars().next(){
				Some('\'') => { "'" },
				Some('"') => { "\"" },
				Some('`') => { "`" },
				_ => { "" },
			};

			return format!(
				"{quote}{argument}{quote}",
				argument = get_encoded_selector(
					&format!(
						"{prefix}{token}",
						prefix = prefix,
						token = capture.at(1).unwrap()
					),
					selectors,
					index,
					alphabet
				),
				quote = quote_type,
			);
		}
	);
}
