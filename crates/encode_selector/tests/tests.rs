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


#[test]
fn index_to_base62_standard() {
	let alphabet = into_alphabet_set(
		&"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
	);

	// 1 character length encoded selector names:
	// - 10 discarded
	// - 52 valid
	assert_eq!(to_radix(&0, &alphabet), "a".to_string());
	assert_eq!(to_radix(&1, &alphabet), "b".to_string());
	assert_eq!(to_radix(&2, &alphabet), "c".to_string());
	assert_eq!(to_radix(&51, &alphabet), "Z".to_string());

	// 2 character length encoded selector names:
	// - 620 discarded
	// - 3224 valid
	assert_eq!(to_radix(&52, &alphabet), "a0".to_string());
	assert_eq!(to_radix(&61, &alphabet), "a9".to_string());
	assert_eq!(to_radix(&62, &alphabet), "aa".to_string());
	assert_eq!(to_radix(&88, &alphabet), "aA".to_string());
	assert_eq!(to_radix(&114, &alphabet), "b0".to_string());
	assert_eq!(to_radix(&176, &alphabet), "c0".to_string());
	assert_eq!(to_radix(&1602, &alphabet), "z0".to_string());
	assert_eq!(to_radix(&1664, &alphabet), "A0".to_string());
	assert_eq!(to_radix(&3214, &alphabet), "Z0".to_string());
	assert_eq!(to_radix(&3249, &alphabet), "Zz".to_string());
	assert_eq!(to_radix(&3250, &alphabet), "ZA".to_string());
	assert_eq!(to_radix(&3275, &alphabet), "ZZ".to_string());

	// 3 character length encoded selector names:
	// - 38440 discarded
	// - 199888 valid
	assert_eq!(to_radix(&3276, &alphabet), "a00".to_string());
	assert_eq!(to_radix(&7120, &alphabet), "b00".to_string());
	assert_eq!(to_radix(&10964, &alphabet), "c00".to_string());
	assert_eq!(to_radix(&110908, &alphabet), "C00".to_string());
	assert_eq!(to_radix(&110970, &alphabet), "C10".to_string());
	assert_eq!(to_radix(&111528, &alphabet), "Ca0".to_string());
	assert_eq!(to_radix(&111537, &alphabet), "Ca9".to_string());
	assert_eq!(to_radix(&111538, &alphabet), "Caa".to_string());
	assert_eq!(to_radix(&111564, &alphabet), "CaA".to_string());
	assert_eq!(to_radix(&111590, &alphabet), "Cb0".to_string());
	assert_eq!(to_radix(&203163, &alphabet), "ZZZ".to_string());

	// 4 character length encoded selector names:
	// - 2383280 discarded
	// - 12393056 valid
	assert_eq!(to_radix(&203164, &alphabet), "a000".to_string());
	assert_eq!(to_radix(&12596219, &alphabet), "ZZZZ".to_string());
}


