# minify-selectors

Post-processor that minifies classes and IDs selector names in CSS, HTML and Javascript files. Each unique identifier — and any subsequent occurances — is converted to an ultracompact CSS valid selector name.

> **Please note:**
minify-selectors only supports regular CSS, HTML and JS files. minify-selectors should be one of the final steps in your build process — SASS/SCSS, LESS, Typescript, JQuery, Handlebars, etc. should be compiled or transpiled first into its respective vanilla form.


## Examples

### CSS
```css
[id='page--default'] { … }
.sidebar, .site-nav { … }
.sidebar .search:focus-within { … }
.sidebar--expanded a.is-active { … }
```

```css
[id='a'], { … }
.b, .c { … }
.b .d:focus-within { … }
.e a.d { … }
```

### HTML
```html
<body id="page--default">
    <nav class="site-nav">
        <div class="search has-content">
            <label for="site-search" class="text--muted text--center">…</label>
            <input type="text" id="site-search" class="form-input--single form-input--lg border--thick">
        </div>
    </nav>
</body>
```

```html
<body id="a">
    <nav class="c">
        <div class="d a1">
            <label for="y" class="F j">…</label>
            <input type="text" id="y" class="A9 t Av">
        </div>
    </nav>
</body>
```

### JS
```js
for (let link of document.querySelectorAll('a.anchor')) {
    link.classList.remove('is-active');
}
```

```js
for (let link of document.querySelectorAll('a.Bd')) {
    link.classList.remove('d');
}
```

For a full outline of capabilities and current limitations, see [parse_selectors info](crates/parse_selectors/info.md)


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
