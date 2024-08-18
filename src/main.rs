use std::io::{self, BufRead};

const PAWN: &str = "pawn__";
const ROOK: &str = "rook__";
const KNIGHT: &str = "knight";
const BISHOP: &str = "bishop";
const QUEEN: &str = "queen_";
const KING: &str = "king__";
const EMPTY: &str = "______";

fn print_state(values: &mut Vec<Vec<&str>>) {
    for x in 0..8 {
        for y in 0..8 {
            print!("{} ", values[x][y]);
        }
        println!();
    }
}

fn move_piece(values: &mut Vec<Vec<&str>>, piece: Vec<usize>, destination: Vec<usize>) {
    // piece and destination have 2 values
    if piece.len() != 2 || piece[0] >= 8 || piece[1] >= 8 {
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
        println!("Destination {}, {} is not empty.", destination[0], destination[1]);
        return;
    }

    let name = values[piece[0]][piece[1]];
    values[destination[0]][destination[1]] = name;
    values[piece[0]][piece[1]] = EMPTY;
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
    let mut values = vec![vec!["______"; 8]; 8];

    // 8 is excluded
    for i in 0..8 {
        // populate pawns
        if i == 1 || i == 6 {
            for j in 0..8 {
                values[i][j] = PAWN;
            }
        }
        if i == 0 || i == 7 {
            values[i][0] = ROOK;
            values[i][1] = KNIGHT;
            values[i][2] = BISHOP;

            values[i][3] = QUEEN;
            values[i][4] = KING;

            values[i][7] = ROOK;
            values[i][6] = KNIGHT;
            values[i][5] = BISHOP;
        }
    }

    while true {
        print_state(&mut values);
        let maybe_movement = parse_input();
        match maybe_movement {
            None => continue,
            Some(movement) => {
                move_piece(&mut values, vec![movement[0], movement[1]], vec![movement[2], movement[3]]);
            }
        }
    }
}
