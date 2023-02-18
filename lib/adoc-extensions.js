const antora = require("@antora/asciidoc-loader");
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
				filepath = `/_attachments/src/day_${numStr}/${filename}`;
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

			const tags = attrs.tag ?? attrs.tags;
			if (typeof tags === "string") {
				attrs.tags = [tags, "!debugging"].join(";");
			}

			const resolvedDir = parent.document.reader.dir;
			const includePath = path.join("solutions", resolvedDir, target);

			const content = fs.readFileSync(includePath).toString();

			return this.createListingBlock(parent, content, {
				...attrs,
				style: "source",
				language: "rust",
			});
		});
	});
};
