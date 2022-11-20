a.classList.replace('CLASS-1', 'CLASS-2');
b.classList.replace("CLASS-2", "CLASS-2");
c.classList.replace(`CLASS-3`, `CLASS-4`);
d
	.classList
	.replace(
		'CLASS-5',
		'CLASS-6',
	);

e.classList.replace(foo, 'CLASS-2');
f.classList.replace('CLASS-1', foo);
g.classList.replace(foo, bar);

a.classList.replace('CLAS\x53-1', 'CLASS-2');
a.classList.replace('CLAS\x53-1', 'CLA\u0053S-2');
a.classList.replace('CLAS\u{53}-1', 'CLA\u{053}S-2');
a.classList.replace('CLAS\u{0053}-1', 'CLA\u{000053}S-2');
