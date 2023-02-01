a.innerHTML = '<div id="a" class="b"></div>';
b.innerHTML == "<div id=\"b\" class=\"c\"></div>";
c.innerHTML === "<div class='d'></div>";
d.innerHTML += "<div class=e></div>";
e.innerHTML -= '<div class="a"></div>';
f.innerHTML = `
	<body class=f>
		<div class="b"></div>
	</body>
`;

i.innerHTML = '';
j.innerHTML = "";
k.innerHTML = ``;
l.innerHTML = foo;
m.innerHTML = '<rect class="a" fill="url(#c)"></rect>'
n.innerHTML = '<div class="a" style="mask-image: url(foo.svg#d)"></div>'
