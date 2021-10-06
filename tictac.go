/*
Adam Rilatt
08 June 2021
Learning Go -- Tic Tac Toe
*/

package main

import "fmt"

var (
    // Tracks the number of turns that have passed. A game of tic-tac-toe cannot
    // possible exceed 10 turns, much less 255, so a single byte can be used.
    counter uint8 = 0

    // Tracks the current player turn.
    x_turn bool = true

    // Tic-tac-toe board used for printing, squares 1 through 9 listed from top-
    // left to bottom right.
    board [9]uint8 = [9]uint8{1, 2, 3, 4, 5, 6, 7, 8, 9}

    // x and y are used as 9-bit integers to store the game state for each
    // player -- 1 for a claim by the respective player, and a 0 if not.
    // x | y represents all claimed spaces on the board, and can be used to
    // determine whether a given move is legal. Similarly, the 8 winning board
    // states can be ANDed against x or y to test for a win condition.
    x, y uint16 = 0, 0
    win_conds [8]uint16 = [8]uint16{7, 56, 448, 292, 146, 73, 273, 84}

)

func print_board([9]uint8) {

    var player string
    if x_turn {
        player = "X"
    } else {
        player = "O"
    }

    fmt.Printf("Turn %v (Player %v)\n\n", counter, player)

}

func main() {

    var winner_found bool = false

    // Start of the game loop
    for !winner_found {

        // Early escape on draw condition
        if counter == 9 {
            break
        }

        counter++

        x_turn = !x_turn

        // Start of input loop
        for {

            print_board(board)

            var user_in string
            fmt.Scanln(&user_in)

        }

    }

}
