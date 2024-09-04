use std::io::{self, BufRead};
use inline_colorization::*;

const PAWN: &str = "pawn  ";
const ROOK: &str = "rook  ";    
const KNIGHT: &str = "knight";
const BISHOP: &str = "bishop";
const QUEEN: &str = "queen ";
const KING: &str = "king  ";
const EMPTY: &str = "      ";

const PIECE_NAMES: [&str; 6] = [PAWN, ROOK, KNIGHT, BISHOP, QUEEN, KING];

#[derive(Debug, PartialEq, Eq, Clone)]
enum Side { White, Black }

#[derive(Debug)]
struct Piece {
    pub typ_index: usize,
    pub position: Vec<usize>,
    pub times_moved: usize,
    pub side: Side,
    pub captured: bool,
}

impl Piece {
    fn typ(&self) -> &str {
        return PIECE_NAMES[self.typ_index];
    }

    fn transform_typ(&mut self, new_typ: &str) {
        for i in 0..PIECE_NAMES.len() {
            if PIECE_NAMES[i] == new_typ {
                self.typ_index = i;
                return;
            }
        }
        panic!("Invalid piece type change: {}", new_typ);
    }
}

fn position_to_piece(pieces: &Vec<Piece>, position: &Vec<usize>) -> Option<usize> {
    // Discovers the piece at the requested position. Returns the address in pieces, if exists.
    // Starting with O(N), I should probably make this O(1) at some point...    
    for piece_index in 0..pieces.len() {
        if pieces[piece_index].captured {
            // Doesn't count as being in a position
            continue;
        }
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
                    if pieces[piece_index].captured {
                        // If captured, don't display
                        continue;
                    }
                    if pieces[piece_index].side == Side::Black {
                        print!("{color_bright_red}");
                    } else {
                        print!("{color_bright_white}");
                    }
                    print!("{}{color_reset}{bg_reset}", pieces[piece_index].typ());
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
            if pieces[piece_index].side != turn {
                println!("Not the turn for the requested piece. It is {:?} turn.", turn);
                return false;
            }
            match pieces[piece_index].typ() {
                PAWN => {
                    // If first move for the pawn, it can move one or two forward. Otherwise, only one forward. Or can capture diagonal left or right. 
                    // Check player side. White can only go up, Black can only go down.
                    if (pieces[piece_index].side == Side::Black && destination[0] <= pieces[piece_index].position[0]) 
                        || (pieces[piece_index].side == Side::White && destination[0] >= pieces[piece_index].position[0]) {
                        println!("Requested moving pawn wrong direction, so incorrect. Piece {:?}, Requested destination {:?}", pieces[piece_index].position, destination);
                        return false;
                    }
                    // Check if trying to move down or diagonal
                    if destination[1] == pieces[piece_index].position[1] {
                        // Trying to move down
                        // If first move, it can move one or two. Otherwise, only one
                        let amt_wanting_to_move: usize;
                        if pieces[piece_index].side == Side::Black {
                            amt_wanting_to_move = destination[0] - pieces[piece_index].position[0];
                        } else {
                            amt_wanting_to_move = pieces[piece_index].position[0] - destination[0];
                        }
                        let mut good_movement: bool = false;
                        if amt_wanting_to_move == 2 && pieces[piece_index].times_moved == 0 {
                            good_movement = true;
                        } else if amt_wanting_to_move == 1 {
                            good_movement = true;
                        } else {
                            good_movement = false;
                        }
                        if good_movement == false {
                            // Not allowed...
                            println!("Bad requested moving pawn. Piece {:?}, Requested destination {:?}", pieces[piece_index].position, destination);
                            return false;
                        }
                        // Allowed!
                        // Check destination is empty
                        if maybe_piece_at_destination.is_none() {
                            // If moving by 2, check that we aren't jumping over a piece
                            if amt_wanting_to_move == 2 {
                                let mut position_of_neighboring_piece: Vec<usize> = vec![];
                                if pieces[piece_index].side == Side::White {
                                    // Extra piece would be above
                                    position_of_neighboring_piece.push(pieces[piece_index].position[0] - 1);
                                } else {
                                    // Extra piece would be below
                                    position_of_neighboring_piece.push(pieces[piece_index].position[0] + 1);
                                }
                                position_of_neighboring_piece.push(pieces[piece_index].position[1]);
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
                        println!("Bad requested moving pawn. Piece {:?}, Requested destination {:?}", pieces[piece_index].position, destination);
                        return false; 
                    }
                    // Maybe trying to move diagonal
                    // For Black, must be down. For White, must be up.
                    // For Black and White, must be either right or left by one
                    // First check up/down by one
                    if (pieces[piece_index].side == Side::Black && pieces[piece_index].position[0] + 1 != destination[0]) 
                        || (pieces[piece_index].side == Side::White && pieces[piece_index].position[0] - 1 != destination[0]) {
                        // Bad
                        println!("Must move pawn up or down depending on side. Piece {:?}, Requested destination {:?}", pieces[piece_index].position, destination);
                        return false;
                    }
                    // Now check moving right/left
                    if !((pieces[piece_index].position[1] > 0 && pieces[piece_index].position[1] - 1 == destination[1]) 
                        || pieces[piece_index].position[1] + 1 == destination[1]) {
                        // Bad
                        println!("Must move pawn right/left or straight up/down. Piece {:?}, Requested destination: {:?}", pieces[piece_index].position, destination);
                        return false;
                    }
                    // We may take over a piece
                    match maybe_piece_at_destination {
                        None => {
                            // Just move
                            move_piece_to_dest(piece_index, &mut pieces, &destination);
                        },
                        Some(destination_piece_index) => {
                            // Capture the piece
                            pieces[destination_piece_index].captured = true;
                            move_piece_to_dest(piece_index, &mut pieces, &destination);
                        }
                    }
                    // If at the end, gets promoted to queen. Has been moved, so check current position.
                    if pieces[piece_index].position[0] == 0 || pieces[piece_index].position[0] == 7 {
                        pieces[piece_index].transform_typ(QUEEN);
                    }
                    return true;
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
    let mut pieces: Vec<Piece> = vec![];

    for i in 0..8 {
        // populate pawns
        if i == 1 || i == 6 {
            for j in 0..8 {
                match i {
                    1 => {
                        let mut new_piece = Piece {
                            typ_index: 0,
                            position: vec![i, j],
                            times_moved: 0,
                            side: Side::Black,
                            captured: false
                        };
                        // Overwrites typ_index
                        new_piece.transform_typ(PAWN);
                        pieces.push(new_piece);
                    },
                    6 => {
                        let mut new_piece = Piece {
                            typ_index: 0,
                            position: vec![i, j],
                            times_moved: 0,
                            side: Side::White,
                            captured: false
                        };
                        new_piece.transform_typ(PAWN);
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
                        let mut new_piece = Piece {
                            typ_index: 0,
                            position: vec![i, piece_typ.1],
                            times_moved: 0,
                            side: Side::Black,
                            captured: false   
                        };
                        new_piece.transform_typ(piece_typ.0);
                        pieces.push(new_piece);
                    },
                    7 => {
                        let mut new_piece = Piece {
                            typ_index: 0,
                            position: vec![i, piece_typ.1],
                            times_moved: 0,
                            side: Side::White,
                            captured: false,   
                        };
                        new_piece.transform_typ(piece_typ.0);
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
        println!("{:?} turn.", turn);
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
