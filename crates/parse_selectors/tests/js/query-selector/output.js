var a = document.querySelector('.a')
var b = document.querySelector(".b")
var c = document.querySelector(`.c`)
var d = document.querySelector('.a');
var e = document.querySelector(".b");
var f = document.querySelector(`.c`);

var a = document.querySelector('.d > .a')
var b = document.querySelector(".e > .a")
var c = document.querySelector(`.f > .a`)
var d = document.querySelector('.d > .a');
var e = document.querySelector(".e > .a");
var f = document.querySelector(`.f > .a`);

var element = "input";
var g = document.querySelector(`${element}.g`);
var h = document.querySelector(`${element} + .g`);
var i = document.querySelector(`${elements.foo}.h`);
var j = document.querySelector(`${".SELECTOR-9"}.i`);
