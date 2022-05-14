use crate::model::Model;
use crate::update::{update, view};
mod drawing;
mod keys;
mod model;
mod text;
mod update;
mod window;

pub const FONT_SIZE: u32 = 24;

fn main() {
    // Setting up the app
    nannou::app(Model::new).update(update).run();
}
