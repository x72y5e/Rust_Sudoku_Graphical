use std::collections::HashSet;
use std::sync::atomic;
use rand::seq::SliceRandom;
use rand::thread_rng;
use ndarray::Array2;
use permutohedron::heap_recursive;
use std::sync::atomic::AtomicBool;


pub fn count_collisions(board: &Array2<u8>, check_rows: bool) -> usize {
    let mut collisions = 0usize;

    for col in board.gencolumns() {
        let all_nonzero: Vec<&u8> = col.iter().filter(|x| **x > 0).collect();
        let unique_nonzero: HashSet<&u8> = all_nonzero.iter().cloned().collect();
        collisions += all_nonzero.len() - unique_nonzero.len();
    }

    // we don't check the rows when solving - candidate rows are sets
    if check_rows {
        for row in board.genrows() {
            let all_nonzero: Vec<&u8> = row.iter().filter(|x| **x > 0).collect();
            let unique_nonzero: HashSet<&u8> = all_nonzero.iter().cloned().collect();
            collisions += all_nonzero.len() - unique_nonzero.len();
        }
    }

    // 3x3 squares
    for row_n in (0..9).step_by(3) {
        for col_n in (0..9).step_by(3) {
            let square = board.slice(s![row_n..row_n + 3,
                                        col_n..col_n + 3]);
            let present: Vec<&u8> = square.iter()
                .filter(|x| **x != 0)
                .collect();
            let unique: HashSet<&u8> = square.iter()
                .filter(|x| **x != 0)
                .collect();
            collisions += present.len() - unique.len();
        }
    }
    collisions
}

pub fn search(mut board: Array2<u8>, shared: &AtomicBool, i: u8) -> Option<(Array2<u8>, usize)> {
    let original_board: Array2<u8> = board.clone();
    let mut n = 0;
    let full_set: HashSet<u8> = (1..10).collect();
    let mut counter = 0_usize;

    while n < 9 {
        // build list of available locations and existing numbers
        let mut available= Vec::new();
        let mut present = Vec::new();
        for (i, x) in board.row(n).iter().enumerate() {
            if *x == 0 {
                available.push(i);
            } else {
                present.push(*x);
            }
        }

        // build list of missing numbers
        let present_set: HashSet<u8> = present.iter().cloned().collect();
        let missing = full_set.difference(&present_set);

        // cycle permutations until collisions == 0, or revert to original board
        let mut missing_vec: Vec<&u8> = missing.collect();
        let mut permutations: Vec<Vec<&u8>> = Vec::new();
        heap_recursive(&mut missing_vec, |permutation| {
            permutations.push(permutation.to_vec())
        });
        let mut best = 81;
        let mut row_solved = false;
        permutations.shuffle(&mut thread_rng());
        for permutation in &permutations {
            if counter % 500 == 0 && shared.load(atomic::Ordering::Relaxed) {
                println!("thread {} stopping...", i);
                return None
            }

            counter += 1;
            for (p, a) in permutation.iter().zip(&available) {
                board[[n, *a]] = **p;
            }
            let c = count_collisions(&board, false);
            if c < best {
                best = c;
            }
            if best == 0 {
                n += 1;
                row_solved = true;
                break
            }
        }

        if !row_solved {
            board = original_board.clone();
            n = 0;
        }
    }
    shared.swap(true, atomic::Ordering::Relaxed);
    Some((board, counter))
}
