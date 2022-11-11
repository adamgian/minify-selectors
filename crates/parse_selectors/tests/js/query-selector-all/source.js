a.querySelectorAll("#ID-1");
b.querySelectorAll('#ID-2');
c.querySelectorAll(`#ID-3`);
d.querySelectorAll(".CLASS-4");
e.querySelectorAll('.CLASS-5');
f.querySelectorAll(`.CLASS-6`);
g.querySelectorAll( ".CLASS-7" );
h.querySelectorAll( '.CLASS-8' );
i.querySelectorAll( `.CLASS-9` );
j.querySelectorAll( " .CLASS-10 " );
k.querySelectorAll( ' .CLASS-11 ' );
l.querySelectorAll( ` .CLASS-12 ` );

m.querySelectorAll(".CLASS-13 .CLASS-4");
n.querySelectorAll('.CLASS-14 .CLASS-4');
o.querySelectorAll(`.CLASS-15 .CLASS-4`);
p.querySelectorAll("div.CLASS-16 .CLASS-4");
q.querySelectorAll('div.CLASS-17 .CLASS-4');
r.querySelectorAll(`div.CLASS-18 .CLASS-4`);
s.querySelectorAll("div.CLASS-19 .CLASS-16");
t.querySelectorAll('div.CLASS-20 .CLASS-17');
u.querySelectorAll(`div.CLASS-21 .CLASS-18`);

v.querySelectorAll(".CLASS-22[href$=.org]");
w.querySelectorAll('.CLASS-23[href$=.org]');
x.querySelectorAll(`.CLASS-24[href$=.org]`);
y.querySelectorAll(".CLASS-25[href$=\".com.au\"]");
z.querySelectorAll('.CLASS-26[href$=\'.com.au\']');
A.querySelectorAll(`.CLASS-27[href$=\'.com.au\']`);
B.querySelectorAll(".CLASS-28[href$='.org' s]");
C.querySelectorAll('.CLASS-29[href$=".org" s]');
D.querySelectorAll(`.CLASS-30[href$='.org' s]`);
E.querySelectorAll(".CLASS-31[aria-labelledby=ID-4]");
F.querySelectorAll('.CLASS-32[aria-labelledby=ID-4]');
G.querySelectorAll(`.CLASS-33[aria-labelledby=ID-4]`);
H.querySelectorAll(".CLASS-34[aria-labelledby=\"ID-4\"]");
I.querySelectorAll('.CLASS-35[aria-labelledby=\'ID-4\']');
J.querySelectorAll(`.CLASS-36[aria-labelledby=\'ID-4\']`);
K.querySelectorAll(".CLASS-37[aria-labelledby='ID-4' s]");
L.querySelectorAll('.CLASS-38[aria-labelledby="ID-4" s]');
M.querySelectorAll(`.CLASS-39[aria-labelledby='ID-4' s]`);

N.querySelectorAll(foo);
O.querySelectorAll( bar );
P.querySelectorAll(foo.bar);
Q.querySelectorAll( foo.bar );

R.querySelectorAll("#ID-\\34");
R.querySelectorAll("#ID-\\34 ");
R.querySelectorAll("#ID-\\000034");
R.querySelectorAll("#ID-\\000034 ");
S.querySelectorAll("#ID-5\\\\"); // id="ID-5\"
T.querySelectorAll("#ID\\:6");
T.querySelectorAll("[id='ID\\:6']");
