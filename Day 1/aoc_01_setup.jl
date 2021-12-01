module Aoc01
using DataStructures

function get_n_increasing_running_avg_depths(n)
    depth_buf = CircularBuffer{Int}(n)
    lines = (strip(line) for line in eachline(joinpath(@__DIR__, "input_01.txt")))

    for line in lines
        depth = parse(Int, line)
        push!(depth_buf, depth)

        if isfull(depth_buf)
            break
        end
    end

    @assert isfull(depth_buf)

    n_increasing = 0

    for line in lines
        new_depth = parse(Int, line)
        old_depth = depth_buf[1]

        push!(depth_buf, new_depth)

        if new_depth > old_depth
            n_increasing += 1
        end
    end

    n_increasing
end

end
