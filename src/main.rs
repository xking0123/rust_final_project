//rust final
//CHECKERS

use std::{fmt::format, iter::zip, str};

fn main() {
    println!("GUESS WHAT... WE'RE PLAYING CHECKERS (rust edition....)");

    //try making starting board/rendering it
    let board = starting_board();
    render_board(&board);

    //try to get moving pieces (validation later)
    //chatgpt- getting moving pieces/selecting squares
}

const FRAME: &str = "├───┼───┼───┼───┼───┼───┼───┼───┼";
const ROW_TEMPLATE: &str = "│ A │ B │ C │ D │ E │ F │ G │ H │";
const ROW_TARGETS: [&str; 8] = ["A", "B", "C", "D", "E", "F", "G", "H"];

enum TokenColor {
    Red,
    Black,
}

enum TokenState {
    Normal,
    King,
}

struct Token {
    color: TokenColor,
    state: TokenState,
}

//reutrning a single character to keep it simple (giving tokens visual representation)
//lowercase = normal, uppercase = king
impl Token {
    fn to_string(&self) -> String {
        match (&self.color, &self.state) {
            (TokenColor::Red, TokenState::Normal) => "r".to_string(),
            (TokenColor::Red, TokenState::King) => "R".to_string(),
            (TokenColor::Black, TokenState::Normal) => "b".to_string(),
            (TokenColor::Black, TokenState::King) => "B".to_string(),
        }
    }
}

//defining board (8x8)
type Board = [Option<Token>; 64];

//initializing board and making it empty
fn empty_board() -> Board {
    std::array::from_fn(|_| None)
}

//starting layout
fn starting_board() -> Board {
    let mut board = empty_board();

    for row in 0..8 {
        for col in 0..8 {
            //idx = index?
            let idx = row * 8 + col;

            //only dark squares
            if (row + col) % 2 == 1 {
                if row < 3 {
                    board[idx] = Some(Token {
                        color: TokenColor::Black,
                        state: TokenState::Normal,
                    });
                } else if row > 4 {
                    board[idx] = Some(Token {
                        color: TokenColor::Red,
                        state: TokenState::Normal,
                    });
                }
            }
        }
    }

    board
}

fn render_row(board: &Board, row: usize) -> String {
    //old way
    //replacing letters...?
    // //making all the rows print out emtpy strings?
    // let row = &board[(row * 8)..(8 + row * 8)];
    // let tokens = row.iter().map(|space| match space {
    //     Some(token) => token.to_string(),
    //     None => " ".to_string(),
    // });

    // //making template for 1 row string so that strings (board spaces? can be replaced with tokens)
    // let mut row = ROW_TEMPLATE.to_string();
    // for (target, token) in ROW_TARGETS.iter().zip(tokens) {
    //     row = row.replace(target, &token);
    // }

    // row

    //building rows dynamically instead of replacing letters
    //new way
    let mut row_str = String::from("|");

    for col in 0..8{
        let idx = row * 8 + col;

        let cell = match &board[idx] {
            Some(token) => token.to_string(),
            None => {
                //show light/dark squares
                if(row + col) % 2 == 1{
                    ".".to_string() //dark square
                } else {
                    " ".to_string() //light square
                }
            }
        };

        row_str += &format!(" {} |", cell);
    }

    row_str
}

fn render_board(board: &Board) {
    //setting up general outline for board, with the frame and then rendering each row afterwards
    let mut output = String::new();
    output += FRAME;
    output += "\n";

    for row in 0..8 {
        output += format!("{}\n{FRAME}\n", render_row(board, row)).as_str();
    }
    println!("{output}");
}
