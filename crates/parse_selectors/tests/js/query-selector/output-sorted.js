var a = document.querySelector('.a')
var b = document.querySelector(".e")
var c = document.querySelector(`.f`)
var d = document.querySelector('.a');
var e = document.querySelector(".e");
var f = document.querySelector(`.f`);

a = document.querySelector('.b > #a')
b = document.querySelector(".c > #a")
c = document.querySelector(`.d > #a`)
d = document.querySelector('.b > #a');
e = document.querySelector(".c > #a");
f = document.querySelector(`.d > #a`);

a = document.querySelector( ' .b  >  .a ' )
b = document.querySelector( " .c  >  .a " )
c = document.querySelector( ` .d  >  .a ` )
d = document.querySelector( ' .b  >  .a ' );
e = document.querySelector( " .c  >  .a " );
f = document.querySelector( ` .d  >  .a ` );

var foo = ".foo";
var g = document.querySelector(`${foo}.g`);
var h = document.querySelector(`${foo} + .g`);
var i = document.querySelector(`${foo.bar}.h`);
var j = document.querySelector(`${".CLASS-9"}.i`);

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

document.querySelector('.a');
document.querySelector('.a');
document.querySelector('.a');
document.querySelector('.a');
document.querySelector('.a');
