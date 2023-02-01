document.querySelector('body').classList = 'a';
document.querySelector('body').classList = 'a b';
document.querySelector('body').classList[0] = 'a';
document.querySelector('body').classList[0] = "b";
document.querySelector('body').classList[99] = 'a';
document.querySelector('body').classList[99] = "b";

document.querySelector('body').classList += " b";
document.querySelector('body').classList += " b a";

document.querySelector('body').classList == 'c';
document.querySelector('body').classList == 'c a';
document.querySelector('body').classList[0] == 'c';
document.querySelector('body').classList[99] == 'c';
document.querySelector('body').classList[99] == 'c a';

document.querySelector('body').classList === 'd';
document.querySelector('body').classList === 'd a';
document.querySelector('body').classList[0] === 'd';
document.querySelector('body').classList[0] === 'd b';
document.querySelector('body').classList[99] === 'd';

document.querySelector('body').classList != 'e';
document.querySelector('body').classList != 'e a';
document.querySelector('body').classList[0] != 'e';
document.querySelector('body').classList[0] != 'e a';
document.querySelector('body').classList[99] != 'e';

document.querySelector('body').classList !== 'f';
document.querySelector('body').classList !== 'f a';
document.querySelector('body').classList[0] !== 'f';
document.querySelector('body').classList[99] !== 'f';
document.querySelector('body').classList[99] !== 'f a';

document.querySelector('body').classList = 'a';
document.querySelector('body').classList = 'a';
document.querySelector('body').classList = 'a';
document.querySelector('body').classList = 'a';
document.querySelector('body').classList = 'a';

document.querySelector('body').classList = '';
document.querySelector('body').classList = "";
