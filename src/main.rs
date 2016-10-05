extern crate gtk;
use gtk::prelude::*;
use gtk::{Window, Builder, Button, Entry, EntryBuffer, CssProvider, StyleContext,
          STYLE_PROVIDER_PRIORITY_APPLICATION};

mod exec;
use self::exec::*;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let builder = Builder::new_from_string(&load_glade_src());
    let window: Window = builder.get_object("window").unwrap();

    setup_window(&window);
    setup_inputs(&builder);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.show_all();

    gtk::main();
}

fn setup_window(window: &Window) {
    // Setup CSS
    let css = CssProvider::new();
    match css.load_from_data(include_str!("main.css")) {
        Ok(_) => {
            let screen = window.get_display().unwrap().get_screen(0);
            let provider = STYLE_PROVIDER_PRIORITY_APPLICATION;

            StyleContext::add_provider_for_screen(&screen, &css, provider);
        },
        Err(msg) => {
            println!("Error loading main.css {}", msg);
        },
    }
}

type ButtoInfo = (&'static str, &'static str);

static STRAIGHT_INPUT_BUTTONS: [ButtoInfo; 15] = [("num_1_button", "1"),
                                                  ("num_2_button", "2"),
                                                  ("num_3_button", "3"),
                                                  ("num_4_button", "4"),
                                                  ("num_5_button", "5"),
                                                  ("num_6_button", "6"),
                                                  ("num_7_button", "7"),
                                                  ("num_8_button", "8"),
                                                  ("num_9_button", "9"),
                                                  ("num_0_button", "0"),
                                                  ("divide_button", " / "),
                                                  ("add_button", " + "),
                                                  ("subtract_button", " - "),
                                                  ("multiply_button", " * "),
                                                  ("decimal_point_button", " . ")];

fn setup_inputs(builder: &Builder) {
    let input: Entry = builder.get_object("input").unwrap();
    input.set_alignment(1f32);

    for &(id, text) in STRAIGHT_INPUT_BUTTONS.iter() {
        let input: Entry = builder.get_object("input").unwrap();
        let input_buffer = input.get_buffer();
        let button: Button = builder.get_object(id).unwrap();

        button.connect_clicked(move |_button| {
            let input_text = input_buffer.get_text();

            input.set_position(input_text.len() as i32);
            input_buffer.insert_text(input_text.len() as u16, text);
        });
    }

    {
        let input_buffer = input.get_buffer();
        let button: Button = builder.get_object("calc_button").unwrap();

        button.connect_clicked(move |_button| {
            calculat(&input_buffer);
            input.grab_focus();
            input.set_position(input_buffer.get_text().len() as i32);
        });
    }
}

fn calculat(buffer: &EntryBuffer) {
    let text = buffer.get_text();

    if text.len() == 0 {
        return;
    }

    let res = exec_expression(&text);

    match res {
        Ok(value) => {
            buffer.set_text(&format!("{:.*}", 0, value));
        },
        Err((msg, pos)) => {
            buffer.set_text(&format!("Error: {} at pos {}", msg, pos));
        },
    }
}

#[cfg(build = "release")]
fn load_glade_src() -> String {
    println!("Bake glade view");
    String::from(include_str!("main.glade"))
}

#[cfg(not(build = "release"))]
fn load_glade_src() -> String {
    println!("Live load glade view");

    use std::path::Path;
    use std::fs::File;
    use std::io::prelude::*;
    use std::error::Error;

    let path = Path::new("src/main.glade");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut file_content = String::new();
    match file.read_to_string(&mut file_content) {
        Err(why) => panic!("couldn't read {}: {}", display, why.description()),
        Ok(_) => {},
    }

    return file_content;
}
