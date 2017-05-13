extern crate gtk;
use gtk::prelude::*;
use gtk::{Window, Builder, Button, Entry, EntryBuffer, TextView, TextBuffer, ScrolledWindow,
          CssProvider, StyleContext, STYLE_PROVIDER_PRIORITY_APPLICATION};

#[macro_use]
extern crate log;
extern crate env_logger;

pub use self::lexer::{tokenize, Token};
pub mod lexer;

pub use self::exec::exec_expression;
pub mod exec;

pub use self::token_types::*;
pub mod token_types;

fn main() {
    env_logger::init().unwrap();

    gtk::init().expect("Failed to initialize GTK");

    launch();

    gtk::main();
}

fn launch() {
    let builder = Builder::new_from_string(include_str!("assets/layout.glade"));
    let window: Window = builder.get_object("window").unwrap();

    setup_window(&window);
    setup_inputs(&builder);

    window.connect_delete_event(|_, _| {
                                    gtk::main_quit();
                                    Inhibit(false)
                                });

    window.show_all();
}

fn setup_window(window: &Window) {
    let css = CssProvider::new();
    match css.load_from_data(include_str!("assets/style.css")) {
        Ok(_) => {
            let screen = window.get_display().unwrap().get_screen(0);
            let provider = STYLE_PROVIDER_PRIORITY_APPLICATION;

            StyleContext::add_provider_for_screen(&screen, &css, provider);
        }
        Err(msg) => {
            error!("Loading main.css failed: {}", msg);
        }
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

    for &(id, text) in &STRAIGHT_INPUT_BUTTONS {
        let input: Entry = input.clone();
        let input_buffer = input.get_buffer();
        let button: Button = builder.get_object(id).unwrap();

        button.connect_clicked(move |_button| {
                                   let pos = input_buffer.get_text().len();

                                   input.set_position(pos as i32);
                                   input_buffer.insert_text(pos as u16, text);
                                   input.grab_focus();
                                   input.set_position((pos + text.len()) as i32);
                               });
    }

    {
        let button: Button = builder.get_object("calc_button").unwrap();
        let input_buffer = input.get_buffer();
        let history: TextView = builder.get_object("history").unwrap();
        let history_buffer = history.get_buffer().unwrap();
        let history_scroll: ScrolledWindow = builder.get_object("history_scroll").unwrap();
        let va = history_scroll.get_vadjustment().unwrap();

        button.connect_clicked(move |_button| {
                                   calculat(&input_buffer, &history_buffer);
                                   input.grab_focus();
                                   va.set_value(va.get_upper());
                               });
    }
}

fn calculat(buffer: &EntryBuffer, history_buffer: &TextBuffer) {
    let text = buffer.get_text();

    if text.is_empty() {
        return;
    }

    let (start, end) = history_buffer.get_bounds();
    if start != end {
        history_buffer.insert(&mut history_buffer.get_end_iter(), "\n");
    }

    let input_text = &buffer.get_text();
    history_buffer.insert(&mut history_buffer.get_end_iter(), input_text);

    match exec_expression(&text) {
        Ok(result) => {
            let mut decimal_digits = 0;
            for n in 0..20usize {
                if (result * (10usize.pow(n as u32)) as f64) % 1f64 == 0f64 {
                    decimal_digits = n;
                    break;
                }
            }

            let res_text = &format!("{:.*}", decimal_digits, result);

            buffer.set_text(res_text);
            history_buffer.insert(&mut history_buffer.get_end_iter(),
                                  &format!("\n= {}", res_text));
        }
        Err((msg, pos)) => {
            buffer.set_text("Error");

            let mut mark = String::from(format!("\n{} ^", msg));

            let pos_from_right = input_text.len() - pos - 1;
            for _ in 0..pos_from_right {
                mark.push(' ');
            }

            history_buffer.insert(&mut history_buffer.get_end_iter(), &mark);
        }
    }
}
