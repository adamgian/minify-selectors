a.closest("div");
a.closest('div');
a.closest(`div`);
a.closest(".SELECTOR-1");
b.closest('.SELECTOR-2');
c.closest(`.SELECTOR-3`);
d.closest(".SELECTOR-4 .SELECTOR-1");
e.closest('.SELECTOR-5 .SELECTOR-2');
f.closest(`.SELECTOR-6 .SELECTOR-3`);
g.closest("#SELECTOR-7");
h.closest(".SELECTOR-8 + .SELECTOR-1");
i.closest(".SELECTOR-9 ~ .SELECTOR-1");
j.closest(".SELECTOR-10 > .SELECTOR-1");

k.closest(
	"  .SELECTOR-11 > .SELECTOR-1 + .SELECTOR-1  "
);
l
	.closest(  "  div .SELECTOR-12  >  .SELECTOR-1  "  );

m.closest(foo);
n.closest(foo.bar);
o.closest( foo );
