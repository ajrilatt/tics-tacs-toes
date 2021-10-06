// Adam Rilatt
// 28 May 2021
// Learning Rust -- Tic Tac Toe

use std::io;
use crossterm::{terminal, ExecutableCommand, Result};

fn get_input() -> String {

    let mut str_input = String::new();

    io::stdin()
        .read_line(&mut str_input)
        .expect("Failed to read line");

    // Remove trailing newline and return
    str_input.trim().to_string()

}

fn print_board(b: [char; 9], t: u8, p: char) {

    // Prints a control character that clears the terminal window
    //print!("{}[2J", 27 as char);
    //print!("{esc}c", esc = 27 as char);
    io::stdout().execute(terminal::Clear(terminal::ClearType::All));

    // Print game header
    println!("Turn {} (Player {})\n", t, p);

    // Print the actual board, row-by-row... I don't know a far better way! -\(:j)/-
    println!(" {} | {} | {} ", b[0], b[1], b[2]);
    println!("-----------");
    println!(" {} | {} | {} ", b[3], b[4], b[5]);
    println!("-----------");
    println!(" {} | {} | {} ", b[6], b[7], b[8]);

}

fn main() -> Result<()> {

    // Tracks the number of turns that have passed.
    // A game of tic-ta-toe cannot possibly exceed 10 turns, much less 255,
    // so a single byte is used.
    let mut counter: u8 = 0;

    // Tracks the player currently making a move. Values are 'X' or 'O'.
    let mut player: char = 'X';

    // Tic-tac-toe board used for printing, squares 1 through 9 listed from top left to bottom right.
    // The board is initialized with period characters as placeholders, and is filled with
    // the digits 1 through 9 in the following loop.
    let mut board: [char; 9] = ['.'; 9];
    for i in 1..10 {

        // 48 base 10 is the start of the UTF-8 digits
        board[i - 1] = (48u8 + i as u8) as char;

    }

    // We're going to try a fun new way of evaluating our board state.
    // Original idea can be found here: https://stackoverflow.com/a/33456912
    // Here's my breakdown of what happens:

    // Create a 9-bit integer to store X's board state, and another for O's board state.
    // Each bit represents a board space -- 1 if X or O have claimed the space, respectively,
    // and 0 if they have not. Create similarly-formatted 9-bit binary numbers that store
    // the winning combinations (111000000, 000111000, etc.). If a board state ANDed with
    // a winning combination is equal to said winning combination, then we have a win condition.

    let mut x: u16 = 0;
    let mut o: u16 = 0;

    let win_conditions: [u16; 8] = [
        // rows
        0b0_0000_0111,
        0b0_0011_1000,
        0b1_1100_0000,
        // columns
        0b1_0010_0100,
        0b0_1001_0010,
        0b0_0100_1001,
        // diagonals
        0b1_0001_0001,
        0b0_0101_0100
    ];

    // Start of the game loop
    let winner = loop {

        counter += 1;

        // Early escape on a draw condition
        if counter == 10 { break "draw." }

        player = if counter % 2 == 0 { 'O' } else { 'X' };

        // Start of input loop
        let selection: u8 = loop {

            print_board(board, counter, player);

            let player_selection = get_input();

            // Attempt to read input as an integer, and attempt to match it to a square
            let square_selection: u8 = match player_selection.parse() {

                // Note that the num < 10 check requires the short-circuit evaluation
                // of &&. If needed, this may be replaced with nested if statements.
                // If neither of these are present, values 9 < num < 256 will cause
                // the right bitshift to overflow.
                Ok(num) => if (num < 10) && ( (x | o) & (0x200u16 >> num) == 0 ) {
                               num
                           }
                           else {
                               continue;
                           },
                Err(_) => { continue; },

            };

            // Successful square placement! Exit loop with square value
            break square_selection

        };

        // Claim the selected square and check for win conditions
        board[(selection as usize) - 1] = player;

        let mut round_winner = "None";
        if player == 'X' {

            x |= 0x200u16 >> selection;
            for condition in win_conditions.iter() {
                if x & *condition == *condition {
                    round_winner = "X win!";
                }
            }

        }
        else {

            o |= 0x200u16 >> selection;
            for condition in win_conditions.iter() {
                if o & *condition == *condition {
                    round_winner = "O win!";
                }
            }

        }

        if round_winner == "None" { continue; }

        break round_winner

    };

    // X win, O win, or draw has been reached

    print_board(board, counter, player);
    println!("The result of the match was a {}", winner);

    Ok(())

}
