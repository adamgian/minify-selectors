use lazy_static::lazy_static;




lazy_static! {
	static ref INVALID_FIRST_CHARACTER: Vec<char> = vec![
		'0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '-', '_',
	];
}




/// Converts an ordinal into an encoded radix.
///
/// Function parameters:
/// - ordinal (&usize) - Integer to encode, must be 0 or greater.
/// - alphabet (&Vec<char>) - String sequence of characters in which to create
///   a radix from. Use into_alphabet_set() to create this vector from a
///   processed string such as: "0123456789ABCDEF".
pub fn to_radix(
	ordinal: &usize,
	alphabet: &[char]
) -> String {
	let invalid_char_positions: Vec<usize> = alphabet
		.iter()
		.enumerate()
		.filter_map(|(index, char)| {
			match INVALID_FIRST_CHARACTER.contains(char) {
				true => Some(index),
				false => None
			}
		})
		.collect();

	// Work out the number of places encoded ordinal will take up.
	let base: usize = alphabet.len();
	let subset: usize = base - invalid_char_positions.len();
	let mut carry: usize = 0;
	let mut exponent: u8 = 0;
	// base ^ exponent
	let mut floor: usize = 1;

	while *ordinal >= subset * floor + carry {
		carry += subset * floor;
		exponent += 1;
		floor = usize::pow(base, exponent.into());
	}

	// Calculate the modulo.
	//
	// Effectively, the first character in the returned encoded selector.
	// The modulo value corresponds to a character (a position) in
	// the alphabet vector. Remember: modulo has a max of the subset (- 1)
	// and not the base (- 1).
	let modulo: usize = (ordinal - carry).wrapping_div(floor);
	let mut offset: usize = 0;

	// Loop over invalid chars vector and make sure offset(s) are accounted for.
	for (index, alphabet_position) in invalid_char_positions.iter().enumerate() {
		if modulo + index < *alphabet_position {
			break;
		}
		offset += 1;
	}

	// Work out index that when converted into alphabet
	// will not start with invalid characters.
	let mut assigned_index: usize = offset * floor + *ordinal - carry;

	// Converting assigned index (base 10) into the supplied alphabet.
	// Right to left operation.
	let mut encoded_selector = String::new();
	for _ in 0..=exponent {
		let remainder = assigned_index.rem_euclid(base);
		encoded_selector.insert(0, *alphabet.get(remainder).unwrap());
		assigned_index = (assigned_index - remainder).wrapping_div(base);
	}

	return encoded_selector;
}


/// Returns a vector of chars that are all unique.
pub fn into_alphabet_set(alphabet: &str) -> Vec<char> {
	let mut alphabet_set: Vec<char> = Vec::new();

	for char in alphabet.chars() {
		if !alphabet_set.contains(&char) {
			alphabet_set.push(char);
		}
	}

	return alphabet_set;
}
