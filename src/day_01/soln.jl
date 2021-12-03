# tag::setup[]
using DataStructures
using Base.Iterators

function get_n_increasing_running_sum_of_depths(n)
    depth_buf = CircularBuffer{Int}(n)
    depths = (parse(Int, strip(line)) for line in eachline(joinpath(@__DIR__, "input.txt")))

    append!(depth_buf, take(depths, n))
    @assert isfull(depth_buf)

    n_increasing = 0

    for new_depth in depths
        old_depth = depth_buf[1]

        push!(depth_buf, new_depth)
        if new_depth > old_depth
            n_increasing += 1
        end
    end

    n_increasing
end
# end::setup[]

# tag::pt1[]
@show get_n_increasing_running_sum_of_depths(1)
# end::pt1[]
# tag::pt2[]
@show get_n_increasing_running_sum_of_depths(3)
# end::pt2[]
