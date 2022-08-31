a.classList.toggle('a');
b.classList.toggle("b");
c.classList.toggle(`c`);
d.classList.toggle("d", isSomething);
e.classList.toggle("e", isSomething());
f.classList.toggle("f", somethingA > 0);
g.classList.toggle("g", somethingA >= somethingB);
h.classList.toggle("h", somethingA < somethingB);
i.classList.toggle("i", somethingA === "foo");

j.classList.toggle(foo);
