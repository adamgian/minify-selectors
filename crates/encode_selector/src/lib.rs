use lazy_static::lazy_static;
use onig::*;




lazy_static! {
	static ref INVALID_FIRST_CHARACTER: Vec<char> = vec![
		'0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '-', '_',
	];

	// Invalid characters in a selector name are:
	// -  \0-\54: null to comma
	// -  \56: period (.)
	// -  \57: slash (/)
	// -  \72-\100: colon (:) to at (@)
	// -  \133-\136: left square bracket ([) to caret (^)
	// -  \140: backtick (`)
	// -  \173-\177: left brace ({) to delete
	static ref INVALID_CHARACTERS: Regex = Regex::new(
		r##"(?x)
			[\0-\54\56\57\72-\100\133-\136\140\173-\177]
		"##
	).unwrap();
}




/// Converts an ordinal into an encoded radix.
///
/// Function parameters:
/// - ordinal (&usize) - Integer to encode, must be 0 or greater.
/// - alphabet (&(Vec<char>, &Vec<usize>)) - A tuple, use into_alphabet_set() to
///   create this tuple from a processed string such as: "0123456789ABCDEF".
pub fn to_radix(
	ordinal: &usize,
	alphabet: &(Vec<char>, Vec<usize>),
) -> String {
	// Work out the number of places encoded ordinal will take up.
	let base: usize = alphabet.0.len();
	let subset: usize = base - alphabet.1.len();
	let mut carry: usize = 0;
	let mut exponent: u8 = 0;
	let mut floor: usize = 1; // base ^ exponent

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
	for (index, alphabet_position) in alphabet.1.iter().enumerate() {
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
		encoded_selector.insert(0, *alphabet.0.get(remainder).unwrap());
		assigned_index = (assigned_index - remainder).wrapping_div(base);
	}

	encoded_selector
}


/// Returns a tuple of:
///  0. sanitised vector of chars that are all unique
///  1. vector of the positions of chars a selector name cannot start with.
pub fn into_alphabet_set(alphabet: &str) -> (Vec<char>, Vec<usize>) {
	let mut alphabet_set: Vec<char> = Vec::new();

	// Sanitise alphabet, remove any invalid characters
	let sanitised_alphabet = INVALID_CHARACTERS.replace_all(alphabet, "");

	// Removing any duplicate characters.
	for char in sanitised_alphabet.chars() {
		if !alphabet_set.contains(&char) {
			alphabet_set.push(char);
		}
	}

	// Noting the positions of characters that are blacklisted from
	// being the first character in a encoded selector name.
	let invalid_as_first_char_positions: Vec<usize> = alphabet_set
		.iter()
		.enumerate()
		.filter_map(|(index, char)| {
			match INVALID_FIRST_CHARACTER.contains(char) {
				true => Some(index),
				false => None,
			}
		})
		.collect();

	(alphabet_set, invalid_as_first_char_positions)
}
