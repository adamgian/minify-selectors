use std::fs;
use std::path::PathBuf;

use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use minify_selectors_utils::*;




pub fn parse_selectors_benchmarks(c: &mut Criterion) {
	c.bench_function("parse_selectors::from_css fn", |b| {
		b.iter(|| {
			process_file(
				"css",
				black_box(
					&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("benches/files/index.css"),
				),
			)
		})
	});

	c.bench_function("parse_selectors::from_html fn", |b| {
		b.iter(|| {
			process_file(
				"html",
				black_box(
					&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("benches/files/index.html"),
				),
			)
		})
	});

	c.bench_function("parse_selectors::from_js fn", |b| {
		b.iter(|| {
			process_file(
				"js",
				black_box(
					&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("benches/files/index.js"),
				),
			)
		})
	});
}

/// Implements mock multi-step processing as found in minify-selectors crate.
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

criterion_group!(benches, parse_selectors_benchmarks);
criterion_main!(benches);
