use crate::lazy_static;
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

	// Extracts classes and IDs from selector rules in
	// stylesheets and embedded styles.
	//
	// See for reference: https://www.w3.org/TR/selectors-3/#grammar
	//
	// 1. Needs '#' or '.' to define in CSS an ID or class respectively.
	// 2. Next character after is '-', which is optional.
	// 3. Next character after is the 'nmstart' which is any of:
	//    a. underscore and lowercase/uppercase latin letters ([A-Za-z_]).
	//    b. anything else that is not ASCII ([^\0-\177]).
	//    c. escaped unicode number or character. Unicode numbers are 6 hex
	//        digits following the backslash. Unicode numbers can also be
	//        terminated earlier by by a space, newline, tab or form feed
	// 4. Finally after the mandatory 'nmstart' character, there are zero,
	//    one or many of 'nmchar' characters. 'nmchar's have exactly the
	//    same rules as 'nmstart' except for part a. — it is acceptable
	//    to have numerical digits and dashes as well (simplified down
	//    to [\w\-]).
	//
	// Caveats:
	// -  This regex in HTML files will match JS functions, objects, inner
	//    HTML, etc. — stuff it should not pick up. To circumvent this
	//    problem, this regex should only be run a subset of the HTML file
	//    string (i.e. content within <style></style>).
	// -  This regex will 'ignore'/blackout CSS blocks ({...}) in the sense
	//    that it will capture everything in the firstmost capture group
	//    and block the main regex portion from ever matching hex color
	//    values, units and the like.
	// -  This regex will 'ignore'/blackout attibutes selectors completely
	//    to avoid any false positives.
	// -  Multiline comments are 'ignored'/blacked out.
	// -  minify-selector specific prefixed selectors are ignored, to prevent
	//    it being encoded twice.
	pub static ref CSS_SELECTORS: Regex = Regex::new(
		r##"(?x)
			{
				[^{}]*
			}
			|
			\[
				\s*
					["']?.*?["']?
				\s*
			\]
			|
			\/\*[^*]*\*+(?>[^\/*][^*]*\*+)*\/
			|
			[\#\.]?__(?:class | id | ignore)?--
			|
			(?<type>[\#\.])
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

	// Extracts classes and IDs from a limited set of
	// attribute selectors. Attribute name must be 'class' or 'id'
	// and use the exact match operator.
	// i.e. [class="foo"][id="bar"]
	pub static ref CSS_ATTRIBUTES: Regex = Regex::new(
		r##"(?x)
			\/\*[^*]*\*+(?>[^\/*][^*]*\*+)*\/
			|
			\[\s*+
			(?<attribute>
				[^\f\n\t\ >"'|^$*~=]++
			)
			(?<operator>
				[~]?=
			)
			(?<quote>
				(?:\\?["'])?
			)
			(?<value>
				-?
				(?>
					[A-Za-z_]
					| [^\0-\177]
					| (?>
						\\[0-9A-Fa-f]{1,6}(?>\r\n|[ \n\r\t\f])?
						| \\[^\n\r\f0-9A-Fa-f"']
					)
				)
				(?>
					[\w\-]
					| [^\0-\177]
					| (?>
						\\[0-9A-Fa-f]{1,6}(?>\r\n|[ \n\r\t\f])?
						| \\[^\n\r\f0-9A-Fa-f"']
					)
				)*
			)
			(\\?["'])?
			(?<flag>
				\s*+
				[IiSs]?
			)
			\s*+\]
		"##
	).unwrap();

	// Extracts arguments from functions that take classes, IDs,
	// URL (which may have a target ID) or a CSS selector string.
	//
	// Objective is to capture a string of the function
	// input (between the parens) for further processing.
	pub static ref JS_ARGUMENTS: Regex = Regex::new(
		r##"(?x)
			\/\*[^*]*\*+(?>[^\/*][^*]*\*+)*\/
			|
			\/\/[^\n\r]*
			|
			(?<function>
				\.insertAdjacentHTML
				| \.querySelectorAll
				| \.querySelector
				| \.closest
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
								\.?
								[$\w]+
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
		"##
	).unwrap();

	// Extract the string value from JS property operations.
	pub static ref JS_PROPERTIES : Regex = Regex::new(
		r##"(?x)
			\/\*[^*]*\*+(?>[^\/*][^*]*\*+)*\/
			|
			\/\/[^\n\r]*
			|
			(?<function>
				window.location.hash
				| window.location.href
				| window.location
				| \.className
				| \.innerHTML
				| \.outerHTML
			)
			(?<join>
				\s*+[=+\-!<>]{1,3}\s*+
			)
			(?<value>
				(?:
					`
					(?:
						[^`\\] | \\.
					)*
					[^)]`
				)
				| (?:
					"
					(?:
						[^"\\] | \\.
					)*
					[^)]"
				)
				| (?:
					'
					(?:
						[^'\\] | \\.
					)*
					[^)]'
				)
			)
		"##
	).unwrap();

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
			|
			(?<attribute>
				[^\s\x00\/>"'=]+
			)
			(?<join>
				\s*=\s*
			)
			(?<value>
				[^\s\\<>"'=]+
				| \\?"(?:[^\\<>"=] | \\[^"'])+
				| \\?'(?:[^\\<>'=] | \\[^"'])+
			)
			(?<quote>
				(?:\\?["'])?
			)
		"##
	).unwrap();

	// Extract ID from anchor links.
	//
	// Only URLs without the protocol will have the inner first
	// and second named capture groups (url and target_id).
	pub static ref INTERNAL_ANCHOR_TARGET_ID: Regex = Regex::new(
		r##"(?x)
			^https?:\/\/.*$
			|
			^(?<url>[^#]*)
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
				["'`]
				(?<string>
					(?:(?<=")[^"]*)
					|
					(?:(?<=')[^']*)
					|
					(?:(?<=`)[^`]*)
				)
				["'`]
			)
			|
			(?<expression>
				(?:
					!*
					(?:
						(?:
							\.?
							[$\w]+
						)
						|
						(?:
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
									|
									(?:(?<=')[^']*)
									|
									(?:(?<=`)[^`]*)
								)
							["'`]
						)
						\s*
					)*
				)+
			)
			|
			(?<object>
				\{
				(?: [^{}] | \k<object> )*
				\}
			)
			|
			(?<array>
				\[
				(?: [^\[\]] | \k<array> )*
				\]
			)
		"##
	).unwrap();

}
