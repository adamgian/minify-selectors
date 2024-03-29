[1]: https://github.com/adamgian/minify-selectors/releases/latest
[2]: https://img.shields.io/npm/v/minify-selectors?color=blue&label=Latest%20release
[3]: https://www.apache.org/licenses/LICENSE-2.0
[4]: https://img.shields.io/badge/License-Apache%202.0-green.svg

[![Latest release version][2]][1]&nbsp;
[![Apache 2.0 license][4]][3]




# minify-selectors

Post-processor that minifies class and ID selector names in CSS, HTML, Javascript and SVG files. minify-selectors aims to offer comprehensive out of the box support with minimal fuss.

Enhance your front-end assets and build optimisations pipeline — wring even more out from your already minified and optimised payload sizes. Additionally can offer a certain degree of obfuscation to your code.

<br>




## What it can do

#### HTML and SVGs

<table>
	<tr>
		<td>
			<p><sub>Input</sub></p>
			<pre lang="html">
&lt;body id="page--default">
  &lt;nav class="site-nav">
    &lt;div class="search has-content">
      &lt;label for="site-search" class="text--muted text--c
        enter">
        Search app
      &lt;/label>
      &lt;input type="text" id="site-search" class="form-inp
        ut--single form-input--lg border--thick">
    &lt;/div>
  &lt;/nav>
&lt;/body><!--
			--></pre>
		</td>
		<td valign="top">
			<p><sub>Output:</sub></p>
			<pre lang="html">
&lt;body id="a">                                            ‎
  &lt;nav class="b">
    &lt;div class="c a1">
      &lt;label for="y" class="F j">
        Search app
      &lt;/label>
      &lt;input type="text" id="y" class="A9 t Av">
    &lt;/div>
  &lt;/nav>
&lt;/body><!--
			--></pre>
		</td>
	</tr>
</table>
<table>
	<tr>
		<td>
			<p><sub>Input</sub></p>
			<pre lang="html">
&lt;svg xmlns="http://www.w3.org/2000/svg">                 ‎
  &lt;style>
    .icon-background { … }
  &lt;/style>
  &lt;g class="icon">
    &lt;circle class="icon-background" fill="url(#dotted)"
      />
  &lt;/g>
&lt;/svg>
<!--
			--></pre>
		</td>
		<td valign="top">
			<p><sub>Output:</sub></p>
			<pre lang="html">
&lt;svg xmlns="http://www.w3.org/2000/svg">                 ‎
  &lt;style>
    .dz { … }
  &lt;/style>
  &lt;g class="d8">
    &lt;circle class="dz" fill="url(#Y)" />
  &lt;/g>
&lt;/svg>
<!--
			--></pre>
		</td>
	</tr>
</table>


#### CSS (and HTML embedded styles)

<table>
	<tr>
		<td>
			<p><sub>Input</sub></p>
			<pre lang="scss">
.sidebar, .site-nav { … }                                ‎
.sidebar .search:focus-within { … }
.sidebar--expanded a.is-active { … }
#page--default { … }<!--
			--></pre>
		</td>
		<td>
			<p><sub>Output:</sub></p>
			<pre lang="scss">
.a, .b { … }                                             ‎
.a .c:focus-within { … }
.d a.e { … }
#a { … }<!--
			--></pre>
		</td>
	</tr>
</table>


#### JS (and HTML embedded scripts)

<table>
	<tr>
		<td><p><sub>Input</sub></p>
		<pre lang="js">
for (let link of document.querySelectorAll('a.anchor')) {‎
  link.classList.remove('is-active');
}<!--
			--></pre>
		</td>
		<td>
			<p><sub>Output:</sub></p>
			<pre lang="js">
for (let link of document.querySelectorAll('a.Bd')) {    ‎
  link.classList.remove('e');
}<!--
			--></pre>
		</td>
	</tr>
</table>


#### Opt-in or opt-out of selector encoding

In cases where minify-selectors is unable to parse selectors, for example: in custom HTML attributes or JS variables. Or forcing a selector to be encoded when it otherwise would not be, such as in a HTML code element or comments. You can prefix your selector names so that minify-selectors knows to parse it and how to encode it:

- `.__--` or `#__--` instead of the selector type ('#' or '.') before selector names
- `__class--` or `__id--` for "name only" selectors, use '\_\_class--' for class selectors and '\_\_id--' for ID selectors

<table>
	<tr>
		<td>
			<p><sub>Input</sub></p>
			<pre lang="html">
&lt;main class="page page-dark">                            ‎
  &lt;button
    class="btn btn-warning"
    data-toggle="modal"
    data-target="#__--prompt-delete-user">
  &lt;/button>
&lt;/main>
&lt;script>
  view.className = isDarkMode
    ? '__class--page __class--page--dark'
    : '__class--page __class--page--light';
&lt;/script>
<!--
			--></pre>
		</td>
		<td valign="top">
			<p><sub>Output:</sub></p>
			<pre lang="html">
&lt;main class="b2 b3">                                     ‎
  &lt;button
    class="a4 a7"
    data-toggle="modal"
    data-target="#bc">
  &lt;/button>
