# %%
# tag::setup[]
heightmap =
    let lines = eachline(joinpath(@__DIR__, "input.txt")),
        vecs = map(line -> reshape(parse.(Int, collect(line)), (1, :)), lines),
        mat = vcat(vecs...)

        mat
    end

function is_lower_than_neighbors(mat, i::CartesianIndex, mat_axes)
    r, c = Tuple(i)
    val = mat[r, c]

    row_ax, col_ax = mat_axes

    neighbor_indices = ((r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1))
    for (nghbr_r, nghbr_c) in neighbor_indices
        if nghbr_r ∈ row_ax && nghbr_c ∈ col_ax && mat[nghbr_r, nghbr_c] <= val
            return false
        end
    end

    return true
end

is_lower_than_neighbors(mat) =
    let mat_axes = axes(mat)
        map(i -> is_lower_than_neighbors(mat, i, mat_axes), CartesianIndices(mat))
    end

risk_level(mat) = mat .+ 1
# end::setup[]

# tag::pt1[]
low_point_risks = risk_level(heightmap) .* is_lower_than_neighbors(heightmap)
@show sum(low_point_risks)
# end::pt1[]

# tag::pt2[]
function get_basin_sizes(mat::AbstractMatrix{T}) where {T}
    row_ax, col_ax = axes(mat)
    basin_sizes = Int[]
    not_yet_visited = mat .!= 9

    while (first_coords = findfirst(not_yet_visited)) !== nothing
        visited = Set{keytype(mat)}()
        coords_stack = [first_coords]

        while !isempty(coords_stack)
            coords = pop!(coords_stack)
            push!(visited, coords)
            not_yet_visited[coords] = false

            r, c = Tuple(coords)
            neighbor_indices = map(
                CartesianIndex, ((r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1))
            )

            for nghbr_idx in neighbor_indices
                nghbr_idx ∈ visited && continue

                nghbr_r, nghbr_c = Tuple(nghbr_idx)
                if nghbr_r ∈ row_ax && nghbr_c ∈ col_ax && mat[nghbr_r, nghbr_c] != 9
                    push!(coords_stack, nghbr_idx)
                end
            end
        end

        push!(basin_sizes, length(visited))
    end

    return basin_sizes
end

basin_sizes = sort(get_basin_sizes(heightmap); rev=true)
@show reduce(*, basin_sizes[1:3])
# end::pt2[]
