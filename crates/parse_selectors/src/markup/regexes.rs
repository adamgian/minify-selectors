use lazy_static::lazy_static;
use onig::*;




lazy_static! {

	// Extract instances of <style></style> from HTML files.
	pub static ref HTML_STYLE_ELEMENT: Regex = Regex::new(
		r##"(?x)
			(?<tag_open>
				<style[^>]*>
			)
			(?<styles>
				(?:.|\n|\r)*?
			)
			(?<tag_close>
				<\/style>
			)
		"##
	).unwrap();

	// Extract instances of <script></script> from HTML files.
	pub static ref HTML_SCRIPT_ELEMENT: Regex = Regex::new(
		r##"(?x)
			(?<tag_open>
				<script[^>]*>
			)
			(?<script>
				(?:.|\n|\r)*?
			)
			(?<tag_close>
				<\/script>
			)
		"##
	).unwrap();

	// Extracts all attributes with values from HTML.
	//
	// Will need additional processing to consider 'whitelisted'
	// attributes and separate out the values.
	//
	// Capture HTML comments, <code>, <script> and <style> elements to prevent
	// false positive matches (i.e. prevent regex from matching into deeper
	// capture groups).
	//
	// See: https://www.w3.org/TR/2018/SPSD-html5-20180327/syntax.html#attributes-0
	//
	// 1. Attribute name - consists of one of more characters.
	//    - Cannot be a whitespace character, null, quotation ("),
	//        apostrophe ('), forward slash (/) or equals sign (=).
	//    - ASCII case insensitive
	//    - Character references:
	//        - Named: e.g. &copy;, &nbsp;
	//        - Decimal numeric: &#931;, &#0931;
	//        - Hexadecimal numeric: &#x3A3;, &#x03A3;, &#x3a3;
	// 2. Then optionally followed by an attribute value. An single equals
	//    sign is used to separate name from the value. Even though values
	//    are optional, we are only interested in attributes that have a
	//    value. Note: it is valid to have one or more whitespace chars
	//    on either side of the equals sign.
	// 3. Attibutes values cannot contain: <, >, `, or =. Additional
	//    rules as follows:
	//    - Unquoted value - cannot have: ", ' or be an empty string.
	//    - Single-quoted value, cannot contain any ' characters.
	//    - Double-quoted value, cannot contain any " characters.
	//    - Like names, values can have character references also.
	//    - If followed by another attribute or /, there must be at least
	//        a whitespace character before them.
	pub static ref HTML_ATTRIBUTES: Regex = Regex::new(
		r##"(?x)
			<!--.*?-->
			| <head[>\s](?:.|\s)*?<\/head>
			| <style[>\s](?:.|\s)*?<\/style>
			| <code[>\s](?:.|\s)*?<\/code>
			| <script[>\s](?:.|\s)*?<\/script>
			| (?<attribute>
				[^\s\x00\/>"'=]+
			)
			(?<join>
				\s*=\s*
			)
			(?<quote>
				(?:\\?["'])?
			)
			(?<value>
				(?:
					(?<=")
					(?:[^"\\] | \\[^"'])+
				)
				| (?:
					(?<=')
					(?:[^'\\] | \\[^"'])+
				)
				| [^\s\\<>"'=]+
			)
		"##
	).unwrap();

	pub static ref ESCAPED_HTML_CHARS: Regex = Regex::new(
		r##"(?x)
			(?<hexdecimal_char_ref>
				&\#x[0-9A-Fa-f]{1,4};
			)
			| (?<decimal_char_ref>
				&\#[0-9]{1,6};
			)
			| (?<named_char_ref>
				&[A-Za-z]*+;?
			)
		"##
	).unwrap();

}