&lt;/main>
&lt;script>
  view.className = isDarkMode
    ? 'b2 b3'
    : 'b2 b4;
&lt;/script>
<!--
			--></pre>
		</td>
	</tr>
</table>

Or, you do not want minify-selectors to encode certain selectors (for reasons such as SEO). You can prefix your selector names so minify-selectors will leave the name as is (the prefix will be omitted):

- `.__ignore--` and `#__ignore--` instead of the selector type ('#' or '.') before selector names
- `__ignore--` for selectors that are "name only"

<table>
	<tr>
		<td>
			<p><sub>Input</sub></p>
			<pre lang="html">
&lt;nav class="site-nav">                                   ‎
  &lt;a href="/faq/#__ignore--new-user">&lt;a>
&lt;nav><!--
			--></pre>
		</td>
		<td valign="top">
			<p><sub>Output:</sub></p>
			<pre lang="html">
&lt;nav class="b">                                          ‎
  &lt;a href="/faq/#new-user">&lt;a>
&lt;/div><!--
			--></pre>
		</td>
	</tr>
</table>

<br>




## How to use

> **Please note:**
> - minify-selectors only supports regular CSS, HTML, JS and SVG files. SASS/SCSS, LESS, TypeScript, JQuery, Handlebars, etc. should be compiled or transpiled first into its respective vanilla form.
> - minify-selectors is currently limited to UTF-8 encoded files.

### Via npm and npm scripts

