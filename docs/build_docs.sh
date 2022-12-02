#!/usr/bin/env sh
# asciidoctor --verbose --warnings --base-dir . --load-path ./docs -r adoc_ruby/syntax-highlighter -r adoc_ruby/aoc-day-link-inserter -r adoc_ruby/source-includer -r adoc_ruby/input-link-inserter docs/day_adocs/day_03.adoc
asciidoctor --verbose --warnings --base-dir . --load-path docs -r adoc_ruby/aoc-day-link-inserter -r adoc_ruby/source-includer -r adoc_ruby/input-link-inserter ./docs/advent_of_code_2021.adoc

for i in $(seq -w 1 25); do
	f="docs/day_adocs/day_$i.adoc"
	{
		echo "include::./docs/shared.adoc[]"
		echo ":stylesheet: ../aoc.css"
		echo
		echo "link:../advent_of_code_2021.html#_problems_and_solutions[pass:n[icon:list-ol[\] Back to Home]]"
		echo
		echo "include::{src-dir}/day_$i/soln.adoc[leveloffset=+1]"
	} >"$f"

done
asciidoctor --verbose --warnings --base-dir . --load-path . -r docs/adoc_ruby/aoc-day-link-inserter -r docs/adoc_ruby/source-includer -r docs/adoc_ruby/input-link-inserter docs/day_adocs/*.adoc

# for f in docs/day_adocs/*.adoc; do
# 	echo "$f"
# 	asciidoctor --verbose --warnings --base-dir . --load-path ./docs -r adoc_ruby/syntax-highlighter -r adoc_ruby/aoc-day-link-inserter -r adoc_ruby/source-includer -r adoc_ruby/input-link-inserter "$f"
# done
