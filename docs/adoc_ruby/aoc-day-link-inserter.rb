require "asciidoctor"
require "asciidoctor/extensions"

class LinkToDayProblemMacro < Asciidoctor::Extensions::InlineMacroProcessor
  use_dsl

  named :link_day
  name_positional_attributes "day_num"

  def process(parent, target, attrs)
    if !(day_num = attrs["day_num"])
      day_num = target
    end

    link = "https://adventofcode.com/2021/day/#{day_num}"

    parent.document.register :links, link
    create_anchor parent, "icon:star-half-o[] Day #{day_num} problem description", type: :link, target: link, attributes: { "subs" => :normal, "window" => "^" }

    ## Graveyard
    # %(#{(create_anchor parent, text, type: :link, target: target).convert}) create_inline_pass parent, adoc_link, attributes: { "subs" => "macros,quotes,attributes" }
    # adoc_link = "link:https://adventofcode.com/2021/day/$$[icon:external-link[] Problem description^]"
    # create_inline_pass parent, "*https://asciidoctor.org[Asciidoctor]*", attributes: { "subs" => :normal }
  end
end

Asciidoctor::Extensions.register do
  inline_macro LinkToDayProblemMacro
end
