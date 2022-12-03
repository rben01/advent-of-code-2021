#!/usr/bin/env sh
# asciidoctor --verbose --warnings --base-dir . --load-path ./docs -r adoc_ruby/syntax-highlighter -r adoc_ruby/aoc-day-link-inserter -r adoc_ruby/source-includer -r adoc_ruby/input-link-inserter docs/day_adocs/day_03.adoc
asciidoctor --verbose --warnings --base-dir . --load-path docs -r adoc_ruby/aoc-day-link-inserter -r adoc_ruby/source-includer -r adoc_ruby/input-link-inserter ./docs/advent_of_code_2021.adoc

soln_adoc() {
	printf "./src/day_%s/soln.adoc" "$(printf '%02d' "$1")"
}

for i in $(seq 1 25); do
	i02="$(printf "%02d" "$i")"
	soln_file="./src/day_$i02/soln.adoc"

	[ -f "$soln_file" ] || continue

	f="docs/day_adocs/day_$i02.adoc"

	prev_i="$((i - 1))"
	next_i="$((i + 1))"

	prev_soln_path="$(soln_adoc "$prev_i")"
	next_soln_path="$(soln_adoc "$next_i")"
	echo "$prev_soln_path" "$next_soln_path" "$([ -f "$next_soln_path" ] && echo 1 || echo 0)"

	prev_html_path="$(printf "./day_%s" "$prev_i").html"
	next_html_path="$(printf "./day_%s" "$next_i").html"
	echo "$prev_html_path" "$next_html_path" "$([ -f "$next_soln_path" ] && echo 1 || echo 0)"

	{
		echo "include::./docs/shared.adoc[]"
		echo ":stylesheet: ../aoc.css"
		echo
		echo "link:../advent_of_code_2021.html#_problems_and_solutions[pass:n[icon:list-ol[\] Back to Home],role=\"header-nav-link first\"]"
		[ -f "$prev_soln_path" ] && echo "link:./${prev_html_path}[pass:n[icon:chevron-left[\] Previous (Day $prev_i)],role=header-nav-link]"
		[ -f "$next_soln_path" ] && echo "link:./${next_html_path}[pass:n[Next (Day $next_i) icon:chevron-right[\]],role=header-nav-link]"
		echo
		echo "include::{src-dir}/day_$i02/soln.adoc[leveloffset=+1]"
	} >"$f"

done
asciidoctor --verbose --warnings --base-dir . --load-path . -r docs/adoc_ruby/aoc-day-link-inserter -r docs/adoc_ruby/source-includer -r docs/adoc_ruby/input-link-inserter docs/day_adocs/*.adoc

# for f in docs/day_adocs/*.adoc; do
# 	echo "$f"
# 	asciidoctor --verbose --warnings --base-dir . --load-path ./docs -r adoc_ruby/syntax-highlighter -r adoc_ruby/aoc-day-link-inserter -r adoc_ruby/source-includer -r adoc_ruby/input-link-inserter "$f"
# done
