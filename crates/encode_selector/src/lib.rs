use bs62;




pub fn to_base62(position: &u32) -> String {
	const BASE: u32 = 62;
	const OFFSET: u32 = 10;
	const SUBSET: u32 = 52;

	let index: u32 = position - 1;
	let mut assigned_index: u32 = 0;
	let mut exponent: u8 = 0;
	let mut carry: u32 = 0;

	while u32::from(index) >= SUBSET * u32::pow(BASE, exponent.into()) + carry {
		carry += SUBSET * u32::pow(BASE, exponent.into());
		exponent += 1;
	}

	assigned_index += OFFSET * u32::pow(BASE, exponent.into()) - carry + u32::from(index);

	return bs62::encode_num(&assigned_index);
}
