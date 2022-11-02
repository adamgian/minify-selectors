document.querySelector('body').classList.value = 'CLASS-1';
document.querySelector('body').classList.value = 'CLASS-1 CLASS-2';
document.querySelector('body').classList.value = 'CLASS-1  CLASS-3';
document.querySelector('body').classList.value = ' CLASS-1 CLASS-4 ';

document.querySelector('body').classList.value += "CLASS-2";
document.querySelector('body').classList.value += " CLASS-2";
document.querySelector('body').classList.value += " CLASS-2 CLASS-1";

document.querySelector('body').classList.value == 'CLASS-3';
document.querySelector('body').classList.value == 'CLASS-3 CLASS-1';

document.querySelector('body').classList.value === 'CLASS-4';
document.querySelector('body').classList.value === 'CLASS-4 CLASS-3';

document.querySelector('body').classList.value != 'CLASS-5';
document.querySelector('body').classList.value != 'CLASS-5 CLASS-4';

document.querySelector('body').classList.value !== 'CLASS-6';
document.querySelector('body').classList.value !== 'CLASS-6 CLASS-5';
