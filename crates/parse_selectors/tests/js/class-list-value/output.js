document.querySelector('body').classList.value = 'a';
document.querySelector('body').classList.value = 'a b';
document.querySelector('body').classList.value = 'a  c';
document.querySelector('body').classList.value = ' a d ';

document.querySelector('body').classList.value += "b";
document.querySelector('body').classList.value += " b";
document.querySelector('body').classList.value += " b a";

document.querySelector('body').classList.value == 'c';
document.querySelector('body').classList.value == 'c a';

document.querySelector('body').classList.value === 'd';
document.querySelector('body').classList.value === 'd c';

document.querySelector('body').classList.value != 'e';
document.querySelector('body').classList.value != 'e d';

document.querySelector('body').classList.value !== 'f';
document.querySelector('body').classList.value !== 'f e';
