# tag::setup[]
import Base.Iterators: Stateful, popfirst!, flatten

"""
Gives progress towards a win as the number of marked squares per row, col. If one of those
numbers hits the max (`size`), the board has won.
"""
mutable struct BoardProgress
    limit::Int

    # From top to bottom
    rows::Vector{Int}

    # From left to right
    cols::Vector{Int}

    BoardProgress(n) = new(n, fill(0, n), fill(0, n))
end

function has_won(progress::BoardProgress)
    return any(count == progress.limit for count ∈ flatten((progress.rows, progress.cols)))
end

"""
Represents a board with numbers of type `T`. Has the following members:

  - `grid`, a `Dict{T, CartesianIndex{2}}` mapping as-yet-undrawn numbers to their
    `CartesianIndex` on the board
  - `progress`, a `BoardProgress`
"""
mutable struct Board{T<:Integer}
    grid::Dict{T,CartesianIndex{2}}
    progress::BoardProgress

    function Board(mat::Matrix{T}) where {T}
        size(mat, 1) == size(mat, 2) || error("grid must be a square; got $(mat)")

        grid_size = size(mat, 1)
        grid = Dict((reverse(p) for p ∈ pairs(IndexCartesian(), mat)))
        return new{T}(grid, BoardProgress(grid_size))
    end
end

function apply_number(board::Board{T}, number::T) where {T}
    cartesian_index = get(board.grid, number, missing)

    if cartesian_index !== missing
        (row, col) = Tuple(cartesian_index)
        board.progress.rows[row] += 1
        board.progress.cols[col] += 1

        delete!(board.grid, number)
    end
end

has_won(board::Board) = has_won(board.progress)

"""
The entire game: a `Vector{Board}` of boards and a `Vector` of numbers that will be drawn
"""
mutable struct Game{T<:Integer}
    boards::Vector{Board{T}}
    numbers::Vector{T}
end

function read_input_into_game(::Type{T}, in_file::AbstractString) where {T<:Integer}
    # Add an empty last element to the iterator so we don't have to special-case collecting
    # the rows of the last board in the event that the file doesn't already end in a newline
    lines = Stateful(strip(line) for line ∈ flatten((eachline(in_file), ("",))))

    numbers_drawn = parse.(Int, split(popfirst!(lines), ','))

    boards = Board{T}[]
    this_mat_rows = Matrix{T}[]

    function add_board(rows)
        board = Board(vcat(rows...))
        return push!(boards, board)
    end

    for line ∈ lines
        if isempty(line)
            if !isempty(this_mat_rows)
                add_board(this_mat_rows)
            end
            this_mat_rows = []
            continue
        end

        row = reshape(parse.(Int, split(line)), (1, :))
        push!(this_mat_rows, row)
    end

    return Game(boards, numbers_drawn)
end

function get_answer_from_final_game_state(winning_number, board::Board)
    unmarked_sum = sum(keys(board.grid))
    return winning_number * unmarked_sum
end

game = read_input_into_game(Int, joinpath(@__DIR__, "input.txt"))
# end::setup[]

# tag::pt1[]
function play_until_first_winner(game::Game{T}) where {T}
    for number ∈ game.numbers, board ∈ game.boards
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
function play_until_last_winner(game::Game{T}) where {T}
    # The boards that haven't won yet
    ongoing_boards = Set{Int}(keys(game.boards))

    for number ∈ game.numbers, (board_index, board) ∈ pairs(game.boards)
        already_won = board_index ∉ ongoing_boards
        already_won && continue

        apply_number(board, number)
        if has_won(board)
            delete!(ongoing_boards, board_index)

            if length(ongoing_boards) == 0
                return get_answer_from_final_game_state(number, board)
            end
        end
    end

    return error("$(length(ongoing_boards)) boards didn't win: boards $(ongoing_boards)))")
end

@show play_until_last_winner(game)
# end::pt2[]
