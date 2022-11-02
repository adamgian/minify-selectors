document.querySelector('body').classList.item(1) = 'CLASS-1';
document.querySelector('body').classList.item(1) = "CLASS-2";
document.querySelector('body').classList.item(foo.index) = "CLASS-2";
document.querySelector('body').classList.item(foo) = 'CLASS-1';
document.querySelector('body').classList.item(foo) = "CLASS-2";
document.querySelector('body').classList.item(foo.index) = 'CLASS-1';
document.querySelector('body').classList.item(foo()) = 'CLASS-1';
document.querySelector('body').classList.item(foo()) = "CLASS-2";
document.querySelector('body').classList.item(foo.bar()) = 'CLASS-1';
document.querySelector('body').classList.item(foo.bar()) = "CLASS-2";
document.querySelector('body').classList.item(foo.bar(baz)) = 'CLASS-1';
document.querySelector('body').classList.item(foo.bar(baz)) = "CLASS-2";
document.querySelector('body').classList.item(foo.bar({baz: true})) = 'CLASS-1';
document.querySelector('body').classList.item(foo.bar({baz: true})) = "CLASS-2";

document.querySelector('body').classList.item(1) == 'CLASS-3';
document.querySelector('body').classList.item(foo) == 'CLASS-3';
document.querySelector('body').classList.item(foo.index) == 'CLASS-3';
document.querySelector('body').classList.item(foo()) == 'CLASS-3';
document.querySelector('body').classList.item(foo.bar()) == 'CLASS-3';
document.querySelector('body').classList.item(foo.bar(baz)) == 'CLASS-3';
document.querySelector('body').classList.item(foo.bar({baz: true})) == 'CLASS-3';

document.querySelector('body').classList.item(1) === 'CLASS-4';
document.querySelector('body').classList.item(foo) === 'CLASS-4';
document.querySelector('body').classList.item(foo.index) === 'CLASS-4';
document.querySelector('body').classList.item(foo()) === 'CLASS-4';
document.querySelector('body').classList.item(foo.bar()) === 'CLASS-4';
document.querySelector('body').classList.item(foo.bar(baz)) === 'CLASS-4';
document.querySelector('body').classList.item(foo.bar({baz: true})) === 'CLASS-4';

document.querySelector('body').classList.item(1) != 'CLASS-5';
document.querySelector('body').classList.item(foo) != 'CLASS-5';
document.querySelector('body').classList.item(foo.index) != 'CLASS-5';
document.querySelector('body').classList.item(foo()) != 'CLASS-5';
document.querySelector('body').classList.item(foo.bar()) != 'CLASS-5';
document.querySelector('body').classList.item(foo.bar(baz)) != 'CLASS-5';
document.querySelector('body').classList.item(foo.bar({baz: true})) != 'CLASS-5';

document.querySelector('body').classList.item(1) !== 'CLASS-6';
document.querySelector('body').classList.item(foo) !== 'CLASS-6';
document.querySelector('body').classList.item(foo.index) !== 'CLASS-6';
document.querySelector('body').classList.item(foo()) !== 'CLASS-6';
document.querySelector('body').classList.item(foo.bar()) !== 'CLASS-6';
document.querySelector('body').classList.item(foo.bar(baz)) !== 'CLASS-6';
document.querySelector('body').classList.item(foo.bar({baz: true})) !== 'CLASS-6';
