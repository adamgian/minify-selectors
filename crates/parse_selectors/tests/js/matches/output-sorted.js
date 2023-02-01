foo.matches(foo);
foo.matches('div');
foo.matches("h1");
foo.matches(`tfoot`);

a.matches("#b");
a.matches(".a");
a.matches("#b.a");
b.matches("#a > .a");
b.matches("#a>.a");
b.matches("#a + .a");
b.matches("#a+.a");
b.matches("#a ~ .a");
b.matches("#a~.a");
b.matches("#a .a");
b.matches("#a .a:first-of-type");
