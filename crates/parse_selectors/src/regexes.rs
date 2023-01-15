use lazy_static::lazy_static;
use onig::*;




lazy_static! {

	// Extracts specially marked selectors. Selectors have a
	// minify-selectors specific prefix.
	//
	// Example usage and matches:
	// -  __class--foo
	// -  __id--foo
	// -  #__ignore--bar, .__ignore--bar and __ignore--bar
	// -  #__--baz and .__--baz
	pub static ref PREFIXED_SELECTORS: Regex = Regex::new(
		r##"(?x)
			(?:
				(?<type>[\#\.]?)
				__
				(?<context>
					(?:class | id | ignore)?
				)
				--
			)
			(?<name>
				-?
				(?>
					[A-Za-z_]
					| [^\0-\177]
					| (?>
						\\[0-9A-Fa-f]{1,6}(?>\r\n|[ \n\r\t\f])?
						| \\[^\n\r\f0-9A-Fa-f]
					)
				)
				(?>
					(?!-->)
					[\w\-]
					| [^\0-\177]
					| (?>
						\\[0-9A-Fa-f]{1,6}(?>\r\n|[ \n\r\t\f])?
						| \\[^\n\r\f0-9A-Fa-f]
					)
				)*
			)
		"##
	).unwrap();

	// Extract ID from anchor links.
	//
	// Only URLs without the protocol ('http://', 'https://' or '//')
	// or not prepended with minify-selector specific prefix
	// will have the inner first and second named capture groups
	// (url and target_id).
	pub static ref INTERNAL_ANCHOR_TARGET_ID: Regex = Regex::new(
		r##"(?x)
			^(?>http:|https:)?\/{2}.*$
			| ^[^#]*[#]__(?:class | id | ignore)?--
			| ^(?<url>[^#]*)
			(?<target_id>\#[^#]*)$
		"##
	).unwrap();

	// Extract tokens (that are valid selector names) — seperated
	// by whitespace(s).
	pub static ref STRING_DELIMITED_BY_SPACE: Regex = Regex::new(
		r##"(?x)
			(?<token>
				-?
				(?>
					[A-Za-z_]
					| [^\0-\177]
					| (?>
						\\[0-9A-Fa-f]{1,6}(?>\r\n|[ \n\r\t\f])?
						| \\[^\n\r\f0-9A-Fa-f]
					)
				)
				(?>
					[\w\-]
					| [^\0-\177]
					| (?>
						\\[0-9A-Fa-f]{1,6}(?>\r\n|[ \n\r\t\f])?
						| \\[^\n\r\f0-9A-Fa-f]
					)
				)*
			)
		"##
	).unwrap();

	// Extract function arguments.
	//   - "foo", 'bar', `baz`, etc.
	//   - foo, foo.bar, foo(), foo.bar(), baz({}), baz(foo()), etc.
	//   - {foo: "foo"}, {foo: {bar: []}}, etc.
	//   - ["foo", "bar"], [{foo}], etc.
	pub static ref STRING_DELIMITED_BY_COMMA: Regex = Regex::new(
		r##"(?x)
			(?<token>
				(?<token_delimiter>["'`])
				(?<token_string>
					(?:
						(?<=")
						(?:[^"\\] | \\.)*
					)
					| (?:
						(?<=')
						(?:[^'\\] | \\.)*
					)
					| (?:
						(?<=`)
						(?:[^`\\] | \\.)*
					)
				)
				["'`]
			)
			| (?<expression>
				(?:
					!*
					(?:
						(?:
							\.?
							[$\w]+
						)
						| (?:
							\(
							(?: [^()] | \k<expression>)*
							\)
						)
					)+
					(?:
						\s*
						(?:
							&&
							| \|\|
							| \?{1,2}
							| :
							| ={1,3}
							| !={1,2}
							| >={0,2}
							| <={0,2}
							| ["'`]
								(?:
									(?:(?<=")[^"]*)
									| (?:(?<=')[^']*)
									| (?:(?<=`)[^`]*)
								)
							["'`]
						)
						\s*
					)*
				)+
			)
			| (?<object>
				\{
				(?: [^{}] | \k<object> )*
				\}
			)
			| (?<array>
				\[
				(?: [^\[\]] | \k<array> )*
				\]
			)
		"##
	).unwrap();

}
