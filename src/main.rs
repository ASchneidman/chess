const PAWN: &str = "pawn__";
const ROOK: &str = "rook__";
const KNIGHT: &str = "knight";
const BISHOP: &str = "bishop";
const QUEEN: &str = "queen_";
const KING: &str = "king__";

fn print_state(values: &mut Vec<Vec<&str>>) {
    for x in 0..8 {
        for y in 0..8 {
            print!("{} ", values[x][y]);
        }
        println!();
    }
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

    print_state(&mut values);
}
