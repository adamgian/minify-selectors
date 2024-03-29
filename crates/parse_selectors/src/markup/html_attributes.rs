use std::collections::HashMap;

use once_cell::sync::OnceCell;




// HTML attributes which its values will contains parsable values
pub static WHITELIST: OnceCell<HashMap<String, String>> = OnceCell::new();

pub fn init(custom_attributes: &Vec<(String, String)>) {
	#[rustfmt::skip]
	WHITELIST.get_or_init(|| {
		let standard_attributes: [(String, String); 114] = [
			// Class
			(String::from("class"), String::from("class")),
			// ID
			(String::from("id"), String::from("id")),
			(String::from("aria-controls"), String::from("id")),
			(String::from("aria-describedby"), String::from("id")),
			(String::from("aria-labelledby"), String::from("id")),
			(String::from("for"), String::from("id")),
			(String::from("form"), String::from("id")),
			(String::from("headers"), String::from("id")),
			(String::from("itemref"), String::from("id")),
			(String::from("list"), String::from("id")),
			// Anchor
			(String::from("href"), String::from("anchor")),
			(String::from("xlink:href"), String::from("anchor")),
			// Style
			(String::from("fill"), String::from("style")),
			(String::from("style"), String::from("style")),
			// Script
			(String::from("onabort"), String::from("script")),
			(String::from("onactivate"), String::from("script")),
			(String::from("onafterprint"), String::from("script")),
			(String::from("onauxclick"), String::from("script")),
			(String::from("onbeforeinput"), String::from("script")),
			(String::from("onbeforematch"), String::from("script")),
			(String::from("onbeforeprint"), String::from("script")),
			(String::from("onbeforeunload"), String::from("script")),
			(String::from("onbegin"), String::from("script")),
			(String::from("onblur"), String::from("script")),
			(String::from("oncancel"), String::from("script")),
			(String::from("oncanplay"), String::from("script")),
			(String::from("oncanplaythrough"), String::from("script")),
			(String::from("onchange"), String::from("script")),
			(String::from("onclick"), String::from("script")),
			(String::from("onclose"), String::from("script")),
			(String::from("oncontextextlost"), String::from("script")),
			(String::from("oncontextextmenu"), String::from("script")),
			(String::from("oncontextextrestored"), String::from("script")),
			(String::from("oncopy"), String::from("script")),
			(String::from("oncuechange"), String::from("script")),
			(String::from("oncut"), String::from("script")),
			(String::from("ondblclick"), String::from("script")),
			(String::from("ondrag"), String::from("script")),
			(String::from("ondragend"), String::from("script")),
			(String::from("ondragenter"), String::from("script")),
			(String::from("ondragexit"), String::from("script")),
			(String::from("ondragleave"), String::from("script")),
			(String::from("ondragover"), String::from("script")),
			(String::from("ondragstart"), String::from("script")),
			(String::from("ondrop"), String::from("script")),
			(String::from("ondurationchange"), String::from("script")),
			(String::from("onemptied"), String::from("script")),
			(String::from("onend"), String::from("script")),
			(String::from("onended"), String::from("script")),
			(String::from("onerror"), String::from("script")),
			(String::from("onfocus"), String::from("script")),
			(String::from("onfocusin"), String::from("script")),
			(String::from("onfocusout"), String::from("script")),
			(String::from("onformdata"), String::from("script")),
			(String::from("onhashchange"), String::from("script")),
			(String::from("oninput"), String::from("script")),
			(String::from("oninvalid"), String::from("script")),
			(String::from("onkeydown"), String::from("script")),
			(String::from("onkeypress"), String::from("script")),
			(String::from("onkeyup"), String::from("script")),
			(String::from("onlanguagechange"), String::from("script")),
			(String::from("onload"), String::from("script")),
			(String::from("onloadeddata"), String::from("script")),
			(String::from("onloadedmetadata"), String::from("script")),
			(String::from("onloadstart"), String::from("script")),
			(String::from("onmessage"), String::from("script")),
			(String::from("onmessageerror"), String::from("script")),
			(String::from("onmousedown"), String::from("script")),
			(String::from("onmouseenter"), String::from("script")),
			(String::from("onmouseleave"), String::from("script")),
			(String::from("onmousemove"), String::from("script")),
			(String::from("onmouseout"), String::from("script")),
			(String::from("onmouseover"), String::from("script")),
			(String::from("onmouseup"), String::from("script")),
			(String::from("onmousewheel"), String::from("script")),
			(String::from("onoffline"), String::from("script")),
			(String::from("ononline"), String::from("script")),
			(String::from("onpagehide"), String::from("script")),
			(String::from("onpageshow"), String::from("script")),
			(String::from("onpaste"), String::from("script")),
			(String::from("onpause"), String::from("script")),
			(String::from("onplay"), String::from("script")),
			(String::from("onplaying"), String::from("script")),
			(String::from("onpopstate"), String::from("script")),
			(String::from("onprogress"), String::from("script")),
			(String::from("onratechange"), String::from("script")),
			(String::from("onrejectionhandled"), String::from("script")),
			(String::from("onrepeat"), String::from("script")),
			(String::from("onresize"), String::from("script")),
			(String::from("onresize"), String::from("script")),
			(String::from("onscroll"), String::from("script")),
			(String::from("onscrollend"), String::from("script")),
			(String::from("onsecuritypolicyviolation"), String::from("script")),
			(String::from("onsearch"), String::from("script")),
			(String::from("onseeked"), String::from("script")),
			(String::from("onseeking"), String::from("script")),
			(String::from("onselect"), String::from("script")),
			(String::from("onslotchange"), String::from("script")),
			(String::from("onshow"), String::from("script")),
			(String::from("onstalled"), String::from("script")),
			(String::from("onstorage"), String::from("script")),
			(String::from("onsubmit"), String::from("script")),
			(String::from("onsuspend"), String::from("script")),
			(String::from("ontimeupdate"), String::from("script")),
			(String::from("ontoggle"), String::from("script")),
			(String::from("onunhandledrejection"), String::from("script")),
			(String::from("onunload"), String::from("script")),
			(String::from("onvolumechange"), String::from("script")),
			(String::from("onwaiting"), String::from("script")),
			(String::from("onwebkitanimationend"), String::from("script")),
			(String::from("onwebkitanimationiteration"), String::from("script")),
			(String::from("onwebkitanimationstart"), String::from("script")),
			(String::from("onwebkitanimationend"), String::from("script")),
			(String::from("onwheel"), String::from("script")),
		];

		let mut attributes = HashMap::from(standard_attributes);

		for (attribute, kind) in custom_attributes {
			attributes.insert(attribute.to_string(), kind.to_string());
		}

		attributes
	});
}
