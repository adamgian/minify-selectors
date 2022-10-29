use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

use encode_selector::*;




pub fn encode_selector_benchmarks(c: &mut Criterion) {
	let alphabet =
		into_alphabet_set(&"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ");

	c.bench_function(
		"encode_selector::to_radix fn (into 1 character length radix)",
		|b| b.iter(|| to_radix(black_box(&51), &alphabet)),
	);

	c.bench_function(
		"encode_selector::to_radix fn (into 2 character length radix)",
		|b| b.iter(|| to_radix(black_box(&3275), &alphabet)),
	);

	c.bench_function(
		"encode_selector::to_radix fn (into 3 character length radix)",
		|b| b.iter(|| to_radix(black_box(&203163), &alphabet)),
	);

	c.bench_function(
		"encode_selector::to_radix fn (into 4 character length radix)",
		|b| b.iter(|| to_radix(black_box(&12596219), &alphabet)),
	);
}

criterion_group!(benches, encode_selector_benchmarks);
criterion_main!(benches);
