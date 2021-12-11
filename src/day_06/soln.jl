module Day6

# tag::common[]
using BenchmarkTools

function get_ages_tup(::Type{T}) where {T<:Integer}
    arr = fill(zero(T), 9)

    contents = read(joinpath(@__DIR__, "sample_input.txt"), String)
    nums_list = parse.(T, split(contents, ','))

    for num in nums_list
        arr[num + 1] += 1
    end

    return Tuple(arr)::NTuple{9,T}
end

get_pop(ages) = sum(ages)
ages = get_ages_tup(Int)
# end::common[]

# tag::array_soln[]
macro bench(obj)
    return :(@benchmark(get_pop(tick(2000, $obj)), samples = 10000, evals = 20))
end

using StaticArrays

function empty_ages_vec(::Type{T}, max_age) where {T}
    age_range = 0:max_age
    return @MVector zeros(T, length(age_range))
end
empty_ages_vec(::Type{T}) where {T} = empty_ages_vec(T, 8)

@generated function tick(ages::StaticVector{N,T}, out::StaticVector{N,T}) where {N,T}
    expr = Expr(:block)
    for (out_i, ages_i) in enumerate(Iterators.flatten((2:N, 1)))
        push!(expr.args, :(out[$out_i] = ages[$ages_i]))
    end

    push!(expr.args, :(out[7] += ages[1]))

    return expr
end

function tick(n, ages::AbstractVector{T}) where {T}
    n = convert(Unsigned, n)
    new_ages = empty_ages_vec(T)

    # Don't mutate original
    ages = copy(ages)

    for _ in 1:n
        tick(ages, new_ages)
        ages, new_ages = new_ages, ages
    end
    return ages
end

tick(ages::AbstractVector{T}) where {T} = tick(1, ages)

ages_vec = convert(MVector{length(ages),eltype(ages)}, ages)
@bench ages_vec
# end::array_soln[]

# tag::tuple_soln[]
@generated function tick(ages::NTuple{N}) where {N}
    tuple_expr = Expr(:tuple)
    for i in Iterators.flatten((2:N, 1))
        push!(tuple_expr.args, :(ages[$i]))
    end

    # We need to update the former location of ages[8]
    # 2, 3, 4, 5, 6, 7, 8 <- 7 items
    tuple_expr.args[7] = :(ages[8] + ages[1])

    return tuple_expr
end

function tick(n, ages::NTuple)
    n = convert(Unsigned, n)
    for _ in 1:n
        ages = tick(ages)
    end
    return ages
end

# end::tuple_soln[]
@bench ages

# tag::ans[]
@show get_pop(tick(80, ages))
@show get_pop(tick(256, ages))
# end::ans[]

end
