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
			(?<quote>
				(?:\\?["'])?
			)
			(?<value>
				-?
				(?>
					[A-Za-z_]
					| [^\0-\177]
					| (?>
						\\[0-9A-Fa-f]{1,6}(?>\r\n|[ \n\r\t\f])?
						| \\[^\n\r\f0-9A-Fa-f"']
					)
				)
				(?>
					[\w\-]
					| [^\0-\177]
					| (?>
						\\[0-9A-Fa-f]{1,6}(?>\r\n|[ \n\r\t\f])?
						| \\[^\n\r\f0-9A-Fa-f"']
					)
				)*
			)
			(\\?["'])?
			(?<flag>
				\s*+
				[IiSs]?
			)
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
			\/\*[^*]*\*+(?>[^\/*][^*]*\*+)*\/
			|
			\/\/[^\n\r]*
			|
			\.
			(?<function>
				insertAdjacentHTML
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
			(?<arguments>
				(?:
					\s*
					(?:
						(?:
							`
							(?:
								[^`\\] | \\.
							)*
							[^)]`
						)
						| (?:
							"
							(?:
								[^"\\] | \\.
							)*
							[^)]"
						)
						| (?:
							'
							(?:
								[^'\\] | \\.
							)*
							[^)]'
						)
					)
					(?:\s*,)?
				)++
			)
		"##
	).unwrap();

	// Extract value from JS property operations.
	static ref JS_PROPERTIES : Regex = Regex::new(
		r##"(?x)
			\/\*[^*]*\*+(?>[^\/*][^*]*\*+)*\/
			|
			\/\/[^\n\r]*
			|
			\.
			(?<function>
				className
				| innerHTML
				| outerHTML
			)
			(?<join>
				\s*+[=+\-!<>]{1,3}\s*+
			)
			(?<value>
				(?:
					`
					(?:
						[^`\\] | \\.
					)*
					[^)]`
				)
				| (?:
					"
					(?:
						[^"\\] | \\.
					)*
					[^)]"
				)
				| (?:
					'
					(?:
						[^'\\] | \\.
					)*
					[^)]'
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
			(?<body>
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
	// Capture HTML comments to prevent false positive matches
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
			<!--.*?-->
			|
			<code[^>]*>
				(?:
					. | \s
				)*?
			<\/code>
			|
			(?<attribute>
				[^\s\x00\/>"'=]+
			)
			(?<join>
				\s*=\s*
			)
			(?<value>
				[^\s\\<>"'=]+
				| \\?"(?:[^\\<>"=] | \\[^"'])+
				| \\?'(?:[^\\<>'=] | \\[^"'])+
			)
			(?<quote>
				(?:\\?["'])?
			)
		"##
	).unwrap();

	// Extract ID from anchor links.
	static ref HREF_ANCHOR_LINKS: Regex = Regex::new(
		r##"(?x)
			\# (?<anchor>[^#])*+$
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
				(?:
					(?<=")
					[^"]*
				)
				|
				(?:
					(?<=')
					[^']*
				)
				|
				(?:
					(?<=`)
					[^`]*
				)
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

		// <a href="/#foo"></a>
		(String::from("href"), String::from("anchor")),
	]);
}




pub fn from_css(
	file_string: &mut String,
	selectors: &mut HashMap<String, String>,
	index: &mut usize,
	alphabet: &[char]
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
	alphabet: &[char]
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
	alphabet: &[char]
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

			encoded_selector
		}
	}
}

fn process_css(
	file_string: &mut String,
	selectors: &mut HashMap<String, String>,
	index: &mut usize,
	alphabet: &[char]
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
}

/// Process HTML.
fn process_html(
	file_string: &mut String,
	selectors: &mut HashMap<String, String>,
	index: &mut usize,
	alphabet: &[char]
) {
	// Initial step — go through <body> and parse attributes
	let mut html: String = HTML_BODY.replace_all(
		file_string,
		|capture: &Captures| {
			let mut body = capture.at(0).unwrap().to_owned();

			process_html_attributes(
				&mut body,
				selectors,
				index,
				alphabet
			);

			body
		}
	);

	// Processing any embedded styles
	// Create subset string(s) to process <style> embeds
	html = HTML_STYLE_ELEMENT.replace_all(
		&html,
		|capture: &Captures| {
			let mut embedded_style = capture.at(2).unwrap().to_owned();

			process_css(
				&mut embedded_style,
				selectors,
				index,
				alphabet
			);

			return format!(
				"{tag_open}{styles}{tag_close}",
				tag_open = capture.at(1).unwrap(),
				styles = embedded_style,
				tag_close = capture.at(3).unwrap()
			);
		}
	);

	// Processing any embedded js
	process_js(
		&mut html,
		selectors,
		index,
		alphabet
	);

	*file_string = html;
}

/// Process Javascript.
fn process_js(
	file_string: &mut String,
	selectors: &mut HashMap<String, String>,
	index: &mut usize,
	alphabet: &[char]
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
}




/// Process classes and IDs in CSS file/embed or as a
/// CSS selector string.
fn process_css_selectors(
	file_string: &mut String,
	selectors: &mut HashMap<String, String>,
	index: &mut usize,
	alphabet: &[char]
) {
	*file_string = CSS_SELECTORS.replace_all(
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
	file_string: &mut String,
	selectors: &mut HashMap<String, String>,
	index: &mut usize,
	alphabet: &[char]
) {
	*file_string = CSS_ATTRIBUTES.replace_all(
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
	alphabet: &[char]
) {
	*file_string = HTML_ATTRIBUTES.replace_all(
		file_string,
		|capture: &Captures| {
			// Matched string is a <code> element or a HTML comment.
			if capture.at(1).is_none() {
				return match capture.at(0).unwrap().starts_with("<code") {
					// HTML comment, leave as is.
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

					// attribute_value will need to be cleaned up, as 'HTML_ATTRIBUTES'
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
	alphabet: &[char]
) {
	*file_string = JS_ARGUMENTS.replace_all(
		file_string,
		|capture: &Captures| {
			// Matched string is a multiline or single line comment
			// i.e. it does not have any further capture groups
			if capture.at(1).is_none() {
				return capture.at(0).unwrap().to_string();
			}

			let mut replacement_args: String = capture.at(3).unwrap().to_string();
			let function: &str = capture.at(1).unwrap();

			// Work out function call and its argument pattern:
			match function {
				// Takes one argument, an CSS selector string.
				"querySelector" | "querySelectorAll" | "closest" => {
					process_css(
						&mut replacement_args,
						selectors,
						index,
						alphabet
					);
				},

				// Takes one argument, a string of classes (no period prefixed)
				// separated by spaces (if more than one) —
				"getElementsByClassName" => {
					process_string_of_tokens(
						&mut replacement_args,
						selectors,
						index,
						alphabet,
						"class"
					);
				},

				// Takes one argument, an ID (no hash prefixed).
				"getElementById" => {
					process_string_of_tokens(
						&mut replacement_args,
						selectors,
						index,
						alphabet,
						"id"
					);
				},

				// Takes two arguments: attribute name and value,
				// process value if attribute is whitelisted.
				"setAttribute" => {
					// Go over the (two) function arguments
					let mut function_args = STRING_DELIMITED_BY_COMMA
						.captures_iter(&replacement_args);

					// Check first arg in function, without the string delimiters
					// and then trimming any whitespace off ends.
					let attribute_name: &str = function_args
						.next()
						.unwrap()
						.at(1)
						.unwrap()
						.trim();

					// Check first argument is an known attribute which its value will have
					// classses or an id. If it is not, leave value as is (second argument).
					if ATTRIBUTES_WHITELIST.contains_key(attribute_name) {

						let attribute_value: String = function_args
							.next()
							.unwrap()
							.at(1)
							.unwrap()
							.to_string();
						let mut replacement_value = attribute_value.clone();

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
							&attribute_value,
							&replacement_value,
						);

					}
				},

				// Takes two arguments: position and html,
				// we are only interested in the latter argument.
				"insertAdjacentHTML" => {
					let html: String = STRING_DELIMITED_BY_COMMA
						.captures_iter(&replacement_args)
						.last()
						.unwrap()
						.at(1)
						.unwrap()
						.to_string();

					let mut replacement_html = html.clone();

					match html.contains("</body>") {
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
						&html,
						&replacement_html,
					);
				},

				// Takes one or more arguments, each argument is for
				// an individual class name (no period prefixed).
				_ if function.contains("classList") => {
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
				".{function}{join}{arguments}",
				function = function,
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
	alphabet: &[char]
) {
	*file_string = JS_PROPERTIES.replace_all(
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
				"className" => {
					process_string_of_tokens(
						&mut property_value,
						selectors,
						index,
						alphabet,
						"class"
					);
					return format!(
						".{name}{operator}{value}",
						name = property_name,
						operator = capture.at(2).unwrap(),
						value = property_value,
					);
				},
				"innerHTML" | "outerHTML" => {
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

					return format!(
						".{name}{operator}{value}",
						name = property_name,
						operator = capture.at(2).unwrap(),
						value = property_value,
					);
				},
				_ => {},
			}

			return capture.at(0).unwrap().to_string();
		}
	);
}

/// Process string with tokens delimited by whitespaces.
///
/// Notes:
///  - As STRING_DELIMITED_BY_SPACE regex is simple - only
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
	context: &str
) {
	let prefix: &str = match context {
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

	// Trim quotes (if any) from value capture group.
	if !quote_type.is_empty() {
		string.pop();
		string.remove(0);
	}

	*string = format!(
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
	context: &str
) {
	let prefix: &str = match context {
		"class" => { "." },
		"id" => { "#" },
		_ => { "" },
	};

	*string = STRING_DELIMITED_BY_COMMA.replace_all(
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

// Process target IDs in anchor link URLs.
fn process_anchor_links(
	string: &mut String,
	selectors: &mut HashMap<String, String>,
	index: &mut usize,
	alphabet: &[char]
) {
	*string = HREF_ANCHOR_LINKS.replace(
		string,
		|capture: &Captures| {
			format!(
				"#{}",
				get_encoded_selector(
					&capture.at(0).unwrap().to_owned(),
					selectors,
					index,
					alphabet
				)
			)
		}
	);
}
