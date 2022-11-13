a.closest("div");
a.closest('div');
a.closest(`div`);
a.closest(".a");
b.closest('.b');
c.closest(`.c`);
d.closest(".d .a");
e.closest('.e .b');
f.closest(`.f .c`);
g.closest("#a");
h.closest(".g + .a");
i.closest(".h ~ .a");
j.closest(".i > .a");

k.closest(
	"  .j > .a + .a  "
);
l
	.closest(  "  div .k  >  .a  "  );

m.closest(foo);
n.closest(foo.bar);
o.closest( foo );

a.closest(".a");
a.closest(".a");
