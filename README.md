[1]: https://github.com/adamgian/minify-selectors/releases/latest
[2]: https://img.shields.io/npm/v/minify-selectors?color=blue&label=Latest%20release
[3]: https://www.apache.org/licenses/LICENSE-2.0
[4]: https://img.shields.io/badge/License-Apache%202.0-green.svg

[![Latest release version][2]][1]&nbsp;
[![Apache 2.0 license][4]][3]




# minify-selectors

Post-processor that minifies classes and IDs selector names in CSS, HTML and Javascript files. Each unique selector, and any subsequent occurances elsewhere, is converted into an ultracompact one.

Enhance your front-end assets and build optimisations pipeline — wring even more out from your already minified and optimised payload sizes. Additionally, can offer a certain degree of obfuscation to your code.

<br>




## Features

For a full outline of capabilities and current limitations, see [parse_selectors/info.md](crates/parse_selectors/info.md).

### Comprehensive out of the box selector support

minify-selectors aims to minify all obvious selectors right out of the gate. Any extra work configuring should be to assist minify-selectors to identify additional and/or ambigious selectors.


#### CSS (or embedded style) example

<table>
	<tr>
		<td>
			<p><sub>Source:</sub></p>
			<pre lang="scss">
[id='page--default'] { … }                               ‎
.sidebar, .site-nav { … }
.sidebar .search:focus-within { … }
.sidebar--expanded a.is-active { … }<!--
			--></pre>
		</td>
		<td>
			<p><sub>Output:</sub></p>
			<pre lang="scss">
[id='a'], { … }                                          ‎
.b, .c { … }
.b .d:focus-within { … }
.e a.f { … }<!--
			--></pre>
		</td>
	</tr>
</table>


#### JS (or embedded script) example

<table>
	<tr>
		<td><p><sub>Source:</sub></p>
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
  link.classList.remove('f');
}<!--
			--></pre>
		</td>
	</tr>
</table>


#### HTML example

<table>
	<tr>
		<td>
			<p><sub>Source:</sub></p>
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
  &lt;nav class="c">
    &lt;div class="d a1">
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


### Opt-in/opt-out selector encoding

<sub>Available in v1.0.0</sub>

In cases where minify-selectors is unable to parse selectors, for example: in custom HTML attributes or JS variables. Or forcing a selector to be encoded when it otherwise would not be, such as in a HTML code element or comments. You can prefix your selector names so that minify-selectors knows to parse it and how to encode it:

- `.__--` or `#__--` — instead of the selector type ('#' or '.') before CSS selector names
- `__class--` or `__id--` — for "name only" selectors, use '\_\_class--' for class selectors and '\_\_id--' for ID selectors

<table>
	<tr>
		<td>
			<p><sub>Source:</sub></p>
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

- `.__ignore--` and `#__ignore--` — instead of the selector type ('#' or '.') before CSS selector names
- `__ignore--` — for selectors that are "name only"

<table>
	<tr>
		<td>
			<p><sub>Source:</sub></p>
			<pre lang="html">
&lt;nav class="site-nav">                                   ‎
  &lt;a href="/faq/#__ignore--new-user">&lt;a>
&lt;nav><!--
			--></pre>
		</td>
		<td valign="top">
			<p><sub>Output:</sub></p>
			<pre lang="html">
&lt;nav class="c">                                          ‎
  &lt;a href="/faq/#new-user">&lt;a>
&lt;/div><!--
			--></pre>
		</td>
	</tr>
</table>

<br>




## Usage

> **Please note:**
minify-selectors only supports regular CSS, HTML and JS files. minify-selectors should be one of the final steps in your build process — SASS/SCSS, LESS, Typescript, JQuery, Handlebars, etc. should be compiled or transpiled first into its respective vanilla form.

### Via npm and npm scripts

1. Install via npm:
	```shell
	npm i minify-selectors
	```

2. Include minify-selectors in your package.json 'scripts' property:
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

### Running as CLI tool

1. Install via homebrew:
	```shell
	brew tap adamgian/minify-selectors && brew install minify-selectors
	```

2. Run in command line:
	```shell
	minify-selectors --input "example/dir/src" --output "example/dir/dist"
	```

<br>





## Options

<table>
	<thead>
		<tr>
			<th align="left" width="165">Flag</th>
			<th align="left">Description</th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td valign="top">
				<code lang="shell">--input</code> (or <code lang="shell">-i</code>)
			</td>
			<td>
				Directory to process. Any CSS, HTML and JS files in the given directory and sub-directories will be parsed.
			</td>
		</tr>
		<tr>
			<td valign="top">
				<code lang="shell">--output</code> (or <code lang="shell">-o</code>)
			</td>
			<td>
				Directory to ouput processed files to. Setting the output path to be the same as the input path will overwrite existing files.
			</td>
		</tr>
		<tr>
			<td valign="top">
				<code lang="shell">--alphabet</code>
			</td>
			<td>
				String sequence of characters to use when encoding.
				<br><br>Default: <code>"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"</code>
			</td>
		</tr>
		<tr>
			<td valign="top">
				<code lang="shell">--start-index</code>
			</td>
			<td>
				Index to start incrementing and encoding from.
				<br><br>Default: <code>0</code>
			</td>
		</tr>
	</tbody>
</table>
