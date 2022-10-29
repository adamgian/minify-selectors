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
	#[rustfmt::skip]
	let alphabet = into_alphabet_set(
		&"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
	);

	// 1 character length encoded selector names:
	// - 10 discarded
	// - 52 valid
	assert_eq!("a".to_string(), to_radix(&0, &alphabet));
	assert_eq!("b".to_string(), to_radix(&1, &alphabet));
	assert_eq!("c".to_string(), to_radix(&2, &alphabet));
	assert_eq!("Z".to_string(), to_radix(&51, &alphabet));

	// 2 character length encoded selector names:
	// - 620 discarded
	// - 3224 valid
	assert_eq!("a0".to_string(), to_radix(&52, &alphabet));
	assert_eq!("a9".to_string(), to_radix(&61, &alphabet));
	assert_eq!("aa".to_string(), to_radix(&62, &alphabet));
	assert_eq!("aA".to_string(), to_radix(&88, &alphabet));
	assert_eq!("b0".to_string(), to_radix(&114, &alphabet));
	assert_eq!("c0".to_string(), to_radix(&176, &alphabet));
	assert_eq!("z0".to_string(), to_radix(&1602, &alphabet));
	assert_eq!("A0".to_string(), to_radix(&1664, &alphabet));
	assert_eq!("Z0".to_string(), to_radix(&3214, &alphabet));
	assert_eq!("Zz".to_string(), to_radix(&3249, &alphabet));
	assert_eq!("ZA".to_string(), to_radix(&3250, &alphabet));
	assert_eq!("ZZ".to_string(), to_radix(&3275, &alphabet));

	// 3 character length encoded selector names:
	// - 38440 discarded
	// - 199888 valid
	assert_eq!("a00".to_string(), to_radix(&3276, &alphabet));
	assert_eq!("b00".to_string(), to_radix(&7120, &alphabet));
	assert_eq!("c00".to_string(), to_radix(&10964, &alphabet));
	assert_eq!("C00".to_string(), to_radix(&110908, &alphabet));
	assert_eq!("C10".to_string(), to_radix(&110970, &alphabet));
	assert_eq!("Ca0".to_string(), to_radix(&111528, &alphabet));
	assert_eq!("Ca9".to_string(), to_radix(&111537, &alphabet));
	assert_eq!("Caa".to_string(), to_radix(&111538, &alphabet));
	assert_eq!("CaA".to_string(), to_radix(&111564, &alphabet));
	assert_eq!("Cb0".to_string(), to_radix(&111590, &alphabet));
	assert_eq!("ZZZ".to_string(), to_radix(&203163, &alphabet));

	// 4 character length encoded selector names:
	// - 2383280 discarded
	// - 12393056 valid
	assert_eq!("a000".to_string(), to_radix(&203164, &alphabet));
	assert_eq!("ZZZZ".to_string(), to_radix(&12596219, &alphabet));
}


