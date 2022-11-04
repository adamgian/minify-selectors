a.innerHTML = '<div id="a" class="a"></div>';
b.innerHTML == "<div id=\"b\" class=\"b\"></div>";
c.innerHTML === "<div class='c'></div>";
d.innerHTML += "<div class=d></div>";
e.innerHTML -= '<div class="e"></div>';
f.innerHTML = `
	<body class=f>
		<div class="a"></div>
	</body>
`;

i.innerHTML = '';
j.innerHTML = "";
k.innerHTML = ``;
l.innerHTML = foo;
m.innerHTML = '<rect class="e" fill="url(#c)"></rect>'
n.innerHTML = '<div class="e" style="mask-image: url(foo.svg#d)"></div>'
