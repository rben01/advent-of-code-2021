# tag::setup[]
elems = vcat(
    (
        begin
            bit_vec = collect(strip(line)) .== '1'
            bit_row = reshape(bit_vec, (1, :))
            bit_row
        end for line ∈ eachline(joinpath(@__DIR__, "input.txt"))
    )...,
)

(n_lines, line_length) = size(elems)

function to_decimal(binary_digit_vector_msbf::AbstractVector)
    n_digits = length(binary_digit_vector_msbf)
    pow_2s = 2 .^ ((n_digits - 1):-1:0)
    return sum(pow_2s .* binary_digit_vector_msbf)
end
# end::setup[]

# tag::pt1[]
n_ones = vec(sum(elems; dims=1))
n_zeros = n_lines .- n_ones
col_has_more_ones_than_zeros = n_ones .> n_zeros

gamma_rate = to_decimal(col_has_more_ones_than_zeros)
epsilon_rate = (2^line_length - 1) - gamma_rate

@show gamma_rate * epsilon_rate
# end::pt1[]

# tag::pt2[]
function value_of_line_chosen_by_criterion(comparison_predicate)
    candidates = fill(true, n_lines)
    for i = 1:line_length
        n_candidates_remaining = sum(candidates)
        if n_candidates_remaining == 1
            break
        end

        digits = elems[candidates, i]
        most_common_digit = comparison_predicate(2 * sum(digits), n_candidates_remaining)
        candidates .&= elems[:, i] .== most_common_digit
    end

    index = findfirst(candidates)
    line = elems[index, :]
    value = to_decimal(line)

    return value
end

(oxy_rate, co2_rate) = map(value_of_line_chosen_by_criterion, (>=, <))
@show oxy_rate * co2_rate

# end::pt2[]
