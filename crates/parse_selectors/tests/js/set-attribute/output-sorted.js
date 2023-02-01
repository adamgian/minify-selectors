a.setAttribute("id", "a");
a.setAttribute('id', 'a');
a.setAttribute(`id`, `a`);
a.setAttribute('ref', 'SELECTOR-1');
a.setAttribute('data-custom', 'SELECTOR-1');

b.setAttribute( "id", "b" );
b.setAttribute( "id" , "b" );
b.setAttribute(  "id"  ,  "b"  );
b.setAttribute(  "id "  ,  "b "  );
b.setAttribute(  " id"  ,  " b"  );
b.setAttribute(  " id "  ,  " b "  );
b.setAttribute(
	"id", "b"
);

c.setAttribute(`class`, `a`);
d.setAttribute(`aria-describedby`, `c`);
e.setAttribute(`aria-labelledby`, `d`);
f.setAttribute(`for`, `e`);
g.setAttribute(`form`, `f`);
h.setAttribute(`headers`, `g`);
i.setAttribute(`itemref`, `h`);
j.setAttribute(`list`, `i`);

k.setAttribute(foo, "foo")
l.setAttribute(bar.bar, "bar")
m.setAttribute("class", "")
n.setAttribute("class", foo)
o.setAttribute(foo, baz)

p.setAttribute("id", "a");
p.setAttribute("id", "a");
p.setAttribute("id", "a");
p.setAttribute("id", "a");
p.setAttribute("id", "a");
p.setAttribute("id", "a");

q.setAttribute("style", "");
q.setAttribute("style", "position:absolute; top:0; left:0;");
q.setAttribute("style", "grid-template: \"a a a\"\"b c c\"");
