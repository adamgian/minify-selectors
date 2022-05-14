var a = document.querySelector('.SELECTOR-1')
var b = document.querySelector(".SELECTOR-2")
var c = document.querySelector(`.SELECTOR-3`)
var d = document.querySelector('.SELECTOR-1');
var e = document.querySelector(".SELECTOR-2");
var f = document.querySelector(`.SELECTOR-3`);

var a = document.querySelector('.SELECTOR-4 > .SELECTOR-1')
var b = document.querySelector(".SELECTOR-5 > .SELECTOR-1")
var c = document.querySelector(`.SELECTOR-6 > .SELECTOR-1`)
var d = document.querySelector('.SELECTOR-4 > .SELECTOR-1');
var e = document.querySelector(".SELECTOR-5 > .SELECTOR-1");
var f = document.querySelector(`.SELECTOR-6 > .SELECTOR-1`);

var element = "input";
var g = document.querySelector(`${element}.SELECTOR-7`);
var h = document.querySelector(`${element} + .SELECTOR-7`);
var i = document.querySelector(`${elements.foo}.SELECTOR-8`);
var j = document.querySelector(`${".SELECTOR-9"}.SELECTOR-10`);
