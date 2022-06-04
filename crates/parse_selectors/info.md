# parse_selectors

## Selector names

### What constitutes a valid identifier?

minify-selectors finds and encodes IDs and classes in selector rules. As long as the IDs and classes are valid as per https://www.w3.org/TR/selectors-3/#lex, minify-selectors will be able to pick them up to encode.

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

<br>




## CSS files and embedded styles support

> **Please note:**
> minify-selector currently does not properly support escaped or unicode characters in CSS selectors. See [#10](https://github.com/adamgian/minify-selectors/issues/10).
>
> ```scss
> /* 😢 */
> .\265F -baz { … }
> .🗿 { … }
> ```


### Selector naming

CSS selector names are case-sensitive.

<table>
<tr><td><p><sub>Source:</sub></p>
<pre lang="scss">
#foo { … }                                   ‎
.foo { … }
#FOO { … }
#FOO { … }
#FOOBAR { … }
.bar { … }
.FooBar { … }
</pre>
</td><td><p><sub>Output:</sub></p>
<pre lang="scss">
#a { … }                                     ‎
.b { … }
#c { … }
#c { … }
#d { … }
.e { … }
.f { … }
</pre>
</td></tr>
</table>

As long as the selector name is valid (see above section: "What constitutes a valid identifier?"), minify-selectors will process it.

<table>
<tr><td><p><sub>Source:</sub></p>
<pre lang="scss">
.foo-bar { … }                               ‎
.foo_bar { … }
.fooBar  { … }
.a { … }
.-foo { … }
._baz { … }
</pre>
</td><td><p><sub>Output:</sub></p>
<pre lang="scss">
.g { … }                                     ‎
.h { … }
.i { … }
.j { … }
.k { … }
.l { … }
</pre>
</td></tr>
</table>

Naming conventions such as BEM are no problem for minify-selectors.

<table>
<tr><td><p><sub>Source:</sub></p>
<pre lang="scss">
.foo__bar { … }                              ‎
.foo--bar { … }
.foo-bar--baz { … }
.foo__bar--baz { … }
.foo-bar__bar--baz { … }
</pre>
</td><td><p><sub>Output:</sub></p>
<pre lang="scss">
.m { … }                                     ‎
.n { … }
.o { … }
.p { … }
.q { … }
</pre>
</td></tr>
</table>


### Chaining and combinations

<table>
<tr><td><p><sub>Source:</sub></p>
<pre lang="scss">
div.foo { … }                                ‎
.foo.foo { … }
.foo.bar { … }
.foo * { … }
.foo .bar a { … }
.foo, .bar { … }
.foo > .bar { … }
.foo + .bar { … }
.foo ~ .bar { … }
</pre>
</td><td><p><sub>Output:</sub></p>
<pre lang="scss">
div.b { … }                                  ‎
.b.b { … }
.b.e { … }
.b * { … }
.b .e a { … }
.b, .e { … }
.b > .e { … }
.b + .e { … }
.b ~ .e { … }
</pre>
</td></tr>
</table>


### Pseudo-classes and elements

<table>
<tr><td><p><sub>Source:</sub></p>
<pre lang="scss">
.foo:link { … }                              ‎
.foo:visited { … }
.foo:hover { … }
.foo:active { … }
.foo:not(.bar) { … }
:is(.foo) .bar { … }
.foo:is(:not(.baz)) .bar { … }
:where(.foo, .baz) .bar { … }
.foo::after { … }
.foo::before { … }
</pre>
</td><td><p><sub>Output:</sub></p>
<pre lang="scss">
.b:link { … }                                ‎
.b:visited { … }
.b:hover { … }
.b:active { … }
.b:not(.e) { … }
:is(.b) .e { … }
.b:is(:not(.r)) .e { … }
:where(.b, .r) .e { … }
.b::after { … }
.b::before { … }
</pre>
</td></tr>
</table>


### Attribute selectors

> **Please note:**
> Operators that do not match by string or word (`|=`, `^=`, `$=`, `*=`) and case-insensitive matches (`[class="Foo" i]`) are not supported.
>
> ```scss
> /* 😢 */
> [class|="foo"] { … }
> [class^="foo"] { … }
> [class$="foo"] { … }
> [class*="foo"] { … }
> [class="foo" i] { … }
> ```
>
> As any non-valid flag in CSS rules are not valid and ignored by browsers, minify-selectors does not bother to process it.
>
> ```scss
> /* 😢 */
> [class="foo" x] { … }
> [class="foo" ?] { … }
> ```

minify-selectors will work on attribute selectors with operators that guarantee a match — such as `=` (attribute value equals exactly) or `~=` (attribute value contains matching word).

<table>
<tr><td><p><sub>Source:</sub></p>
<pre lang="scss">
.foo[disabled] { … }                         ‎
[id="foo"] { … }
[id='foo'] { … }
[id=foo] { … }
[class="bar"] { … }
[class='bar'] { … }
[class=bar] { … }
[class~="baz"] { … }
.foo[href$=".com.au"] { … }
</pre>
</td><td><p><sub>Output:</sub></p>
<pre lang="scss">
.b[disabled] { … }                           ‎
[id="b"] { … }
[id='b'] { … }
[id=b] { … }
[class="e"] { … }
[class='e'] { … }
[class=e] { … }
[class~="r"] { … }
.b[href$=".com.au"] { … }
</pre>
</td></tr>
</table>

<br>




## JS files and embedded scripts support

> **Please note:**
> minify-selectors will not be able to detect selectors that are in variables. Selector names will need to be prefixed (see: [Marking selectors](#marking-selectors)).
>
> ```js
> // 😢
> let foo = "foo";
> document.getElementById(foo);
>
> // 😢
> bar.innerHtml = `<button class="btn btn-danger" id="${foo}"></button>`;
> ```
>
> minify-selectors currently does not support parsing of selector names in expressions and logic within the function arguments. Selector names will need to be prefixed (see: [Marking selectors](#marking-selectors)). [#15](https://github.com/adamgian/minify-selectors/issues/15) aims to resolve this a certain extent.
>
> ```js
> // 😢
> foo.classList.add(foo > bar ? "foo" : "bar");
> foo.classList.add(isFoo() ?? "foo");
> foo.classList.add(isFoo && "foo");
> ```


### Classes

<table>
<tr><td><p><sub>Source:</sub></p>
<pre lang="js">
document.getElementsByClassName('foo');      ‎
bar.setAttribute('class', 'bar');
baz.classList.add('foo', 'bar', 'baz');
baz.classList.remove('foo', 'bar');
baz.classList.contains('baz');
baz.classList.replace('baz', 'foo');
baz.classList.toggle('baz', foo > bar);
baz.className += "foo";
baz.className = "bar";
</pre>
</td><td><p><sub>Output:</sub></p>
<pre lang="js">
document.getElementsByClassName('b');        ‎
bar.setAttribute('class', 'e');
baz.classList.add('b', 'e', 'r');
baz.classList.remove('b', 'e');
baz.classList.contains('r');
baz.classList.replace('r', 'b');
baz.classList.toggle('r', foo > bar);
baz.className += "b";
baz.className = "e";
</pre>
</td></tr>
</table>


### IDs

<table>
<tr><td><p><sub>Source:</sub></p>
<pre lang="js">
document.getElementById('foo');              ‎
bar.setAttribute('id', 'bar');
baz.setAttribute('for', 'foo');
baz.setAttribute('form', 'foo');
baz.setAttribute('list', 'foo');
baz.setAttribute('headers', 'foo');
</pre>
</td><td><p><sub>Output:</sub></p>
<pre lang="js">
document.getElementById('a');                ‎
bar.setAttribute('id', 'e');
baz.setAttribute('for', 'a');
baz.setAttribute('form', 'a');
baz.setAttribute('list', 'a');
baz.setAttribute('headers', 'a');
</pre>
</td></tr>
</table>


### Selector strings

<table>
<tr><td><p><sub>Source:</sub></p>
<pre lang="js">
document.querySelector('#foo');              ‎
document.querySelector('.foo > .bar');
document.querySelectorAll('.foo');
document.querySelector('p.baz:disabled');
</pre>
</td><td><p><sub>Output:</sub></p>
<pre lang="js">
document.querySelector('#a');                ‎
document.querySelector('.b > .e');
document.querySelectorAll('.b');
document.querySelector('p.r:disabled');
</pre>
</td></tr>
</table>

<br>




## HTML support

### Standard attributes

minify-selectors supports all the standard HTML attibutes that contain selector classes and IDs — `aria-describedby`, `aria-labelledby`, `class`, `for`, `form`, `headers`, `id`, `itemref` and `list`.

<table>
<tr><td><p><sub>Source:</sub></p>
<pre lang="html">
&lt;input id="foo" type="text">                 ‎
&lt;div class="foo bar">&lt;/div>
&lt;label for="foo">&lt;/label>
&lt;a href="#" aria-labelledby="foo">&lt;/a>
&lt;a href="#" aria-describedby="foo">&lt;/a>
&lt;input form="foo">
&lt;input list="foo">
&lt;td headers="foo">&lt;/td>
&lt;div itemref="foo bar">&lt;/div>
</pre>
</td><td><p><sub>Output:</sub></p>
<pre lang="html">
&lt;input id="a" type="text">                   ‎
&lt;div class="b e">&lt;/div>
&lt;label for="a">&lt;/label>
&lt;a href="#" aria-labelledby="a">&lt;/a>
&lt;a href="#" aria-describedby="a">&lt;/a>
&lt;input form="a">
&lt;input list="a">
&lt;td headers="a">&lt;/td>
&lt;div itemref="a e">&lt;/div>
</pre>
</td></tr>
</table>


### Custom attributes

Selectors in custom attribute values need to be prefixed (see: [Marking selectors](#marking-selectors)). In future, [#12](https://github.com/adamgian/minify-selectors/issues/12) will be another method to support custom attributes.

<table>
<tr><td><p><sub>Source:</sub></p>
<pre lang="html">
&lt;button                                      ‎
  data-toggle="modal"
  data-target="#__--modal-confirm-delete">
&lt;/button>
</pre>
</td><td><p><sub>Output:</sub></p>
<pre lang="html">
&lt;button                                      ‎
  data-toggle="modal"
  data-target="#t">
&lt;/button>
</pre>
</td></tr>
</table>


### URLs

Target IDs in relative anchor links are also handled.

<table>
<tr><td><p><sub>Source:</sub></p>
<pre lang="html">
&lt;a href="#foo">&lt;/a>                          ‎
&lt;a href="/#bar">&lt;/a>
&lt;a href="faq/#baz">&lt;/a>
&lt;a href="https://example.com/foo/#bar">&lt;/a>
</pre>
</td><td><p><sub>Output:</sub></p>
<pre lang="html">
&lt;a href="#a">&lt;/a>                            ‎
&lt;a href="/#e">&lt;/a>
&lt;a href="faq/#s">&lt;/a>
&lt;a href="https://example.com/foo/#bar">&lt;/a>
</pre>
</td></tr>
</table>

<br>




## Marking selectors

<sub>New feature in v1.0.0</sub>

You can instruct minify-selectors to either encode or ignore certain selectors by setting the appropriate prefix.

### CSS selectors

Using `.__--` instead of `.` or `#__--` instead of `#` before the selector name will instruct minify-selectors to encode a class or ID respectively.

<table>
<tr><td><p><sub>Source:</sub></p>
<pre lang="html">
&lt;button                                         ‎
  class="btn btn-default"
  data-toggle="modal"
  data-target="#__--confirm-prompt">
&lt;/button>
&lt;div class="modal" id="confirm-prompt">&lt;/div>
</pre>
</td><td><p><sub>Output:</sub></p>
<pre lang="html">
&lt;button                                         ‎
  class="a b"
  data-toggle="modal"
  data-target="#c">
&lt;/button>
&lt;div class="d" id="c">&lt;/div>
</pre>
</td></tr>
</table>


### Selector names only

Adding a `__class--` or `__id--` before a selector name will instruct minify-selectors to encode a selector as a class and ID respectively.

<table>
<tr><td><p><sub>Source:</sub></p>
<pre lang="js">
const ACTIVE_STATE = '__class--is_active';         ‎
component.classList.toggle(ACTIVE_STATE);
</pre>
</td><td><p><sub>Output:</sub></p>
<pre lang="js">
const ACTIVE_STATE = 'a';                          ‎
component.classList.toggle(ACTIVE_STATE);
</pre>
</td></tr>
</table>


### Ignoring selectors

Adding a `__ignore--` before a selector name will instruct minify-selectors not to encode the selector.

<table>
<tr><td><p><sub>Source:</sub></p>
<pre lang="html">
&lt;nav class="site-nav">
  &lt;a href="about/#__ignore--contact">&lt;/a>
&lt;/nav>
&lt;script>
  ctx = document.getElementByID('__ignore--canvas');
&lt;/script>
</pre>
</td><td><p><sub>Output:</sub></p>
<pre lang="html">
&lt;nav class="a">                                 ‎
  &lt;a href="about/#contact">&lt;/a>
&lt;/nav>
&lt;script>
  ctx = document.getElementByID('canvas');
&lt;/script>
</pre>
</td></tr>
</table>
