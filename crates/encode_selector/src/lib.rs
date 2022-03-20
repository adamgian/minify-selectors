use bs62;

pub fn to_base62(position: &u32) -> String {
	const BASE: u32 = 62;
	const OFFSET: u32 = 10;
	const SUBSET: u32 = 52;

	let index: u32 = position - 1;
	let mut assigned_index: u32 = 0;
	let mut exponent: u32 = 0;
	let mut carry: u32 = 0;

	while index >= SUBSET * u32::pow(BASE, exponent) + carry {
		carry += SUBSET * u32::pow(BASE, exponent);
		exponent += 1;
	}

	assigned_index += OFFSET * u32::pow(BASE, exponent) - carry + index;

	return bs62::encode_num(&assigned_index);
}