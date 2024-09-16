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
        fn move_up(amount: usize, piece: (usize, usize)) -> Option<(usize, usize)> {
            if amount > piece.0 {
                // Can't move up that much
                return None;
            }
            return Some((piece.0 - amount, piece.1));
        }
        fn move_down(amount: usize, piece: (usize, usize)) -> Option<(usize, usize)> {
            if amount + piece.0 >= 8 {
                // Can't move down that much
                return None;
            }
            return Some((piece.0 + amount, piece.1));
        }
        fn move_left(amount: usize, piece: (usize, usize)) -> Option<(usize, usize)> {
            if amount > piece.1 {
                // Can't move left that much
                return None;
            }
            return Some((piece.0, piece.1 - amount));
        }
        fn move_right(amount: usize, piece: (usize, usize)) -> Option<(usize, usize)> {
            if amount + piece.1 >= 8 {
                // Can't move right that much
                return None;
            }
            return Some((piece.0, piece.1 + amount));
        }
        fn move_diagonal_up_left(amount: usize, piece: (usize, usize)) -> Option<(usize, usize)> {
            if amount > piece.0 {
                // Can't move up that much
                return None;
            }
            if amount > piece.1 {
                // Can't move left that much
                return None;
            }
            return Some((piece.0 - amount, piece.1 - amount));
        }
        fn move_diagonal_up_right(amount: usize, piece: (usize, usize)) -> Option<(usize, usize)> {
            if amount > piece.0 {
                // Can't move up that much
                return None;
            }
            if amount + piece.1 >= 8 {
                // Can't move right that much
                return None;
            }
            return Some((piece.0 - amount, piece.1 + amount));
        }
        fn move_diagonal_down_left(amount: usize, piece: (usize, usize)) -> Option<(usize, usize)> {
            if amount + piece.0 >= 8 {
                // Can't move down that much
                return None;
            }
            if amount > piece.1 {
                // Can't move left that much
                return None;
            }
            return Some((piece.0 + amount, piece.1 - amount));
        }
        fn move_diagonal_down_right(amount: usize, piece: (usize, usize)) -> Option<(usize, usize)> {
            if amount + piece.0 >= 8 {
                // Can't move down that much
                return None;
            }
            if amount + piece.1 >= 8 {
                // Can't move right that much
                return None;
            }
            return Some((piece.0 + amount, piece.1 + amount));
        }
        fn find_all_movements(pieces: &Vec<Piece>, piece: &Piece, all_valid_movements: &mut Vec<(usize, usize)>, straight: bool, diagonal: bool) {
            // Populates all_valid_movements with the allowed movements of straight and/or diagonal without passing over pieces
            // can't jump over another piece
            // can capture piece

            // Can only move up 7 (the entire board)
            let mut done_up = !straight;
            let mut done_down = !straight;
            let mut done_left = !straight;
            let mut done_right = !straight;
            let mut done_up_left = !diagonal;
            let mut done_down_right = !diagonal;
            let mut done_up_right = !diagonal;
            let mut done_down_left = !diagonal;
            for amount in 1..8 {
                let maybe_movement_up = move_up(amount, (piece.position[0], piece.position[1]));
                let maybe_movement_down = move_down(amount, (piece.position[0], piece.position[1]));
                let maybe_movement_left: Option<(usize, usize)> = move_left(amount, (piece.position[0], piece.position[1]));
                let maybe_movement_right: Option<(usize, usize)> = move_right(amount, (piece.position[0], piece.position[1]));
                let maybe_movement_up_left = move_diagonal_up_left(amount, (piece.position[0], piece.position[1]));
                let maybe_movement_down_right = move_diagonal_down_right(amount, (piece.position[0], piece.position[1]));
                let maybe_movement_down_left: Option<(usize, usize)> = move_diagonal_down_left(amount, (piece.position[0], piece.position[1]));
                let maybe_movement_up_right: Option<(usize, usize)> = move_diagonal_up_right(amount, (piece.position[0], piece.position[1]));
                for (maybe_movement, done) in [
                    (maybe_movement_up, &mut done_up), 
                    (maybe_movement_down, &mut done_down), 
                    (maybe_movement_left, &mut done_left), 
                    (maybe_movement_right, &mut done_right),
                    (maybe_movement_up_left, &mut done_up_left), 
                    (maybe_movement_down_right, &mut done_down_right), 
                    (maybe_movement_down_left, &mut done_down_left), 
                    (maybe_movement_up_right, &mut done_up_right)] {
                    if *done == true {
                        continue;
                    }
                    match maybe_movement {
                        None => {
                            // Not allowed, done.
                            *done = true;
                            continue;
                        },
                        Some((pos0, pos1)) => {
                            // If another piece there that is ours, is done
                            let pos = vec![pos0, pos1];
                            let maybe_other_piece = position_to_piece(&pieces, &pos);
                            match maybe_other_piece {
                                Some(other_piece_index) => {
                                    if pieces[other_piece_index].side == piece.side {
                                        // Our piece. Not allowed, done.
                                        *done = true;
                                        continue;
                                    }
                                    // Not our piece, can take it over, then done.
                                    all_valid_movements.push((pos0, pos1));
                                    *done = true;
                                    continue;
                                },
                                _ => {
                                    // No piece there, can move!
                                    all_valid_movements.push((pos0, pos1));
                                }
                            }
                        }
                    }
                }
            }
        }

        let mut all_valid_movements: Vec<(usize, usize)> = vec![];
        match self.typ() {
            PAWN => {
                {
                    // Can move up if white, down if black
                    let one_move = if self.side == Side::Black { move_down(1, (self.position[0], self.position[1])) } else { move_up(1, (self.position[0], self.position[1])) };
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
                        let two_move = if self.side == Side::Black { move_down(2, (self.position[0], self.position[1])) } else { move_up(2, (self.position[0], self.position[1])) };
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
                    let diag_right = if self.side == Side::Black { move_diagonal_down_right(1, (self.position[0], self.position[1])) } else { move_diagonal_up_right(1, (self.position[0], self.position[1])) };
                    let diag_left = if self.side == Side::Black { move_diagonal_down_left(1, (self.position[0], self.position[1])) } else { move_diagonal_up_left(1, (self.position[0], self.position[1])) };
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
            ROOK => {
                find_all_movements(&pieces, &self, &mut all_valid_movements, true, false);
            }
            KNIGHT => {
                // up,down,left,right by 2 then right/left by 1
                // Curry all the move helpers to make the main helper cleaner
                let move_right = |amount| move |piece| move_right(amount, piece);
                let move_left = |amount| move |piece| move_left(amount, piece);
                let move_up = |amount| move |piece| move_up(amount, piece);
                let move_down = |amount| move |piece| move_down(amount, piece);
                // Helper which tries to move the knight by two moves. If the move is valid, adds to all_valid_movements
                fn move_fn(
                    first_move: impl Fn((usize, usize)) -> Option<(usize, usize)>, 
                    second_move: impl Fn((usize, usize)) -> Option<(usize, usize)>, 
                    start_pos: (usize, usize),
                    pieces: &Vec<Piece>,
                    piece: &Piece,
                    all_valid_movements: &mut Vec<(usize, usize)>)  {
                    // first_move and second_move are curried with the amount they need to move by (2 or 1 each)
                    match first_move(start_pos) {
                        Some((pov0, pov1)) =>  {
                            // Where second move should start from
                            match second_move((pov0, pov1)) {
                                Some((final_pov0, final_pov1))  => {
                                    // Can only move here if enemy piece or no piece
                                    let final_pos = vec![final_pov0, final_pov1];
                                    match position_to_piece(&pieces, &final_pos) {
                                        None => {
                                            // Can go there
                                            all_valid_movements.push((final_pov0, final_pov1));
                                        },
                                        Some(other_index) => {
                                            if pieces[other_index].side != piece.side {
                                                // Enemy, so we can move there
                                                all_valid_movements.push((final_pov0, final_pov1));
                                            }
                                        }
                                    }
                                },
                                None => {
                                    // Can't move there
                                }
                            }
                        },
                        None => {
                            // Can't move there
                        }
                    }
                }
                // up 2, left 1
                move_fn(
                    move_up(2), 
                    move_left(1), 
                    (self.position[0], self.position[1]),
                    &pieces,
                    &self,
                    &mut all_valid_movements);
                // up 2, right 1
                move_fn(
                    move_up(2), 
                    move_right(1), 
                    (self.position[0], self.position[1]),
                    &pieces,
                    &self,
                    &mut all_valid_movements);
                // down 2, left 1
                move_fn(
                    move_down(2), 
                    move_left(1), 
                    (self.position[0], self.position[1]),
                    &pieces,
                    &self,
                    &mut all_valid_movements);
                // down 2, right 1
                move_fn(
                    move_down(2), 
                    move_right(1), 
                    (self.position[0], self.position[1]),
                    &pieces,
                    &self,
                    &mut all_valid_movements);
                // right 2, up 1
                move_fn(
                    move_right(2), 
                    move_up(1), 
                    (self.position[0], self.position[1]),
                    &pieces,
                    &self,
                    &mut all_valid_movements);
                // right 2, down 1
                move_fn(
                    move_right(2), 
                    move_down(1), 
                    (self.position[0], self.position[1]),
                    &pieces,
                    &self,
                    &mut all_valid_movements);
                // left 2, up 1
                move_fn(
                    move_left(2), 
                    move_up(1), 
                    (self.position[0], self.position[1]),
                    &pieces,
                    &self,
                    &mut all_valid_movements);
                // left 2, down 1
                move_fn(
                    move_left(2), 
                    move_down(1), 
                    (self.position[0], self.position[1]),
                    &pieces,
                    &self,
                    &mut all_valid_movements);
            },
            BISHOP => {
                // can move max 7 diagonal all directions, not over any pieces
                // very similar to rooks except diagonal instead
                find_all_movements(&pieces, &self, &mut all_valid_movements, false, true);
            },
            QUEEN => {
                find_all_movements(&pieces, &self, &mut all_valid_movements, true, true);
            },
            _ => {
            },
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
                    // If pawn reaches the end, turns into a QUEEN
                    if pieces[piece_index].typ() == PAWN && (pieces[piece_index].position[0] == 0 || pieces[piece_index].position[0] == 7) {
                        pieces[piece_index].transform_typ(QUEEN);           
                    }
                    return true;
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
