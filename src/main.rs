fn print_state(values: &mut Vec<Vec<&str>>) {
    for x in 0..7 {
        for y in 0..7 {
            print!("{} ", values[x][y]);
        }
        println!();
    }
}

fn main() {
    // pawn, rook, knight, bishop, queen, king
    // 6 is the largest name, so everything needs to be 6
    let mut values = vec![vec!["______"; 8]; 8];
    values[0][0] = "hmm";
    print_state(&mut values);
}
