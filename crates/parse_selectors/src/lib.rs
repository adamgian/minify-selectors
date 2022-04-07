use lazy_static::lazy_static;
use onig::*;
use std::{
	collections::HashMap,
	collections::HashSet,
};

use encode_selector;




lazy_static! {
	// Current limitations:
	// - classes or ids in attribute selectors
	//   (i.e. [id="foo"], [class="bar"])
	static ref CSS_CLASSES_OR_IDS: Regex = Regex::new(
		r##"(?x)
			(?<type>[\#\.])
			(?<name>
				(?>[A-Za-z\_]|\-[A-Za-z\_])
				[\w\-]*+
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

	// Extracts arguments from DOM
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
					[\,]?
					[\w\-\ \#\.\*\:\>\[\]\+\~\"\']*+
				)
			)
		"##
	).unwrap();

	// Extracts all attributes with values
	//
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
	return CSS_CLASSES_OR_IDS.replace_all(
		&file_string,
		|capture: &Captures| {
			if !selectors.contains_key(&capture.at(0).unwrap().to_owned()) {
				*index += 1;
				let encoded_selector: String = encode_selector::to_base62(index);

				selectors.insert(
					capture.at(0).unwrap().to_owned(),
					encoded_selector.clone()
				);

				return format!(
					"{prefix}{identifier}",
					prefix = &capture.at(1).unwrap(),
					identifier = encoded_selector
				);
			}
			else {
				return format!(
					"{prefix}{identifier}",
					prefix = selectors
						.get_key_value(capture.at(0).unwrap())
						.unwrap().0
						.chars().next().unwrap(),
					identifier = selectors
						.get_key_value(capture.at(0).unwrap())
						.unwrap().1
				);
			}
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
					let selector: String = format!("{}{}", prefix, value);

					// Adding space between values
					if !replacement_value.is_empty() {
						replacement_value.push_str(" ");
					}

					if !selectors.contains_key(&selector) {
						*index += 1;
						let encoded_selector: String = encode_selector::to_base62(index);

						selectors.insert(
							selector.to_owned(),
							encoded_selector.clone()
						);

						replacement_value.push_str(&encoded_selector);
					}
					else {
						replacement_value.push_str(
							selectors
								.get_key_value(capture.at(0).unwrap())
								.unwrap().1
						);
					}
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

// pub fn from_js() -> String {}