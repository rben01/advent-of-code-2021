# %%
module Day10

# tag::setup[]
lines = collect(eachline(joinpath(@__DIR__, "input.txt")))

"""
```jldoctest
julia> braces
4×2 Matrix{Char}:
 '('  ')'
 '['  ']'
 '{'  '}'
 '<'  '>'
```
"""
const braces = ['(' ')'; '[' ']'; '{' '}'; '<' '>']

const bracetype_axis, orientation_axis = axes(braces)

struct BraceId
    id::Int
    function BraceId(id::Int)
        id in bracetype_axis || error("Invalid brace: $(id)")
        return new(id)
    end
end
struct OrientationId
    id::Int
    function OrientationId(id::Int)
        id in orientation_axis || error("Invalid brace: $(id)")
        return new(id)
    end
end

flip(o::OrientationId) = OrientationId(length(orientation_axis) - o.id + 1)

BraceKind{N} = @NamedTuple {id::BraceId, elems::NTuple{2,Char}}
Orientation{N} = @NamedTuple {id::OrientationId, elems::NTuple{4,Char}}

const parens, squares, curlys, angles = map(
    i -> (; id=BraceId(i), elems=Tuple(braces[i, :]))::BraceKind, bracetype_axis
)

const lefts, rights = map(
    i -> (; id=OrientationId(i), elems=Tuple(braces[:, i]))::Orientation, orientation_axis
)

struct Token
    brace::BraceId
    orientation::OrientationId

    function Token(brace::BraceId, orientation::OrientationId)
        brace.id ∈ bracetype_axis || error("Invalid brace: $(brace)")
        orientation.id ∈ orientation_axis || error("Invalid orientation: $(orientation)")
        return new(brace, orientation)
    end
end

function get_container(elem, sets)
    maybe_superset = Iterators.peel(a for a in sets if elem ∈ a.elems)
    maybe_superset === nothing && error("invalid: $(elem)")
    superset, _rest = maybe_superset
    return superset
end
function Token(c::Char)
    brace = get_container(c, (parens, squares, curlys, angles)).id
    orientation = get_container(c, (lefts, rights)).id

    return Token(brace, orientation)
end

Base.show(io::IO, token::Token) = Base.show(io, braces[token.orientation, token.brace])

brace_kind(t::Token) = t.brace
orientation(t::Token) = t.orientation
flip(t::Token) = Token(brace_kind(t), flip(orientation(t)))

matches(t::Token, b::BraceKind) = brace_kind(t) == b.id
matches(t::Token, o::Orientation) = orientation(t) == o.id

struct Ok end
struct Corrupted
    token::Token
end
struct Incomplete
    tokens::Vector{Token}
end

function parse_line(line::AbstractString)::Union{Ok,Incomplete,Corrupted}
    token_stack = Token[]

    for c in line
        t = Token(c)

        if isempty(token_stack)
            push!(token_stack, t)
            continue
        end

        prev = token_stack[end]

        if matches(prev, lefts) && matches(t, rights)
            brace_kind(prev) == brace_kind(t) || return Corrupted(t)
            pop!(token_stack)
        else
            push!(token_stack, t)
        end
    end

    isempty(token_stack) && return Ok()

    remaining_tokens = flip.(reverse(token_stack))
    return Incomplete(remaining_tokens)
end

results = parse_line.(lines)
# end::setup[]
# tag::pt1[]
function score(corr::Corrupted)
    t = corr.token
    scores = (
        (parens, 3), (squares, 57), (curlys, 1197), (angles, 25137)
    )::NTuple{4,Tuple{BraceKind,Int}}

    idx = findfirst(((brace_type, _),) -> matches(t, brace_type), scores)
    _, score = scores[idx]
    return score
end

@show sum(score(r) for r in results if r isa Corrupted)
# end::pt1[]
# tag::pt2[]
function score(inco::Incomplete)
    scores = (
        (parens, 1), (squares, 2), (curlys, 3), (angles, 4)
    )::NTuple{4,Tuple{BraceKind,Int}}

    net_score = 0
    for t in inco.tokens
        net_score *= 5

        idx = findfirst(((brace_type, _),) -> matches(t, brace_type), scores)
        _, score = scores[idx]
        net_score += score
    end

    return net_score
end

incomplete_scores = sort(score.(filter(r -> r isa Incomplete, results)))
@show incomplete_scores[div(
    firstindex(incomplete_scores) + lastindex(incomplete_scores), 2
)]
# end::pt2[]

end
