//! Gameboard controller.

use piston::input::GenericEvent;
use crate::Gameboard;

/// handles events
pub struct GameboardController {
    /// stores gameboard state
    pub gameboard: Gameboard,
    /// selected cell
    pub selected_cell: Option<[usize; 2]>,
    /// cursor pos
    cursor_pos: [f64; 2],
}

impl GameboardController {
    /// creates new gameboard controller
    pub fn new(gameboard: Gameboard) -> GameboardController {
        GameboardController {
            gameboard: gameboard,
            selected_cell: None,
            cursor_pos: [0.0; 2],
        }
    }

    /// handles events
    pub fn event<E: GenericEvent>(&mut self, pos: [f64; 2], size: f64, e: &E) {

        use piston::input::{Button, Key, MouseButton};

        if let Some(pos) = e.mouse_cursor_args() {
            self.cursor_pos = pos;
        }
        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            // find coords relative to upper left
            let x = self.cursor_pos[0] - pos[0];
            let y = self.cursor_pos[1] - pos[1];
            // check inside boundaries
            if x >= 0.0 && x < size && y >= 0.0 && y < size {
                let cell_x = (x / size * 9.0) as usize;
                let cell_y = (y / size * 9.0) as usize;
                self.selected_cell = Some([cell_x, cell_y]);
            }
        }
        let mut solving = self.gameboard.solving;
        if let Some(Button::Keyboard(key)) = e.press_args() {
            if let Some(ind) = self.selected_cell {
                // set cell value
                match key {
                    Key::D1 => self.gameboard.set(ind, 1),
                    Key::D2 => self.gameboard.set(ind, 2),
                    Key::D3 => self.gameboard.set(ind, 3),
                    Key::D4 => self.gameboard.set(ind, 4),
                    Key::D5 => self.gameboard.set(ind, 5),
                    Key::D6 => self.gameboard.set(ind, 6),
                    Key::D7 => self.gameboard.set(ind, 7),
                    Key::D8 => self.gameboard.set(ind, 8),
                    Key::D9 => self.gameboard.set(ind, 9),
                    Key::Return => solving = true,
                    Key::Space => {
                        if !solving && !self.gameboard.solving {
                            self.gameboard = Gameboard::new();
                        }
                    },
                    _ => {},
                }
                self.gameboard.validate();
                if !self.gameboard.is_valid {
                    println!("invalid board.");
                    solving = false;
                }
            }
        }
        self.gameboard.solving = solving;
        if self.gameboard.solving {
            println!("solving board...");
            self.gameboard.get_solution();
        }
    }
}
