module Day5

# tag::setup[]
using DataStructures
import Base.Iterators: flatten

Point{T} = NTuple{2,T}
EndpointPair{T} = NTuple{2,Point{T}}
LinesVec{T} = Vector{EndpointPair{T}}

line_endpoints = [
    begin
        point_re = r"\s*(\d+),(\d+)\s*->\s*(\d+),(\d+)\s*"
        m = match(point_re, line)
        m === nothing && error("could not parse input: $(line)")
        x1, y1, x2, y2 = parse.(Int, m.captures)
        ((x1, y1), (x2, y2))::EndpointPair{Int}
    end for line in eachline(joinpath(@__DIR__, "input.txt"))
]

function range_between(a, b)
    step = a < b ? 1 : -1
    return a:step:b
end

function get_ans(point_counts)
    return count(p -> last(p) >= 2, collect(point_counts))
end

# end::setup[]

# tag::pt1[]
function get_hv_point_counts(endpoints::LinesVec{T}) where {T}
    acc = Accumulator{Point{T},Int}()
    for ((x1, y1), (x2, y2)) in endpoints
        x1 == x2 || y1 == y2 || continue
        for x in range_between(x1, x2), y in range_between(y1, y2)
            inc!(acc, (x, y))
        end
    end
    return acc
end

hv_point_counts = get_hv_point_counts(line_endpoints)
@show get_ans(hv_point_counts)
# end::pt1[]

# tag::pt2[]
function get_diag_point_counts(endpoints::LinesVec{T}) where {T}
    acc = Accumulator{Point{T},Int}()
    for ((x1, y1), (x2, y2)) in endpoints
        abs(x2 - x1) == abs(y2 - y1) || continue
        for (x, y) in zip(range_between(x1, x2), range_between(y1, y2))
            inc!(acc, (x, y))
        end
    end
    return acc
end

all_point_counts = get_diag_point_counts(line_endpoints)
merge!(all_point_counts, hv_point_counts)
@show get_ans(all_point_counts)
# end::pt2[]

end
