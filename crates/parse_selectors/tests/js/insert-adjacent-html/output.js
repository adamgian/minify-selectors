a.insertAdjacentHTML("beforeend", "<div id='a'></div>");
b.insertAdjacentHTML('beforeend', '<div id="b"></div>');
c.insertAdjacentHTML(`beforeend`, `<div id="c"></div>`);
d.insertAdjacentHTML("afterbegin", "<div class='d'></div>");
e.insertAdjacentHTML('afterbegin', '<div class="e"></div>');
f.insertAdjacentHTML(`afterbegin`, `<div class="f"></div>`);
g.insertAdjacentHTML("afterend", "<input type='text' for='g'>");
h.insertAdjacentHTML('afterend', '<input type="text" for="h">');
i.insertAdjacentHTML(`afterend`, `<input type="text" for="i">`);


j.insertAdjacentHTML( 'beforebegin', '<div class="j"></div>' );
k.insertAdjacentHTML( 'beforebegin' , '<div class="k"></div>' );
l
	.insertAdjacentHTML(
		'beforebegin',
		'<div class="k"></div>'
	);

m.insertAdjacentHTML("afterbegin",m.content)
