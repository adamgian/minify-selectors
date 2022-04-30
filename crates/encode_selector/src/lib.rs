use bs62;
use lazy_static::lazy_static;




lazy_static! {
	static ref INVALID_FIRST_CHARACTER: Vec<char> = vec![
		'0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '-', '_',
	];
}




// Deprecated
pub fn to_base62(position: &u32) -> String {
	const BASE: u32 = 62;
	const OFFSET: u32 = 10;
	const SUBSET: u32 = 52;

	let index: u32 = position - 1;
	let mut assigned_index: u32 = 0;
	let mut exponent: u8 = 0;
	let mut carry: u32 = 0;

	while index >= SUBSET * u32::pow(BASE, exponent.into()) + carry {
		carry += SUBSET * u32::pow(BASE, exponent.into());
		exponent += 1;
	}

	assigned_index += OFFSET * u32::pow(BASE, exponent.into()) - carry + index;

	return bs62::encode_num(&assigned_index);
}


// Converts an ordinal into an encoded radix.
//
// Function parameters:
// - ordinal (&usize) - (must be 0 or greater)
// - alphabet (&str) - String sequence of characters in which to create
//   a radix from. for example:
//   "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
pub fn to_radix(
	ordinal: &usize,
	alphabet: &Vec<char>
) -> String {
	let invalid_char_positions: Vec<usize> = alphabet
		.iter()
		.enumerate()
		.filter_map(|(index, char)| {
			match INVALID_FIRST_CHARACTER.contains(&char) {
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


// Returns a vector of chars that doesn't not contain
// more than one of each charactor or symbol.
pub fn into_alphabet_set (alphabet: &str) -> Vec<char> {
	let mut alphabet_set: Vec<char> = Vec::new();

	for char in alphabet.chars() {
		if !alphabet_set.contains(&char) {
			alphabet_set.push(char);
		}
	}

	return alphabet_set;
}
