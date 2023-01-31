use std::fs;
use std::path::PathBuf;

use minify_selectors_utils::*;
use parse_selectors;




#[test]
fn css_files() {
	let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/css/");

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

	// functions
	assert_eq!(
		fs::read_to_string(dir.clone().join("functions/output.css")).unwrap(),
		process_file("css", &dir.clone().join("functions/source.css")),
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

	// pseudo classes
	assert_eq!(
		fs::read_to_string(dir.clone().join("pseudo-classes/output.css")).unwrap(),
		process_file("css", &dir.clone().join("pseudo-classes/source.css")),
	);

	// pseudo elements
	assert_eq!(
		fs::read_to_string(dir.clone().join("pseudo-elements/output.css")).unwrap(),
		process_file("css", &dir.clone().join("pseudo-elements/source.css")),
	);
}

#[test]
fn js_files() {
	let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/js/");

	// classList
	assert_eq!(
		fs::read_to_string(dir.clone().join("class-list/output.js")).unwrap(),
		process_file("js", &dir.clone().join("class-list/source.js"))
	);

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

	// classList.item()
	assert_eq!(
		fs::read_to_string(dir.clone().join("class-list-item/output.js")).unwrap(),
		process_file("js", &dir.clone().join("class-list-item/source.js"))
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

	// classList.toggle()
	assert_eq!(
		fs::read_to_string(dir.clone().join("class-list-toggle/output.js")).unwrap(),
		process_file("js", &dir.clone().join("class-list-toggle/source.js"))
	);

	// classList.value
	assert_eq!(
		fs::read_to_string(dir.clone().join("class-list-value/output.js")).unwrap(),
		process_file("js", &dir.clone().join("class-list-value/source.js"))
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

	// matches()
	assert_eq!(
		fs::read_to_string(dir.clone().join("matches/output.js")).unwrap(),
		process_file("js", &dir.clone().join("matches/source.js"))
	);

	// getElementById()
	assert_eq!(
		fs::read_to_string(dir.clone().join("get-element-by-id/output.js")).unwrap(),
		process_file("js", &dir.clone().join("get-element-by-id/source.js"))
	);

	// getElementsByClassName()
	assert_eq!(
		fs::read_to_string(dir.clone().join("get-elements-by-class-name/output.js")).unwrap(),
		process_file(
			"js",
			&dir.clone().join("get-elements-by-class-name/source.js")
		)
	);

	// history.pushState()
	assert_eq!(
		fs::read_to_string(dir.clone().join("history-push-state/output.js")).unwrap(),
		process_file("js", &dir.clone().join("history-push-state/source.js"))
	);

	// history.replaceState()
	assert_eq!(
		fs::read_to_string(dir.clone().join("history-replace-state/output.js")).unwrap(),
		process_file("js", &dir.clone().join("history-replace-state/source.js"))
	);

	// id
	assert_eq!(
		fs::read_to_string(dir.clone().join("id/output.js")).unwrap(),
		process_file("js", &dir.clone().join("id/source.js"))
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

	// window.location
	assert_eq!(
		fs::read_to_string(dir.clone().join("location/output.js")).unwrap(),
		process_file("js", &dir.clone().join("location/source.js"))
	);

	// window.location.assign()
	assert_eq!(
		fs::read_to_string(dir.clone().join("window-location-assign/output.js")).unwrap(),
		process_file("js", &dir.clone().join("window-location-assign/source.js"))
	);

	// window.location.replace()
	assert_eq!(
		fs::read_to_string(dir.clone().join("window-location-replace/output.js")).unwrap(),
		process_file("js", &dir.clone().join("window-location-replace/source.js"))
	);

	// window.open()
	assert_eq!(
		fs::read_to_string(dir.clone().join("window-open/output.js")).unwrap(),
		process_file("js", &dir.clone().join("window-open/source.js"))
	);
}

#[test]
fn html_files() {
	let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/html/");

	// Anchor links
	assert_eq!(
		fs::read_to_string(dir.clone().join("anchor-links/output.html")).unwrap(),
		process_file("html", &dir.clone().join("anchor-links/source.html")),
	);

	// Attributes
	assert_eq!(
		fs::read_to_string(dir.clone().join("attributes/output.html")).unwrap(),
		process_file("html", &dir.clone().join("attributes/source.html")),
	);

	// Body only
	assert_eq!(
		fs::read_to_string(dir.clone().join("body-only/output.html")).unwrap(),
		process_file("html", &dir.clone().join("body-only/source.html")),
	);

	// Escaped chars
	assert_eq!(
		fs::read_to_string(dir.clone().join("escaped-chars/output.html")).unwrap(),
		process_file("html", &dir.clone().join("escaped-chars/source.html")),
	);

	// Edge cases
	assert_eq!(
		fs::read_to_string(dir.clone().join("edge-cases/output.html")).unwrap(),
		process_file("html", &dir.clone().join("edge-cases/source.html")),
	);

	// Placeholders
	assert_eq!(
		fs::read_to_string(dir.clone().join("prefixed-selectors/output.html")).unwrap(),
		process_file("html", &dir.clone().join("prefixed-selectors/source.html")),
	);
}

#[test]
fn svg_files() {
	let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/svg/");

	// General selector names
	assert_eq!(
		fs::read_to_string(dir.clone().join("general-selectors/output.svg")).unwrap(),
		process_file("svg", &dir.clone().join("general-selectors/source.svg")),
	);

	// Attributes
	assert_eq!(
		fs::read_to_string(dir.clone().join("attributes/output.svg")).unwrap(),
		process_file("svg", &dir.clone().join("attributes/source.svg")),
	);
}




fn process_file(
	file_type: &str,
	file_path: &PathBuf,
) -> String {
	let mut file = fs::read_to_string(file_path).unwrap();
	let mut selectors = Selectors::new();
	let mut config = Config::default();

	match file_type {
		"css" => parse_selectors::read_from_css(&mut file, &mut selectors, &config),
		"js" => parse_selectors::read_from_js(&mut file, &mut selectors, &config),
		"html" | "svg" => parse_selectors::read_from_html(&mut file, &mut selectors, &config),
		_ => panic!("file_type not one of the following: css, js, html or svg."),
	}

	config.current_step = ProcessingSteps::EncodingSelectors;
	selectors.process(&mut config);
	config.current_step = ProcessingSteps::WritingToFiles;

	match file_type {
		"css" => parse_selectors::write_to_css(&mut file, &mut selectors, &config),
		"js" => parse_selectors::write_to_js(&mut file, &mut selectors, &config),
		"html" | "svg" => parse_selectors::write_to_html(&mut file, &mut selectors, &config),
		_ => panic!("file_type not one of the following: css, js, html or svg."),
	}

	file.to_owned()
}