1. Install [minify-selectors](https://www.npmjs.com/package/minify-selectors) via npm:
	```shell
	npm i minify-selectors
	```

2. Include minify-selectors in your package.json 'scripts' property, for example:
	```json
	"scripts": {
	  "build": "npm run build:webpack && npm run build:minify-selectors",
	  "build:minify-selectors": "minify-selectors --input \"example/dir/src/\" --output \"example/dir/dist/\"",
	  "build:webpack": "webpack --config config/webpack-prod.config.js"
	},
	```

3. Run npm script command, for example:
	```shell
	npm run build
	```

### Running as a CLI tool

1. Install via homebrew:
	```shell
	brew tap adamgian/minify-selectors && brew install minify-selectors
	```

2. Run in command line:
	```shell
	minify-selectors --input "example/dir/src" --output "example/dir/dist"
	```

<br>




## Configuration

### Command line options

<table>
	<thead>
		<tr>
			<th align="left" width="180">Option</th>
			<th align="left">Description</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td valign="top">
				<code lang="shell">--input</code>, <code lang="shell">-i</code>
			</td>
			<td>
				Directory to process. Any CSS, HTML, JS and SVG files in the given directory and sub-directories will be parsed.
			</td>
		</tr>
		<tr>
			<td valign="top">
				<code lang="shell">--output</code>, <code lang="shell">-o</code>
			</td>
			<td>
				Directory to place processed files into. Setting the output path to be the same as the input path will overwrite existing files.
			</td>
		</tr>
		<tr>
			<td valign="top">
				<code lang="shell">--config</code>, <code lang="shell">-c</code>
			</td>
			<td>
				Alternatively instead of using CLI arguments, an external JSON config file can be provided instead. See section below: '<a href="#external-json-config-options">External JSON config options</a>' for more information.
			</td>
		</tr>
		<tr>
			<td valign="top">
				<code lang="shell">--alphabet</code>
			</td>
			<td>
				<sup><i>Optional</i></sup><br>
				String sequence of characters to use when encoding.
				<br><br>Default: <code>"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"</code>
			</td>
		</tr>
		<tr>
			<td valign="top">
				<code lang="shell">--start-index</code>
			</td>
			<td>
				<sup><i>Optional</i></sup><br>
				Index to start incrementing and encoding from.
				<br><br>Default: <code>0</code>
			</td>
		</tr>
		<tr>
			<td valign="top">
				<code lang="shell">--parallel</code>
			</td>
			<td>
				<sup><i>Optional</i></sup><br>
				Enable concurrent processing.
				<br><br>Default: <code>false</code>
			</td>
		</tr>
		<tr>
			<td valign="top">
				<code lang="shell">--sort</code>
			</td>
			<td>
				<sup><i>Optional</i></sup><br>
				Reorder selectors by descending frequency before assigning indexes and minifying.
				<br><br>Default: <code>true</code>
			</td>
		</tr>
		<tr>
			<td valign="top">
				<code lang="shell">--custom-id-attribute</code>
			</td>
			<td>
				<sup><i>Optional</i></sup><br>
				Custom HTML and SVG attributes that contain an ID (or space-separated list of IDs). For example, in Bootstrap you may have <code lang="html">data-bs-target=&quot;#exampleModal&quot;</code> and <code lang=html>data-bs-parent=&quot;#accordionExample&quot;</code>
				<br><br>Usage: <code lang="shell">--custom-id-attribute data-bs-target data-bs-parent</code>
			</td>
		</tr>
		<tr>
			<td valign="top">
				<code lang="shell">--custom-class-attribute</code>
			</td>
			<td>
				<sup><i>Optional</i></sup><br>
				Custom HTML and SVG attributes that contain a space-separated list of classes.
			</td>
		</tr>
		<tr>
			<td valign="top">
				<code lang="shell">--custom-selector-attribute</code>
			</td>
			<td>
				<sup><i>Optional</i></sup><br>
				Custom HTML and SVG attributes that contain a selector string.
			</td>
		</tr>
		<tr>
			<td valign="top">
				<code lang="shell">--custom-anchor-attribute</code>
			</td>
			<td>
				<sup><i>Optional</i></sup><br>
				Custom HTML and SVG attributes that contain a URL.
			</td>
		</tr>
		<tr>
			<td valign="top">
				<code lang="shell">--custom-style-attribute</code>
			</td>
			<td>
				<sup><i>Optional</i></sup><br>
				Custom HTML and SVG attributes that contain CSS styles.
			</td>
		</tr>
		<tr>
			<td valign="top">
				<code lang="shell">--custom-script-attribute</code>
			</td>
			<td>
				<sup><i>Optional</i></sup><br>
				Custom HTML and SVG attributes that contain JS code.
			</td>
		</tr>
	</tbody>
</table>

### External JSON config options

An example config file:

```json
{
    "input": "/User/Adam/Github/project/dist",
    "output": "/User/Adam/Github/project/dist",
    "parallel": true,
    "customAttributes": {
        "id": [
            "data-bs-target",
            "data-bs-parent"
        ]
    }
}
```

<table>
	<thead>
		<tr>
			<th align="left" width="180">Option</th>
			<th align="left">Description</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td valign="top">
				<code lang="shell">input</code>
			</td>
			<td>
				Directory to process. Any CSS, HTML, JS and SVG files in the given directory and sub-directories will be parsed.
			</td>
		</tr>
		<tr>
			<td valign="top">
				<code lang="shell">output</code>
			</td>
			<td>
				Directory to place processed files into. Setting the output path to be the same as the input path will overwrite existing files.
			</td>
		</tr>
		<tr>
			<td valign="top">
				<code lang="shell">alphabet</code>
			</td>
			<td>
				<sup><i>Optional</i></sup><br>
				String sequence of characters to use when encoding.
				<br><br>Default: <code>"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"</code>
			</td>
		</tr>
		<tr>
			<td valign="top">
				<code lang="shell">start-index</code>
			</td>
			<td>
				<sup><i>Optional</i></sup><br>
				Index to start incrementing and encoding from.
				<br><br>Default: <code>0</code>
			</td>
		</tr>
		<tr>
			<td valign="top">
				<code lang="shell">parallel</code>
			</td>
			<td>
				<sup><i>Optional</i></sup><br>
				Enable concurrent processing.
				<br><br>Default: <code>false</code>
			</td>
		</tr>
		<tr>
			<td valign="top">
				<code lang="shell">sort</code>
			</td>
			<td>
				<sup><i>Optional</i></sup><br>
				Reorder selectors by descending frequency before assigning indexes and minifying.
				<br><br>Default: <code>true</code>
			</td>
		</tr>
		<tr>
			<td valign="top">
				<code lang="shell">customAttributes.id</code>
			</td>
			<td>
				<sup><i>Optional</i></sup><br>
				Custom HTML and SVG attributes that contain an ID (or space-separated list of IDs).
				<br><br>Usage: <code lang="shell">"id": [ "foo", ..., "baz" ]</code>
			</td>
		</tr>
		<tr>
			<td valign="top">
				<code lang="shell">customAttributes.class</code>
			</td>
			<td>
				<sup><i>Optional</i></sup><br>
				Custom HTML and SVG attributes that contain a space-separated list of classes.
				<br><br>Usage: <code lang="shell">"class": [ "foo", ..., "baz" ]</code>
			</td>
		</tr>
		<tr>
			<td valign="top">
				<code lang="shell">customAttributes.selector</code>
			</td>
			<td>
				<sup><i>Optional</i></sup><br>
				Custom HTML and SVG attributes that contain a selector string.
				<br><br>Usage: <code lang="shell">"selector": [ "foo", ..., "baz" ]</code>
			</td>
		</tr>
		<tr>
			<td valign="top">
				<code lang="shell">customAttributes.anchor</code>
			</td>
			<td>
				<sup><i>Optional</i></sup><br>
				Custom HTML and SVG attributes that contain a URL.
				<br><br>Usage: <code lang="shell">"anchor": [ "foo", ..., "baz" ]</code>
			</td>
		</tr>
		<tr>
			<td valign="top">
				<code lang="shell">customAttributes.style</code>
			</td>
			<td>
				<sup><i>Optional</i></sup><br>
				Custom HTML and SVG attributes that contain CSS styles.
				<br><br>Usage: <code lang="shell">"style": [ "foo", ..., "baz" ]</code>
			</td>
		</tr>
		<tr>
			<td valign="top">
				<code lang="shell">customAttributes.script</code>
			</td>
			<td>
				<sup><i>Optional</i></sup><br>
				Custom HTML and SVG attributes that contain JS code.
				<br><br>Usage: <code lang="shell">"script": [ "foo", ..., "baz" ]</code>
			</td>
		</tr>
	</tbody>
</table>
