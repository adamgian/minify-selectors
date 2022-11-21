a.insertAdjacentHTML("beforeend", "<div id='ID-1'></div>");
b.insertAdjacentHTML('beforeend', '<div id="ID-2"></div>');
c.insertAdjacentHTML(`beforeend`, `<div id="ID-3"></div>`);
d.insertAdjacentHTML("afterbegin", "<div class='CLASS-1'></div>");
e.insertAdjacentHTML('afterbegin', '<div class="CLASS-2"></div>');
f.insertAdjacentHTML(`afterbegin`, `<div class="CLASS-3"></div>`);
g.insertAdjacentHTML("afterend", "<input type='text' for='ID-4'>");
h.insertAdjacentHTML('afterend', '<input type="text" for="ID-5">');
i.insertAdjacentHTML(`afterend`, `<input type="text" for="ID-6">`);


j.insertAdjacentHTML( 'beforebegin', '<div class="CLASS-4"></div>' );
k.insertAdjacentHTML( 'beforebegin' , '<div class="CLASS-5"></div>' );
l
	.insertAdjacentHTML(
		'beforebegin',
		'<div class="CLASS-6"></div>'
	);

m.insertAdjacentHTML("afterbegin",m.content)
n.insertAdjacentHTML("afterbegin", n.foo)
o.insertAdjacentHTML("afterbegin",'')
p.insertAdjacentHTML("afterbegin","")
q.insertAdjacentHTML("afterbegin",``)
r.insertAdjacentHTML("afterbegin", '')
s.insertAdjacentHTML("afterbegin", "")
t.insertAdjacentHTML("afterbegin", ``)
u.insertAdjacentHTML(foo, bar)

v.insertAdjacentHTML(foo, "<div class=\"CLASS-1\"></div>")
v.insertAdjacentHTML(foo, "<div class='' id='ID-1'></div>")
