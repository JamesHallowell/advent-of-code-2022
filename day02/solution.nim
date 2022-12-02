import std/strutils
import std/sequtils

type
    Move = enum
        Rock, Paper, Scissors

type
    Outcome = enum
        Win, Lose, Draw

type
    Response = enum
        X, Y, Z

type
    Input = object
        opponent: Move
        response: Response

type
    Round = object
        opponent: Move
        player: Move


proc parse_move(input: string): Move =
    case input
        of "A":
            return Move.Rock
        of "B":
            return Move.Paper
        of "C":
            return Move.Scissors
        else:
            raise newException(ValueError, "Invalid move: " & input)


proc parse_response(input: string): Response =
    case input
        of "X":
            return Response.X
        of "Y":
            return Response.Y
        of "Z":
            return Response.Z
        else:
            raise newException(ValueError, "Invalid suggestion: " & input)

proc parse_input(input: string): seq[Input] =
    result = input.splitLines().map(proc (line: string): Input =
        let input = line.split(" ")
        let opponent = parse_move(input[0])
        let response = parse_response(input[1])
        Input(opponent: opponent, response: response)
    )

proc response_as_suggested_move(input: Input): Round =
    case input.response
        of Response.X:
            return Round(opponent: input.opponent, player: Move.Rock)
        of Response.Y:
            return Round(opponent: input.opponent, player: Move.Paper)
        of Response.Z:
            return Round(opponent: input.opponent, player: Move.Scissors)

proc response_as_suggested_outcome(input: Input): Round =
    case input.response
        of Response.X: # force a loss
            case input.opponent
                of Move.Rock:
                    return Round(opponent: input.opponent, player: Move.Scissors)
                of Move.Paper:
                    return Round(opponent: input.opponent, player: Move.Rock)
                of Move.Scissors:
                    return Round(opponent: input.opponent, player: Move.Paper)
        of Response.Y: # force a draw
            return Round(opponent: input.opponent, player: input.opponent)
        of Response.Z: # force a win
            case input.opponent
                of Move.Rock:
                    return Round(opponent: input.opponent, player: Move.Paper)
                of Move.Paper:
                    return Round(opponent: input.opponent, player: Move.Scissors)
                of Move.Scissors:
                    return Round(opponent: input.opponent, player: Move.Rock)

proc outcome(round: Round): Outcome =
    if round.opponent == Move.Rock:
        if round.player == Move.Rock:
            return Outcome.Draw
        elif round.player == Move.Paper:
            return Outcome.Win
        elif round.player == Move.Scissors:
            return Outcome.Lose
    elif round.opponent == Move.Paper:
        if round.player == Move.Rock:
            return Outcome.Lose
        elif round.player == Move.Paper:
            return Outcome.Draw
        elif round.player == Move.Scissors:
            return Outcome.Win
    elif round.opponent == Move.Scissors:
        if round.player == Move.Rock:
            return Outcome.Win
        elif round.player == Move.Paper:
            return Outcome.Lose
        elif round.player == Move.Scissors:
            return Outcome.Draw

proc score(round: Round): int =
    var score = 0
    
    case outcome(round)
        of Outcome.Win:
            score += 6
        of Outcome.Lose:
            score += 0
        of Outcome.Draw:
            score += 3

    case round.player
        of Move.Rock:
            score += 1
        of Move.Paper:
            score += 2
        of Move.Scissors:
            score += 3

    return score

const input = staticRead("input.txt")
let rounds = parse_input(input)

let part_1 = rounds.map(response_as_suggested_move).map(score).foldl(a + b, 0)
echo part_1

let part_2 = rounds.map(response_as_suggested_outcome).map(score).foldl(a + b, 0)
echo part_2