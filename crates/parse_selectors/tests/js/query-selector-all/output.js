a.querySelectorAll("#a");
b.querySelectorAll('#b');
c.querySelectorAll(`#c`);
d.querySelectorAll(".a");
e.querySelectorAll('.b');
f.querySelectorAll(`.c`);
g.querySelectorAll( ".d" );
h.querySelectorAll( '.e' );
i.querySelectorAll( `.f` );
j.querySelectorAll( " .g " );
k.querySelectorAll( ' .h ' );
l.querySelectorAll( ` .i ` );

m.querySelectorAll(".j .a");
n.querySelectorAll('.k .a');
o.querySelectorAll(`.l .a`);
p.querySelectorAll("div.m .a");
q.querySelectorAll('div.n .a');
r.querySelectorAll(`div.o .a`);
s.querySelectorAll("div.p .m");
t.querySelectorAll('div.q .n');
u.querySelectorAll(`div.r .o`);

v.querySelectorAll(".s[href$=.org]");
w.querySelectorAll('.t[href$=.org]');
x.querySelectorAll(`.u[href$=.org]`);
y.querySelectorAll(".v[href$=\".com.au\"]");
z.querySelectorAll('.w[href$=\'.com.au\']');
A.querySelectorAll(`.x[href$=\'.com.au\']`);
B.querySelectorAll(".y[href$='.org' s]");
C.querySelectorAll('.z[href$=".org" s]');
D.querySelectorAll(`.A[href$='.org' s]`);
E.querySelectorAll(".B[aria-labelledby=d]");
F.querySelectorAll('.C[aria-labelledby=d]');
G.querySelectorAll(`.D[aria-labelledby=d]`);
H.querySelectorAll(".E[aria-labelledby=\"d\"]");
I.querySelectorAll('.F[aria-labelledby=\'d\']');
J.querySelectorAll(`.G[aria-labelledby=\'d\']`);
K.querySelectorAll(".H[aria-labelledby='d' s]");
L.querySelectorAll('.I[aria-labelledby="d" s]');
M.querySelectorAll(`.J[aria-labelledby='d' s]`);

N.querySelectorAll(foo);
O.querySelectorAll( bar );
P.querySelectorAll(foo.bar);
Q.querySelectorAll( foo.bar );

R.querySelectorAll("#d");
R.querySelectorAll("#d");
R.querySelectorAll("#d");
R.querySelectorAll("#d");
S.querySelectorAll("#e"); // id="ID-5\"
T.querySelectorAll("#f");
T.querySelectorAll("[id='f']");
