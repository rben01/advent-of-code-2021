# %%
# tag::setup[]
using Statistics

positions =
    let fn = joinpath(@__DIR__, "input.txt"),
        content = read(fn, String),
        items = strip.(split(content, ','))

        parse.(Int, items)
    end
# end::setup[]
# %%
# tag::pt1[]
md = round(Int, median(positions))
@show sum(abs.(positions .- md))
# end::pt1[]
# %%
# tag::pt2[]
mn_lo, mn_hi = round.(Int, mean(positions), (RoundDown, RoundUp))
cost(diff) =
    let diff = abs(diff)
        div(diff * (diff + 1), 2)
    end
@show minimum(sum(cost.(positions .- [mn_lo mn_hi]); dims=1))

# end::pt2[]
