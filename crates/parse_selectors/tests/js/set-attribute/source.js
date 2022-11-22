a.setAttribute("id", "SELECTOR-1");
a.setAttribute('id', 'SELECTOR-1');
a.setAttribute(`id`, `SELECTOR-1`);
a.setAttribute('ref', 'SELECTOR-1');
a.setAttribute('data-custom', 'SELECTOR-1');

b.setAttribute( "id", "SELECTOR-2" );
b.setAttribute( "id" , "SELECTOR-2" );
b.setAttribute(  "id"  ,  "SELECTOR-2"  );
b.setAttribute(  "id "  ,  "SELECTOR-2 "  );
b.setAttribute(  " id"  ,  " SELECTOR-2"  );
b.setAttribute(  " id "  ,  " SELECTOR-2 "  );
b.setAttribute(
	"id", "SELECTOR-2"
);

c.setAttribute(`class`, `SELECTOR-3`);
d.setAttribute(`aria-describedby`, `SELECTOR-4`);
e.setAttribute(`aria-labelledby`, `SELECTOR-5`);
f.setAttribute(`for`, `SELECTOR-6`);
g.setAttribute(`form`, `SELECTOR-7`);
h.setAttribute(`headers`, `SELECTOR-8`);
i.setAttribute(`itemref`, `SELECTOR-9`);
j.setAttribute(`list`, `SELECTOR-10`);

k.setAttribute(foo, "foo")
l.setAttribute(bar.bar, "bar")
m.setAttribute("class", "")
n.setAttribute("class", foo)
o.setAttribute(foo, baz)

p.setAttribute("id", "SELECTOR-\x31");
p.setAttribute("id", "SELECTOR-\u0031");
p.setAttribute("id", "SELECTOR-\u{31}");
p.setAttribute("id", "SELECTOR-\u{031}");
p.setAttribute("id", "SELECTOR-\u{0031}");
p.setAttribute("id", "SELECTOR-\u{00031}");

q.setAttribute("style", "");
q.setAttribute("style", "position:absolute; top:0; left:0;");
q.setAttribute("style", "grid-template: \"a a a\"\"b c c\"");