#[test]
fn index_to_base62_nums_last() {
	#[rustfmt::skip]
	let alphabet = into_alphabet_set(
		&"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
	);

	// 1 character length encoded selector names:
	// - 10 discarded
	// - 52 valid
	assert_eq!("a".to_string(), to_radix(&0, &alphabet));
	assert_eq!("Z".to_string(), to_radix(&51, &alphabet));

	// 2 character length encoded selector names:
	// - 620 discarded
	// - 3224 valid
	assert_eq!("aa".to_string(), to_radix(&52, &alphabet));
	assert_eq!("az".to_string(), to_radix(&77, &alphabet));
	assert_eq!("aA".to_string(), to_radix(&78, &alphabet));
	assert_eq!("aZ".to_string(), to_radix(&103, &alphabet));
	assert_eq!("a0".to_string(), to_radix(&104, &alphabet));
	assert_eq!("a9".to_string(), to_radix(&113, &alphabet));
	assert_eq!("ba".to_string(), to_radix(&114, &alphabet));
	assert_eq!("ca".to_string(), to_radix(&176, &alphabet));
	assert_eq!("za".to_string(), to_radix(&1602, &alphabet));
	assert_eq!("Aa".to_string(), to_radix(&1664, &alphabet));
	assert_eq!("Za".to_string(), to_radix(&3214, &alphabet));
	assert_eq!("Zz".to_string(), to_radix(&3239, &alphabet));
	assert_eq!("ZA".to_string(), to_radix(&3240, &alphabet));
	assert_eq!("ZZ".to_string(), to_radix(&3265, &alphabet));
	assert_eq!("Z9".to_string(), to_radix(&3275, &alphabet));

	// 3 character length encoded selector names:
	// - 38440 discarded
	// - 199888 valid
	assert_eq!("aaa".to_string(), to_radix(&3276, &alphabet));
	assert_eq!("aaz".to_string(), to_radix(&3301, &alphabet));
	assert_eq!("aaz".to_string(), to_radix(&3301, &alphabet));
	assert_eq!("aza".to_string(), to_radix(&4826, &alphabet));
	assert_eq!("aAa".to_string(), to_radix(&4888, &alphabet));
	assert_eq!("aZa".to_string(), to_radix(&6438, &alphabet));
	assert_eq!("aZz".to_string(), to_radix(&6463, &alphabet));
	assert_eq!("aZA".to_string(), to_radix(&6464, &alphabet));
	assert_eq!("aZZ".to_string(), to_radix(&6489, &alphabet));
	assert_eq!("a99".to_string(), to_radix(&7119, &alphabet));
	assert_eq!("baa".to_string(), to_radix(&7120, &alphabet));
	assert_eq!("bab".to_string(), to_radix(&7121, &alphabet));
	assert_eq!("caa".to_string(), to_radix(&10964, &alphabet));
	assert_eq!("Aaa".to_string(), to_radix(&103220, &alphabet));
	assert_eq!("Zaa".to_string(), to_radix(&199320, &alphabet));
	assert_eq!("Z99".to_string(), to_radix(&203163, &alphabet));

	// 4 character length encoded selector names:
	// - 2383280 discarded
	// - 12393056 valid
	assert_eq!("aaaa".to_string(), to_radix(&203164, &alphabet));
	assert_eq!("Z999".to_string(), to_radix(&12596219, &alphabet));
}


#[test]
fn index_to_base62_nums_scattered() {
	#[rustfmt::skip]
	let alphabet = into_alphabet_set(
		&"a0bc1d234ef5ghijklmn6opqr78s9tuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
	);

	// 1 character length encoded selector names:
	// - 10 discarded
	// - 52 valid
	assert_eq!("a".to_string(), to_radix(&0, &alphabet));
	assert_eq!("b".to_string(), to_radix(&1, &alphabet));
	assert_eq!("c".to_string(), to_radix(&2, &alphabet));
	assert_eq!("d".to_string(), to_radix(&3, &alphabet));
	assert_eq!("e".to_string(), to_radix(&4, &alphabet));
	assert_eq!("k".to_string(), to_radix(&10, &alphabet));
	assert_eq!("u".to_string(), to_radix(&20, &alphabet));
	assert_eq!("Z".to_string(), to_radix(&51, &alphabet));

	// 2 character length encoded selector names:
	// - 620 discarded
	// - 3224 valid
	assert_eq!("aa".to_string(), to_radix(&52, &alphabet));
	assert_eq!("a0".to_string(), to_radix(&53, &alphabet));
	assert_eq!("ba".to_string(), to_radix(&114, &alphabet));
	assert_eq!("ca".to_string(), to_radix(&176, &alphabet));
	assert_eq!("da".to_string(), to_radix(&238, &alphabet));
	assert_eq!("ea".to_string(), to_radix(&300, &alphabet));
	assert_eq!("ka".to_string(), to_radix(&672, &alphabet));
	assert_eq!("ua".to_string(), to_radix(&1292, &alphabet));
	assert_eq!("Z9".to_string(), to_radix(&3242, &alphabet));
	assert_eq!("ZZ".to_string(), to_radix(&3275, &alphabet));

	// 3 character length encoded selector names:
	// - 38440 discarded
	// - 199888 valid
	assert_eq!("aaa".to_string(), to_radix(&3276, &alphabet));
	assert_eq!("ZZZ".to_string(), to_radix(&203163, &alphabet));

	// 4 character length encoded selector names:
	// - 2383280 discarded
	// - 12393056 valid
	assert_eq!("aaaa".to_string(), to_radix(&203164, &alphabet));
	assert_eq!("ZZZZ".to_string(), to_radix(&12596219, &alphabet));
}


