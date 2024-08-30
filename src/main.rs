use std::io::{self, BufRead};
use inline_colorization::*;

const PAWN: &str = "pawn  ";
const ROOK: &str = "rook  ";
const KNIGHT: &str = "knight";
const BISHOP: &str = "bishop";
const QUEEN: &str = "queen ";
const KING: &str = "king  ";
const EMPTY: &str = "      ";

#[derive(Debug, PartialEq, Eq, Clone)]
enum Side { White, Black }

#[derive(Debug)]
struct Piece<'a> {
    pub typ: &'a str,
    pub position: Vec<usize>,
    pub times_moved: usize,
    pub side: Side,
}

fn position_to_piece(pieces: &Vec<Piece>, position: &Vec<usize>) -> Option<usize> {
    // Discovers the piece at the requested position. Returns the address in pieces, if exists.
    // Starting with O(N), I should probably make this O(1) at some point...    
    for piece_index in 0..pieces.len() {
        if pieces[piece_index].position[0] == position[0] && pieces[piece_index].position[1] == position[1] {
            return Some(piece_index);
        }
    }
    return None;
}

fn print_board(pieces: &Vec<Piece>) {
    print!("{style_bold}");
    println!("   0     1     2     3     4     5     6     7");
    for x in 0..8 {
        print!("{}  ", x);
        for y in 0..8 {
            let pos: Vec<usize> = vec![x, y];
            if x % 2 == 0 && y % 2 == 0 {
                print!("{bg_white}");
            } else if x % 2 == 1 && y % 2 == 1 {
                print!("{bg_white}");
            }
            match position_to_piece(&pieces, &pos) {
                None => print!("{}{bg_reset}", EMPTY),
                Some(piece_index) => {
                    if pieces[piece_index].side == Side::Black {
                        print!("{color_bright_red}");
                    } else {
                        print!("{color_bright_white}");
                    }
                    print!("{}{color_reset}{bg_reset}", pieces[piece_index].typ);
                }
            }
        }
        println!();
    }
}

fn move_piece(mut pieces: &mut Vec<Piece>, requested_piece: Vec<usize>, destination: Vec<usize>, turn: Side) -> bool {
    fn move_piece_to_dest(piece_index: usize, pieces: &mut Vec<Piece>, destination: &Vec<usize>) {
        pieces[piece_index].position = vec![destination[0], destination[1]];
        pieces[piece_index].times_moved += 1;
    }

    // Check that the requested destination position is somewhat correct
    if destination.len() != 2 || destination[0] >= 8 || destination[1] >= 8 {
        println!("Invalid destination: {:?}", destination);
        return false;
    }

    let maybe_piece = position_to_piece(&pieces, &requested_piece);
    // Could be EMPTY (None), or another piece
    let maybe_piece_at_destination = position_to_piece(&pieces, &destination);
    match maybe_piece {
        None => {
            println!("Invalid requested piece: ({}, {})", requested_piece[0], requested_piece[1]);
            return false;
        },
        Some(piece_index) => {
            // Evaluate the requested destination based on the piece type
            let piece = &pieces[piece_index];
            if piece.side != turn {
                println!("Not the turn for the requested piece. It is {:?} turn.", turn);
                return false;
            }
            match piece.typ {
                PAWN => {
                    // If first move for the pawn, it can move one or two forward. Otherwise, only one forward. Or can capture diagonal left or right. 
                    // Check player side. White can only go up, Black can only go down.
                    if (piece.side == Side::Black && destination[0] <= piece.position[0]) || (piece.side == Side::White && destination[0] >= piece.position[0]) {
                        println!("Requested moving pawn wrong direction, so incorrect. Piece {:?}, Requested destination {:?}", piece.position, destination);
                        return false;
                    }
                    // Check if trying to move down or diagonal
                    if destination[1] == piece.position[1] {
                        // Trying to move down
                        // If first move, it can move one or two. Otherwise, only one
                        // let amt_wanting_to_move = destination[0] - piece.position[0];
                        let amt_wanting_to_move: usize;
                        if piece.side == Side::Black {
                            amt_wanting_to_move = destination[0] - piece.position[0];
                        } else {
                            amt_wanting_to_move = piece.position[0] - destination[0];
                        }
                        let mut good_movement: bool = false;
                        if amt_wanting_to_move == 2 && piece.times_moved == 0 {
                            good_movement = true;
                        } else if amt_wanting_to_move == 1 {
                            good_movement = true;
                        } else {
                            good_movement = false;
                        }
                        if good_movement == false {
                            // Not allowed...
                            println!("Bad requested moving pawn. Piece {:?}, Requested destination {:?}", piece.position, destination);
                            return false;
                        }
                        // Allowed!
                        // Check destination is empty
                        if maybe_piece_at_destination.is_none() {
                            // If moving by 2, check that we aren't jumping over a piece
                            if amt_wanting_to_move == 2 {
                                let mut position_of_neighboring_piece: Vec<usize> = vec![];
                                if piece.side == Side::White {
                                    // Extra piece would be above
                                    position_of_neighboring_piece.push(piece.position[0] + 1);
                                } else {
                                    // Extra piece would be below
                                    position_of_neighboring_piece.push(piece.position[0] - 1);
                                }
                                position_of_neighboring_piece.push(piece.position[1]);
                                let maybe_piece = position_to_piece(&pieces, &position_of_neighboring_piece);
                                if maybe_piece.is_some() {
                                    // Not allowed!
                                    println!("Cannot move pawn over piece.");
                                    return false;
                                }
                            }
                            // Good to go!
                            move_piece_to_dest(piece_index, &mut pieces, &destination);
                            return true;
                        }
                        println!("Bad requested moving pawn. Piece {:?}, Requested destination {:?}", piece.position, destination);
                        return false; 
                    }
                },
                _ => {
                    // Won't happen
                    return false;
                }
            }
        }
    }
    return false;
}

