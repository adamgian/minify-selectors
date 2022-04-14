use lazy_static::lazy_static;
use onig::*;
use std::collections::HashMap;

use encode_selector;




lazy_static! {
	// Extracts classes and IDs from selector rules in
	// stylesheets and embedded styles.
	//
	// See for reference: https://www.w3.org/TR/selectors-3/#grammar
	//
	// 1. Needs '#' or '.' to define in CSS an ID or class respectively.
	// 2. Next character after is '-', which is optional.
	// 3. Next character after is the 'nmstart' which is any of:
	//    a. underscore and lowercase/uppercase latin letters ([A-Za-z\_]).
	//    b. anything else that is not ASCII ([^\0-\177]).
	//    c. escaped unicode number or character. Unicode numbers are 6 hex
	//        digits following the backslash. Unicode numbers can also be
	//        terminated earlier by by a space, newline, tab or form feed
	// 4. Finally after the mandatory 'nmstart' character, there are zero,
	//    one or many of 'nmchar' characters. 'nmchar's have exactly the
	//    same rules as 'nmstart' except for part a. — it is acceptable
	//    to have numerical digits and dashes as well (simplified down
	//    to [\w\-]).
	static ref CSS_CLASSES_AND_IDS: Regex = Regex::new(
		r##"(?x)
			(?<type>[\#\.])
			(?<name>
				-?
				(?>
					[A-Za-z\_]
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
			(?=
				[^\{]*
				\{
			)

		"##
	).unwrap();

	// Extracts classes and IDs from selector strings
	static ref CSS_SELECTOR_STRING: Regex = Regex::new(
		r##"(?x)
			(?<type>[\#\.])
			(?<name>
				-?
				(?>
					[A-Za-z\_]
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
	static ref CSS_ATTRIBUTE_SELECTORS: Regex = Regex::new(
		r##"(?x)
			\[\s*+
			(?<type>class|id)
			=[\"\']?
			(?<name>
				-?
				(?>
					[A-Za-z\_]
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
			[\"\']?\s*+\]
		"##
	).unwrap();

	// Extracts arguments from functions that take
	// classes, IDs or a CSS selector string.
	static ref JS_ARGUMENTS: Regex = Regex::new(
		r##"(?x)
			\.
			(?<function>
				className
				| querySelectorAll
				| querySelector
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
					(?>
						[\w\-\ \#\.\*\:\>\[\]\+\~\"\']*+
						[\,]?
					)++
				)
			)
		"##
	).unwrap();

	// Extracts all attributes with values from HTML.
	//
	// Will need additional processing to consider
	// 'whitelisted' attributes and separate values.
	static ref HTML_ATTRIBUTES: Regex = Regex::new(
		r##"(?x)
			(?<attribute>
				[^\f\n\t\ \>\"\'\=]++
			)
			=
			(?<value>
				(?([\"\'])
					[\w\-\s\#\.]++[\"\']
					| [\w\-\#\.]++
				)
			)
			(?=
				(?>
					[\s]++
					[^\f\n\t\ \>\"\'\=]++
					(?>
						=
						[^\ \>]++
					)?+
				)*+
				[\s]*+[\/]?>
			)
		"##
	).unwrap();

	//
	static ref STRING_DELIMITED_BY_SPACE: Regex = Regex::new(
		r##"(?x)
			(?<token>
				(?>[A-Za-z\_\\]|\-[A-Za-z\-\_])
				[\w\-\\]*+
			)
		"##
	).unwrap();

	//
	static ref STRING_DELIMITED_BY_COMMA: Regex = Regex::new(
		r##"(?x)
			(?<=[\"\'])
			(?<token>
				(?>[A-Za-z\_\\]|\-[A-Za-z\-\_])
				[\w\-\\]*+
			)
		"##
	).unwrap();

	// HTML attributes which its values will contain classes/ids
	static ref HTML_ATTRIBUTES_WHITELIST: HashMap<String, String> = HashMap::from([
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
	file_string: &mut String,
	selectors: &mut HashMap<String, String>,
	index: &mut u32
) -> String {
	return process_css_selectors(
		file_string,
		selectors,
		index
	);
}

pub fn from_html(
	file_string: &mut String,
	selectors: &mut HashMap<String, String>,
	index: &mut u32
) -> String {
	let mut replacement_string: String = String::new();

	// Processing HTML attributes
	replacement_string = HTML_ATTRIBUTES.replace_all(
		&file_string,
		|capture: &Captures| {
			let attribute_name: &str = capture.at(1).unwrap();
			let mut attribute_values: String = capture.at(2).unwrap().to_string();

			// TODO:
			// Attributes whitelist of which its
			// values should be processed.
			match HTML_ATTRIBUTES_WHITELIST.contains_key(attribute_name) {
				true => {
					// Work out if value(s) are classes, ids or selectors.
					let attribute_type_designation: &str = HTML_ATTRIBUTES_WHITELIST
						.get(capture.at(1).unwrap())
						.unwrap();

					match attribute_type_designation {
						"id" | "class" => {
							attribute_values = process_string_of_tokens(
								&mut attribute_values,
								selectors,
								index,
								attribute_type_designation
							);
						},

						"selector" => {
							attribute_values = process_css_selector_string(
								&mut attribute_values,
								selectors,
								index
							);
						},

						_ => {}
					}

					return format!(
						"{attribute}={value}",
						attribute = attribute_name,
						value = attribute_values,
					);
				},

				// Attribute does not contain classes and/or ids.
				// Leave it as is.
				false => {
					return format!("{}", capture.at(0).unwrap());
				},
			}

		}
	);

	// Processing any embedded styles
	replacement_string = process_css_selectors(
		&mut replacement_string,
		selectors,
		index
	);

	// Processing any embedded js
	return process_js(
		&mut replacement_string,
		selectors,
		index
	);

	return replacement_string;
}

pub fn from_js(
	file_string: &mut String,
	selectors: &mut HashMap<String, String>,
	index: &mut u32
) -> String {
	return process_js(
		file_string,
		selectors,
		index
	);
}

/// Fetch encoded selector from selectors hashmap.
/// If selector is new and unique, generate one for it
/// and add it to selectors.
fn get_encoded_selector(
	selector: &str,
	selectors: &mut HashMap<String, String>,
	index: &mut u32
) -> String {
	match selectors.contains_key(selector) {
		true => {
			return selectors
				.get_key_value(selector)
				.unwrap().1.to_string();
		},

		false => {
			*index += 1;
			let encoded_selector: String = encode_selector::to_base62(index);

			selectors.insert(
				selector.to_owned(),
				encoded_selector.clone()
			);

			return encoded_selector;
		}
	}
}

/// Process CSS selectors.
fn process_css_selectors(
	file_string: &mut String,
	selectors: &mut HashMap<String, String>,
	index: &mut u32
) -> String {
	return CSS_CLASSES_AND_IDS.replace_all(
		&file_string,
		|capture: &Captures| {
			return format!(
				"{prefix}{identifier}",
				prefix = &capture.at(1).unwrap(),
				identifier = get_encoded_selector(
					&capture.at(0).unwrap().to_owned(),
					selectors,
					index
				)
			);
		}
	);
}

/// Process CSS selector string.
fn process_css_selector_string(
	string: &str,
	selectors: &mut HashMap<String, String>,
	index: &mut u32
) -> String {
	return CSS_SELECTOR_STRING.replace_all(
		&string,
		|capture: &Captures| {
			return format!(
				"{prefix}{identifier}",
				prefix = &capture.at(1).unwrap(),
				identifier = get_encoded_selector(
					&capture.at(0).unwrap().to_owned(),
					selectors,
					index
				)
			);
		}
	);
}

/// Process Javascript.
fn process_js(
	file_string: &mut String,
	selectors: &mut HashMap<String, String>,
	index: &mut u32
) -> String {
	return JS_ARGUMENTS.replace_all(
		&file_string,
		|capture: &Captures| {
			let mut replacement_value: String = capture.at(3).unwrap().to_string();

			// Work out function call and its argument pattern:
			match capture.at(1).unwrap() {
				// Takes one argument, an CSS selector string.
				"querySelector" | "querySelectorAll" => {
					replacement_value = process_css_selector_string(
						&mut replacement_value,
						selectors,
						index
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
						"class"
					);
				},

				// Takes one argument, an ID (no hash prefixed).
				"getElementById" => {
					replacement_value = process_string_of_tokens(
						&mut replacement_value,
						selectors,
						index,
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

					match HTML_ATTRIBUTES_WHITELIST.contains_key(attribute_name) {
						true => {
							let attribute_type_designation: &str = HTML_ATTRIBUTES_WHITELIST
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
												attribute_type_designation
											);
										},

										"selector" => {
											return process_css_selector_string(
												&mut current_value.to_string(),
												selectors,
												index
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
							return format!("{}", capture.at(0).unwrap());
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

///
///
/// selector_type
fn process_string_of_tokens(
	string: &mut String,
	selectors: &mut HashMap<String, String>,
	index: &mut u32,
	selector_type: &str
) -> String {
	let prefix: String = match selector_type {
		"id" => { "#" },
		"class" => { "." },
		_ => { "" }
	}.to_string();

	return STRING_DELIMITED_BY_SPACE.replace_all(
		&string,
		|capture: &Captures| {
			return get_encoded_selector(
				&format!(
					"{prefix}{token}",
					prefix = prefix,
					token = &capture.at(1).unwrap()
				),
				selectors,
				index
			);
		}
	);
}

///
///
/// selector_type
fn process_string_of_arguments(
	string: &mut String,
	selectors: &mut HashMap<String, String>,
	index: &mut u32,
	selector_type: &str
) -> String {
	let prefix: String = match selector_type {
		"id" => { "#" },
		"class" => { "." },
		_ => { "" }
	}.to_string();

	return STRING_DELIMITED_BY_COMMA.replace_all(
		&string,
		|capture: &Captures| {
			return get_encoded_selector(
				&format!(
					"{prefix}{token}",
					prefix = prefix,
					token = capture.at(1).unwrap()
				),
				selectors,
				index
			);
		}
	);
}
