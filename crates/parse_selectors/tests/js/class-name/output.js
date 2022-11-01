a.className = "a";
a.className="a";
b.className = 'b';
b.className= 'b';
c.className = `c`;
c.className =`c`;
d.className = "d a";
e.className = 'e b';
f.className = `f c`;

g.className == "g";
h.className == 'h';
i.className == `i`;
j.className == "j g a";
k.className == 'k i';
l.className == `l h`;
m.className === "m";
n.className === 'n';
o.className === `o`;
p.className === "p p";
q.className === 'q p q';
r.className === `r a`;

s.className != "s";
t.className != 't';
u.className != `u`;
v.className !== "v";
w.className !== 'w';
x.className !== `x`;

y.className += "y";
y.className += " y";
z.className += 'z';
z.className += ' z';
A.className += `A`;
A.className += ` A`;

B.className = foo;
C.className = foo.bar;
D.className = '';
E.className = "";
F.className = ``;
