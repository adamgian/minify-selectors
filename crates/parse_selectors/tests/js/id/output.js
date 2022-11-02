if (foo.id == 'b') {
	foo.id = 'b'
} else if (bar.id === 'c') {
	foo.id == 'c'
}

document.querySelector("#a").id = "d";
document.querySelector('#a').id = 'd';
document.querySelector(`#a`).id = `d`;

document.querySelector("#a").id == "d";
document.querySelector('#a').id == 'd';
document.querySelector(`#a`).id == `d`;

document.querySelector("#a").id === "d";
document.querySelector('#a').id === 'd';
document.querySelector(`#a`).id === `d`;

document.querySelector("#a").id != "d";
document.querySelector('#a').id != 'd';
document.querySelector(`#a`).id != `d`;

document.querySelector("#a").id !== "d";
document.querySelector('#a').id !== 'd';
document.querySelector(`#a`).id !== `d`;
