use encode_selector::*;




// A CSS class and ID selector name cannot start with
// 0, 1, 2, 3, 4, 5, 6, 7, 8, 9 - and for simplicity - '-' and '_'.
// to_radix() should 'skip' over any index that gives an invalid
// starting character.
//
// Amount of discarded names for a given character length can be calculated
// with formula: x * (b ^ n-1) — where x is the number of 'skipped'
// characters; b is length of the alphabet; and n is the encoded character
// length.
//
// Amount of valid names for a given character length can be calculated
// with the formula: x * (b ^ n) — where x is the number of 'skipped'
// characters; b is length of the alphabet; and n is the encoded character
// length.


// Given the following alphabet:
// 0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz
#[test]
fn index_mapping() {
	// 1 character length encoded selector names:
	// - 10 discarded
	// - 52 valid
	assert_eq!(to_base62(&1), "A".to_string());
	assert_eq!(to_base62(&52), "z".to_string());
	// 2 character length encoded selector names:
	// - 620 discarded
	// - 3224 valid
	assert_eq!(to_base62(&53), "A0".to_string());
	assert_eq!(to_base62(&3276), "zz".to_string());
	// 3 character length encoded selector names:
	// - 38440 discarded
	// - 199888 valid
	assert_eq!(to_base62(&3277), "A00".to_string());
	assert_eq!(to_base62(&203164), "zzz".to_string());
	// 4 character length encoded selector names:
	// - 2383280 discarded
	// - 12393056 valid
	assert_eq!(to_base62(&203165), "A000".to_string());
	assert_eq!(to_base62(&12596220), "zzzz".to_string());
}