fn parse_input() -> Option<Vec<usize>> {
    let mut numbers = String::new();

    let raw = io::stdin()
        .read_line(&mut numbers)
        .ok();
    let mut result: Vec<usize> = vec![];
    match raw {
        None => return None,
        Some(_) => {
            let mut iter = numbers.split_whitespace();
            for _ in 0..4 {
                let x = iter.next();
                match x {
                    None => return None,
                    Some(y) => {
                        let maybe_usize: Option<usize> = y.parse().ok();
                        match maybe_usize {
                            None => return None,
                            Some(val) => { 
                                result.push(val);
                            }
                        }
                    }
                }
            }
        },
    }
    return Some(result);
}

fn main() {
    let mut pieces: Vec<Piece<'_>> = vec![];

    for i in 0..8 {
        // populate pawns
        if i == 1 || i == 6 {
            for j in 0..8 {
                match i {
                    1 => {
                        let new_piece = Piece {
                            typ: PAWN,
                            position: vec![i, j],
                            times_moved: 0,
                            side: Side::Black   
                        };
                        pieces.push(new_piece);
                    },
                    6 => {
                        let new_piece = Piece {
                            typ: PAWN,
                            position: vec![i, j],
                            times_moved: 0,
                            side: Side::White   
                        };
                        pieces.push(new_piece);
                    },
                    _ => {
                        // not gonna happen
                    }
                }
            }
        }
        if i == 0 || i == 7 {
            for piece_typ in [(ROOK, 0), (ROOK, 7), (KNIGHT, 1), (KNIGHT, 6), (BISHOP, 2), (BISHOP, 5), (QUEEN, 3), (KING, 4)] {
                match i {
                    0 => {
                        let new_piece = Piece {
                            typ: piece_typ.0,
                            position: vec![i, piece_typ.1],
                            times_moved: 0,
                            side: Side::Black   
                        };
                        pieces.push(new_piece);
                    },
                    7 => {
                        let new_piece = Piece {
                            typ: piece_typ.0,
                            position: vec![i, piece_typ.1],
                            times_moved: 0,
                            side: Side::White   
                        };
                        pieces.push(new_piece);
                    }
                    _ => {
                        // not gonna happen
                    }
                }
            }
        }
    }

    let mut turn = Side::White;
    loop {
        print_board(&pieces);
        let maybe_movement = parse_input();
        match maybe_movement {
            None => continue,
            Some(movement) => {
                if move_piece(&mut pieces, vec![movement[0], movement[1]], vec![movement[2], movement[3]], turn.clone()) == true {
                    // Move was successful, switch turn to other player
                    if turn == Side::White {
                        turn = Side::Black;
                    } else {
                        turn = Side::White;
                    }
                }
            }
        }
    }
}
