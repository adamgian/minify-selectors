use std::collections::HashMap;
use std::path::PathBuf;

use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use encode_selector;
use minify_selectors_utils::*;
use parse_selectors::*;




pub fn parse_selectors_benchmarks(c: &mut Criterion) {
	let config = Config {
		source: PathBuf::from(""),
		output: PathBuf::from(""),
		alphabet: encode_selector::into_alphabet_set(concat!(
			"0123456789",
			"abcdefghijklmnopqrstuvwxyz",
			"ABCDEFGHIJKLMNOPQRSTUVWXYZ",
		)),
		start_index: 0,
	};


	let mut css_sample = String::from(
		r##"
			.bd-masthead {
				--bd-pink-rgb: 214, 51, 132;
				padding: 3rem 0;
				background-image: linear-gradient(
						180deg,
						rgba(var(--bs-body-bg-rgb), 0.01),
						rgba(var(--bs-body-bg-rgb), 1) 85%
					),
					radial-gradient(
						ellipse at top left,
						rgba(var(--bs-primary-rgb), 0.5),
						transparent 50%
					),
					radial-gradient(
						ellipse at top right,
						rgba(var(--bd-accent-rgb), 0.5),
						transparent 50%
					),
					radial-gradient(
						ellipse at center right,
						rgba(var(--bd-violet-rgb), 0.5),
						transparent 50%
					),
					radial-gradient(
						ellipse at center left,
						rgba(var(--bd-pink-rgb), 0.5),
						transparent 50%
					);
			}

			.bd-masthead h1 {
				font-size: calc(1.525rem + 3.3vw);
				line-height: 1;
			}

			@media (min-width: 1200px) {
				.bd-masthead h1 {
					font-size: 4rem;
				}
			}

			.bd-masthead .lead {
				font-size: 1rem;
				font-weight: 400;
				color: #495057;
			}

			.bd-masthead .bd-code-snippet {
				margin: 0;
				border-radius: 0.5rem;
			}

			.bd-masthead .highlight {
				width: 100%;
				padding: 0.5rem 1rem;
				overflow: hidden;
				text-overflow: ellipsis;
				white-space: nowrap;
				background-color: rgba(var(--bs-body-color-rgb), 0.075);
				border-radius: 0.5rem;
			}
		"##,
	);

	let mut css_sample_selectors = Selectors {
		map: HashMap::new(),
		class_index: config.start_index,
		id_index: config.start_index,
	};

	c.bench_function("parse_selectors::from_css fn", |b| {
		b.iter(|| {
			from_css(
				black_box(&mut css_sample),
				black_box(&mut css_sample_selectors),
				black_box(&config),
			)
		})
	});


	let mut html_sample_selectors = Selectors {
		map: HashMap::new(),
		class_index: config.start_index,
		id_index: config.start_index,
	};

	let mut html_sample = String::from(
		r##"
			<div class="bd-masthead mb-3" id="content">
				<div class="container-xxl bd-gutter">
					<div class="col-md-8 mx-auto text-center">
						<a class="d-flex flex-column flex-lg-row justify-content-center align-items-center mb-4 text-dark lh-sm text-decoration-none" href="https://blog.getbootstrap.com/2022/07/19/bootstrap-5-2-0/">
							<strong class="d-sm-inline-block p-2 me-2 mb-2 mb-lg-0 rounded-3 masthead-notice">New in v5.2</strong>
							<span class="text-muted">CSS variables, responsive offcanvas, new utilities, and more!</span>
						</a>
						<img src="docs/5.2/assets/brand/bootstrap-logo-shadow.png" width="200" height="165" alt="Bootstrap" class="d-block mx-auto mb-3">
						<h1 class="mb-3 fw-semibold">Build fast, responsive sites with&nbsp;Bootstrap</h1>
						<p class="lead mb-4">
							Powerful, extensible, and feature-packed frontend toolkit. Build and customize with Sass, utilize prebuilt grid system and components, and bring projects to life with powerful JavaScript plugins.
						</p>
						<div class="d-flex flex-column flex-lg-row align-items-md-stretch justify-content-md-center gap-3 mb-4">
							<div class="d-inline-block v-align-middle fs-5">
								<div class="highlight"><pre tabindex="0" class="chroma"><code class="language-sh" data-lang="sh"><span class="line"><span class="cl">npm i bootstrap@5.2.2</span></span></code></pre></div>
							</div>
							<a href="/docs/5.2/getting-started/introduction/" class="btn btn-lg bd-btn-lg btn-bd-primary d-flex align-items-center justify-content-center fw-semibold" onclick="ga('send', 'event', 'Jumbotron actions', 'Get started', 'Get started');">
								<svg class="bi me-2" aria-hidden="true"><use xlink:href="#book-half"></use></svg>
								Read the docs
							</a>
						</div>
						<p class="text-muted mb-0">
							Currently <strong>v5.2.2</strong>
							<span class="px-1">&middot;</span>
							<a href="/docs/5.2/getting-started/download/" class="link-secondary">Download</a>
							<span class="px-1">&middot;</span>
							<a href="https://getbootstrap.com/docs/4.6/getting-started/introduction/" class="link-secondary text-nowrap">v4.6.x docs</a>
							<span class="px-1">&middot;</span>
							<a href="/docs/versions/" class="link-secondary text-nowrap">All releases</a>
						</p>
					</div>
				</div>
			</div>
		"##,
	);

	c.bench_function("parse_selectors::from_html fn", |b| {
		b.iter(|| {
			from_html(
				black_box(&mut html_sample),
				black_box(&mut html_sample_selectors),
				black_box(&config),
			)
		})
	});


	let mut js_sample_selectors = Selectors {
		map: HashMap::new(),
		class_index: config.start_index,
		id_index: config.start_index,
	};

	let mut js_sample = String::from(
		r##"
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
		"##,
	);

	c.bench_function("parse_selectors::from_js fn", |b| {
		b.iter(|| {
			from_js(
				black_box(&mut js_sample),
				black_box(&mut js_sample_selectors),
				black_box(&config),
			)
		})
	});
}

criterion_group!(benches, parse_selectors_benchmarks);
criterion_main!(benches);
