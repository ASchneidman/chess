use std::collections::HashMap;

use crate::piece::{self, Piece};

const FEN_SPACE: [char; 8] = ['1', '2', '3', '4', '5', '6', '7', '8'];

pub fn fen_to_board(fen: &String) -> Vec<Piece> {
    let mut board: Vec<Piece> = vec![];

    let mut fen_black_to_index: HashMap<char, usize> = HashMap::new();
    let mut fen_white_to_index: HashMap<char, usize> = HashMap::new();
    fen_black_to_index.insert('p', 0);
    fen_black_to_index.insert('r', 1);
    fen_black_to_index.insert('n', 2);
    fen_black_to_index.insert('b', 3);
    fen_black_to_index.insert('q', 4);
    fen_black_to_index.insert('k', 5);

    fen_white_to_index.insert('P', 0);
    fen_white_to_index.insert('R', 1);
    fen_white_to_index.insert('N', 2);
    fen_white_to_index.insert('B', 3);
    fen_white_to_index.insert('Q', 4);
    fen_white_to_index.insert('K', 5);

    let mut x: usize = 0;
    let mut y: usize = 0;
    for line in fen.split('/') {
        for input in line.chars() {
            if fen_black_to_index.contains_key(&input) {
                board.push(piece::Piece {
                    typ_index: fen_black_to_index[&input],
                    position: (x, y),
                    times_moved: 0, // we don't actually know the times moved...
                    side: piece::Side::Black,
                    captured: false,
                });
                
                // Move over to the right by one
                y += 1;
            } else if fen_white_to_index.contains_key(&input) {
                board.push(piece::Piece {
                    typ_index: fen_white_to_index[&input],
                    position: (x, y),
                    times_moved: 0, // we don't actually know the times moved...
                    side: piece::Side::White,
                    captured: false,
                });

                // Move over to the right by one
                y += 1;
            } else if FEN_SPACE.contains(&input) {
                // Move over to the right by int(input)
                let z = (input.to_string()).parse::<usize>().unwrap();
                y += z;
            } else if input == '\n' {
                // done!
                break;
            } else {
                panic!("Not valid fen: {}", input);
            }
        }
        // Move to next line
        x += 1;
        y = 0;
    }

    return board;
}

pub fn board_to_fen(board: &Vec<Piece>) -> &str {
    panic!("Not yet implemented");
}
