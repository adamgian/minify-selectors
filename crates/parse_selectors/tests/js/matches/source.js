foo.matches(foo);
foo.matches('div');
foo.matches("h1");
foo.matches(`tfoot`);

a.matches("#ID-1");
a.matches(".CLASS-1");
a.matches("#ID-1.CLASS-1");
b.matches("#ID-2 > .CLASS-1");
b.matches("#ID-2>.CLASS-1");
b.matches("#ID-2 + .CLASS-1");
b.matches("#ID-2+.CLASS-1");
b.matches("#ID-2 ~ .CLASS-1");
b.matches("#ID-2~.CLASS-1");
b.matches("#ID-2 .CLASS-1");
b.matches("#ID-2 .CLASS-1:first-of-type");
