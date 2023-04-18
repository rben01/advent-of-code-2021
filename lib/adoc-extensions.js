const antora = require("@antora/asciidoc-loader");
const asciidoc = require("asciidoctor")();
const asciidoctorKatex = require("asciidoctor-katex");
const highlightJs = require("asciidoctor-highlight.js");
const path = require("path");
const fs = require("fs");

// https://stackoverflow.com/a/2998874
const zeroPad = (num, places) => String(num).padStart(places, "0");

function isInt(s) {
	return String(parseInt(s)) === s;
}

module.exports.register = function (registry, _context) {
	const rootDir = "/advent-of-code-2021";
	asciidoctorKatex.register(registry);
	highlightJs.register(registry);
	registry.inlineMacro("link_day", function () {
		this.process((parent, target, attrs) => {
			const day_num =
				attrs.day_num ?? target ?? parent.document.attributes.day_num;
			target = `https://adventofcode.com/2021/day/${day_num}`;

			return this.createInline(
				parent,
				"anchor",
				`icon:star-half-o[]Day #${day_num} problem description`,
				{
					type: "link",
					target,
					attributes: { subs: "normal", window: "^" },
				},
			);
		});
	});
	registry.inlineMacro("link_input", function () {
		this.process((parent, target, attrs) => {
			let filepath;
			let filename = attrs.name;
			filename = filename === undefined ? "input.txt" : filename;

			const num = attrs.path ?? target ?? parent.document.attributes.day_num;
			if (isInt(num)) {
				const numStr = zeroPad(+num, 2);
				filepath = `${rootDir}/_attachments/src/day_${numStr}/${filename}`;
			} else if (filepath.endsWith("/")) {
				filepath = `${filepath}${filename}`;
			}

			filepath = filepath.split("/").map(encodeURIComponent).join("/");

			return this.createInline(
				parent,
				"anchor",
				"icon:file-text-o[]Problem input",
				{
					type: "link",
					target: filepath,
					attributes: { subs: "normal", window: "^" },
				},
			);
		});
	});
	registry.blockMacro("include_source", function () {
		this.process((parent, target, attrs) => {
			let lang = attrs.lang;
			if (lang === undefined) {
				if (target.endsWith(".rs")) lang = "rust";
			}

			let tags = attrs.tag ?? attrs.tags;
			if (typeof tags === "string") {
				tags = [tags];
			}
			tags.push("!debugging");
			attrs.tags = tags.join(";");
			delete attrs.tag;

			const attrStr = Object.entries(attrs)
				.map(([k, v]) => `${k}=${v}`)
				.join(",");

			const resolvedDir = parent.document.reader.dir;
			const includePath = path.join("solutions", resolvedDir, target);

			const adocContent = (adoc_content = [
				`[source,${lang},indent=0]`,
				"----",
				`include::${includePath}[${attrStr}]`,
				"----",
			].join("\n"));

			const doc = asciidoc.load(adocContent, {
				safe: "unsafe",
				attributes: parent.document.attribtues,
			});

			return doc.blocks[0];
		});
	});
};
