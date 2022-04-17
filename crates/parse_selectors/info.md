# Capabilities of parse_selectors

## CSS

### Classes and IDs names

minify-selectors finds and encodes IDs and classes in selector rules.
```scss
#foo { … }         // #a { … }  
.foo { … }         // .b { … }
#FOO { … }         // #c { … }
#FOOBAR { … }      // #d { … }
.bar { … }         // .e { … }
.FooBar { … }      // .f { … }
.\265F -baz { … }  // 😢
.\🗿 { … }         // 😢
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

⚠️ **Please note:** 
<br>minify-selector currently does not support escaped or unicode characters in CSS selectors.

```scss
.foo-bar { … }  // .g { … }
.foo_bar { … }  // .h { … }
.fooBar  { … }  // .i { … }
.a { … }        // .j { … }
.-foo { … }     // .k { … }
._baz { … }     // .l { … }
```

Naming conventions such as BEM should be no problem for minify-selectors.
```scss
.foo__bar { … }           // .m { … }
.foo--bar { … }           // .n { … }
.foo-bar--baz { … }       // .o { … }
.foo__bar--baz { … }      // .p { … }
.foo-bar__bar--baz { … }  // .q { … }
```

### Chaining and combinations

```scss
div.foo { … }      // div.b { … }
.foo.foo { … }     // .b.b { … }
.foo.bar { … }     // .b.e { … }
.foo * { … }       // .b * { … }
.foo .bar a { … }  // .b .e a { … }
.foo, .bar { … }   // .b, .e { … }
.foo > .bar { … }  // .b > .e { … }
.foo + .bar { … }  // .b + .e { … }
.foo ~ .bar { … }  // .b ~ .e { … }
```

### Pseudo-classes and elements

```scss
.foo:link { … }                 // .b:link { … }
.foo:visited { … }              // .b:visited { … }
.foo:hover { … }                // .b:hover { … }
.foo:active { … }               // .b:active { … }
.foo:not(.bar) { … }            // .b:not(.e) { … }
:is(.foo) .bar { … }            // :is(.b) .e { … }
.foo:is(:not(.baz)) .bar { … }  // .b:is(:not(.r)) .e { … }
:where(.foo, .baz) .bar { … }   // :where(.b, .r) .e { … }
.foo::after { … }               // .b::after { … }
.foo::before { … }              // .b::before { … }
```

### Attribute selectors

minify-selectors will only work on attribute selectors with operators that can guarantee a match — such as `=` (value equals exactly) or `~=` (contains word match).

```scss
.foo[disabled] { … }         // .b[disabled] { … }
[id="foo"] { … }             // [id="b"] { … }
[id='foo'] { … }             // [id='b'] { … }
[id=foo] { … }               // [id=b] { … }
[class="bar"] { … }          // [class="e"] { … }
[class='bar'] { … }          // [class='e'] { … }
[class=bar] { … }            // [class=e] { … }
[class~="baz"] { … }         // [class~="r"] { … }
.foo[href$=".com.au"] { … }  // .b[href$=".com.au"] { … }
```

⚠️ **Please note:**
<br>Operators other than exact match (`|=`, `^=`, `$=`, `*=`) and case-insensitive matches `[class="Foo" i]` are not supported.

```scss
[class^="foo"] { … }   // 😢
[class$="foo"] { … }   // 😢
[class*="foo"] { … }   // 😢
[class="foo" i] { … }  // 😢
```

## JS

### Classes

```js
document.getElementsByClassName('foo');  // document.getElementsByClassName('b');
bar.setAttribute('class', 'bar');        // bar.setAttribute('class', 'e');
baz.classList.add('foo', 'bar', 'baz');  // baz.classList.add('b', 'e', 'r');
baz.classList.remove('foo', 'bar');      // baz.classList.remove('b', 'e');
baz.classList.contains('baz');           // baz.classList.contains('r');
baz.classList.replace('baz', 'foo');     // baz.classList.replace('r', 'b');
baz.classList.toggle('baz', foo > bar);  // baz.classList.toggle('r', b > e);
baz.className += "foo";                  // baz.className += "b";
```

### IDs

```js
document.getElementById('foo');    // document.getElementById('a');
bar.setAttribute('id', 'bar');     // bar.setAttribute('id', 'e');
input.setAttribute('for', 'foo');  // input.setAttribute('for', 'a');
```

### Selector strings

```js
document.querySelector('#foo');            // document.querySelector('#a');
document.querySelector('.foo > .bar');     // document.querySelector('.b > .e');
document.querySelectorAll('.foo');         // document.querySelectorAll('.b');
document.querySelector('p.baz:disabled');  // document.querySelector('p.r:disabled');
```

## HTML

```html
<input id="foo" type="text">            // <input id="a" type="text">
<div class="foo bar"></div>             // <div class="b e"></div>
<label for="foo"></label>               // <label for="a"></label>
<a href="#" aria-labelledby="foo"></a>  // <a href="#" aria-labelledby="a"></a>
```
