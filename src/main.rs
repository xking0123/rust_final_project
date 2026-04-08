//rust final
//CHECKERS

use std::{iter::zip, str};

fn main() {
    println!("GUESS WHAT... WE'RE PLAYING CHECKERS (rust edition....)");

    //try making starting board/rendering it
    
    //try to get moving pieces (validation later)
}

const FRAME: &str = "├───┼───┼───┼───┼───┼───┼───┼───┼";
const ROW_TEMPLATE : &str = "│ A │ B │ C │ D │ E │ F │ G │ H │";
const ROW_TARGETS: [&str; 8] = ["A", "B", "C", "D", "E", "F", "G", "H"];

enum TokenColor{
    Red,
    Black,
}

enum TokenState{
    Normal,
    King,
}

struct Token{
    color: TokenColor,
    state: TokenState,
}

impl Token {
    fn to_string(&self) -> String {
        todo!()
    }
}

type Board = [Option<Token>; 64];

fn render_Row(board: &Board, row: usize) -> String {
    //
    let row = &board[(row * 8)..(8 + row * 8)];
    let tokens = row.iter().map(|space| match space {
        Some(token) => token.to_string(),
        None => " ".to_string(),
    });

    //making template for 1 row string so that strings (board spaces? can be replaced with tokens)
    let mut row = ROW_TEMPLATE.to_string();
    for (target, token) in ROW_TARGETS.iter().zip(tokens) {
        row = row.replace(target, &token);
    }
    
    row
}

fn render_Board(board: &Board){
    let mut output = String::new();
    output += FRAME;
    output += "\n";

    for row in 0..8{
        output += format!("{}\n{FRAME}\n", render_Row(board, row)).as_str();
    }
    println!("{output}");
}
