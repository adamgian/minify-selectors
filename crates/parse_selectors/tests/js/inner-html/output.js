a.innerHTML = '<div class="a"></div>';
b.innerHTML == "<div class=\"b\"></div>";
c.innerHTML === "<div class='c'></div>";
d.innerHTML += "<div class=d></div>";
e.innerHTML -= '<div class="e"></div>';
f.innerHTML = `
	<body class=f>
		<div class="a"></div>
	</body>
`;
