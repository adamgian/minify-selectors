# v2.8.2
<sub>27 Jan 2025</sub>

### Updates
- Add build for Windows ARM64
- Add build for Linux RISC-V64 and ARMv7

<br>




# v2.8.1
<sub>7 Jan 2025</sub>

### Updates
- Support bracket notation accessors for the `.children` property
- Process files with the following extensions as well: `.htm`, `.mjs` and `.cjs`

### Fixes
- Ignore file extensions casing

<br>




# v2.6.0
<sub>7 Mar 2024</sub>

### Updates
- Accept JSON configuration file
- Breaking change: use white-space character to delimiter CLI arguments that can accept multiple values

<br>




# v2.4.1
<sub>18 Feb 2023</sub>

### Updates
- Ability to define custom attributes

### Fixes
- Rename `--disable-sort` CLI option to `--sort`

<br>




# v2.3.3
<sub>4 Feb 2023</sub>

### Updates
- Add `--parallel` option to enable concurrent file processing
- Sort selectors by frequency before generating minified identifiers

### Fixes
- Missing support for `aria-controls` HTML attribute
- Do not skip IDs that are only used in markup attributes
- Encode skipped classes if it conflicts with an encoded one

<br>




# v2.1.3
<sub>17 Jan 2023</sub>

### Updates
- Skip replacing selectors if they are only used in HTML attributes

### Fixes
- Convert any CSS character escapes in prefixed selectors before adding to map
- HTML closing comment tag being errantly included as part of prefixed selector
- Accept characters that would be invalid CSS characters in HTML attributes

<br>




# v2.0.0
<sub>13 Jan 2023</sub>

### Updates
- Switch from a single-pass to multi-pass proccess

<br>




# v1.10.1
<sub>1 Dec 2022</sub>

### Fixes
- `--input` flag not properly defined

<br>




# v1.10.0
<sub>30 Nov 2022</sub>

### Updates
- Add support for named character references in HTML

<br>




# v1.9.4
<sub>22 Nov 2022</sub>

### Updates
- Add support for escaped decimal and hexadecimal character references in HTML
- Add support for encoded URLs in JS

### Fixes
- Extend escaped CSS character support to include JS selector strings
- Extend escaped JS character support to include additional JS escapes: hexadecimal, unicode and unicode code points
- Parse HTML and SVG event attribute values as JS
- Delimiter char that is not a delimiter can be be mistaken for one when delimiters are not supplied

<br>




# v1.7.2
<sub>9 Nov 2022</sub>

### Updates
- Add support for escaped characters in CSS

### Fixes
- Ignore CSS `@import` URLs
- Missing support for `matches()` JS method

<br>




# v1.6.2
<sub>5 Nov 2022</sub>

### Updates
- Add support for `fill` and `style` HTML attributes
- Add support for `url()` functions in CSS
- Add support for `.classList`, `.classList.item()`, `.classList.value` and `.id` properties
- Add support for `xlink:href` HTML attribute
- Calculate the positions of invalid first characters in the alphabet set only once, instead of everytime a radix encoding is required (approximately -90% change to `encode_selector::to_radix()`)
- Upgrade clap to v4

### Fixes
- Account for whitespaces that may be before and after the operator in CSS attribute selectors (e.g. `[lang = 'nl']`)
- Anchor IDs in `url()` being processed twice if they are used in `fill` and `style` HTML attributes

<br>




# v1.0.0
<sub>16 Oct 2022</sub>

No noteworthy changes since the last version

<br>




# v1.0.0-beta.4
<sub>21 Sep 2022</sub>

### Updates
- Add support for SVG files
- Keep track of IDs and classes separately

### Fixes
- Convert captured attribute names to lowercase before using

<br>




# v1.0.0-beta.3
<sub>10 Sep 2022</sub>

### Fixes
- More explicit HTML attribute regex, to address catastrophic backtracking when a HTML element name begins with one of the ignored elements (`code`, `head`, `script` and `style`)
- JS function arguments regex not properly matching strings when it is part of a variable assignment or expression
- Variable arguments in `getElementById()` and `get ElementsByClassName()` getting incorrectly processed as a string
- Ignore glob path matches that are not files

