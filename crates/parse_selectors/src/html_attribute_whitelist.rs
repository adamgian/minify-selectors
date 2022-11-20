use std::collections::HashMap;

use lazy_static::lazy_static;




lazy_static! {
	// HTML attributes which its values will contains parsable values
	pub static ref ATTRIBUTES_WHITELIST: HashMap<String, String> = HashMap::from([
		(String::from("class"), String::from("class")),

		(String::from("id"), String::from("id")),
		(String::from("aria-describedby"), String::from("id")),
		(String::from("aria-labelledby"), String::from("id")),
		(String::from("for"), String::from("id")),
		(String::from("form"), String::from("id")),
		(String::from("headers"), String::from("id")),
		(String::from("itemref"), String::from("id")),
		(String::from("list"), String::from("id")),

		(String::from("href"), String::from("anchor")),
		(String::from("xlink:href"), String::from("anchor")),

		(String::from("fill"), String::from("style")),
		(String::from("style"), String::from("style")),
	]);
}
