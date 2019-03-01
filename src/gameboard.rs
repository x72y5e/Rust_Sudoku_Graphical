//! Game board logic.

use std::thread;
use std::sync::mpsc;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use ndarray::prelude::Array2;
use crate::solve::{search, count_collisions};


/// stores game board information
#[derive(Clone)]
pub struct Gameboard {
    /// array of values; 0 = empty
    pub as_array: Array2<u8>,
    /// whether the board is valid
    pub is_valid: bool,
    /// whether the board is in set up mode or solving mode
    pub solving: bool,
    /// whether solved
    pub solved: bool,
}

impl Gameboard {
    /// create new game board
    pub fn new() -> Gameboard {
        Gameboard {
            as_array: Array2::zeros((9, 9)),
            is_valid: true,
            solving: false,
            solved: false,
        }
    }

    /// update as_array field and check whether the board is valid
    pub fn validate(&mut self) {
        self.is_valid = count_collisions(&self.as_array, true) == 0;
    }

    /// gets char at cell location
    pub fn char(&self, ind: [usize; 2]) -> Option<char> {
        Some(match self.as_array[[ind[1], ind[0]]] {
            1 => '1',
            2 => '2',
            3 => '3',
            4 => '4',
            5 => '5',
            6 => '6',
            7 => '7',
            8 => '8',
            9 => '9',
            _ => return None,
        })
    }

    /// set cell value
    pub fn set(&mut self, ind: [usize; 2], val: u8) {
        if !self.solving && !self.solved {
            self.as_array[[ind[1], ind[0]]] = val;
        }
    }

    /// find a solution for the board
    pub fn get_solution(&mut self) {
        let (tx, rx) = mpsc::channel();
        let shared = Arc::new(AtomicBool::new(false));

        for thread_num in 0..7 {
            let tx1 = mpsc::Sender::clone(&tx);
            let shared1 = Arc::clone(&shared);
            let board_clone = self.clone().as_array;
            thread::spawn(move || {
                let b = search(board_clone, &shared1, thread_num);
                let r = tx1.send(b);
                match r {
                    Ok(_) => {},
                    Err(_) => {}, //println!("{}", e),
                }
            });
        }

        //let (solved_board, counter) = rx.recv(); //.unwrap();
        if let Ok(Some(result)) = rx.recv() {
            println!("solved after trying {} permutations", result.1);
            self.as_array = result.0;
            self.solved = true;
            self.solving = false;
        }
    }
}
