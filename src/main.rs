//rust final
//CHECKERS

use std::str;

fn main() {
    println!("GUESS WHAT... WE'RE PLAYING CHECKERS (rust edition....)");

    
}

const FRAME: &str = "├───┼───┼───┼───┼───┼───┼───┼───┼";
const ROW_TEMPLATE : &str = "│ A │ B │ C │ D │ E │ F │ G │ H │";

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

type Board = [Option<Token>; 64];

fn renderRow(board: &Board, row: usize) -> String{
    format!("{}", ROW_TEMPLATE
        .replace("A", )
        .replace("B", )
        .replace("C", )
        .replace("D", )
        .replace("E", )
        .replace("F", )
        .replace("G", )
        .replace("H", ))
        //.replace("A", getTokenChar(board[8 * row + 0]))
        //getTokenChar????
}

fn renderBoard(board: &Board){
    let mut output = String::new();
    output += FRAME + "\n";

    for row in 0..8{
        output += renderRow(board) + "\n" + FRAME + "\n"
    }

    println!("{output}");
}