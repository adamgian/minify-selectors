var a = document.querySelector('.SELECTOR-1')
var b = document.querySelector(".SELECTOR-2")
var c = document.querySelector(`.SELECTOR-3`)
var d = document.querySelector('.SELECTOR-1');
var e = document.querySelector(".SELECTOR-2");
var f = document.querySelector(`.SELECTOR-3`);

a = document.querySelector('.SELECTOR-4 > .SELECTOR-1')
b = document.querySelector(".SELECTOR-5 > .SELECTOR-1")
c = document.querySelector(`.SELECTOR-6 > .SELECTOR-1`)
d = document.querySelector('.SELECTOR-4 > .SELECTOR-1');
e = document.querySelector(".SELECTOR-5 > .SELECTOR-1");
f = document.querySelector(`.SELECTOR-6 > .SELECTOR-1`);

a = document.querySelector( ' .SELECTOR-4  >  .SELECTOR-1 ' )
b = document.querySelector( " .SELECTOR-5  >  .SELECTOR-1 " )
c = document.querySelector( ` .SELECTOR-6  >  .SELECTOR-1 ` )
d = document.querySelector( ' .SELECTOR-4  >  .SELECTOR-1 ' );
e = document.querySelector( " .SELECTOR-5  >  .SELECTOR-1 " );
f = document.querySelector( ` .SELECTOR-6  >  .SELECTOR-1 ` );

var foo = ".foo";
var g = document.querySelector(`${foo}.SELECTOR-7`);
var h = document.querySelector(`${foo} + .SELECTOR-7`);
var i = document.querySelector(`${foo.bar}.SELECTOR-8`);
var j = document.querySelector(`${".SELECTOR-9"}.SELECTOR-10`);

var k = document
	.querySelector(isSomething ? 'SELECTOR-11' : 'SELECTOR-12');
var l = document.querySelector(
	(isSomethingA > 0 && isSomethingB.includes('.com'))
		? 'SELECTOR-11'
		: 'SELECTOR-12'
);
var m = document.querySelector(
	isSomethingA === 'deleted'
		? getStuff()
		: 'SELECTOR-13'
);
var n = document.querySelector(foo.classList.contains('SELECTOR-14') && ".SELECTOR-15");
var o = document.querySelector(!isSomething ?? ".SELECTOR-16");
var p = document.querySelector(doSomething() || ".SELECTOR-17");

var q = document.querySelector(foo);
var r = document.querySelector( bar );
var s = document.querySelector(foo.bar);
var t = document.querySelector( foo.bar );
