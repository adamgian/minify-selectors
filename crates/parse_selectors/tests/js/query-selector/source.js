var a = document.querySelector('.CLASS-1')
var b = document.querySelector(".CLASS-2")
var c = document.querySelector(`.CLASS-3`)
var d = document.querySelector('.CLASS-1');
var e = document.querySelector(".CLASS-2");
var f = document.querySelector(`.CLASS-3`);

a = document.querySelector('.CLASS-4 > #CLASS-1')
b = document.querySelector(".CLASS-5 > #CLASS-1")
c = document.querySelector(`.CLASS-6 > #CLASS-1`)
d = document.querySelector('.CLASS-4 > #CLASS-1');
e = document.querySelector(".CLASS-5 > #CLASS-1");
f = document.querySelector(`.CLASS-6 > #CLASS-1`);

a = document.querySelector( ' .CLASS-4  >  .CLASS-1 ' )
b = document.querySelector( " .CLASS-5  >  .CLASS-1 " )
c = document.querySelector( ` .CLASS-6  >  .CLASS-1 ` )
d = document.querySelector( ' .CLASS-4  >  .CLASS-1 ' );
e = document.querySelector( " .CLASS-5  >  .CLASS-1 " );
f = document.querySelector( ` .CLASS-6  >  .CLASS-1 ` );

var foo = ".foo";
var g = document.querySelector(`${foo}.CLASS-7`);
var h = document.querySelector(`${foo} + .CLASS-7`);
var i = document.querySelector(`${foo.bar}.CLASS-8`);
var j = document.querySelector(`${".CLASS-9"}.CLASS-10`);

var k = document
	.querySelector(isSomething ? '.CLASS-11' : '.CLASS-12');
var l = document.querySelector(
	(isSomethingA > 0 && isSomethingB.includes('.com'))
		? '.CLASS-11'
		: '.CLASS-12'
);
var m = document.querySelector(
	isSomethingA === 'deleted'
		? getStuff()
		: '.CLASS-13'
);
var n = document.querySelector(foo.classList.contains('CLASS-14') && ".CLASS-15");
var o = document.querySelector(!isSomething ?? ".CLASS-16");
var p = document.querySelector(doSomething() || ".CLASS-17");

var q = document.querySelector(foo);
var r = document.querySelector( bar );
var s = document.querySelector(foo.bar);
var t = document.querySelector( foo.bar );

document.querySelector('.CLA\x53S-1');
document.querySelector('.CLA\u0053S-1');
document.querySelector('.CLA\u{53}S-1');
document.querySelector('.CLA\u{0053}S-1');
document.querySelector('.CLA\u{000053}S-1');