#[test]
fn index_to_base62_nums_last() {
	let alphabet = into_alphabet_set(
		&"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
	);

	// 1 character length encoded selector names:
	// - 10 discarded
	// - 52 valid
	assert_eq!(to_radix(&0, &alphabet), "a".to_string());
	assert_eq!(to_radix(&51, &alphabet), "Z".to_string());

	// 2 character length encoded selector names:
	// - 620 discarded
	// - 3224 valid
	assert_eq!(to_radix(&52, &alphabet), "aa".to_string());
	assert_eq!(to_radix(&77, &alphabet), "az".to_string());
	assert_eq!(to_radix(&78, &alphabet), "aA".to_string());
	assert_eq!(to_radix(&103, &alphabet), "aZ".to_string());
	assert_eq!(to_radix(&104, &alphabet), "a0".to_string());
	assert_eq!(to_radix(&113, &alphabet), "a9".to_string());
	assert_eq!(to_radix(&114, &alphabet), "ba".to_string());
	assert_eq!(to_radix(&176, &alphabet), "ca".to_string());
	assert_eq!(to_radix(&1602, &alphabet), "za".to_string());
	assert_eq!(to_radix(&1664, &alphabet), "Aa".to_string());
	assert_eq!(to_radix(&3214, &alphabet), "Za".to_string());
	assert_eq!(to_radix(&3239, &alphabet), "Zz".to_string());
	assert_eq!(to_radix(&3240, &alphabet), "ZA".to_string());
	assert_eq!(to_radix(&3265, &alphabet), "ZZ".to_string());
	assert_eq!(to_radix(&3275, &alphabet), "Z9".to_string());

	// 3 character length encoded selector names:
	// - 38440 discarded
	// - 199888 valid
	assert_eq!(to_radix(&3276, &alphabet), "aaa".to_string());
	assert_eq!(to_radix(&3301, &alphabet), "aaz".to_string());
	assert_eq!(to_radix(&3301, &alphabet), "aaz".to_string());
	assert_eq!(to_radix(&4826, &alphabet), "aza".to_string());
	assert_eq!(to_radix(&4888, &alphabet), "aAa".to_string());
	assert_eq!(to_radix(&6438, &alphabet), "aZa".to_string());
	assert_eq!(to_radix(&6463, &alphabet), "aZz".to_string());
	assert_eq!(to_radix(&6464, &alphabet), "aZA".to_string());
	assert_eq!(to_radix(&6489, &alphabet), "aZZ".to_string());
	assert_eq!(to_radix(&7119, &alphabet), "a99".to_string());
	assert_eq!(to_radix(&7120, &alphabet), "baa".to_string());
	assert_eq!(to_radix(&7121, &alphabet), "bab".to_string());
	assert_eq!(to_radix(&10964, &alphabet), "caa".to_string());
	assert_eq!(to_radix(&103220, &alphabet), "Aaa".to_string());
	assert_eq!(to_radix(&199320, &alphabet), "Zaa".to_string());
	assert_eq!(to_radix(&203163, &alphabet), "Z99".to_string());

	// 4 character length encoded selector names:
	// - 2383280 discarded
	// - 12393056 valid
	assert_eq!(to_radix(&203164, &alphabet), "aaaa".to_string());
	assert_eq!(to_radix(&12596219, &alphabet), "Z999".to_string());
}


#[test]
fn index_to_base62_nums_scattered() {
	let alphabet = into_alphabet_set(
		&"a0bc1d234ef5ghijklmn6opqr78s9tuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
	);

	// 1 character length encoded selector names:
	// - 10 discarded
	// - 52 valid
	assert_eq!(to_radix(&0, &alphabet), "a".to_string());
	assert_eq!(to_radix(&1, &alphabet), "b".to_string());
	assert_eq!(to_radix(&2, &alphabet), "c".to_string());
	assert_eq!(to_radix(&3, &alphabet), "d".to_string());
	assert_eq!(to_radix(&4, &alphabet), "e".to_string());
	assert_eq!(to_radix(&10, &alphabet), "k".to_string());
	assert_eq!(to_radix(&20, &alphabet), "u".to_string());
	assert_eq!(to_radix(&51, &alphabet), "Z".to_string());

	// 2 character length encoded selector names:
	// - 620 discarded
	// - 3224 valid
	assert_eq!(to_radix(&52, &alphabet), "aa".to_string());
	assert_eq!(to_radix(&53, &alphabet), "a0".to_string());
	assert_eq!(to_radix(&114, &alphabet), "ba".to_string());
	assert_eq!(to_radix(&176, &alphabet), "ca".to_string());
	assert_eq!(to_radix(&238, &alphabet), "da".to_string());
	assert_eq!(to_radix(&300, &alphabet), "ea".to_string());
	assert_eq!(to_radix(&672, &alphabet), "ka".to_string());
	assert_eq!(to_radix(&1292, &alphabet), "ua".to_string());
	assert_eq!(to_radix(&3242, &alphabet), "Z9".to_string());
	assert_eq!(to_radix(&3275, &alphabet), "ZZ".to_string());

	// 3 character length encoded selector names:
	// - 38440 discarded
	// - 199888 valid
	assert_eq!(to_radix(&3276, &alphabet), "aaa".to_string());
	assert_eq!(to_radix(&203163, &alphabet), "ZZZ".to_string());

	// 4 character length encoded selector names:
	// - 2383280 discarded
	// - 12393056 valid
	assert_eq!(to_radix(&203164, &alphabet), "aaaa".to_string());
	assert_eq!(to_radix(&12596219, &alphabet), "ZZZZ".to_string());
}


