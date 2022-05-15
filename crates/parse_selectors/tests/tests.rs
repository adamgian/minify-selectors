use std::{
	collections::HashMap,
	fs,
	path::PathBuf,
};

use parse_selectors;
use encode_selector;





#[test]
fn css_files() {
	let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
		.join("tests/css/");
	let alphabet: Vec<char> = encode_selector::into_alphabet_set(
		concat!(
			"0123456789",
			"abcdefghijklmnopqrstuvwxyz",
			"ABCDEFGHIJKLMNOPQRSTUVWXYZ",
		)
	);

	// at rules
	assert_eq!(
		fs::read_to_string(dir.clone().join("at-rules/output.css")).unwrap(),
		parse_selectors::from_css(
			&mut fs::read_to_string(dir.clone().join("at-rules/source.css")).unwrap(),
			&mut HashMap::<String, String>::new(),
			&mut usize::from(false),
			&alphabet
		)
	);

	// attribute selectors
	assert_eq!(
		fs::read_to_string(dir.clone().join("attribute-selectors/output.css")).unwrap(),
		parse_selectors::from_css(
			&mut fs::read_to_string(dir.clone().join("attribute-selectors/source.css")).unwrap(),
			&mut HashMap::<String, String>::new(),
			&mut usize::from(false),
			&alphabet
		)
	);

	// comments
	assert_eq!(
		fs::read_to_string(dir.clone().join("comments/output.css")).unwrap(),
		parse_selectors::from_css(
			&mut fs::read_to_string(dir.clone().join("comments/source.css")).unwrap(),
			&mut HashMap::<String, String>::new(),
			&mut usize::from(false),
			&alphabet
		)
	);

	// general selectors
	assert_eq!(
		fs::read_to_string(dir.clone().join("general-selectors/output.css")).unwrap(),
		parse_selectors::from_css(
			&mut fs::read_to_string(dir.clone().join("general-selectors/source.css")).unwrap(),
			&mut HashMap::<String, String>::new(),
			&mut usize::from(false),
			&alphabet
		)
	);

	// nesting
	assert_eq!(
		fs::read_to_string(dir.clone().join("nesting/output.css")).unwrap(),
		parse_selectors::from_css(
			&mut fs::read_to_string(dir.clone().join("nesting/source.css")).unwrap(),
			&mut HashMap::<String, String>::new(),
			&mut usize::from(false),
			&alphabet
		)
	);
}

#[test]
fn js_files() {
	let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
		.join("tests/js/");
	let alphabet: Vec<char> = encode_selector::into_alphabet_set(
		concat!(
			"0123456789",
			"abcdefghijklmnopqrstuvwxyz",
			"ABCDEFGHIJKLMNOPQRSTUVWXYZ",
		)
	);

	// classList.add()
	assert_eq!(
		fs::read_to_string(dir.clone().join("class-list-add/output.js")).unwrap(),
		parse_selectors::from_js(
			&mut fs::read_to_string(dir.clone().join("class-list-add/source.js")).unwrap(),
			&mut HashMap::<String, String>::new(),
			&mut usize::from(false),
			&alphabet
		)
	);

	// classList.contains()
	assert_eq!(
		fs::read_to_string(dir.clone().join("class-list-contains/output.js")).unwrap(),
		parse_selectors::from_js(
			&mut fs::read_to_string(dir.clone().join("class-list-contains/source.js")).unwrap(),
			&mut HashMap::<String, String>::new(),
			&mut usize::from(false),
			&alphabet
		)
	);

	// classList.remove()
	assert_eq!(
		fs::read_to_string(dir.clone().join("class-list-remove/output.js")).unwrap(),
		parse_selectors::from_js(
			&mut fs::read_to_string(dir.clone().join("class-list-remove/source.js")).unwrap(),
			&mut HashMap::<String, String>::new(),
			&mut usize::from(false),
			&alphabet
		)
	);

	// classList.replace()
	assert_eq!(
		fs::read_to_string(dir.clone().join("class-list-replace/output.js")).unwrap(),
		parse_selectors::from_js(
			&mut fs::read_to_string(dir.clone().join("class-list-replace/source.js")).unwrap(),
			&mut HashMap::<String, String>::new(),
			&mut usize::from(false),
			&alphabet
		)
	);

	// className
	assert_eq!(
		fs::read_to_string(dir.clone().join("class-name/output.js")).unwrap(),
		parse_selectors::from_js(
			&mut fs::read_to_string(dir.clone().join("class-name/source.js")).unwrap(),
			&mut HashMap::<String, String>::new(),
			&mut usize::from(false),
			&alphabet
		)
	);

	// querySelector()
	assert_eq!(
		fs::read_to_string(dir.clone().join("query-selector/output.js")).unwrap(),
		parse_selectors::from_js(
			&mut fs::read_to_string(dir.clone().join("query-selector/source.js")).unwrap(),
			&mut HashMap::<String, String>::new(),
			&mut usize::from(false),
			&alphabet
		)
	);
}

#[test]
fn html_files() {
	//
}
