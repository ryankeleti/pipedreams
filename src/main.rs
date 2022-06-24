use std::str::FromStr;

use pipedreams::{
    dream::{Dream, ReducedDreams, Tile},
    matrix::SqMat,
    perm::Perm,
    poly::Schubert,
};

// An example.
fn main() {
    // Choose a permutation (zero-indexed).
    let perm = Perm::from_str("0,3,2,1").expect("Invalid permutation");
    println!("Permutation: {}", perm);

    // Calculate the reduced pipe dreams.
    let dreams = ReducedDreams::for_perm(&perm);

    println!("Reduced dreams:");
    for dream in dreams.iter() {
        println!("{}", display_dream(dream));
    }

    // Compute the Schubert polynomial.
    let schubert = Schubert::from_dreams(&dreams);
    println!("{}", schubert);
}

fn display_dream(dream: &Dream) -> SqMat<char> {
    dream.tiles().map(|tile| match tile {
        Tile::Elbow => '.',
        Tile::Cross => '+',
    })
}
