a.innerHTML = '<div id="SELECTOR-1" class="SELECTOR-1"></div>';
b.innerHTML == "<div id=\"SELECTOR-2\" class=\"SELECTOR-2\"></div>";
c.innerHTML === "<div class='SELECTOR-3'></div>";
d.innerHTML += "<div class=SELECTOR-4></div>";
e.innerHTML -= '<div class="SELECTOR-5"></div>';
f.innerHTML = `
	<body class=SELECTOR-6>
		<div class="SELECTOR-1"></div>
	</body>
`;

i.innerHTML = '';
j.innerHTML = "";
k.innerHTML = ``;
l.innerHTML = foo;
