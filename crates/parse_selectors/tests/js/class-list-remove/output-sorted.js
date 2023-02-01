a.classList.remove("b");
b.classList.remove('a');
c.classList.remove(`e`);
d.classList.remove('c', 'e', 'a');
e.classList.remove("f", 'b', `a`);
f.classList.remove(
	'g',
	'b',
);
g.classList.remove( 'd' , 'c' , 'a' );
h.classList.remove(  'h'  ,  'c'  ,  'd'  );
i.classList.remove( 'i', 'c', 'd' );
j.classList.remove( 'j', 'k', 'l' );

k.classList.remove(foo, 'a');
l.classList.remove('b', foo);
m.classList.remove(foo, bar);
