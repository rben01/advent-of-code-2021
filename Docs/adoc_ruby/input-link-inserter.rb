require "asciidoctor"
require "asciidoctor/extensions"
require "erb"

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
    if filepath.end_with? "/"
      filepath = "#{filepath}input.txt"
    end

    path_comps = filepath.split "/"
    link = path_comps.map { |pc| url_encode(pc) }.join("/")

    logger.info(filepath)
    logger.info(link)

    parent.document.register :links, link
    create_anchor parent, "Problem input", type: :link, target: link, attributes: { "subs" => :normal, "window" => "^" }
  end
end

Asciidoctor::Extensions.register do
  inline_macro InputLinkInserterMacro
end
