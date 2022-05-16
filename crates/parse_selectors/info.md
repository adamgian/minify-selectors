# parse_selectors

## CSS

### Classes and IDs names

minify-selectors finds and encodes IDs and classes in selector rules.
```scss
#foo { … }
.foo { … }
#FOO { … }
#FOOBAR { … }
.bar { … }
.FooBar { … }
```

```scss
#a { … }
.b { … }
#c { … }
#d { … }
.e { … }
.f { … }
```

As long as the IDs and classes are valid as per https://www.w3.org/TR/selectors-3/#lex, minify-selectors will be able to pick them up to encode.

1. Starts with `#` or `.`.
2. Then optionally can be followed by a `-`.
3. Then a 'nmstart' character, which is any one of:
	- `[a-z]` — lowercase and uppercase Latin alphabet.
	- `[^\0-\177]` ('nonascii') — other characters that are not part of ASCII.
	- `{unicode}|\\[^\n\r\f0-9a-f]` ('escape') — escaping a character that is not a newline, return or unicode (which has it's own set of rules, see next point below). 
	- `\\[0-9a-f]{1,6}(\r\n|[ \n\r\t\f])?` ('unicode') an unicode number (`\01F60E`) which is up to six hexadecimal characters long. Note: shorter unicode numbers can be terminated earlier by a space, newline, tab or form feed (`\265F `) rather than padding the leading digit(s) with zeros (`\00265F`) to reach the six character limit.
4. Finally none, one or multiple 'nmchar' characters, these are:
	- `[_a-z0-9-]` — lowercase and uppercase Latin alphabet, underscore and dash.
	- `[^\0-\177]` ('nonascii') — other characters that are not part of ASCII.
	- `{unicode}|\\[^\n\r\f0-9a-f]` ('escape') — escaping a character that is not a newline, return or unicode (which has it's own set of rules, see next point below). 
	- `\\[0-9a-f]{1,6}(\r\n|[ \n\r\t\f])?` ('unicode') an unicode number (`\01F60E`) which is up to six hexadecimal characters long. Note: shorter unicode numbers can be terminated earlier by a space, newline, tab or form feed (`\265F `) rather than padding the leading digit(s) with zeros (`\00265F`) to reach the six character limit.

> **Please note:**
minify-selector currently does not support escaped or unicode characters in CSS selectors.

```scss
.\265F -baz { … }  /* 😢 */
.🗿 { … }          /* 😢 */
```

```scss
.foo-bar { … }
.foo_bar { … }
.fooBar  { … }
.a { … }
.-foo { … }
._baz { … }
```

```scss
.g { … }
.h { … }
.i { … }
.j { … }
.k { … }
.l { … }
```

Naming conventions such as BEM are no problem for minify-selectors.
```scss
.foo__bar { … }
.foo--bar { … }
.foo-bar--baz { … }
.foo__bar--baz { … }
.foo-bar__bar--baz { … }
```

```scss
.m { … }
.n { … }
.o { … }
.p { … }
.q { … }
```

### Chaining and combinations

```scss
div.foo { … }
.foo.foo { … }
.foo.bar { … }
.foo * { … }
.foo .bar a { … }
.foo, .bar { … }
.foo > .bar { … }
.foo + .bar { … }
.foo ~ .bar { … }
```

```scss
div.b { … }
.b.b { … }
.b.e { … }
.b * { … }
.b .e a { … }
.b, .e { … }
.b > .e { … }
.b + .e { … }
.b ~ .e { … }
```

### Pseudo-classes and elements

```scss
.foo:link { … }
.foo:visited { … }
.foo:hover { … }
.foo:active { … }
.foo:not(.bar) { … }
:is(.foo) .bar { … }
.foo:is(:not(.baz)) .bar { … }
:where(.foo, .baz) .bar { … }
.foo::after { … }
.foo::before { … }
```

```scss
.b:link { … }
.b:visited { … }
.b:hover { … }
.b:active { … }
.b:not(.e) { … }
:is(.b) .e { … }
.b:is(:not(.r)) .e { … }
:where(.b, .r) .e { … }
.b::after { … }
.b::before { … }
```

### Attribute selectors

minify-selectors will only work on attribute selectors with operators that can guarantee a match — such as `=` (value equals exactly) or `~=` (contains word match).

```scss
.foo[disabled] { … }
[id="foo"] { … }
[id='foo'] { … }
[id=foo] { … }
[class="bar"] { … }
[class='bar'] { … }
[class=bar] { … }
[class~="baz"] { … }
.foo[href$=".com.au"] { … }
```

```scss
.b[disabled] { … }
[id="b"] { … }
[id='b'] { … }
[id=b] { … }
[class="e"] { … }
[class='e'] { … }
[class=e] { … }
[class~="r"] { … }
.b[href$=".com.au"] { … }
```

> **Please note:**
Operators other than exact match (`|=`, `^=`, `$=`, `*=`) and case-insensitive matches `[class="Foo" i]` are not supported.

```scss
[class^="foo"] { … }   /* 😢 */
[class$="foo"] { … }   /* 😢 */
[class*="foo"] { … }   /* 😢 */
[class="foo" i] { … }  /* 😢 */
```


## JS

### Classes

```js
document.getElementsByClassName('foo');
bar.setAttribute('class', 'bar');
baz.classList.add('foo', 'bar', 'baz');
baz.classList.remove('foo', 'bar');
baz.classList.contains('baz');
baz.classList.replace('baz', 'foo');
baz.classList.toggle('baz', foo > bar);
baz.className += "foo";
baz.className = "bar";
```

```js
document.getElementsByClassName('b');
bar.setAttribute('class', 'e');
baz.classList.add('b', 'e', 'r');
baz.classList.remove('b', 'e');
baz.classList.contains('r');
baz.classList.replace('r', 'b');
baz.classList.toggle('r', b > e);
baz.className += "b";
baz.className = "e";
```

### IDs

```js
document.getElementById('foo');
bar.setAttribute('id', 'bar');
input.setAttribute('for', 'foo');
```
```js
document.getElementById('a');
bar.setAttribute('id', 'e');
input.setAttribute('for', 'a');
```

### Selector strings

```js
document.querySelector('#foo');
document.querySelector('.foo > .bar');
document.querySelectorAll('.foo');
document.querySelector('p.baz:disabled');
```
```js
document.querySelector('#a');
document.querySelector('.b > .e');
document.querySelectorAll('.b');
document.querySelector('p.r:disabled');
```

> **Please note:**
minify-selectors will not be able to detect class or ID names that are in variables or in strings.

```js
// 😢
let foo = "foo";
document.getElementById(foo);

// 😢
bar.innerHtml = `<button class="btn btn-danger" id="${foo}"></button>`;
```

## HTML

```html
<input id="foo" type="text">
<div class="foo bar"></div>
<label for="foo"></label>
```
```html
<input id="a" type="text">
<div class="b e"></div>
<label for="a"></label>
```

minify-selectors supports all native HTML attibutes that contain IDs.

```html
<a href="#" aria-labelledby="foo"></a>
<a href="#" aria-describedby="foo"></a>
<input form="foo">
<input list="foo">
<td headers="foo"></td>
<div itemref="foo bar"></div>
```
```html
<a href="#" aria-labelledby="a"></a>
<a href="#" aria-describedby="a"></a>
<input form="a">
<input list="a">
<td headers="a"></td>
<div itemref="a e"></div>
```

> **Please note:**
Custom attributes are currently not supported.

```html
<!-- 😢 -->
<button data-toggle="modal" data-target="#modal-confirm-order-delete">
</button>
```
