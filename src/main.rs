#[macro_use]
extern crate log;
extern crate env_logger;

extern crate gtk;
extern crate glib;

mod exec;
mod gui;
mod input_error;
mod parser;
mod text_range;

fn main() {
    env_logger::init().unwrap();
    gui::launch();
}
