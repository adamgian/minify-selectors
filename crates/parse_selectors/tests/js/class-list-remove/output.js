a.classList.remove("a");
b.classList.remove('b');
c.classList.remove(`c`);
d.classList.remove('d', 'c', 'b');
e.classList.remove("e", 'a', `b`);
f.classList.remove(
	'f',
	'a',
);
g.classList.remove( 'g' , 'd' , 'b' );
h.classList.remove(  'h'  ,  'd'  ,  'g'  );
i.classList.remove( 'i', 'd', 'g' );
j.classList.remove( 'j', 'k', 'l' );

k.classList.remove(foo, 'b');
l.classList.remove('a', foo);
m.classList.remove(foo, bar);
