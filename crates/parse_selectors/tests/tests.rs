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

	// at rules
	assert_eq!(
		fs::read_to_string(dir.clone().join("at-rules/output.css")).unwrap(),
		process_file("css", &dir.clone().join("at-rules/source.css")),
	);

	// attribute selectors
	assert_eq!(
		fs::read_to_string(dir.clone().join("attribute-selectors/output.css")).unwrap(),
		process_file("css", &dir.clone().join("attribute-selectors/source.css")),
	);

	// comments
	assert_eq!(
		fs::read_to_string(dir.clone().join("comments/output.css")).unwrap(),
		process_file("css", &dir.clone().join("comments/source.css")),
	);

	// general selectors
	assert_eq!(
		fs::read_to_string(dir.clone().join("general-selectors/output.css")).unwrap(),
		process_file("css", &dir.clone().join("general-selectors/source.css")),
	);

	// nesting
	assert_eq!(
		fs::read_to_string(dir.clone().join("nesting/output.css")).unwrap(),
		process_file("css", &dir.clone().join("nesting/source.css")),
	);
}

#[test]
fn js_files() {
	let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
		.join("tests/js/");

	// classList.add()
	assert_eq!(
		fs::read_to_string(dir.clone().join("class-list-add/output.js")).unwrap(),
		process_file("js", &dir.clone().join("class-list-add/source.js"))
	);

	// classList.contains()
	assert_eq!(
		fs::read_to_string(dir.clone().join("class-list-contains/output.js")).unwrap(),
		process_file("js", &dir.clone().join("class-list-contains/source.js"))
	);

	// classList.remove()
	assert_eq!(
		fs::read_to_string(dir.clone().join("class-list-remove/output.js")).unwrap(),
		process_file("js", &dir.clone().join("class-list-remove/source.js"))
	);

	// classList.replace()
	assert_eq!(
		fs::read_to_string(dir.clone().join("class-list-replace/output.js")).unwrap(),
		process_file("js", &dir.clone().join("class-list-replace/source.js"))
	);

	// className
	assert_eq!(
		fs::read_to_string(dir.clone().join("class-name/output.js")).unwrap(),
		process_file("js", &dir.clone().join("class-name/source.js"))
	);

	// closest()
	assert_eq!(
		fs::read_to_string(dir.clone().join("closest/output.js")).unwrap(),
		process_file("js", &dir.clone().join("closest/source.js"))
	);

	// getElementById()
	assert_eq!(
		fs::read_to_string(dir.clone().join("get-element-by-id/output.js")).unwrap(),
		process_file("js", &dir.clone().join("get-element-by-id/source.js"))
	);

	// getElementsByClassName()
	assert_eq!(
		fs::read_to_string(dir.clone().join("get-elements-by-class-name/output.js")).unwrap(),
		process_file("js", &dir.clone().join("get-elements-by-class-name/source.js"))
	);

	// innerHTML()
	assert_eq!(
		fs::read_to_string(dir.clone().join("inner-html/output.js")).unwrap(),
		process_file("js", &dir.clone().join("inner-html/source.js"))
	);

	// insertAdjacentHTML()
	assert_eq!(
		fs::read_to_string(dir.clone().join("insert-adjacent-html/output.js")).unwrap(),
		process_file("js", &dir.clone().join("insert-adjacent-html/source.js"))
	);

	// outerHTML()
	assert_eq!(
		fs::read_to_string(dir.clone().join("outer-html/output.js")).unwrap(),
		process_file("js", &dir.clone().join("outer-html/source.js"))
	);

	// querySelector()
	assert_eq!(
		fs::read_to_string(dir.clone().join("query-selector/output.js")).unwrap(),
		process_file("js", &dir.clone().join("query-selector/source.js"))
	);

	// querySelectorAll()
	assert_eq!(
		fs::read_to_string(dir.clone().join("query-selector-all/output.js")).unwrap(),
		process_file("js", &dir.clone().join("query-selector-all/source.js"))
	);

	// setAttribute()
	assert_eq!(
		fs::read_to_string(dir.clone().join("set-attribute/output.js")).unwrap(),
		process_file("js", &dir.clone().join("set-attribute/source.js"))
	);
}

#[test]
fn html_files() {
	let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
		.join("tests/html/");

	// Attributes
	assert_eq!(
		fs::read_to_string(dir.clone().join("attributes/output.html")).unwrap(),
		process_file("html", &dir.clone().join("attributes/source.html")),
	);

	// Edge cases
	assert_eq!(
		fs::read_to_string(dir.clone().join("edge-cases/output.html")).unwrap(),
		process_file("html", &dir.clone().join("edge-cases/source.html")),
	);
}




fn process_file(
	file_type: &str,
	file_path: &PathBuf,
) -> String {
	let mut file = fs::read_to_string(file_path).unwrap();
	let alphabet: Vec<char> = encode_selector::into_alphabet_set(
		concat!(
			"0123456789",
			"abcdefghijklmnopqrstuvwxyz",
			"ABCDEFGHIJKLMNOPQRSTUVWXYZ",
		)
	);

	if file_type == "css" {
		parse_selectors::from_css(
			&mut file,
			&mut HashMap::<String, String>::new(),
			&mut usize::from(false),
			&alphabet
		);
	} else if file_type == "js" {
		parse_selectors::from_js(
			&mut file,
			&mut HashMap::<String, String>::new(),
			&mut usize::from(false),
			&alphabet
		);
	} else if file_type == "html" {
		parse_selectors::from_html(
			&mut file,
			&mut HashMap::<String, String>::new(),
			&mut usize::from(false),
			&alphabet
		);
	}

	file.to_owned()
}