<br>




# v1.0.0-beta.2
<sub>23 Aug 2022</sub>

### Updates
- Parse and encode minify-selectors specific prefixed selectors ([#11](https://github.com/adamgian/minify-selectors/issues/11))
- Add support for JS Location interface properties: `window.location.hash`, `window.location.href` and `window.location`
- Add support for JS URL methods: `history.pushState()`, `history.replaceState()`, `window.location.assign()`, `window.location.replace()` and `window.open()`

### Fixes
- Only encode target IDs for relative URLs (i.e. without the protocol)
- Strip string delimiters (if any) from URL values before parsing
- JS arguments regex not matching empty strings, variables and objects

<br>




# v0.11.3
<sub>25 May 2022</sub>

### Fixes
- Address regression in npm configuration and subsequently the 'binary-install' dependency
- More robust regex pattern for string delimited by space, match only tokens that are valid selector names
- When processing HTML files, only parse JS inside `<script>` tags
- Move HTML `<code>` and comments 'filters' to the regex-level, HTML attributes regex extended to also 'ignore' `<head>`, `<script>` and `<style>` elements

<br>




# v0.11.0
<sub>22 May 2022</sub>

### Updates
- Encode target ID in anchor links (in HTML `href` attribute)
- Encode selectors in `innerHTML`, `outerHTML` and `insertAdjacentHTML` string argument

### Fixes
- Ignore single and multi-line comments in JS code
- Ignore any selectors within the `<code>` element
- Sanitise alphabet string, any invalid characters are removed

<br>




# v0.9.15
<sub>17 May 2022</sub>

### Fixes
- Rewrite parsing logic for `setAttribute()`
- Improve JS arguments regex parse order of whitespace and following formatting of source
- Escaped string delimiters support for attribute selectors and retain source formatting when attribute flag is set
- Address rigid pattern match logic for `classList` methods which did not account for whitespaces (e.g. `.classList ↵ .add('foo')`)
- Account for whitespace before comma between JS function arguments
- Trim string delimiters before processing space separated tokens
- Missing logic for backtick delimiters support

<br>




# v0.9.8
<sub>13 May 2022</sub>

### Updates:
- Add build for macOS 64-bit ARM platforms
- `.closest()` function support in JS file and scripts
- Support for an optional custom alphabet argument (`--alphabet`)
- Support for an optional starting index argument (`--start-index`)

### Fixes:
- Ignore comments when searching for attribute selectors in CSS
- Attribute selector flag can be uppercase or in lowercase

<br>




# v0.7.8
<sub>23 Apr 2022</sub>

### Updates:
- Adjust profile configuration for an optimised release build
- Handle CSS attribute selectors in stylesheets
- Rewrite 'HTML_ATTRIBUTES' regex
- Updates to documentation

### Fixes:
- Unnecessary capture group removed from 'STRING_DELIMITED_BY_COMMA' regex
- String delimiters (`"` or `'`) not being accounted for in function argument(s)
- Missing optional quantifier in 'HTML_ATTRIBUTES' regex, causing attributes with unquoted values to not be picked up
- Fix 'JS_ARGUMENTS' regex — 'arguments' capture group was far too overzealous
- Dial back 'CSS_SELECTORS' regex, only ignore innermost block
- Account for the fact that HTML attribute names are ASCII case-insensitive
- Remove lead quote delimiter from attribute value string before parsing values

<br>




# v0.4.3
<sub>16 Apr 2022</sub>

### Updates:
- Handle errors gracefully on directory creation and file writes (refactor)
- Only try to find CSS selectors within style tags in HTML files + ignore attribute selectors (should be processed separately) — saves on logic trying to prevent false positives elsewhere (feature)
- Reassign integer types of certain variables (refactor)
- Add binary for Windows x32 (feature)
- Binaries published onto npm (feature)

### Fixes:
- binary-install scripts
