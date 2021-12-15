require "asciidoctor"
require "asciidoctor/extensions"
require "erb"

def is_integer? s
  s.to_i.to_s == s
end

class InputLinkInserterMacro < Asciidoctor::Extensions::InlineMacroProcessor
  include ERB::Util
  include Asciidoctor::Logging

  use_dsl

  named :link_input
  name_positional_attributes "filename"

  def process(parent, target, attrs)
    if !(filepath = attrs["path"])
      filepath = target
    end

    if is_integer? filepath
      n = "%02d" % filepath.to_i
      filepath = "/src/day_#{n}/input.txt"
    elsif filepath.end_with? "/"
      filepath = "#{filepath}input.txt"
    end

    path_comps = filepath.split "/"
    link = path_comps.map { |pc| url_encode(pc) }.join("/")

    parent.document.register :links, link
    create_anchor parent, "icon:file-text-o[] Problem input", type: :link, target: link, attributes: { "subs" => :normal, "window" => "^" }
  end
end

Asciidoctor::Extensions.register do
  inline_macro InputLinkInserterMacro
end
