# minify-selectors

Post-processor that minifies classes and IDs in CSS, HTML and Javascript files.

Each unique identifier is assigned an index which is converted into an ultra compact CSS valid selector.

## Examples

CSS (source):
```css
#page--default { … }
.sidebar, .site-nav { … }
.sidebar .search:focus-within { … }
.sidebar--expanded a.is-active { … }
```

CSS (output):
```css
#a { … }
.b, .c { … }
.b .d:focus-within { … }
.e a.d { … }
```

HTML (source):
```html
<main id="page--default">
    <nav class="site-nav">
        <div class="search has-content">
            <label for="site-search" class="text--muted text--center">…</label>
            <input type="text" id="site-search" class="form-input--single form-input--lg border--thick">
        </div>
    </nav>
</main>
```

HTML (output):
```html
<main id="a">
    <nav class="c">
        <div class="d a1">
            <label for="y" class="F j">…</label>
            <input type="text" id="y" class="A9 t Av">
        </div>
    </nav>
</main>
```

JS (source)
```js
for (let link of document.querySelectorAll('a.anchor')) {
    link.classList.remove('is-active');
}
```

JS (output)
```js
for (let link of document.querySelectorAll('a.Bd')) {
    link.classList.remove('d');
}
```

For a full outline of capabilities and current limitations, see [parse_selectors info](crates/parse_selectors/info.md)

## Usage

Install from npm
```shell
npm i minify-selectors
```

Run within npm scripts or in command line
```shell
minify-selectors --input "example/dir/src" --output "example/dir/dist"
```

minify-selectors only supports regular CSS, HTML and JS files. minify-selectors should be one of the final steps in your build process — SASS/SCSS, LESS, Typescript, JQuery, Handlebars, etc. should be processed first.

## Options

### CLI flags

| Flag  | Description  |
|-------|--------------|
| `--input` (or&nbsp;`-i`) | Directory or file to process. If a directory path is provided — any CSS, HTML and JS files in the given directory and sub-directories will be parsed. If only a filepath is provided — only the given file will be parsed. |
| `--output` (or&nbsp;`-o`) | Directory to ouput processed files to. Setting the output path to be the same as the input path will overwrite existing files. |
