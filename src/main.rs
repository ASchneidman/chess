use std::{cmp::{max, min}, io::{self, BufRead}, iter::Rev, ops::Range};
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

    fn valid_movements(&self, pieces: &Vec<Piece>) -> Vec<(usize, usize)> {
        // These are helpers which just check if a particular directional movement
        // is on the board
        fn move_up(amount: usize, piece: &Piece) -> Option<(usize, usize)> {
            if amount > piece.position[0] {
                // Can't move up that much
                return None;
            }
            return Some((piece.position[0] - amount, piece.position[1]));
        }
        fn move_down(amount: usize, piece: &Piece) -> Option<(usize, usize)> {
            if amount + piece.position[0] >= 8 {
                // Can't move down that much
                return None;
            }
            return Some((piece.position[0] + amount, piece.position[1]));
        }
        fn move_diagonal_up_left(amount: usize, piece: &Piece) -> Option<(usize, usize)> {
            if amount > piece.position[0] {
                // Can't move up that much
                return None;
            }
            if amount > piece.position[1] {
                // Can't move left that much
                return None;
            }
            return Some((piece.position[0] - amount, piece.position[1] - amount));
        }
        fn move_diagonal_up_right(amount: usize, piece: &Piece) -> Option<(usize, usize)> {
            if amount > piece.position[0] {
                // Can't move up that much
                return None;
            }
            if amount + piece.position[1] >= 8 {
                // Can't move right that much
                return None;
            }
            return Some((piece.position[0] - amount, piece.position[1] + amount));
        }
        fn move_diagonal_down_left(amount: usize, piece: &Piece) -> Option<(usize, usize)> {
            if amount + piece.position[0] >= 8 {
                // Can't move down that much
                return None;
            }
            if amount > piece.position[1] {
                // Can't move left that much
                return None;
            }
            return Some((piece.position[0] + amount, piece.position[1] - amount));
        }
        fn move_diagonal_down_right(amount: usize, piece: &Piece) -> Option<(usize, usize)> {
            if amount + piece.position[0] >= 8 {
                // Can't move down that much
                return None;
            }
            if amount + piece.position[1] >= 8 {
                // Can't move right that much
                return None;
            }
            return Some((piece.position[0] + amount, piece.position[1] + amount));
        }

        let mut all_valid_movements: Vec<(usize, usize)> = vec![];
        match self.typ() {
            PAWN => {
                {
                    // Can move up if white, down if black
                    let one_move = if self.side == Side::Black { move_down(1, &self) } else { move_up(1, &self) };
                    match one_move {
                        Some((new_pos0, new_pos1)) => {
                            // Check that there isn't a piece there
                            let new_pos = vec![new_pos0, new_pos1];
                            if position_to_piece(&pieces, &new_pos).is_none() {
                                // Can go there!
                                all_valid_movements.push((new_pos0, new_pos1));
                            }
                        },
                        _ => {}
                    }
                }
                {
                    // If first turn, can move two spots up/down
                    if self.times_moved == 0 {
                        // First turn!
                        let two_move = if self.side == Side::Black { move_down(2, &self) } else { move_up(2, &self) };
                        match two_move {
                            Some((new_pos0, new_pos1)) => {
                                // Check that there isn't a piece there
                                let new_pos = vec![new_pos0, new_pos1];
                                if position_to_piece(&pieces, &new_pos).is_none() {
                                    // Can go there!
                                    all_valid_movements.push((new_pos0, new_pos1));
                                }
                            },
                            _ => {}
                        }
                        }
                }
                {
                    // Can only move diagonal by capturing
                    let mut potential_diag_positions: Vec<Option<(usize, usize)>> = vec![];
                    let diag_right = if self.side == Side::Black { move_diagonal_down_right(1, &self) } else { move_diagonal_up_right(1, &self) };
                    let diag_left = if self.side == Side::Black { move_diagonal_down_left(1, &self) } else { move_diagonal_up_left(1, &self) };
                    potential_diag_positions.push(diag_right);
                    potential_diag_positions.push(diag_left);

                    for diag in potential_diag_positions {
                        match diag {
                            Some((pos_0, pos_1)) => {
                                let new_pos = vec![pos_0, pos_1];
                                let maybe_piece_index_approaching = position_to_piece(&pieces, &new_pos);
                                match maybe_piece_index_approaching {
                                    Some(piece_index_approaching) => {
                                        if pieces[piece_index_approaching].side != self.side {
                                            // Enemy!
                                            all_valid_movements.push((pos_0, pos_1));
                                        }
                                    },
                                    None => {
                                        // Can't move there if no enemy piece there
                                    }
                                }
                            },
                            _ => {}
                        }
                    }
                }
            },
            _ => {
                panic!("Bad piece!");
            }
        }
        return all_valid_movements;
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

fn is_piece_jumping_over_piece(pieces: &Vec<Piece>, piece: &Piece, destination: &Vec<usize>, is_jumping_onto_piece: &mut usize) -> bool {
    // Tells whether a piece moving from its current location to destination is jumping over another piece.
    // Returns true if jumping over another piece. 
    // Returns false if not. If jumping onto another piece, populates is_jumping_onto_piece with index of target piece. 

    let max_0th: usize = max(piece.position[0], destination[0]);
    let max_1th: usize = max(piece.position[1], destination[1]);
    let min_0th: usize = min(piece.position[0], destination[0]);
    let min_1th: usize = min(piece.position[1], destination[1]);

    // Movement must be up/down, left/right, or diagonal
    if piece.position[0] == destination[0] && piece.position[1] != destination[1] {
        // Moving left/right
        let mut positions_to_check: Vec<(usize, usize)> = vec![];
    } else if piece.position[1] == destination[1] && piece.position[0] == destination[0] {
        // Moving up/down
        panic!("Not implemented");
    } else if (max_0th - min_0th) == (max_1th - min_1th) {
        // Moving diagonal
        let mut positions_to_check: Vec<(usize, usize)> = vec![];
        if piece.position[0] > destination[0] && piece.position[1] > destination[1] {
            // up/left
            // 1..5 => 1,2,3,4 => 4,3,2,1
            // 1..1 => 1 => 1
            // full example: (2,5) -> (0,0) => (1,4)
            let mut pos_1 = piece.position[1] - 1;
            for pos_0 in ((destination[0] + 1)..(piece.position[0] - 1)).rev() {
                positions_to_check.push((pos_0, pos_1));
                pos_1 -= 1;
            }
        } else if piece.position[0] > destination[0] && piece.position[1] < destination[1] {
            // up/right
            // 1..5 => 1,2,3,4 => 4,3,2,1
            // 1..1 => 1 => 1
            // full example: (2,1) -> (0,3) => (1,2)
            let mut pos_1 = piece.position[1] + 1;
            for pos_0 in ((destination[0] + 1)..(piece.position[0] - 1)).rev() {
                positions_to_check.push((pos_0, pos_1));
                pos_1 += 1;
            }
        } else if piece.position[0] < destination[0] && piece.position[1] > destination[1] {
            // down/left
            // 1..5 => 1,2,3,4 => 4,3,2,1
            // 1..1 => 1 => 1
            // full example: (0,3) -> (2,1) => (1,2)
            let mut pos_1 = piece.position[1] - 1;
            for pos_0 in (piece.position[0]+1)..destination[0] {
                positions_to_check.push((pos_0, pos_1));
                pos_1 -= 1;
            }
        } else if piece.position[0] < destination[0] && piece.position[1] < destination[1] {
            // down/right
            // 1..5 => 1,2,3,4 => 4,3,2,1
            // 1..1 => 1 => 1
            // full example: (0,3) -> (2,5) => (1,4)
            let mut pos_1 = piece.position[1] + 1;
            for pos_0 in (piece.position[0]+1)..destination[0] {
                positions_to_check.push((pos_0, pos_1));
                pos_1 += 1;
            }
        } else {
            panic!("Got invalid diagonal movement: Piece {:?} Destination {:?}", piece.position, destination);
        }
        // end might be smaller than start
        // pos_1 might be larger than destination[1]
        for pos in positions_to_check {
            let next_pos = vec![pos.0, pos.1];
            let maybe_another_piece = position_to_piece(&pieces, &next_pos);
            if !maybe_another_piece.is_none() {
                // Piece there, not good! Jumping over piece
                return true;
            }
        }
    }

    // Check capturing piece.
    let maybe_another_piece = position_to_piece(&pieces, &destination);
    match maybe_another_piece {
        None => {
            // Don't populate, not landing on another piece
        },
        Some(index) => {
            // Populate, landing on another piece
            *is_jumping_onto_piece = index;
        }
    }
    return false;
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

            let allowed_positions = pieces[piece_index].valid_movements(&pieces);
            println!("{:?}", allowed_positions);
            for allowed_pos in allowed_positions {
                if allowed_pos.0 == destination[0] && allowed_pos.1 == destination[1] {
                    // Check if there's a enemy piece there
                    match maybe_piece_at_destination {
                        Some(other_piece_index) => {
                            pieces[other_piece_index].captured = true;
                        },
                        None => {},
                    }
                    move_piece_to_dest(piece_index, &mut pieces, &destination);
                    return true;
                }
            }

            /*
            if destination[0] == pieces[piece_index].position[0] && destination[1] == pieces[piece_index].position[1] {
                // Have to move somewhere!
                println!("Have to move the piece somewhere.");
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
                            // Not allowed to move diagonal if no piece there
                            println!("Can only move pawn diagonal if capturing a piece.");
                            return false;
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
                ROOK => {
                    panic!("Not yet implemented.");
                    // Check that the destination is directly up/down or left/right from rook
                    if !((pieces[piece_index].position[0] == destination[0] && pieces[piece_index].position[1] != destination[1])
                        || (pieces[piece_index].position[0] != destination[0] && pieces[piece_index].position[1] == destination[1])) {
                            // NOT! correctly moving up/down or left/right. 
                            println!("Can only move rook up/down or left/right");
                            return false;
                        }
                    // Rook cannot jump over pieces. Follow from positions from current piece to destination to detect if there's a piece there its jumping over.
                    let mut piece_start: usize = 0;
                    let mut piece_end: usize = 0;
                    // Moving up
                    if pieces[piece_index].position[0] == destination[0] && destination[1] < pieces[piece_index].position[1] {
                        
                    }
                },
                ROOK {
                    
                },
                _ => {
                    // Won't happen
                    return false;
                }
            }
            */
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
