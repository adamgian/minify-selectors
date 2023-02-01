a.className = "a";
a.className="a";
b.className = 'b';
b.className= 'b';
c.className = `c`;
c.className =`c`;
d.className = "l a";
e.className = 'm b';
f.className = `n c`;

g.className == "e";
h.className == 'f';
i.className == `g`;
j.className == "o e a";
k.className == 'p g';
l.className == `q f`;
m.className === "r";
n.className === 's';
o.className === `t`;
p.className === "d d";
q.className === 'h d h';
r.className === `u a`;

s.className != "v";
t.className != 'w';
u.className != `x`;
v.className !== "y";
w.className !== 'z';
x.className !== `A`;

y.className += "i";
y.className += " i";
z.className += 'j';
z.className += ' j';
A.className += `k`;
A.className += ` k`;

B.className = foo;
C.className = foo.bar;
D.className = '';
E.className = "";
F.className = ``;
