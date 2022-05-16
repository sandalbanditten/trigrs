use crate::keys::{key_pressed, key_released};
use crate::view;
use crate::window::window_resized;
use nannou::prelude::{App, Draw, LoopMode, Rect, Vec2};
use std::f32::consts::FRAC_PI_4 as QUARTER_PI;

pub struct Model {
    pub win_rect: Rect,
    pub draw: Draw,
    pub diameter: f32,
    pub angle: f32,
    pub pos: Vec2,
}

impl Model {
    // Our constructor
    pub fn new(app: &App) -> Self {
        // Only update the app when user interacts
        app.set_loop_mode(LoopMode::Wait);

        // Creating the window
        let _window = app
            .new_window()
            .title("Trigonometry!")
            // Functions to call at certain event
            .key_pressed(key_pressed)
            .key_released(key_released)
            .resized(window_resized)
            .view(view)
            .build()
            .expect("Unable to build the app");

        // The window rect
        let win_rect = app.window_rect();

        // Our model is the state of our application, which can be accessed from all functions
        Model {
            win_rect,
            draw: app.draw(),
            diameter: win_rect.w() / 2.0,
            angle: QUARTER_PI,
            pos: Vec2::ZERO,
        }
    }

    pub fn radius(&self) -> f32 {
        self.diameter / 2.0
    }
}
