if (foo.id == 'ID-2') {
	foo.id = 'ID-2'
} else if (bar.id === 'ID-3') {
	foo.id == 'ID-3'
}

document.querySelector("#ID-1").id = "ID-4";
document.querySelector('#ID-1').id = 'ID-4';
document.querySelector(`#ID-1`).id = `ID-4`;

document.querySelector("#ID-1").id == "ID-4";
document.querySelector('#ID-1').id == 'ID-4';
document.querySelector(`#ID-1`).id == `ID-4`;

document.querySelector("#ID-1").id === "ID-4";
document.querySelector('#ID-1').id === 'ID-4';
document.querySelector(`#ID-1`).id === `ID-4`;

document.querySelector("#ID-1").id != "ID-4";
document.querySelector('#ID-1').id != 'ID-4';
document.querySelector(`#ID-1`).id != `ID-4`;

document.querySelector("#ID-1").id !== "ID-4";
document.querySelector('#ID-1').id !== 'ID-4';
document.querySelector(`#ID-1`).id !== `ID-4`;
