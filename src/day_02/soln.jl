
# tag::pt1[]
horizontal = vertical = 0
for line in eachline(joinpath(@__DIR__, "input.txt"))
    dir, dist = split(strip(line))
    dist = parse(Int, dist)

    if dir == "forward"
        horizontal += dist
    elseif dir == "up"
        vertical -= dist
    elseif dir == "down"
        vertical += dist
    else
        throw("Invalid direction $(dir)")
    end
end

ans = horizontal * vertical
@show ans
# end::pt1[]

# tag::pt2[]
horizontal = vertical = aim = 0
for line in eachline(joinpath(@__DIR__, "input.txt"))
    dir, dist = split(strip(line))
    dist = parse(Int, dist)

    if dir == "forward"
        horizontal += dist
        vertical += aim * dist
    elseif dir == "up"
        aim -= dist
    elseif dir == "down"
        aim += dist
    else
        throw("Invalid direction $(dir)")
    end
end

ans = horizontal * vertical
@show ans
# end::pt2[]
