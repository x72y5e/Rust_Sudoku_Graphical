//! Gameboard view

use graphics::types::Color;
use graphics::{Context, Graphics};
use graphics::character::CharacterCache;
use crate::GameboardController;


/// stores gameboard view settings.
pub struct GameboardViewSettings {
    /// position from left-top corner
    pub position: [f64; 2],
    /// size of gameboard along horizontal and vertical edge
    pub size: f64,
    /// background colour
    pub background_color: Color,
    /// border colour
    pub border_color: Color,
    /// border edge colour
    pub border_edge_color: Color,
    /// section edge colour
    pub section_edge_color: Color,
    /// cell edge colour
    pub cell_edge_color: Color,
    /// edge radius around whole board
    pub board_edge_radius: f64,
    /// edge radius between 3x3 cells
    pub section_edge_radius: f64,
    /// edge radius between cells
    pub cell_edge_radius: f64,
    /// selected cell color
    pub selected_cell_background_color: Color,
    /// text colour
    pub text_color: Color,
    /// colour of text when board invalid
    pub invalid_color: Color,
    /// colour of text when board is solved
    pub solved_color: Color,
}

impl GameboardViewSettings {
    /// creates new gameboard view settings
    pub fn new() -> GameboardViewSettings {
        GameboardViewSettings {
            position: [39.0, 45.0],
            size: 400.0,
            background_color: [0.8, 0.8, 1.0, 1.0],
            border_color: [0.0, 0.0, 0.2, 1.0],
            border_edge_color: [0.0, 0.0, 0.2, 1.0],
            section_edge_color: [0.0, 0.0, 0.2, 1.0],
            cell_edge_color: [0.0, 0.0, 0.2, 1.0],
            board_edge_radius: 3.0,
            section_edge_radius: 2.0,
            cell_edge_radius: 1.0,
            selected_cell_background_color: [0.9, 0.9, 1.0, 1.0],
            text_color: [0.0, 0.0, 0.1, 1.0],
            invalid_color: [0.8, 0.0, 0.2, 1.0],
            solved_color: [0.0, 0.6, 0.2, 1.0],
        }
    }
}

/// stores visial info about a gameboard
pub struct GameboardView {
    /// stores gameboard view settings
    pub settings: GameboardViewSettings,
}

impl GameboardView {
    /// creates new gameboard view
    pub fn new(settings: GameboardViewSettings) -> GameboardView {
        GameboardView {
            settings: settings,
        }
    }

    /// draw
    pub fn draw<G: Graphics, C>(&self,
                                controller: &GameboardController,
                                glyphs: &mut C,
                                c: &Context,
                                g: &mut G)
                                where C: CharacterCache<Texture = G::Texture>
    {
        use graphics::{Image, Line, Rectangle, Transformed};

        let ref settings = self.settings;
        let board_rect = [
            settings.position[0], settings.position[1],
            settings.size, settings.size,
        ];

        // draw background
        Rectangle::new(settings.background_color)
            .draw(board_rect, &c.draw_state, c.transform, g);

        // draw selected cell background
        if let Some(ind) = controller.selected_cell {
            let cell_size = settings.size / 9.0;
            let pos = [ind[0] as f64 * cell_size, ind[1] as f64 * cell_size];
            let cell_rect = [
                settings.position[0] + pos[0], settings.position[1] + pos[1],
                cell_size, cell_size
            ];
            Rectangle::new(settings.selected_cell_background_color)
                .draw(cell_rect, &c.draw_state, c.transform, g);
        }

        // draw characters
        let text_image = {
            if controller.gameboard.solved {
                Image::new_color(settings.solved_color)
            } else if controller.gameboard.is_valid {
                Image::new_color(settings.text_color)
            } else {
                Image::new_color(settings.invalid_color)
            }
        };
        let cell_size = settings.size / 9.0;
        for j in 0..9 {
            for i in 0..9 {
                if let Some(ch) = controller.gameboard.char([i, j]) {
                    let pos = [
                        settings.position[0] + i as f64 * cell_size + 15.0,
                        settings.position[1] + j as f64 * cell_size + 34.0
                        ];
                    if let Ok(character) = glyphs.character(34, ch) {
                        let ch_x = pos[0] + character.left();
                        let ch_y = pos[1] - character.top();
                        text_image.draw(character.texture,
                            &c.draw_state,
                            c.transform.trans(ch_x, ch_y),
                            g);
                    }
                }
            }
        }

        // draw cell borders
        let cell_edge = Line::new(settings.cell_edge_color, settings.cell_edge_radius);
        for i in 0..9 {
            // skip lines covered by sections
            if (i % 3) == 0 {continue;}

            let x = settings.position[0] + i as f64 / 9.0 * settings.size;
            let y = settings.position[1] + i as f64 / 9.0 * settings.size;
            let x2 = settings.position[0] + settings.size;
            let y2 = settings.position[1] + settings.size;

            let vline = [x, settings.position[1], x, y2];
            cell_edge.draw(vline, &c.draw_state, c.transform, g);

            let hline = [settings.position[0], y, x2, y];
            cell_edge.draw(hline, &c.draw_state, c.transform, g);
        }

        // draw section borders
        let section_edge = Line::new(settings.section_edge_color, settings.section_edge_radius);
        for i in 0..3 {
            // set up coordinates
            let x = settings.position[0] + i as f64 / 3.0 * settings.size;
            let y = settings.position[1] + i as f64 / 3.0 * settings.size;
            let x2 = settings.position[0] + settings.size;
            let y2 = settings.position[1] + settings.size;

            let vline = [x, settings.position[1], x, y2];
            section_edge.draw(vline, &c.draw_state, c.transform, g);

            let hline = [settings.position[0], y, x2, y];
            section_edge.draw(hline, &c.draw_state, c.transform, g);
        }

        // draw board edge
        Rectangle::new_border(settings.border_edge_color, settings.board_edge_radius)
            .draw(board_rect, &c.draw_state, c.transform, g);
    }
}
