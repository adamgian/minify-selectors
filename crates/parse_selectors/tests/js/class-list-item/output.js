document.querySelector('body').classList.item(1) = 'a';
document.querySelector('body').classList.item(1) = "b";
document.querySelector('body').classList.item(foo.index) = "b";
document.querySelector('body').classList.item(foo) = 'a';
document.querySelector('body').classList.item(foo) = "b";
document.querySelector('body').classList.item(foo.index) = 'a';
document.querySelector('body').classList.item(foo()) = 'a';
document.querySelector('body').classList.item(foo()) = "b";
document.querySelector('body').classList.item(foo.bar()) = 'a';
document.querySelector('body').classList.item(foo.bar()) = "b";
document.querySelector('body').classList.item(foo.bar(baz)) = 'a';
document.querySelector('body').classList.item(foo.bar(baz)) = "b";
document.querySelector('body').classList.item(foo.bar({baz: true})) = 'a';
document.querySelector('body').classList.item(foo.bar({baz: true})) = "b";

document.querySelector('body').classList.item(1) == 'c';
document.querySelector('body').classList.item(foo) == 'c';
document.querySelector('body').classList.item(foo.index) == 'c';
document.querySelector('body').classList.item(foo()) == 'c';
document.querySelector('body').classList.item(foo.bar()) == 'c';
document.querySelector('body').classList.item(foo.bar(baz)) == 'c';
document.querySelector('body').classList.item(foo.bar({baz: true})) == 'c';

document.querySelector('body').classList.item(1) === 'd';
document.querySelector('body').classList.item(foo) === 'd';
document.querySelector('body').classList.item(foo.index) === 'd';
document.querySelector('body').classList.item(foo()) === 'd';
document.querySelector('body').classList.item(foo.bar()) === 'd';
document.querySelector('body').classList.item(foo.bar(baz)) === 'd';
document.querySelector('body').classList.item(foo.bar({baz: true})) === 'd';

document.querySelector('body').classList.item(1) != 'e';
document.querySelector('body').classList.item(foo) != 'e';
document.querySelector('body').classList.item(foo.index) != 'e';
document.querySelector('body').classList.item(foo()) != 'e';
document.querySelector('body').classList.item(foo.bar()) != 'e';
document.querySelector('body').classList.item(foo.bar(baz)) != 'e';
document.querySelector('body').classList.item(foo.bar({baz: true})) != 'e';

document.querySelector('body').classList.item(1) !== 'f';
document.querySelector('body').classList.item(foo) !== 'f';
document.querySelector('body').classList.item(foo.index) !== 'f';
document.querySelector('body').classList.item(foo()) !== 'f';
document.querySelector('body').classList.item(foo.bar()) !== 'f';
document.querySelector('body').classList.item(foo.bar(baz)) !== 'f';
document.querySelector('body').classList.item(foo.bar({baz: true})) !== 'f';
