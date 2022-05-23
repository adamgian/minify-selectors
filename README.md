# minify-selectors

Post-processor that minifies classes and IDs selector names in CSS, HTML and Javascript files. Each unique identifier, and any subsequent occurances, is converted to an ultracompact CSS valid selector name.

Wrings out that little bit more out of your payload sizes and shave off a wee bit off file parse times. Additionally adds a certain degree of obfuscation to your selector names and stylesheets.

> **Please note:**
minify-selectors only supports regular CSS, HTML and JS files. minify-selectors should be one of the final steps in your build process — SASS/SCSS, LESS, Typescript, JQuery, Handlebars, etc. should be compiled or transpiled first into its respective vanilla form.



## Examples

### CSS

<table>
<tr><td><p><sub>Source:</sub></p>
<pre lang="scss">
[id='page--default'] { … }                               ‎
.sidebar, .site-nav { … }
.sidebar .search:focus-within { … }
.sidebar--expanded a.is-active { … }
</pre>
</td><td><p><sub>Output:</sub></p>
<pre lang="scss">
[id='a'], { … }                                          ‎
.b, .c { … }
.b .d:focus-within { … }
.e a.f { … }
</pre>
</td></tr>
</table>


### HTML

<table>
<tr><td><p><sub>Source:</sub></p>
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
&lt;/body>
</pre>
</td><td valign="top"><p><sub>Output:</sub></p>
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
&lt;/body>
</pre>
</td></tr>
</table>


### JS

<table>
<tr><td><p><sub>Source:</sub></p>
<pre lang="js">
for (let link of document.querySelectorAll('a.anchor')) {‎
  link.classList.remove('is-active');
}
</pre>
</td><td><p><sub>Output:</sub></p>
<pre lang="js">
for (let link of document.querySelectorAll('a.Bd')) {    ‎
  link.classList.remove('f');
}
</pre>
</td></tr>
</table>

For a full outline of capabilities and current limitations, see [parse_selectors/info.md](crates/parse_selectors/info.md).


## Usage

### CLI

1. Install from npm:
	```shell
	npm i -g minify-selectors
	```

2. Run in command line:
	```shell
	minify-selectors --input "example/dir/src" --output "example/dir/dist"
	```

### npm scripts

1. Install from npm:
	```shell
	npm i minify-selectors
	```

2. Include minify-selectors in your package.json scripts:
	```json
	"scripts": {
	  "build": "npm run build:webpack && npm run build:minify-selectors",
	  "build:minify-selectors": "minify-selectors --input \"example/dir/src/\" --output \"example/dir/dist/\"",
	  "build:webpack": "webpack --config config/webpack-prod.config.js"
	},
	```

2. Run npm script:
	```shell
	npm run build
	```


## Options

| Flag  | Description  |
|:------|:-------------|
| `--input` (or&nbsp;`-i`) | Directory or file to process. If a directory path is provided — any CSS, HTML and JS files in the given directory and sub-directories will be parsed. If only a filepath is provided — only the given file will be parsed. |
| `--output` (or&nbsp;`-o`) | Directory to ouput processed files to. Setting the output path to be the same as the input path will overwrite existing files. |
| `--alphabet` | Custom sequence of characters to use when encoding. <br><br>By default, selector names will be encoded using the following base 62 string: `"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"` |
| `--start-index` | Index to start incrementing and encoding from. <br><br>By default, this will begin from `0` (essentially `a` if using the default alphabet). |
