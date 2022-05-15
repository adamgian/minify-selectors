var a = document.querySelector('.a')
var b = document.querySelector(".b")
var c = document.querySelector(`.c`)
var d = document.querySelector('.a');
var e = document.querySelector(".b");
var f = document.querySelector(`.c`);

a = document.querySelector('.d > .a')
b = document.querySelector(".e > .a")
c = document.querySelector(`.f > .a`)
d = document.querySelector('.d > .a');
e = document.querySelector(".e > .a");
f = document.querySelector(`.f > .a`);

a = document.querySelector( ' .d  >  .a ' )
b = document.querySelector( " .e  >  .a " )
c = document.querySelector( ` .f  >  .a ` )
d = document.querySelector( ' .d  >  .a ' );
e = document.querySelector( " .e  >  .a " );
f = document.querySelector( ` .f  >  .a ` );

var foo = ".foo";
var g = document.querySelector(`${foo}.g`);
var h = document.querySelector(`${foo} + .g`);
var i = document.querySelector(`${foo.bar}.h`);
var j = document.querySelector(`${".SELECTOR-9"}.i`);

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
var n = document.querySelector(foo.classList.contains('j') && ".SELECTOR-15");
var o = document.querySelector(!isSomething ?? ".SELECTOR-16");
var p = document.querySelector(doSomething() || ".SELECTOR-17");
