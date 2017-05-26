extern crate gtk;

mod app;
mod history;

use self::app::App;

pub fn launch() {
    gtk::init().expect("Failed to initialize GTK");

    let app = App::setup_new();
    app.on_close(
        || {
            gtk::main_quit();
            gtk::Inhibit(false)
        }
    );

    gtk::main();
}
