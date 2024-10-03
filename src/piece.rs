pub const PAWN: &str = "pawn  ";
pub const ROOK: &str = "rook  ";    
pub const KNIGHT: &str = "knight";
pub const BISHOP: &str = "bishop";
pub const QUEEN: &str = "queen ";
pub const KING: &str = "king  ";
pub const EMPTY: &str = "      ";

pub const PIECE_NAMES: [&str; 6] = [PAWN, ROOK, KNIGHT, BISHOP, QUEEN, KING];

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Side { White, Black }

pub fn position_to_piece(pieces: &Vec<Piece>, position: (usize, usize)) -> Option<usize> {
    // Discovers the piece at the requested position. Returns the address in pieces, if exists.
    // Starting with O(N), I should probably make this O(1) at some point...    
    for piece_index in 0..pieces.len() {
        if pieces[piece_index].captured {
            // Doesn't count as being in a position
            continue;
        }
        if pieces[piece_index].position.0 == position.0 && pieces[piece_index].position.1 == position.1 {
            return Some(piece_index);
        }
    }
    return None;
}

#[derive(Debug)]
pub struct Piece {
    pub typ_index: usize,
    pub position: (usize, usize),
    pub times_moved: usize,
    pub side: Side,
    pub captured: bool,
}

pub struct Game {
    pub pieces: Vec<Piece>,
    pub side: Side,
}

impl Piece {
    pub fn typ(&self) -> &str {
        return PIECE_NAMES[self.typ_index];
    }

    pub fn transform_typ(&mut self, new_typ: &str) {
        for i in 0..PIECE_NAMES.len() {
            if PIECE_NAMES[i] == new_typ {
                self.typ_index = i;
                return;
            }
        }
        panic!("Invalid piece type change: {}", new_typ);
    }

    pub fn valid_movements(&self, pieces: &Vec<Piece>) -> Vec<(usize, usize)> {
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
        fn find_all_movements(pieces: &Vec<Piece>, piece: &Piece, all_valid_movements: &mut Vec<(usize, usize)>, straight: bool, diagonal: bool, max_total_movement_amount: usize) {
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
            for amount in 1..(max_total_movement_amount + 1) {
                let maybe_movement_up = move_up(amount, (piece.position.0, piece.position.1));
                let maybe_movement_down = move_down(amount, (piece.position.0, piece.position.1));
                let maybe_movement_left: Option<(usize, usize)> = move_left(amount, (piece.position.0, piece.position.1));
                let maybe_movement_right: Option<(usize, usize)> = move_right(amount, (piece.position.0, piece.position.1));
                let maybe_movement_up_left = move_diagonal_up_left(amount, (piece.position.0, piece.position.1));
                let maybe_movement_down_right = move_diagonal_down_right(amount, (piece.position.0, piece.position.1));
                let maybe_movement_down_left: Option<(usize, usize)> = move_diagonal_down_left(amount, (piece.position.0, piece.position.1));
                let maybe_movement_up_right: Option<(usize, usize)> = move_diagonal_up_right(amount, (piece.position.0, piece.position.1));
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
                            let maybe_other_piece = position_to_piece(&pieces, (pos0, pos1));
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
                    let one_move = if self.side == Side::Black { move_down(1, (self.position.0, self.position.1)) } else { move_up(1, (self.position.0, self.position.1)) };
                    match one_move {
                        Some((new_pos0, new_pos1)) => {
                            // Check that there isn't a piece there
                            if position_to_piece(&pieces, (new_pos0, new_pos1)).is_none() {
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
                        let two_move = if self.side == Side::Black { move_down(2, (self.position.0, self.position.1)) } else { move_up(2, (self.position.0, self.position.1)) };
                        match two_move {
                            Some((new_pos0, new_pos1)) => {
                                // Check that there isn't a piece there
                                if position_to_piece(&pieces, (new_pos0, new_pos1)).is_none() {
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
                    let diag_right = if self.side == Side::Black { move_diagonal_down_right(1, (self.position.0, self.position.1)) } else { move_diagonal_up_right(1, (self.position.0, self.position.1)) };
                    let diag_left = if self.side == Side::Black { move_diagonal_down_left(1, (self.position.0, self.position.1)) } else { move_diagonal_up_left(1, (self.position.0, self.position.1)) };
                    potential_diag_positions.push(diag_right);
                    potential_diag_positions.push(diag_left);

                    for diag in potential_diag_positions {
                        match diag {
                            Some((pos_0, pos_1)) => {
                                let maybe_piece_index_approaching = position_to_piece(&pieces, (pos_0, pos_1));
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
                find_all_movements(&pieces, &self, &mut all_valid_movements, true, false, 7);
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
                                    match position_to_piece(&pieces, (final_pov0, final_pov1)) {
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
                    (self.position.0, self.position.1),
                    &pieces,
                    &self,
                    &mut all_valid_movements);
                // up 2, right 1
                move_fn(
                    move_up(2), 
                    move_right(1), 
                    (self.position.0, self.position.1),
                    &pieces,
                    &self,
                    &mut all_valid_movements);
                // down 2, left 1
                move_fn(
                    move_down(2), 
                    move_left(1), 
                    (self.position.0, self.position.1),
                    &pieces,
                    &self,
                    &mut all_valid_movements);
                // down 2, right 1
                move_fn(
                    move_down(2), 
                    move_right(1), 
                    (self.position.0, self.position.1),
                    &pieces,
                    &self,
                    &mut all_valid_movements);
                // right 2, up 1
                move_fn(
                    move_right(2), 
                    move_up(1), 
                    (self.position.0, self.position.1),
                    &pieces,
                    &self,
                    &mut all_valid_movements);
                // right 2, down 1
                move_fn(
                    move_right(2), 
                    move_down(1), 
                    (self.position.0, self.position.1),
                    &pieces,
                    &self,
                    &mut all_valid_movements);
                // left 2, up 1
                move_fn(
                    move_left(2), 
                    move_up(1), 
                    (self.position.0, self.position.1),
                    &pieces,
                    &self,
                    &mut all_valid_movements);
                // left 2, down 1
                move_fn(
                    move_left(2), 
                    move_down(1), 
                    (self.position.0, self.position.1),
                    &pieces,
                    &self,
                    &mut all_valid_movements);
            },
            BISHOP => {
                // can move max 7 diagonal all directions, not over any pieces
                // very similar to rooks except diagonal instead
                find_all_movements(&pieces, &self, &mut all_valid_movements, false, true, 7);
            },
            QUEEN => {
                find_all_movements(&pieces, &self, &mut all_valid_movements, true, true, 7);
            },
            KING => {
                // Any direction, just by one
                find_all_movements(&pieces, &self, &mut all_valid_movements, true, true, 1);
            },
            _ => {
            },
        }
        return all_valid_movements;
    }
}
