#![deny(missing_docs)]

//! a sudoku game

extern crate piston;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate rand;
#[macro_use(s)]
extern crate ndarray;
//extern crate permutohedron;


use glutin_window::GlutinWindow;
use piston::window::WindowSettings;
use piston::event_loop::{Events, EventLoop, EventSettings};
use piston::input::RenderEvent;
use opengl_graphics::{OpenGL, Filter, GlGraphics, GlyphCache, TextureSettings};
pub use crate::gameboard::Gameboard;
pub use crate::gameboard_controller::GameboardController;
pub use crate::gameboard_view::{GameboardView, GameboardViewSettings};
//use crate::solve::{search, count_collisions};
//use std::thread;
//use std::sync::mpsc;
//use ndarray::Array2;

mod gameboard;
mod gameboard_controller;
mod gameboard_view;
mod solve;


fn main() {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Sudoku", [475; 2])
        .opengl(opengl)
        .exit_on_esc(true);

    let mut window: GlutinWindow = settings.build()
        .expect("Could not create window");

    let mut events = Events::new(EventSettings::new().lazy(true));
    let mut gl = GlGraphics::new(opengl);

    let gameboard = Gameboard::new();
    let mut gameboard_controller = GameboardController::new(gameboard);
    let gameboard_view_settings = GameboardViewSettings::new();
    let gameboard_view = GameboardView::new(gameboard_view_settings);
    let texture_settings = TextureSettings::new().filter(Filter::Nearest);
    let ref mut glyphs = GlyphCache::new("assets/FiraSans-Regular.ttf", (), texture_settings)
        .expect("Could not load font");

    while let Some(e) = events.next(&mut window) {
        gameboard_controller.event(gameboard_view.settings.position,
                                   gameboard_view.settings.size,
                                   &e);
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                use graphics::{clear};
                clear([1.0; 4], g);
                gameboard_view.draw(&gameboard_controller, glyphs, &c, g)
            });
        }
    }
}
