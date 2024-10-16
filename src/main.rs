use std::{collections::HashMap, io::{self}};
use inline_colorization::*;
mod piece;
mod fen;

fn print_board(pieces: &Vec<piece::Piece>) {
    let piece_positions = vec![8, 7, 6, 5, 4, 3, 2, 1];
    print!("{style_bold}");
    for x in 0..8 {
        print!("{:}  ", piece_positions[x]);
        for y in 0..8 {
            if x % 2 == 0 && y % 2 == 0 {
                print!("{bg_white}");
            } else if x % 2 == 1 && y % 2 == 1 {
                print!("{bg_white}");
            }
            match piece::position_to_piece(&pieces, (x, y)) {
                None => print!("{}{bg_reset}", piece::EMPTY),
                Some(piece_index) => {
                    if pieces[piece_index].captured {
                        // If captured, don't display
                        continue;
                    }
                    if pieces[piece_index].side == piece::Side::Black {
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
    println!("   a     b     c     d     e     f     g     h");
}

fn move_piece(mut pieces: &mut Vec<piece::Piece>, requested_piece: (usize, usize), destination: (usize, usize), turn: piece::Side) -> bool {
    fn move_piece_to_dest(piece_index: usize, pieces: &mut Vec<piece::Piece>, destination: (usize, usize)) {
        pieces[piece_index].position = (destination.0, destination.1);
        pieces[piece_index].times_moved += 1;
    }

    // Check that the requested destination position is somewhat correct
    if destination.0 >= 8 || destination.1 >= 8 {
        println!("Invalid destination: {:?}", destination);
        return false;
    }

    let maybe_piece = piece::position_to_piece(&pieces, requested_piece);
    // Could be EMPTY (None), or another piece
    let maybe_piece_at_destination = piece::position_to_piece(&pieces, destination);
    match maybe_piece {
        None => {
            println!("Invalid requested piece: ({}, {})", requested_piece.0, requested_piece.1);
            return false;
        },
        Some(piece_index) => {
            // Evaluate the requested destination based on the piece type
            if pieces[piece_index].side != turn {
                println!("Not the turn for the requested piece. It is {:?} turn.", turn);
                return false;
            }

            let allowed_positions = pieces[piece_index].valid_movements(&pieces);
            println!("{:?}", allowed_positions);
            for allowed_pos in allowed_positions {
                if allowed_pos.0 == destination.0 && allowed_pos.1 == destination.1 {
                    // Check if there's a enemy piece there
                    match maybe_piece_at_destination {
                        Some(other_piece_index) => {
                            pieces[other_piece_index].captured = true;
                        },
                        None => {},
                    }
                    move_piece_to_dest(piece_index, &mut pieces, destination);
                    // If pawn reaches the end, turns into a piece::QUEEN
                    if pieces[piece_index].typ() == piece::PAWN && (pieces[piece_index].position.0 == 0 || pieces[piece_index].position.0 == 7) {
                        pieces[piece_index].transform_typ(piece::QUEEN);           
                    }
                    return true;
                }
            }
        }
    }
    return false;
}

fn parse_movement() -> Option<Vec<usize>> {
    let mut numbers = String::new();

    let mut letters_to_numbers: HashMap<char, usize> = HashMap::new();
    letters_to_numbers.insert('a', 0);
    letters_to_numbers.insert('b', 1);
    letters_to_numbers.insert('c', 2);
    letters_to_numbers.insert('d', 3);
    letters_to_numbers.insert('e', 4);
    letters_to_numbers.insert('f', 5);
    letters_to_numbers.insert('g', 6);
    letters_to_numbers.insert('h', 7);

    let mut numbers_to_numbers: HashMap<char, usize> = HashMap::new();
    numbers_to_numbers.insert('1', 7);
    numbers_to_numbers.insert('2', 6);
    numbers_to_numbers.insert('3', 5);
    numbers_to_numbers.insert('4', 4);
    numbers_to_numbers.insert('5', 3);
    numbers_to_numbers.insert('6', 2);
    numbers_to_numbers.insert('7', 1);
    numbers_to_numbers.insert('8', 0);


    let mut result: Vec<usize> = vec![0, 0, 0, 0];
    match io::stdin().read_line(&mut numbers).ok() {
        None => return None,
        Some(_) => {
            for (pos, movement) in numbers.replace('\n', "").split(' ').enumerate() {
                if movement.len() != 2 {
                    return None;
                }

                for (inp_pos, char) in movement.chars().enumerate() {
                    match if inp_pos == 0 { letters_to_numbers.get(&char) } else { numbers_to_numbers.get(&char) } {
                        None => return None,
                        Some(numbered_value) => {
                            if pos == 0 && inp_pos == 0 {
                                result[1] = *numbered_value;
                            } else if pos == 0 && inp_pos == 1 {
                                result[0] = *numbered_value;
                            } else if pos == 1 && inp_pos == 0 {
                                result[3] = *numbered_value;
                            } else if pos == 1 && inp_pos == 1 {
                                result[2] = *numbered_value;
                            }
                        }
                    }
                }
            }
        }
    }
    return Some(result);
}

fn parse_fen() -> Option<piece::Game> {
    let mut fen_buf = String::new();
    io::stdin().read_line(&mut fen_buf).ok();
    if fen_buf == "\n" {
        return None;
    }
    return Some(fen::fen_to_board(&fen_buf));
}

fn main() {
    println!("Press RETURN to start a fresh game, or enter a FEN notated game to start the game from that state.");

    let mut pieces = vec![];
    let mut side = piece::Side::White;

    match parse_fen() {
        None => {
            let start_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".into();
            let game = fen::fen_to_board(&start_fen);
            pieces = game.pieces;
            side = game.side;
        },
        Some(game) => {
            pieces = game.pieces;
            side = game.side;
        }
    }

    let mut turn = side;
    loop {
        println!("{:?} turn.", turn);
        print_board(&pieces);
        let maybe_movement = parse_movement();
        match maybe_movement {
            None => continue,
            Some(movement) => {
                if move_piece(&mut pieces, (movement[0], movement[1]), (movement[2], movement[3]), turn.clone()) == true {
                    // Move was successful, switch turn to other player
                    if turn == piece::Side::White {
                        turn = piece::Side::Black;
                    } else {
                        turn = piece::Side::White;
                    }
                }
            }
        }
    }
}