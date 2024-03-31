use lazy_static::lazy_static;
use onig::*;




lazy_static! {

	// Extracts arguments from functions that take classes, IDs,
	// URL (which may have a target ID) or a CSS selector string.
	//
	// Objective is to capture a string of the function
	// input (between the parens) for further processing.
	pub static ref JS_ARGUMENTS: Regex = Regex::new(
		r#"(?x)
			\/\*[^*]*\*+(?>[^\/*][^*]*\*+)*\/
			| \/\/[^\n\r]*
			| (?<function>
				\.insertAdjacentHTML
				| \.querySelectorAll
				| \.querySelector
				| \.closest
				| \.matches
				| \.getElementById
				| \.getElementsByClassName
				| \.classList\s*+\.(?> add | remove | contains | replace | toggle )
				| \.setAttribute
				| history\s*+\.(?> pushState | replaceState )
				| window\s*+\.(?> location\s*+\.assign | location\s*+\.replace | open )
			)
			(?<join>
				\(\s*+
			)
			(?<arguments>
				(?:
					\s*
					(?:
						(?:
							(?<quote>[`"'])
							(?:
								(?:
									(?<=`)
									(?:[^`\\] | \\.)*
									`
								)
								| (?:
									(?<=")
									(?:[^"\\] | \\.)*
									"
								)
								| (?:
									(?<=')
									(?:[^'\\] | \\.)*
									'
								)
							)
						)
						| (?:
							!*
							(?:
								\.?[$\w]+
							)+
						)
						| (?:
							\(
							(?: [^()] | \k<arguments>)*
							\)
						)
						| (?:
							\{
							(?: [^{}] | \k<arguments> )*
							\}
						)
						| (?:
							\[
							(?: [^\[\]] | \k<arguments> )*
							\]
						)
					)
					(?:
						\s*
						(?:
							,
							| &&
							| \|\|
							| \?{1,2}
							| :
							| ={1,3}
							| !={1,2}
							| >={0,2}
							| <={0,2}
						)
					)?
				)++
			)
		"#
	).unwrap();

	// Extract the string value from JS property operations.
	pub static ref JS_PROPERTIES: Regex = Regex::new(
		r#"(?x)
			\/\*[^*]*\*+(?>[^\/*][^*]*\*+)*\/
			| \/\/[^\n\r]*
			| (?<function>
				window.location.hash
				| window.location.href
				| window.location
				| \.id
				| \.className
				| \.classList
					(?>
						\[[0-9]++\]
						| \.value
						| \.item
							(?<arguments>
								\(
								[^)(]*+(?:(\g<arguments>)[^)(]*)*+
								\)
							)
					)?
				| \.innerHTML
				| \.outerHTML
			)
			(?<join>
				\s*+[=+\-!<>]{1,3}\s*+
			)
			(?<value>
				(?:
					`(?:[^`\\] | \\.)*
					[^)]`
				)
				| (?:
					"(?:[^"\\] | \\.)*
					[^)]
				)
				| (?:
					'(?:[^'\\] | \\.)*
					[^)]'
				)
			)
		"#
	).unwrap();

	// Extracts string value from bracket notation property accessors
	//
	// Try to match multi-line or single-line comments first, to prevent
	// any matches that occur within code comments. These kinds of
	// matches will not have anything in the named groups.
	pub static ref JS_BRACKET_ACCESSORS: Regex = Regex::new(
		r#"(?x)
			\/\*[^*]*\*+(?>[^\/*][^*]*\*+)*\/
			| \/\/[^\n\r]*
			| (?<property>
				\.children
			)
			(?<open_bracket>
				\s*+\[\s*+
			)
			(?<value>
				(?:
					`(?:[^`\\] | \\.)*
					[^)]`
				)
				| (?:
					"(?:[^"\\] | \\.)*
					[^)]"
				)
				| (?:
					'(?:[^'\\] | \\.)*
					[^)]'
				)
			)
		"#
	).unwrap();

	// Extract instances of <script></script> from HTML files.
	pub static ref HTML_SCRIPT_ELEMENT: Regex = Regex::new(
		r"(?x)
			(?<tag_open>
				<script[^>]*>
			)
			(?<script>
				(?:.|\n|\r)*?
			)
			(?<tag_close>
				<\/script>
			)
		"
	).unwrap();

	pub static ref ESCAPED_JS_CHARS: Regex = Regex::new(
		r"(?x)
			(?<url_encoded_char>
				%[0-9A-Fa-f]{2}
			)
			| (?<hexdecimal>
				\\x[0-9A-Fa-f]{2}
			)
			| (?<unicode>
				\\u[0-9A-Fa-f]{4}
			)
			| (?<unicode_codepoint>
				\\u{[0-9A-Fa-f]{1,}}
			)
		"
	).unwrap();

}
