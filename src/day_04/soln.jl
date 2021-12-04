# tag::setup[]
import Base.Iterators: Stateful, popfirst!, flatten

"""
Gives distance from winning as the number of not-yet-drawn squares per row, col, and the
diagonals. If one of those numbers hits zero, the board has won.

Initially, the `N` rows, the `N` columns, and the two diagonals all have `N` undrawn squares
each.
"""
mutable struct BoardState{N}
    # From top to bottom
    rows::Vector{Int}

    # From left to right
    cols::Vector{Int}

    diag_nw_se::Int
    diag_ne_sw::Int

    BoardState{N}() where {N} = new{N}(fill(N, N), fill(N, N), N, N)
end

function has_won(board_state::BoardState)
    return any(r == 0 for r in board_state.rows) ||
           any(c == 0 for c in board_state.cols) ||
           board_state.diag_nw_se == 0 ||
           board_state.diag_ne_sw == 0
end

"""
Represents a board of size `N` with numbers of type `T`. Has the following members:

  - `grid`, a `Dict{T, CartesianIndex{2}}` mapping as-yet-undrawn numbers to their
    `CartesianIndex` on the board
  - `state`, a `BoardState{N}`
"""
mutable struct Board{N,T<:Integer}
    grid::Dict{T,CartesianIndex{2}}
    state::BoardState{N}

    function Board(mat::Matrix{T}) where {T}
        size(mat, 1) == size(mat, 2) || error("grid must be a square; got $(mat)")

        grid_size = size(mat, 1)
        grid = Dict((reverse(p) for p in pairs(IndexCartesian(), mat)))
        return new{grid_size,T}(grid, BoardState{grid_size}())
    end
end

function apply_number(board::Board{N}, number) where {N}
    cartesian_index = get(board.grid, number, missing)

    if cartesian_index !== missing
        (row, col) = Tuple(cartesian_index)
        board.state.rows[row] -= 1
        board.state.cols[col] -= 1

        if row == col
            board.state.diag_nw_se -= 1
        elseif row + col == N + 1
            board.state.diag_ne_sw -= 1
        end

        delete!(board.grid, number)
    end
end

has_won(board::Board) = has_won(board.state)

"""
The entire game: a `Vector` of boards and a `Vector` of numbers that will be drawn
"""
mutable struct Game{N,T<:Integer}
    boards::Vector{Board{N,T}}
    numbers::Vector{T}
end

function read_input_into_game(::Type{T}, in_file::AbstractString) where {T<:Integer}
    # Add an empty last element to the iterator so we don't have to special-case collecting
    # the rows of the last board in the event that the file doesn't already end in a newline
    lines = Stateful(strip(line) for line in flatten((eachline(in_file), ("",))))

    numbers_drawn = parse.(Int, split(popfirst!(lines), ','))

    mats = Matrix{T}[]
    this_mat_rows = Matrix{T}[]

    function add_mat(rows)
        mat = vcat(rows...)
        return push!(mats, mat)
    end

    for line in lines
        if isempty(line)
            if !isempty(this_mat_rows)
                add_mat(this_mat_rows)
            end
            this_mat_rows = []
            continue
        end

        row = reshape(parse.(Int, split(line)), (1, :))
        push!(this_mat_rows, row)
    end

    boards = Board.(mats)

    return Game(boards, numbers_drawn)
end

function get_answer_from_final_game_state(winning_number, board::Board)
    unmarked_sum = sum(keys(board.grid))
    return winning_number * unmarked_sum
end

game = read_input_into_game(Int, joinpath(@__DIR__, "input.txt"))
# end::setup[]

# tag::pt1[]
function play_until_first_winner(game::Game{N,T}) where {N,T}
    for number in game.numbers, board in game.boards
        apply_number(board, number)
        if has_won(board)
            return get_answer_from_final_game_state(number, board)
        end
    end

    return error("no boards won")
end

@show play_until_first_winner(game)
# end::pt1[]

# tag::pt2[]
function play_until_last_winner(game::Game{N,T}) where {N,T}
    n_boards_remaining = length(game.boards)
    boards_remaining = fill(true, n_boards_remaining)

    for number in game.numbers
        for ((i, hasnt_won_yet), board) in zip(enumerate(boards_remaining), game.boards)
            apply_number(board, number)
            if hasnt_won_yet && has_won(board)
                n_boards_remaining -= 1
                boards_remaining[i] = false
                if n_boards_remaining == 0
                    return get_answer_from_final_game_state(number, board)
                end
            end
        end
    end

    return error("$(n_boards_remaining) boards didn't win: boards $(findall(boards_remaining))")
end

@show play_until_last_winner(game)
# end::pt2[]
