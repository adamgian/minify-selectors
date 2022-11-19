document.getElementsByClassName("SELECTOR-1");
document.getElementsByClassName('SELECTOR-2');
document.getElementsByClassName(`SELECTOR-3`);
document.getElementsByClassName( "SELECTOR-4" );
document.getElementsByClassName( 'SELECTOR-5' );
document.getElementsByClassName( `SELECTOR-6` );
document.getElementsByClassName("SELECTOR-7 SELECTOR-1");
document.getElementsByClassName('SELECTOR-8 SELECTOR-1');
document.getElementsByClassName(`SELECTOR-9 SELECTOR-1`);
document.getElementsByClassName(
	"SELECTOR-10"
);
document
	.getElementsByClassName(
		"SELECTOR-11"
	);

document.getElementsByClassName(foo);
document.getElementsByClassName( bar );
document.getElementsByClassName( foo.bar );

document.getElementsByClassName('\\\\');
document.getElementsByClassName('\#');
document.getElementsByClassName('SELECTOR-\31');
document.getElementsByClassName('SELECTOR-\31 ');
document.getElementsByClassName('SELECTOR-\031');
document.getElementsByClassName('SELECTOR-\031 ');
document.getElementsByClassName('SELECTOR-\0031');
document.getElementsByClassName('SELECTOR-\0031 ');
document.getElementsByClassName('SELECTOR-\000031');
document.getElementsByClassName('SELECTOR-\000031 ');

document.getElementsByClassName('\xA9');
document.getElementsByClassName('\u00A9');
document.getElementsByClassName('\u00a9');
document.getElementsByClassName('\u2665');
document.getElementsByClassName('\u{1D306}');
document.getElementsByClassName('\u{1d306}');
document.getElementsByClassName('\u{001d306}');
