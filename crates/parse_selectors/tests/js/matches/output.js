foo.matches(foo);
foo.matches('div');
foo.matches("h1");
foo.matches(`tfoot`);

a.matches("#a");
a.matches(".a");
a.matches("#a.a");
b.matches("#b > .a");
b.matches("#b>.a");
b.matches("#b + .a");
b.matches("#b+.a");
b.matches("#b ~ .a");
b.matches("#b~.a");
b.matches("#b .a");
b.matches("#b .a:first-of-type");
