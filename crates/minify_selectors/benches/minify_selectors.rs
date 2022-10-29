use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;




pub fn minify_selectors_benchmarks(c: &mut Criterion) {
	c.bench_function(
		"todo",
		|b| b.iter(|| {})
	);
}

criterion_group!(benches, minify_selectors_benchmarks);
criterion_main!(benches);
