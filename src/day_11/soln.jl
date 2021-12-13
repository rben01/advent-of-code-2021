# %%
module Day11
# %%
# tag::setup[]
octopi = let
    lines = eachline(joinpath(@__DIR__, "input.txt"))
    parsed_lines = (parse.(Int, reshape(collect(line), (1, :))) for line in lines)
    vcat(parsed_lines...)
end
# end::setup[]

# tag::pt1[]
function tick_in_place(octopi::AbstractArray)
    octopi .+= 1

    row_ax, col_ax = axes(octopi)
    n_flashes = 0

    # We use equality with FLASH_THRESH to mean "will flash right now", whereas being
    # greater than FLASH_THRESH means "has already flashed (and won't flash again)"

    FLASH_THRESH = 10
    while true
        flashing_octopi_idxs = findall(octopi .== FLASH_THRESH)
        isempty(flashing_octopi_idxs) && break

        n_flashes += length(flashing_octopi_idxs)
        for idx in flashing_octopi_idxs
            octopi[idx] += 1

            flash_r, flash_c = Tuple(idx)
            for dr in -1:1, dc in -1:1
                r = flash_r + dr
                c = flash_c + dc
                r ∈ row_ax && c ∈ col_ax || continue

                if octopi[r, c] < FLASH_THRESH
                    octopi[r, c] += 1
                end
            end
        end
    end

    octopi[octopi .>= FLASH_THRESH] .= 0

    return n_flashes
end

function tick(n::Int, octopi)
    octopi = copy(octopi)
    n_flashes = 0
    for _ in 1:n
        n_flashes += tick_in_place(octopi)
    end

    return (n_flashes, octopi)
end

@show tick(100, octopi)
# end::pt1[]

# tag::pt2[]
function find_simultaneous_flash_tick(octopi)
    octopi = copy(octopi)

    i = 0
    while any(octopi .!= 0)
        tick_in_place(octopi)
        i += 1
    end

    return i
end

@show find_simultaneous_flash_tick(octopi)

# end::pt2[]
# %%
end
