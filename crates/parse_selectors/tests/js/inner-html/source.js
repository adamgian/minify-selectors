a.innerHTML = '<div id="ID-1" class="CLASS-1"></div>';
b.innerHTML == "<div id=\"ID-2\" class=\"CLASS-2\"></div>";
c.innerHTML === "<div class='CLASS-3'></div>";
d.innerHTML += "<div class=CLASS-4></div>";
e.innerHTML -= '<div class="CLASS-5"></div>';
f.innerHTML = `
	<body class=CLASS-6>
		<div class="CLASS-1"></div>
	</body>
`;

i.innerHTML = '';
j.innerHTML = "";
k.innerHTML = ``;
l.innerHTML = foo;
m.innerHTML = '<rect class="CLASS-5" fill="url(#ID-3)"></rect>'
n.innerHTML = '<div class="CLASS-5" style="mask-image: url(foo.svg#ID-4)"></div>'
