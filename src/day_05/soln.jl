# tag::setup[]
using DataStructures
import Base.Iterators: flatten

Point{T} = NTuple{2,T}
EndpointPair{T} = NTuple{2,Point{T}}

line_endpoints = [
    begin
        point_re = r"\s*(\d+),(\d+)\s*->\s*(\d+),(\d+)\s*"
        m = match(point_re, line)
        m === nothing && error("could not parse input: $(line)")
        x1, y1, x2, y2 = parse.(Int, m.captures)
        ((x1, y1), (x2, y2))
    end for line ∈ eachline(joinpath(@__DIR__, "input.txt"))
]

function range_between(a, b)
    step = a < b ? 1 : -1
    return a:step:b
end

# end::setup[]

# tag::pt1[]
function get_hv_point_counts(endpoints::Vector{EndpointPair{T}}) where {T}
    acc = Accumulator{Point{T},Int}()
    for ((x1, y1), (x2, y2)) ∈ endpoints
        x1 == x2 || y1 == y2 || continue
        for x ∈ range_between(x1, x2), y ∈ range_between(y1, y2)
            inc!(acc, (x, y))
        end
    end
    return acc
end

hv_point_counts = get_hv_point_counts(line_endpoints)
pt1_ans = count(p -> last(p) >= 2, collect(hv_point_counts))
@show pt1_ans
# end::pt1[]

# tag::pt2[]
function get_diag_point_counts(endpoints::Vector{EndpointPair{T}}) where {T}
    acc = Accumulator{Point{T},Int}()
    for ((x1, y1), (x2, y2)) ∈ endpoints
        abs(x2 - x1) == abs(y2 - y1) || continue
        for (x, y) in zip(range_between(x1, x2), range_between(y1, y2))
            inc!(acc, (x, y))
        end
    end
    return acc
end

diag_point_counts = get_diag_point_counts(line_endpoints)
all_point_counts = merge(diag_point_counts, hv_point_counts)
pt2_ans = count(p -> last(p) >= 2, collect(all_point_counts))
@show pt2_ans
# end::pt2[]
