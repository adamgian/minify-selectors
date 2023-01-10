"use strict";
document.querySelectorAll('[data-bs-toggle="tooltip"]').forEach(e => {
	new bootstrap.Tooltip(e)
}), document.querySelectorAll('[data-bs-toggle="popover"]').forEach(e => {
	new bootstrap.Popover(e)
});
const e = document.getElementById("toastPlacement");
e && document.getElementById("selectToastPlacement").addEventListener("change", function() {
	e.dataset.originalClass || (e.dataset.originalClass = e.className), e.className = `${e.dataset.originalClass} ${this.value}`
}), document.querySelectorAll(".bd-example .toast").forEach(e => {
	const t = new bootstrap.Toast(e, {
		autohide: !1
	});
	t.show()
});
const n = document.getElementById("liveToastBtn"),
	i = document.getElementById("liveToast");
n && n.addEventListener("click", () => {
	const e = new bootstrap.Toast(i);
	e.show()
});
const a = document.getElementById("liveAlertPlaceholder"),
	s = document.getElementById("liveAlertBtn"),
	r = (e, t) => {
		const n = document.createElement("div");
		n.innerHTML = [`<div class="alert alert-${t} alert-dismissible" role="alert">`, `   <div>${e}</div>`, '   <button type="button" class="btn-close" data-bs-dismiss="alert" aria-label="Close"></button>', "</div>"].join(""), a.append(n)
	};
s && s.addEventListener("click", () => {
	r("Nice, you triggered this alert message!", "success")
}), document.querySelectorAll('.bd-example-indeterminate [type="checkbox"]').forEach(e => {
	e.id.includes("Indeterminate") && (e.indeterminate = !0)
}), document.querySelectorAll('.bd-content [href="#"]').forEach(e => {
	e.addEventListener("click", e => {
		e.preventDefault()
	})
});
const t = document.getElementById("exampleModal");
t && t.addEventListener("show.bs.modal", e => {
	const s = e.relatedTarget,
		n = s.getAttribute("data-bs-whatever"),
		o = t.querySelector(".modal-title"),
		i = t.querySelector(".modal-body input");
	o.textContent = `New message to ${n}`, i.value = n
});
const o = document.querySelectorAll(".bd-example-offcanvas .offcanvas");
o && o.forEach(e => {
	e.addEventListener("show.bs.offcanvas", e => {
		e.preventDefault()
	}, !1)
})