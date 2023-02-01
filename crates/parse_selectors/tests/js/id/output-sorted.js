if (foo.id == 'c') {
	foo.id = 'c'
} else if (bar.id === 'd') {
	foo.id == 'd'
}

document.querySelector("#a").id = "b";
document.querySelector('#a').id = 'b';
document.querySelector(`#a`).id = `b`;

document.querySelector("#a").id == "b";
document.querySelector('#a').id == 'b';
document.querySelector(`#a`).id == `b`;

document.querySelector("#a").id === "b";
document.querySelector('#a').id === 'b';
document.querySelector(`#a`).id === `b`;

document.querySelector("#a").id != "b";
document.querySelector('#a').id != 'b';
document.querySelector(`#a`).id != `b`;

document.querySelector("#a").id !== "b";
document.querySelector('#a').id !== 'b';
document.querySelector(`#a`).id !== `b`;
