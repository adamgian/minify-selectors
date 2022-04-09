use lazy_static::lazy_static;
use onig::*;
use std::collections::HashMap;

use encode_selector;




lazy_static! {
	// Extracts classes and IDs from selector rules in
	// stylesheets and embedded styles.
	static ref CSS_CLASSES_AND_IDS: Regex = Regex::new(
		r##"(?x)
			(?<type>[\#\.])
			(?<name>
				(?>[A-Za-z\_\\]|\-[A-Za-z\-\_])
				[\w\-\\]*+
			)
			(?=
				\s*+
				[\{\*\#\,\:\>\[\+\~]
				|
				\s*+
				\.[\w\-\s\.\*]*+[\{\[]
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
				(?>[A-Za-z\_\\]|\-[A-Za-z\-\_])
				[\w\-\\]*+
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
				querySelectorAll
				| querySelector
				| getElementById
				| getElementsByClassName
				| classList\s*+\.(?> add | remove | contains | replace | toggle )
			)
			\(
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

pub fn from_html(
	file_string: &mut String,
	selectors: &mut HashMap<String, String>,
	index: &mut u32
) -> String {
	return HTML_ATTRIBUTES.replace_all(
		&file_string,
		|capture: &Captures| {
			let mut values: String = capture.at(2).unwrap().to_string();
			let mut replacement_value: String = String::new();

			// Quote type will either be single or double
			let quote_type: String = match values.chars().nth(0){
				Some('\'') => { r#"'"# },
				Some('\"') => { r#"""# },
				_ => { "" }
			}.to_string();

			// Trim quotes (if any) from value capture group.
			// Should not need to worry about string length,
			// HTML_ATTRIBUTES regex only picks up values that have
			// at least a character in it.
			if !quote_type.is_empty() {
				values.pop();
				values.remove(0);
			}

			// TODO:
			// Attributes whitelist of which its
			// values should be processed.
			if HTML_ATTRIBUTES_WHITELIST.contains_key(capture.at(1).unwrap()) {

				// Work out if value(s) are classes, ids or selectors.
				let identifier = HTML_ATTRIBUTES_WHITELIST.get(capture.at(1).unwrap());

				// TODO: handle CSS selector string

				let prefix: String = match identifier.unwrap().as_str() {
					"id" => { "#" },
					"class" => { "." },
					_ => { "" }
				}.to_string();

				for value in values.split_whitespace() {
					let encoded_selector: String = get_encoded_selector(
						&format!("{}{}", prefix, value),
						selectors,
						index
					);

					// Adding space between values
					if !replacement_value.is_empty() {
						replacement_value.push_str(" ");
					}

					replacement_value.push_str(&encoded_selector);
				}

				return format!(
					"{attribute}={quote}{value}{quote}",
					attribute = capture.at(1).unwrap(),
					value = replacement_value,
					quote = quote_type
				);
			}

			// Attribute does not contain classes and/or ids.
			// Leave it as is.
			else {
				return format!("{}", capture.at(0).unwrap());
			}
		}
	);
}

pub fn from_js(
	file_string: &mut String,
	selectors: &mut HashMap<String, String>,
	index: &mut u32
) -> String {
	return JS_ARGUMENTS.replace_all(
		&file_string,
		|capture: &Captures| {
			let mut replacement_value: String = String::new();
			// TODO: remove quotes?
			// TODO: remove commas later in match when necessary?

			// Work out function call and its argument pattern:
			match capture.at(1).unwrap() {
				// Takes one argument, an CSS selector string.
				"querySelector" | "querySelectorAll" => {},

				// Takes one argument, a string of classes (no period prefixed)
				// separated by spaces (if more than one).
				"getElementsByClassName" => {},

				// Takes one argument, an ID (no hash prefixed).
				"getElementsById" => {},

				// Takes one or more arguments, each argument is for
				// an individual class name (no period prefixed).
				"classList.add"
				| "classList.replace"
				| "classList.remove"
				| "classList.contains"
				| "classList.toggle" => {},

				_ => {},
			}

			return format!(
				".{function}({arguments}",
				function = capture.at(1).unwrap(),
				arguments = capture.at(2).unwrap()
			);
		}
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