!(function () {
	"use strict";
	var e,
		o,
		r,
		s = /^sect(\d)$/,
		i = document.querySelector(".nav-container"),
		a = document.querySelector(".nav-toggle"),
		c = i.querySelector(".nav"),
		l =
			(a.addEventListener("click", function (e) {
				if (a.classList.contains("is-active")) return u(e);
				v(e);
				var e = document.documentElement,
					t =
						(e.classList.add("is-clipped--nav"),
						a.classList.add("is-active"),
						i.classList.add("is-active"),
						c.getBoundingClientRect()),
					n = window.innerHeight - Math.round(t.top);
				// Math.round(t.height) !== n && (c.style.height = n + "px");
				e.addEventListener("click", u);
			}),
			i.addEventListener("click", v),
			i.querySelector("[data-panel=explore]"));
	function t() {
		var e,
			t,
			n = window.location.hash;
		if (
			n &&
			(n.indexOf("%") && (n = decodeURIComponent(n)),
			!(e = l.querySelector('.nav-link[href="' + n + '"]')))
		) {
			n = document.getElementById(n.slice(1));
			if (n)
				for (
					var i = n, a = document.querySelector("article.doc");
					(i = i.parentNode) && i !== a;

				) {
					var c = i.id;
					if (
						(c =
							c ||
							((c = s.test(i.className)) && (i.firstElementChild || {}).id)) &&
						(e = l.querySelector('.nav-link[href="#' + c + '"]'))
					)
						break;
				}
		}
		if (e) t = e.parentNode;
		else {
			if (!r) return;
			e = (t = r).querySelector(".nav-link");
		}
		t !== o &&
			(h(l, ".nav-item.is-active").forEach(function (e) {
				e.classList.remove("is-active", "is-current-path", "is-current-page");
			}),
			t.classList.add("is-current-page"),
			d((o = t)),
			p(l, e));
	}
	function d(e) {
		for (
			var t, n = e.parentNode;
			!(t = n.classList).contains("nav-panel-explore");

		) {
			"LI" === n.tagName &&
				t.contains("nav-item") &&
				t.add("is-active", "is-current-path"),
				(n = n.parentNode);
		}
		e.classList.add("is-active");
	}
	function n() {
		var e, t, n, i;
		this.classList.toggle("is-active") &&
			((e = parseFloat(window.getComputedStyle(this).marginTop)),
			(t = this.getBoundingClientRect()),
			(n = l.getBoundingClientRect()),
			0 < (i = (t.bottom - n.top - n.height + e).toFixed())) &&
			(l.scrollTop += Math.min((t.top - n.top - e).toFixed(), i));
	}
	function u(e) {
		v(e);
		e = document.documentElement;
		e.classList.remove("is-clipped--nav"),
			a.classList.remove("is-active"),
			i.classList.remove("is-active"),
			e.removeEventListener("click", u);
	}
	function v(e) {
		e.stopPropagation();
	}
	function p(e, t) {
		var n = e.getBoundingClientRect(),
			i = n.height,
			a = window.getComputedStyle(c);
		"sticky" === a.position && (i -= n.top - parseFloat(a.top)),
			(e.scrollTop = Math.max(
				0,
				0.5 * (t.getBoundingClientRect().height - i) + t.offsetTop,
			));
	}
	function h(e, t) {
		return [].slice.call(e.querySelectorAll(t));
	}
	l &&
		((e = l),
		(o = l.querySelector(":not(.component).is-current-page")),
		(r = o) ? (d(o), p(l, o.querySelector(".nav-text"))) : (l.scrollTop = 0),
		h(l, ".nav-item-expander").forEach(function (e) {
			var t = e.parentElement,
				e =
					(e.addEventListener("click", n.bind(t)),
					(function (e, t) {
						e = e.nextElementSibling;
						return (
							(!e || !t || e[e.matches ? "matches" : "msMatchesSelector"](t)) &&
							e
						);
					})(e, ".nav-text"));
			e && ((e.style.cursor = "pointer"), e.addEventListener("click", n.bind(t)));
		}),
		// This used to toggle the nav bar on tap, but that's not necessary anymore
		// e &&
		// 	e.querySelector(".context").addEventListener("click", function () {
		// 		h(c, "[data-panel]").forEach(function (e) {
		// 			e.classList.toggle("is-active");
		// 		});
		// 	}),
		l.querySelector('.nav-link[href^="#"]')) &&
		(window.location.hash && t(), window.addEventListener("hashchange", t));
})();
function __get_copy_icon_path() {
	return window.matchMedia && window.matchMedia("(prefers-color-scheme: dark)").matches
		? "/img/dark-theme/octicons-16.svg#icon-clippy"
		: "/img/light-theme/octicons-16.svg#icon-clippy";
}
!(function () {
	"use strict";
	var e = document.querySelector("aside.toc.sidebar");
	if (e) {
		if (document.querySelector("body.-toc")) return e.parentNode.removeChild(e);
		var t = parseInt(e.dataset.levels || 2, 10);
		if (!(t < 0)) {
			for (
				var o = "article.doc", d = document.querySelector(o), n = [], i = 0;
				i <= t;
				i++
			) {
				var r = [o];
				if (i) {
					for (var c = 1; c <= i; c++)
						r.push((2 === c ? ".sectionbody>" : "") + ".sect" + c);
					r.push("h" + (i + 1) + "[id]");
				} else r.push("h1[id].sect0");
				n.push(r.join(">"));
			}
			(m = n.join(",")), (f = d.parentNode);
			var a,
				s = [].slice.call((f || document).querySelectorAll(m));
			if (!s.length) return e.parentNode.removeChild(e);
			var l = {},
				u = s.reduce(function (e, t) {
					var o = document.createElement("a"),
						n =
							((o.textContent = t.textContent),
							(l[(o.href = "#" + t.id)] = o),
							document.createElement("li"));
					return (
						(n.dataset.level = parseInt(t.nodeName.slice(1), 10) - 1),
						n.appendChild(o),
						e.appendChild(n),
						e
					);
				}, document.createElement("ul")),
				f = e.querySelector(".toc-menu"),
				m =
					(f || ((f = document.createElement("div")).className = "toc-menu"),
					document.createElement("h3")),
				e =
					((m.textContent = e.dataset.title || "Contents"),
					f.appendChild(m),
					f.appendChild(u),
					!document.getElementById("toc") &&
						(d.querySelector(".title-container .subtitle") ||
							d.querySelector("h1.page ~ :not(.is-before-toc)")));
			e &&
				(((m = document.createElement("aside")).className = "toc embedded"),
				m.appendChild(f.cloneNode(!0)),
				e.parentNode.insertBefore(m, e.nextSibling)),
				// FALSE: "this scrolls the window to the target #anchor when loaded, which in
				// theory we shouldn't need"
				// TRUE: "this scrolls the window to the target #anchor when loaded AND
				// keeps the table of contents in sync with the scroll position, which we
				// absolutely do need"
				window.addEventListener("load", function () {
					p(), window.addEventListener("scroll", p);
				});
			undefined;
		}
	}
	function p() {
		var n,
			i,
			t,
			e = window.pageYOffset,
			o = 1.15 * h(document.documentElement, "fontSize"),
			r = d.offsetTop;
		e && window.innerHeight + e + 2 >= document.documentElement.scrollHeight
			? ((a = Array.isArray(a) ? a : Array(a || 0)),
			  (n = []),
			  (i = s.length - 1),
			  s.forEach(function (e, t) {
					var o = "#" + e.id;
					t === i || e.getBoundingClientRect().top + h(e, "paddingTop") > r
						? (n.push(o), a.indexOf(o) < 0 && l[o].classList.add("is-active"))
						: ~a.indexOf(o) && l[a.shift()].classList.remove("is-active");
			  }),
			  (u.scrollTop = u.scrollHeight - u.offsetHeight),
			  (a = 1 < n.length ? n : n[0]))
			: (Array.isArray(a) &&
					(a.forEach(function (e) {
						l[e].classList.remove("is-active");
					}),
					(a = void 0)),
			  s.some(function (e) {
					if (e.getBoundingClientRect().top + h(e, "paddingTop") - o > r)
						return !0;
					t = "#" + e.id;
			  }),
			  t
					? t !== a &&
					  (a && l[a].classList.remove("is-active"),
					  (e = l[t]).classList.add("is-active"),
					  u.scrollHeight > u.offsetHeight &&
							(u.scrollTop = Math.max(
								0,
								e.offsetTop + e.offsetHeight - u.offsetHeight,
							)),
					  (a = t))
					: a && (l[a].classList.remove("is-active"), (a = void 0)));
	}
	function h(e, t) {
		return parseFloat(window.getComputedStyle(e)[t]);
	}
})();
!(function () {
	"use strict";
	var o = document.querySelector("article.doc"),
		t = document.querySelector(".toolbar");
	function i(e) {
		return e && (~e.indexOf("%") ? decodeURIComponent(e) : e).slice(1);
	}
	function r(e) {
		if (e) {
			if (e.altKey || e.ctrlKey) return;
			(window.location.hash = "#" + this.id), e.preventDefault();
		}
		window.scrollTo(
			0,
			(function e(t, n) {
				return o.contains(t) ? e(t.offsetParent, t.offsetTop + n) : n;
			})(this, 0) - t.getBoundingClientRect().bottom,
		);
	}
	window.addEventListener("load", function e(t) {
		var n;
		(n = i(window.location.hash)) &&
			(n = document.getElementById(n)) &&
			(r.bind(n)(), setTimeout(r.bind(n), 0)),
			window.removeEventListener("load", e);
	}),
		Array.prototype.slice
			.call(document.querySelectorAll('a[href^="#"]'))
			.forEach(function (e) {
				var t;
				(t = i(e.hash)) &&
					(t = document.getElementById(t)) &&
					e.addEventListener("click", r.bind(t));
			});
})();
!(function () {
	"use strict";
	var t,
		e = document.querySelector(".page-versions .version-menu-toggle");
	e &&
		((t = document.querySelector(".page-versions")),
		e.addEventListener("click", function (e) {
			t.classList.toggle("is-active"), e.stopPropagation();
		}),
		document.documentElement.addEventListener("click", function () {
			t.classList.remove("is-active");
		}));
})();
!(function () {
	"use strict";
	var t = document.querySelector(".navbar-burger");
	t &&
		t.addEventListener(
			"click",
			function (t) {
				t.stopPropagation(),
					document.documentElement.classList.toggle("is-clipped--navbar"),
					this.classList.toggle("is-active");
				t = document.getElementById(this.dataset.target);
				{
					var e;
					t.classList.toggle("is-active") &&
						((t.style.maxHeight = ""),
						(e = window.innerHeight - Math.round(t.getBoundingClientRect().top)),
						parseInt(window.getComputedStyle(t).maxHeight, 10) !== e) &&
						(t.style.maxHeight = e + "px");
				}
			}.bind(t),
		);
})();
!(function () {
	"use strict";
	var o = /^\$ (\S[^\\\n]*(\\\n(?!\$ )[^\\\n]*)*)(?=\n|$)/gm,
		s = /( ) *\\\n *|\\\n( ?) */g,
		l = / +$/gm,
		e = (document.getElementById("site-script") || { dataset: {} }).dataset,
		d = null == e.uiRootPath ? "." : e.uiRootPath,
		r = "svg",
		p = window.navigator.clipboard;
	[].slice
		.call(document.querySelectorAll(".doc pre.highlight, .doc .literalblock pre"))
		.forEach(function (e) {
			var t, n, a, c;
			if (e.classList.contains("highlight"))
				(i = (t = e.querySelector("code")).dataset.lang) &&
					"console" !== i &&
					(((a = document.createElement("span")).className = "source-lang"),
					a.appendChild(document.createTextNode(i)));
			else {
				if (!e.innerText.startsWith("$ ")) return;
				var i = e.parentNode.parentNode;
				i.classList.remove("literalblock"),
					i.classList.add("listingblock"),
					e.classList.add("highlightjs", "highlight"),
					((t = document.createElement("code")).className =
						"language-console hljs"),
					(t.dataset.lang = "console"),
					t.appendChild(e.firstChild),
					e.appendChild(t);
			}
			((i = document.createElement("div")).className = "source-toolbox"),
				a && i.appendChild(a),
				p &&
					(((n = document.createElement("button")).className = "copy-button"),
					n.setAttribute("title", "Copy to clipboard"),
					"svg" === r
						? ((a = document.createElementNS(
								"http://www.w3.org/2000/svg",
								"svg",
						  )).setAttribute("class", "copy-icon"),
						  (c = document.createElementNS(
								"http://www.w3.org/2000/svg",
								"use",
						  )).setAttribute("href", "#icon-clipboard"),
						  a.appendChild(c),
						  n.appendChild(a))
						: (((c = document.createElement("img")).src =
								d + __get_copy_icon_path()),
						  (c.alt = "copy icon"),
						  (c.className = "copy-icon"),
						  n.appendChild(c)),
					((a = document.createElement("span")).className = "copy-toast"),
					a.appendChild(document.createTextNode("Copied!")),
					n.appendChild(a),
					i.appendChild(n),
					window
						.matchMedia("(prefers-color-scheme: dark)")
						.addEventListener("change", (event) => {
							if (c) {
								c.src = d + __get_copy_icon_path(s);
							}
						})),
				e.parentNode.appendChild(i),
				n &&
					n.addEventListener(
						"click",
						function (e) {
							var t = e.innerText.replace(l, "");
							"console" === e.dataset.lang &&
								t.startsWith("$ ") &&
								(t = (function (e) {
									var t,
										n = [];
									for (; (t = o.exec(e)); ) n.push(t[1].replace(s, "$1$2"));
									return n.join(" && ");
								})(t));
							window.navigator.clipboard.writeText(t).then(
								function () {
									this.classList.add("clicked"),
										this.offsetHeight,
										this.classList.remove("clicked");
								}.bind(this),
								function () {},
							);
						}.bind(n, t),
					);
		});
})();
