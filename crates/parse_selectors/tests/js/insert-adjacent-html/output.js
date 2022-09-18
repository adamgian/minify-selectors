a.insertAdjacentHTML("beforeend", "<div id='a'></div>");
b.insertAdjacentHTML('beforeend', '<div id="b"></div>');
c.insertAdjacentHTML(`beforeend`, `<div id="c"></div>`);
d.insertAdjacentHTML("afterbegin", "<div class='a'></div>");
e.insertAdjacentHTML('afterbegin', '<div class="b"></div>');
f.insertAdjacentHTML(`afterbegin`, `<div class="c"></div>`);
g.insertAdjacentHTML("afterend", "<input type='text' for='d'>");
h.insertAdjacentHTML('afterend', '<input type="text" for="e">');
i.insertAdjacentHTML(`afterend`, `<input type="text" for="f">`);


j.insertAdjacentHTML( 'beforebegin', '<div class="d"></div>' );
k.insertAdjacentHTML( 'beforebegin' , '<div class="e"></div>' );
l
	.insertAdjacentHTML(
		'beforebegin',
		'<div class="e"></div>'
	);

m.insertAdjacentHTML("afterbegin",m.content)
n.insertAdjacentHTML("afterbegin", n.foo)
o.insertAdjacentHTML("afterbegin",'')
p.insertAdjacentHTML("afterbegin","")
q.insertAdjacentHTML("afterbegin",``)
r.insertAdjacentHTML("afterbegin", '')
s.insertAdjacentHTML("afterbegin", "")
t.insertAdjacentHTML("afterbegin", ``)
