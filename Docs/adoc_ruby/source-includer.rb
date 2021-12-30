require "asciidoctor"
require "asciidoctor/extensions"

class SourceIncluderMacro < Asciidoctor::Extensions::BlockMacroProcessor
  include Asciidoctor::Logging

  use_dsl

  named :include_source

  def process(parent, target, attrs)
    # todo: figure out why this doesn't work
    # attrs[:indent] ||= 3

    lang = ""
    if target.end_with? ".jl"
      lang = "julia"
    elsif target.end_with? ".rs"
      lang = "rust"
    end

    attr_comps = []
    attrs.each_pair { |k, v|
      if k == "tag"
        k = "tags"
        v = %(#{v};!debugging)
      end
      attr_comps.push(%(#{k}=#{v}))
    }
    attr_str = attr_comps.join(",")

    resolved_dir = parent.document.reader.dir
    include_path = File.join(resolved_dir, target)
    adoc_content = [
      "[source,#{lang},indent=0]",
      "----",
      "include::#{include_path}[#{attr_str}]",
      "----",
    ].join("\n")

    doc = Asciidoctor.load adoc_content, safe: :unsafe, attributes: parent.document.attributes
    doc.blocks[0]
  end
end

Asciidoctor::Extensions.register do
  block_macro SourceIncluderMacro
end
