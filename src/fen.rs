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

    // Outer-most parts of FEN notation are separated by space
    // Where pieces are
    let mut pieces_encoding: String = String::new();
    // Whose turn it is
    let mut side_encoding: String = String::new();

    // The next 4 are not yet supported by this program, but are captured for future implementation
    let mut castling_availability_encoding: String = String::new();
    let mut en_passant_encoding: String = String::new();
    let mut halfmove_clock_encoding: String = String::new();
    let mut fullmove_clock_encoding: String = String::new();

    {
        // Remove the last \n
        let mut fen_copy = fen.clone();
        if fen_copy.ends_with('\n') {
            fen_copy.remove(fen_copy.len() - 1);
        }
        // Put encodings into a vector so we can quickly populate them
        let mut encoding_vec = vec![&mut pieces_encoding, &mut side_encoding, &mut castling_availability_encoding, &mut en_passant_encoding, &mut halfmove_clock_encoding, &mut fullmove_clock_encoding];
        for (pos, encoding) in fen_copy.split(' ').enumerate() {
            encoding_vec[pos].insert_str(0, encoding);
        }
    }

    // First one is piece positions
    {
        let mut x: usize = 0;
        let mut y: usize = 0;
        for line in pieces_encoding.split('/') {
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
                } else if input == ' ' {
                    // This means we're done with the pieces
                    break;
                } else {
                    panic!("Not valid fen: {}", input);
                }
            }
            // Move to next line
            x += 1;
            y = 0;
        }
    }

    return board;
}

pub fn board_to_fen(board: &Vec<Piece>) -> &str {
    panic!("Not yet implemented");
}
