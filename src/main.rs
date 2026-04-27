//expand ideas...
//show legal moves b4 player moves/add ai?

//rust final
//CHECKERS

use clearscreen::clear;
use std::io;

//make colors comparable
//checking if piece selected is current player's piece
#[derive(PartialEq, Clone, Copy)]
enum TokenColor {
    Red,
    Black,
}

//had to derive wit partialEq to appease rust
#[derive(PartialEq, Clone, Copy)]
enum TokenState {
    Normal,
    King,
}

#[derive(Clone, Copy)]
struct Token {
    color: TokenColor,
    state: TokenState,
}

fn main() {
    //clearing screen at the start of every game run
    clearscreen::clear().expect("couldn't clear screen... me sorry");

    println!("GUESS WHAT... WE'RE PLAYING CHECKERS (rust edition....)\n");

    //making it so player is locked into 1 piece on their turn
    let mut forced_piece: Option<(usize, usize)> = None;

    //tracking whose turn it is
    let mut current_turn = TokenColor::Red;

    //making starting board/rendering it
    let mut board = starting_board(); //making board mutable so it can be defined again l8ter?

    loop {
        render_board(&board);

        //showing whose turn it is
        println!(
            "\nCurrent turn: {}",
            match current_turn {
                TokenColor::Red => "Red",
                TokenColor::Black => "Black",
            }
        );

        //checking for winner before asking for user input
        //Some = any value? (player/token value???)
        if let Some(winner) = check_winner(&board) {
            render_board(&board);

            println!(
                "\n🏆 Winner: {}",
                match winner {
                    TokenColor::Red => "Red",
                    TokenColor::Black => "Black",
                }
            );

            break;
        }

        //try to get moving pieces (validation later)
        //chatgpt- getting moving pieces/selecting squares
        if let Some((from, to)) = get_move() {
            if let Some(fp) = forced_piece {
                if from != fp {
                    println!("You must continue with the same piece!");
                    continue;
                }
            }

            let moved = try_move(&mut board, from, to, current_turn);

            if moved {
                maybe_promote(&mut board, to);

                if can_capture_from(&board, to) {
                    println!("You must continue capturing!");
                    forced_piece = Some(to);
                    continue;
                }

                forced_piece = None;

                current_turn = match current_turn {
                    TokenColor::Red => TokenColor::Black,
                    TokenColor::Black => TokenColor::Red,
                };
            }
        }
        //implement ai in game loop?
    }
}

const FRAME: &str = "├───┼───┼───┼───┼───┼───┼───┼───┼";
const ROW_TEMPLATE: &str = "│ A │ B │ C │ D │ E │ F │ G │ H │";
const ROW_TARGETS: [&str; 8] = ["A", "B", "C", "D", "E", "F", "G", "H"];

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

//representing board positions
fn index(row: usize, col: usize) -> usize {
    row * 8 + col //board position index = row number x 8 + the column number
}

//check if any piece can capture
fn player_must_capture(board: &Board, current_turn: TokenColor) -> bool {
    for row in 0..8 {
        for col in 0..8 {
            let idx = index(row, col);

            if let Some(piece) = board[idx] {
                if piece.color == current_turn && can_capture_from(board, (row, col)) {
                    return true;
                }
            }
        }
    }
    false
}

//capture function
fn can_capture_from(board: &Board, pos: (usize, usize)) -> bool {
    get_legal_moves(board, pos)
        .into_iter()
        .any(|to| is_capture(pos, to))
}

//getting and reading user input...
fn get_move() -> Option<((usize, usize), (usize, usize))> {
    println!("row range is 0 - 7, column range is 0 - 7");
    println!("Enter move as: from_row from_col to_row to_col");
    println!("Example: 2 3 3 4");

    let mut input = String::new();
    io::stdin().read_line(&mut input).ok()?;

    let parts: Vec<_> = input
        .split_whitespace() //splitting up whitespace between numbers
        .filter_map(|x| x.parse::<usize>().ok())
        .collect();

    if parts.len() != 4 {
        println!("Invalid input.");
        return None;
    }

    Some(((parts[0], parts[1]), (parts[2], parts[3])))
}

//moving piece
//rejects moving the wrong color
//full move validation
fn try_move(
    board: &mut Board,
    from: (usize, usize),
    to: (usize, usize),
    current_turn: TokenColor,
) -> bool {
    //enforce forced capture
    let must_capture = player_must_capture(board, current_turn);

    if must_capture && !is_capture(from, to) {
        println!("You must capture!");
        return false;
    }

    //applying move
    let from_idx = index(from.0, from.1);
    let to_idx = index(to.0, to.1);

    let piece = board[from_idx].unwrap();

    // capture
    if is_capture(from, to) {
        let mid_row = (from.0 + to.0) / 2;
        let mid_col = (from.1 + to.1) / 2;
        let mid_idx = index(mid_row, mid_col);
        board[mid_idx] = None;
    }

    board[from_idx] = None;
    board[to_idx] = Some(piece);

    true
}

//makes sure player is in bounds of game
fn in_bounds(row: usize, col: usize) -> bool {
    row < 8 && col < 8
}

