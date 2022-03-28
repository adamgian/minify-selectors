use lazy_static::lazy_static;
use onig::*;
use std::collections::HashMap;

use encode_selector;




lazy_static! {
	static ref SELECTORS_IN_CSS: Regex = Regex::new(
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

	// FIXME
	static ref SELECTORS_IN_JS: Regex = Regex::new(
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

	// FIXME
	// https://html.spec.whatwg.org/#valid-custom-element-name
	static ref SELECTORS_IN_HTML: Regex = Regex::new(
		r##"(?x)
			(?<attribute>
				[^\f\n\t\ \>\"\'\=]++
			)
			=[\"\']?+
			(?<value>
				(?(?<=[\"\'])
					[\w\-\ ]*+
					| [\w\-]*+
				)
			)
			(?=
				[\"\']?+
				(?>
					[\ ]++
					[^\f\n\t\ \>\"\'\=]++
					(?>
						=
						[^\ \>]++
					)?+
				)*+
				[\s]*+\/?>
			)
		"##
	).unwrap();
}




pub fn from_css(
	file_string: &mut String,
	selectors: &mut HashMap<String, String>,
	index: &mut u32
) -> String {
	return SELECTORS_IN_CSS.replace_all(
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

			return format!(
				"{prefix}{identifier}",
				prefix = selectors
					.get_key_value(capture.at(0).unwrap())
					.unwrap().0.
					chars().next().unwrap(),
				identifier = selectors
					.get_key_value(capture.at(0).unwrap())
					.unwrap().1
			);
		}
	);
}

// pub fn from_html() -> String {}

// pub fn from_js() -> String {}