#[test]
fn index_to_base64_custom() {
	#[rustfmt::skip]
	let alphabet = into_alphabet_set(
		&"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ-_"
	);

	// 1 character length encoded selector names:
	// - 12 discarded
	// - 52 valid
	assert_eq!("a".to_string(), to_radix(&0, &alphabet));
	assert_eq!("b".to_string(), to_radix(&1, &alphabet));
	assert_eq!("c".to_string(), to_radix(&2, &alphabet));
	assert_eq!("Z".to_string(), to_radix(&51, &alphabet));

	// 2 character length encoded selector names:
	// - 768 discarded
	// - 3328 valid
	assert_eq!("a0".to_string(), to_radix(&52, &alphabet));
	assert_eq!("Z_".to_string(), to_radix(&3379, &alphabet));

	// 3 character length encoded selector names:
	// - 49152 discarded
	// - 212992 valid
	assert_eq!("a00".to_string(), to_radix(&3380, &alphabet));
	assert_eq!("Z_-".to_string(), to_radix(&216370, &alphabet));
	assert_eq!("Z__".to_string(), to_radix(&216371, &alphabet));

	// 4 character length encoded selector names:
	// - 2859936 discarded
	// - 13631488 valid
	assert_eq!("a000".to_string(), to_radix(&216372, &alphabet));
	assert_eq!("Z__-".to_string(), to_radix(&13847858, &alphabet));
	assert_eq!("Z___".to_string(), to_radix(&13847859, &alphabet));
}


#[test]
fn index_to_base26_latin_letters() {
	let alphabet = into_alphabet_set(&"abcdefghijklmnopqrstuvwxyz");

	// 1 character length encoded selector names:
	// - 26 valid combinations
	assert_eq!("a".to_string(), to_radix(&0, &alphabet));
	assert_eq!("b".to_string(), to_radix(&1, &alphabet));
	assert_eq!("c".to_string(), to_radix(&2, &alphabet));
	assert_eq!("k".to_string(), to_radix(&10, &alphabet));
	assert_eq!("u".to_string(), to_radix(&20, &alphabet));
	assert_eq!("z".to_string(), to_radix(&25, &alphabet));

	// 2 character length encoded selector names:
	// - 676 valid combinations
	assert_eq!("aa".to_string(), to_radix(&26, &alphabet));
	assert_eq!("ak".to_string(), to_radix(&36, &alphabet));
	assert_eq!("au".to_string(), to_radix(&46, &alphabet));
	assert_eq!("az".to_string(), to_radix(&51, &alphabet));
	assert_eq!("ba".to_string(), to_radix(&52, &alphabet));
	assert_eq!("zz".to_string(), to_radix(&701, &alphabet));

	// 3 character length encoded selector names:
	// - 17576 valid combinations
	assert_eq!("aaa".to_string(), to_radix(&702, &alphabet));
	assert_eq!("aak".to_string(), to_radix(&712, &alphabet));
	assert_eq!("aau".to_string(), to_radix(&722, &alphabet));
	assert_eq!("zzz".to_string(), to_radix(&18277, &alphabet));

	// 4 character length encoded selector names:
	// - 456976 valid combinations
	assert_eq!("aaaa".to_string(), to_radix(&18278, &alphabet));
	assert_eq!("zzzz".to_string(), to_radix(&475253, &alphabet));
}


#[test]
fn alphabet_sanitisation() {
	assert_eq!(
		into_alphabet_set(&"`~!@#$%^&*()_-+=<>?[]{}|abc"),
		['_', '-', 'a', 'b', 'c']
	);
}