//core move generator
fn get_legal_moves(board: &Board, from: (usize, usize)) -> Vec<(usize, usize)> {
    let mut moves = Vec::new();

    if !in_bounds(from.0, from.1) {
        return moves;
    }

    let piece = match board[index(from.0, from.1)] {
        Some(p) => p,
        None => return moves,
    };

    let directions: &[(isize, isize)] = match piece.state {
        TokenState::King => &[(-1, -1), (-1, 1), (1, -1), (1, 1)],
        TokenState::Normal => match piece.color {
            TokenColor::Red => &[(-1, -1), (-1, 1)],
            TokenColor::Black => &[(1, -1), (1, 1)],
        },
    };

    let r = from.0 as isize;
    let c = from.1 as isize;

    for (dr, dc) in directions {
        let nr = r + dr;
        let nc = c + dc;

        // normal move
        if nr >= 0 && nr < 8 && nc >= 0 && nc < 8 {
            let to_idx = index(nr as usize, nc as usize);
            if board[to_idx].is_none() {
                moves.push((nr as usize, nc as usize));
            }
        }

        // capture move
        let jr = r + dr * 2;
        let jc = c + dc * 2;

        if jr >= 0 && jr < 8 && jc >= 0 && jc < 8 {
            let mid_idx = index((r + dr) as usize, (c + dc) as usize);
            let jump_idx = index(jr as usize, jc as usize);

            if let Some(mid_piece) = board[mid_idx] {
                if mid_piece.color != piece.color && board[jump_idx].is_none() {
                    moves.push((jr as usize, jc as usize));
                }
            }
        }
    }

    moves
}

//helper for captures
fn is_capture(from: (usize, usize), to: (usize, usize)) -> bool {
    (to.0 as isize - from.0 as isize).abs() == 2
}

//check if player has pieces
fn has_pieces(board: &Board, color: TokenColor) -> bool {
    board.iter().any(|cell| {
        if let Some(piece) = cell {
            piece.color == color
        } else {
            false
        }
    })
}

//win detection
fn check_winner(board: &Board) -> Option<TokenColor> {
    let red_has_pieces = has_pieces(board, TokenColor::Red);
    let black_has_pieces = has_pieces(board, TokenColor::Black);

    if !red_has_pieces {
        return Some(TokenColor::Black);
    }

    if !black_has_pieces {
        return Some(TokenColor::Red);
    }

    let red_has_moves = has_any_moves(board, TokenColor::Red);
    let black_has_moves = has_any_moves(board, TokenColor::Black);

    if !red_has_moves {
        return Some(TokenColor::Black);
    }

    if !black_has_moves {
        return Some(TokenColor::Red);
    }

    None
}

//king promotion
fn maybe_promote(board: &mut Board, pos: (usize, usize)) {
    let idx = index(pos.0, pos.1);

    if let Some(mut piece) = board[idx] {
        match piece.color {
            TokenColor::Red if pos.0 == 0 => {
                piece.state = TokenState::King;
                board[idx] = Some(piece);
            }
            TokenColor::Black if pos.0 == 7 => {
                piece.state = TokenState::King;
                board[idx] = Some(piece);
            }
            _ => {}
        }
    }
}

fn has_any_moves(board: &Board, color: TokenColor) -> bool {
    let must_capture = player_must_capture(board, color);

    for row in 0..8 {
        for col in 0..8 {
            let idx = index(row, col);

            let piece = match board[idx] {
                Some(p) if p.color == color => p,
                _ => continue,
            };

            let moves = get_legal_moves(board, (row, col));

            for m in moves {
                if must_capture {
                    if is_capture((row, col), m) {
                        return true;
                    }
                } else {
                    return true;
                }
            }
        }
    }

    false
}

//direction check
fn is_forward_move(color: TokenColor, row_diff: isize) -> bool {
    match color {
        TokenColor::Red => row_diff == -1, //red pieces move up, row decreases
        TokenColor::Black => row_diff == 1, //black pieces move down, row increases
    }
}

//example validation
fn is_valid_simple_move(from: (usize, usize), to: (usize, usize)) -> bool {
    let row_diff = from.0 as isize - to.0 as isize;
    let col_diff = from.1 as isize - to.1 as isize;

    row_diff.abs() == 1 && col_diff.abs() == 1
}

//initializing board and making it empty
fn empty_board() -> Board {
    std::array::from_fn(|_| None)
}

//starting layout
fn starting_board() -> Board {
    let mut board = empty_board();

    for row in 0..8 {
        for col in 0..8 {
            //idx (variable name) = short for index?
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

    //new way
    //building rows dynamically instead of replacing letters
    let mut row_str = String::from("|");

    for col in 0..8 {
        let idx = row * 8 + col;

        let cell = match &board[idx] {
            Some(token) => token.to_string(),
            None => {
                //show light/dark squares
                if (row + col) % 2 == 1 {
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
    output += "\n"; //adding new line (for spacing purposes?)

    //adding column numbers
    println!("  0   1   2   3   4   5   6   7");

    for row in 0..8 {
        output += format!("{}\n{FRAME}\n", render_row(board, row)).as_str();
    }
    println!("{output}");
}