#[test]
fn index_to_base64_custom() {
	let alphabet = into_alphabet_set(
		&"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ-_"
	);

	// 1 character length encoded selector names:
	// - 12 discarded
	// - 52 valid
	assert_eq!(to_radix(&0, &alphabet), "a".to_string());
	assert_eq!(to_radix(&1, &alphabet), "b".to_string());
	assert_eq!(to_radix(&2, &alphabet), "c".to_string());
	assert_eq!(to_radix(&51, &alphabet), "Z".to_string());

	// 2 character length encoded selector names:
	// - 768 discarded
	// - 3328 valid
	assert_eq!(to_radix(&52, &alphabet), "a0".to_string());
	assert_eq!(to_radix(&3379, &alphabet), "Z_".to_string());

	// 3 character length encoded selector names:
	// - 49152 discarded
	// - 212992 valid
	assert_eq!(to_radix(&3380, &alphabet), "a00".to_string());
	assert_eq!(to_radix(&216370, &alphabet), "Z_-".to_string());
	assert_eq!(to_radix(&216371, &alphabet), "Z__".to_string());

	// 4 character length encoded selector names:
	// - 2859936 discarded
	// - 13631488 valid
	assert_eq!(to_radix(&216372, &alphabet), "a000".to_string());
	assert_eq!(to_radix(&13847858, &alphabet), "Z__-".to_string());
	assert_eq!(to_radix(&13847859, &alphabet), "Z___".to_string());
}


#[test]
fn index_to_base26_latin_letters() {
	let alphabet = into_alphabet_set(
		&"abcdefghijklmnopqrstuvwxyz"
	);

	// 1 character length encoded selector names:
	// - 26 valid combinations
	assert_eq!(to_radix(&0, &alphabet), "a".to_string());
	assert_eq!(to_radix(&1, &alphabet), "b".to_string());
	assert_eq!(to_radix(&2, &alphabet), "c".to_string());
	assert_eq!(to_radix(&10, &alphabet), "k".to_string());
	assert_eq!(to_radix(&20, &alphabet), "u".to_string());
	assert_eq!(to_radix(&25, &alphabet), "z".to_string());

	// 2 character length encoded selector names:
	// - 676 valid combinations
	assert_eq!(to_radix(&26, &alphabet), "aa".to_string());
	assert_eq!(to_radix(&36, &alphabet), "ak".to_string());
	assert_eq!(to_radix(&46, &alphabet), "au".to_string());
	assert_eq!(to_radix(&51, &alphabet), "az".to_string());
	assert_eq!(to_radix(&52, &alphabet), "ba".to_string());
	assert_eq!(to_radix(&701, &alphabet), "zz".to_string());

	// 3 character length encoded selector names:
	// - 17576 valid combinations
	assert_eq!(to_radix(&702, &alphabet), "aaa".to_string());
	assert_eq!(to_radix(&712, &alphabet), "aak".to_string());
	assert_eq!(to_radix(&722, &alphabet), "aau".to_string());
	assert_eq!(to_radix(&18277, &alphabet), "zzz".to_string());

	// 4 character length encoded selector names:
	// - 456976 valid combinations
	assert_eq!(to_radix(&18278, &alphabet), "aaaa".to_string());
	assert_eq!(to_radix(&475253, &alphabet), "zzzz".to_string());
}
