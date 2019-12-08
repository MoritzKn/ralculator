#[macro_use]
extern crate log;
extern crate env_logger;

extern crate glib;
extern crate gtk;

mod exec;
mod gui;
mod input_error;
mod parser;
mod text_range;

fn main() {
    env_logger::init().unwrap();
    gui::launch();
}
