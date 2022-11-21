document.querySelector('body').classList = 'CLASS-1';
document.querySelector('body').classList = 'CLASS-1 CLASS-2';
document.querySelector('body').classList[0] = 'CLASS-1';
document.querySelector('body').classList[0] = "CLASS-2";
document.querySelector('body').classList[99] = 'CLASS-1';
document.querySelector('body').classList[99] = "CLASS-2";

document.querySelector('body').classList += " CLASS-2";
document.querySelector('body').classList += " CLASS-2 CLASS-1";

document.querySelector('body').classList == 'CLASS-3';
document.querySelector('body').classList == 'CLASS-3 CLASS-1';
document.querySelector('body').classList[0] == 'CLASS-3';
document.querySelector('body').classList[99] == 'CLASS-3';
document.querySelector('body').classList[99] == 'CLASS-3 CLASS-1';

document.querySelector('body').classList === 'CLASS-4';
document.querySelector('body').classList === 'CLASS-4 CLASS-1';
document.querySelector('body').classList[0] === 'CLASS-4';
document.querySelector('body').classList[0] === 'CLASS-4 CLASS-2';
document.querySelector('body').classList[99] === 'CLASS-4';

document.querySelector('body').classList != 'CLASS-5';
document.querySelector('body').classList != 'CLASS-5 CLASS-1';
document.querySelector('body').classList[0] != 'CLASS-5';
document.querySelector('body').classList[0] != 'CLASS-5 CLASS-1';
document.querySelector('body').classList[99] != 'CLASS-5';

document.querySelector('body').classList !== 'CLASS-6';
document.querySelector('body').classList !== 'CLASS-6 CLASS-1';
document.querySelector('body').classList[0] !== 'CLASS-6';
document.querySelector('body').classList[99] !== 'CLASS-6';
document.querySelector('body').classList[99] !== 'CLASS-6 CLASS-1';

document.querySelector('body').classList = 'CLA\x53S-1';
document.querySelector('body').classList = 'CLA\u0053S-1';
document.querySelector('body').classList = 'CLA\u{53}S-1';
document.querySelector('body').classList = 'CLA\u{0053}S-1';
document.querySelector('body').classList = 'CLA\u{000053}S-1';

document.querySelector('body').classList = '';
document.querySelector('body').classList = "";
