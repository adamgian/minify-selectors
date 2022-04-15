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
#AA { … }
.AB, .AC { … }
.AB .AD:focus-within { … }
.AE a.AD { … }
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
<main id="AA">
    <nav class="AC">
        <div class="AD B2">
            <label for="Ay" class="D3 D4">…</label>
            <input type="text" id="Ay" class="C9 CH Di">
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
    link.classList.remove('AD');
}
```

For a full outline of capabilities and current limitations, see TODO

## Usage

Install from npm
```shell
npm i minify-selectors
```

Running in the command line
```shell
minify-selectors --input "example/dir/src" --output "example/dir/dist"
```

For more CLI options, see the 'Options' section below.

## Options
