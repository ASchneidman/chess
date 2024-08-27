use std::{default, io::{self, BufRead}};

const PAWN: &str = "pawn__";
const ROOK: &str = "rook__";
const KNIGHT: &str = "knight";
const BISHOP: &str = "bishop";
const QUEEN: &str = "queen_";
const KING: &str = "king__";
const EMPTY: &str = "______";

enum Side { White, Black }

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

fn print_state(values: &mut Vec<Vec<&str>>) {
    for x in 0..8 {
        for y in 0..8 {
            print!("{} ", values[x][y]);
        }
        println!();
    }
}

fn move_piece(board: &mut Vec<Vec<&str>>, mut pieces: &Vec<Piece>, requested_piece: Vec<usize>, destination: Vec<usize>) {
    // Check that the requested destination position is somewhat correct
    if destination.len() != 2 || destination[0] >= 8 || destination[1] >= 8 {
        println!("Invalid destination: {:?}", destination);
        return;
    }

    let maybe_piece = position_to_piece(&mut pieces, &requested_piece);
    match maybe_piece {
        None => {
            println!("Invalid requested piece: ({}, {})", requested_piece[0], requested_piece[1]);
            return;
        },
        Some(piece_index) => {
            // Evaluate the requested destination based on the piece type
            let mut piece = &pieces[piece_index];
            match piece.typ {
                PAWN => {
                    // If first move for the pawn, it can move one or two forward. Otherwise, only one forward. Or can capture diagonal left or right. 
                    // Check if trying to move upward/downward or diagonal
                    if piece.position[1] == destination[1] {
                        // This means it is a forward/downward move
                    } 
                },
                _ => {
                    // Won't happen
                    return;
                }
            }

        }
    }

    // piece and destination have 2 values
    /*
    if requested_piece.len() != 2 || requested_piece[0] >= 8 || requested_piece[1] >= 8 {
        println!("Invalid start piece: {:?}", piece);
        return;
    }
    if destination.len() != 2 || destination[0] >= 8 || destination[1] >= 8 {
        println!("Invalid destination: {:?}", destination);
        return;
    }
    if values[piece[0]][piece[1]] == EMPTY {
        println!("Piece {}, {} is empty.", piece[0], piece[1]);
        return;
    }
    if values[destination[0]][destination[1]] != EMPTY {
        println!("Destination {}, {} is not empty: {}.", destination[0], destination[1], values[destination[0]][destination[1]]);
        return;
    }
    
    let name = values[piece[0]][piece[1]];
    let destination_name = values[destination[0]][destination[1]];

    
    values[destination[0]][destination[1]] = name;
    values[piece[0]][piece[1]] = EMPTY;
    */
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
    // pawn, rook, knight, bishop, queen, king
    // 6 is the largest name, so everything needs to be 6
    let mut board = vec![vec!["______"; 8]; 8];
    let mut pieces: Vec<Piece<'_>> = vec![];

    // 8 is excluded
    for i in 0..8 {
        // populate pawns
        if i == 1 || i == 6 {
            for j in 0..8 {
                board[i][j] = PAWN;
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
            board[i][0] = ROOK;
            board[i][1] = KNIGHT;
            board[i][2] = BISHOP;

            board[i][3] = QUEEN;
            board[i][4] = KING;

            board[i][7] = ROOK;
            board[i][6] = KNIGHT;
            board[i][5] = BISHOP;

            for piece_typ in [(ROOK, 0), (ROOK, 7), (KNIGHT, 1), (KNIGHT, 6), (BISHOP, 2), (BISHOP, 5), (QUEEN, 3), (KING, 4)] {
                match i {
                    1 => {
                        let new_piece = Piece {
                            typ: piece_typ.0,
                            position: vec![i, piece_typ.1],
                            times_moved: 0,
                            side: Side::Black   
                        };
                        pieces.push(new_piece);
                    },
                    6 => {
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

    while true {
        print_state(&mut board);
        let maybe_movement = parse_input();
        match maybe_movement {
            None => continue,
            Some(movement) => {
                move_piece(&mut board, &pieces, vec![movement[0], movement[1]], vec![movement[2], movement[3]]);
            }
        }
    }
}
