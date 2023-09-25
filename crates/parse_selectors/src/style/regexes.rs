use lazy_static::lazy_static;
use onig::*;




lazy_static! {

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
	// -  @import url is ignored.
	// -  minify-selector specific prefixed selectors are ignored, to prevent
	//    it being encoded twice.
	pub static ref CSS_SELECTORS: Regex = Regex::new(
		r#"(?x)
			{[^{}]*}
			| \[
				\s*
					["']?.*?["']?
				\s*
			\]
			| \/\*[^*]*\*+(?>[^\/*][^*]*\*+)*\/
			| @import\s++(?:
				url\([^)]*\)
				| (?:"(?:[^"])*")
				| (?:'(?:[^'])*')
			)
			| [\#\.]?__(?:class | id | ignore)?--
			| (?<type>[\#\.])
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
		"#
	).unwrap();

	// Extracts classes and IDs from a limited set of
	// attribute selectors. Attribute name must be 'class' or 'id'
	// and use the exact match operator.
	// i.e. [class="foo"][id="bar"]
	pub static ref CSS_ATTRIBUTES: Regex = Regex::new(
		r#"(?x)
			\/\*[^*]*\*+(?>[^\/*][^*]*\*+)*\/
			| \[\s*+
			(?<attribute>
				(?>
					[^\f\n\t\ \\>"'|:^$*~=]
					| (?>
						\\[0-9A-Fa-f]{1,6}(?>\r\n|[ \n\r\t\f])?
						| \\[^\n\r\f0-9A-Fa-f]
					)
				)++
			)
			\s*+
			(?<operator>
				[~]?=
			)
			\s*+
			(?<quote>
				(?:\\?["'])?
			)
			(?<value>
				(?:
					(?<=["'])
					(?:
						[^"'\\]
						| (?>
							\\[0-9A-Fa-f]{1,6}(?>\r\n|[ \n\r\t\f])?
							| \\[^\n\r\f0-9A-Fa-f"']
						)
					)*
				)
				| (?:
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
			)
			(?>\\?["'])?
			(?<spacer>\s*+)
			(?<flag>[IiSs]?)
			\s*+\]
		"#
	).unwrap();

	pub static ref CSS_FUNCTIONS: Regex = Regex::new(
		r#"(?x)
			(?<function>
				url
			)
			(?<join>
				\(\s*+
			)
			(?<quote>
				(?:\\?["'])?
			)
			(?<argument>
				(?:
					(?<=")(?:[^"])*
				)
				| (?:
					(?<=')(?:[^'])*
				)
				| (?:
					[^\s\)]*
				)
			)
		"#
	).unwrap();

	pub static ref ESCAPED_CSS_CHARS: Regex = Regex::new(
		r"(?x)
			(?<unicode>
				\\[0-9A-Fa-f]{1,6}(?>\r\n|[ \n\r\t\f])?
			)
			| (?<character>
				\\[^\n\r\f0-9A-Fa-f]
			)
		"
	).unwrap();

	// Invalid characters in a selector name are:
	// -  \0-\54: null to comma
	// -  \56: period (.)
	// -  \57: slash (/)
	// -  \72-\100: colon (:) to at (@)
	// -  \133-\136: left square bracket ([) to caret (^)
	// -  \140: backtick (`)
	// -  \173-\177: left brace ({) to delete
	pub static ref INVALID_CSS_CHARACTERS: Regex = Regex::new(
		r"(?x)
			[\0-\54\56\57\72-\100\133-\136\140\173-\177]
		"
	).unwrap();

}
