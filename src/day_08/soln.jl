module Day8

# tag::digit[]
N_SEGMENTS = 7

struct Digit
    segments::NTuple{N_SEGMENTS,Bool}
    n_on::UInt8

    Digit(segments::NTuple{N_SEGMENTS,Bool}) = new(segments, sum(segments))
end
Digit(b::Bool) = Digit(ntuple(Returns(b), N_SEGMENTS))

function Digit(segments::AbstractString)
    v = fill(false, N_SEGMENTS)
    for c in segments
        i = 1 + Int(lowercase(c)) - Int('a')
        v[i] = true
    end
    return Digit(Tuple(v))
end

segments(d::Digit) = d.segments
Base.sum(d::Digit) = d.n_on

DIGITS = Dict(
    Digit("abcefg") => 0,
    Digit("cf") => 1,
    Digit("acdeg") => 2,
    Digit("acdfg") => 3,
    Digit("bcdf") => 4,
    Digit("abdfg") => 5,
    Digit("abdefg") => 6,
    Digit("acf") => 7,
    Digit("abcdefg") => 8,
    Digit("abcdfg") => 9,
)

binop(op, d::Digit...) = Digit(op(segments.(d)...))
Base.:(|)(x::Digit, y::Digit...) = binop(.|, x, y...)
Base.:(&)(x::Digit, y::Digit...) = binop(.&, x, y...)
Base.:(~)(x::Digit) = binop(.~, x)

Base.:(⊆)(x::Digit, y::Digit) = (x & y) == x
Base.isless(x::Digit, y::Digit) = string(x) < string(y)

Base.print(io::IO, d::Digit) =
    let s = segments(d), on_indices = findall(s)
        if length(on_indices) == 0
            print(io, '∅')
        else
            join(io, ("ABCDEFG"[i] for i in on_indices))
        end
        index = findfirst(==(d), DIGITS)
        if index !== nothing
            print(io, '=', index - 1)
        end
    end
Base.show(io::IO, d::Digit) = print(io, d)
# end::digit[]
# tag::setup[]
using Combinatorics

function mapping_from_garbled_digits(garbled_digits::AbstractVector{Digit})
    mappings = Dict{Digit,Set{Digit}}()
    for gd in garbled_digits
        mappings[gd] = Set(Iterators.filter(d -> sum(d) == sum(gd), keys(DIGITS)))
    end

    while true
        new_mappings = typeof(mappings)()

        garbled_keys = collect(keys(mappings))
        for (garbled1, garbled2) in combinations(garbled_keys, 2)
            choices1 = mappings[garbled1]
            choices2 = mappings[garbled2]

            ops = (identity, ~)
            for op1 in ops, op2 in ops
                op1 == op2 == (~) && continue

                new_garbled = op1(garbled1) & op2(garbled2)
                0 < sum(new_garbled) < N_SEGMENTS || continue

                new_goods = Set{Digit}()

                for good1 in choices1, good2 in choices2
                    new_good = op1(good1) & op2(good2)
                    sum(new_good) == sum(new_garbled) || continue
                    push!(new_goods, new_good)
                end

                if new_garbled ∉ keys(new_mappings)
                    new_mappings[new_garbled] = new_goods
                else
                    intersect!(new_mappings[new_garbled], new_goods)
                end
            end
        end

        # Remove all keys that can be written as the disjoint-bitwise-or of two other keys,
        # as they're redundant. This means if e.g., A and BC are present, then remove ABC.
        # But if only AB and BC are present, then do *not* remove ABC (as AB and BC are not
        # disjoint)
        redundant_keys = Set{Digit}()
        new_garbled_keys = keys(new_mappings)
        for (garbled1, garbled2) in combinations(collect(new_garbled_keys), 2)
            sum(garbled1 & garbled2) == 0 || continue

            bwo = garbled1 | garbled2
            if bwo ∈ new_garbled_keys
                push!(redundant_keys, bwo)
            end
        end

        for k in redundant_keys
            delete!(new_mappings, k)
        end

        if (length(mappings) == N_SEGMENTS && all(length(o) == 1 for o in values(mappings)))
            return Dict((k => first(o) for (k, o) in mappings)...)
        elseif mappings == new_mappings
            return (mappings, "could not narrow down to just one choice per segment")
        end

        mappings = new_mappings
    end
end

function apply_mapping_to_garbled_digit(
    mapping::AbstractDict{Digit,Digit}, garbled_digit::Digit
)
    result = Digit(false)
    for (k, v) in mapping
        if sum(garbled_digit & k) > 0
            result |= v
        end
    end
    return DIGITS[result]
end

Line = NamedTuple{(:in, :out),Tuple{Vector{Digit},Vector{Digit}}}
lines = let
    arr = Line[]
    for line in eachline(joinpath(@__DIR__, "input.txt"))
        line = strip(line)
        length(line) == 0 && continue

        in, out = strip.(split(line, '|'))

        push!(arr, (; in=Digit.(split(in)), out=Digit.(split(out))))
    end
    arr
end

function translate_line_to_digits(line::Line)
    mapping = mapping_from_garbled_digits(line.in)
    digits = apply_mapping_to_garbled_digit.(Ref(mapping), line.out)
    return digits
end
# end::setup[]

# tag::pt1[]
all_output_digits = hcat(translate_line_to_digits.(lines)...)
@show sum(all_output_digits .∈ Ref((1, 4, 7, 8)))
# end::pt1[]

# tag::pt2[]
function translate_line_to_base10(line::Line)
    digits = translate_line_to_digits(line)
    pow10s = 10 .^ ((length(digits) - 1):-1:0)
    return sum(digits .* pow10s)
end

@show sum(translate_line_to_base10.(lines))
# end::pt2[]

end
