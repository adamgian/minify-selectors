use std::collections::HashMap;




#[derive(Debug)]
pub struct Config {
	pub alphabet: Vec<char>,
	pub start_index: usize,
}


#[derive(Debug)]
pub struct Selectors {
	pub map: HashMap<String, String>,
	pub class_index: usize,
	pub id_index: usize,
}

impl Selectors {
	pub fn increment_class_index(&mut self) {
		self.class_index += 1;
	}

	pub fn increment_id_index(&mut self) {
		self.id_index += 1;
	}

	pub fn contains(&self, selector: &str) -> bool {
		self.map.contains_key(selector)
	}

	// Note: assumes that key exists, should check first with contains().
	pub fn get(&self, selector: &str) -> String {
		self.map.get_key_value(selector).unwrap().1.to_string()
	}
}
