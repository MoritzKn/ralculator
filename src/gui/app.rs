use gtk::prelude::*;
use gtk::{Window, Builder, Button, Entry, EntryBuffer, TextView, TextBuffer, ScrolledWindow,
          CssProvider, StyleContext, STYLE_PROVIDER_PRIORITY_APPLICATION, Error};

use input_error::InputError;
use exec::parse_and_execute;
use text_range::TextRange;

static LAYOUT_GLADE: &str = include_str!("layout.glade");
static STYLE_CSS: &str = include_str!("style.css");

type ButtoInfo = (&'static str, &'static str);

static STRAIGHT_INPUT_BUTTONS: [ButtoInfo; 15] = [
    ("num_1_button", "1"),
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
    ("decimal_point_button", " . "),
];

pub struct App {
    builder: Builder,
    window: Window,
}

impl App {
    pub fn new() -> App {
        let builder = Builder::new_from_string(LAYOUT_GLADE);
        let window: Window = builder.get_object("window").unwrap();

        App { builder, window }
    }

    pub fn init(&self) {
        self.window.show_all();
        self.add_css();
        self.setup_inputs();
    }

    pub fn setup_new() -> App {
        let app = App::new();
        app.init();
        app
    }

    pub fn on_close<F: Fn() -> Inhibit + 'static>(&self, f: F) -> u64 {
        self.window.connect_delete_event(move |_, _| f())
    }

    fn add_css(&self) {
        let screen = self.window.get_display().unwrap().get_screen(0);

        match load_css() {
            Ok(provider) => {
                StyleContext::add_provider_for_screen(
                    &screen,
                    &provider,
                    STYLE_PROVIDER_PRIORITY_APPLICATION,
                );
            }
            Err(err) => {
                error!("Loading CSS failed: {}", err);
            }
        }
    }

    fn setup_inputs(&self) {
        let input: Entry = self.builder.get_object("input").unwrap();
        input.set_alignment(1f32);

        let history = {
            let text_view = self.builder.get_object("history").unwrap();
            let history_scroll = self.builder.get_object("history_scroll").unwrap();

            History::new(text_view, history_scroll)
        };

        for &(id, text) in &STRAIGHT_INPUT_BUTTONS {
            let input = input.clone();
            let button: Button = self.builder.get_object(id).unwrap();
            button.connect_clicked(move |_| insert_text(&input, text));
        }

        {
            let input_buffer = input.get_buffer();
            let button: Button = self.builder.get_object("calc_button").unwrap();

            button.connect_clicked(
                move |_| {
                    handle_execute(&input_buffer, &history);
                    input.grab_focus();
                }
            );
        }
    }
}

fn load_css() -> Result<CssProvider, Error> {
    let provider = CssProvider::new();
    match provider.load_from_data(STYLE_CSS) {
        Ok(_) => Ok(provider),
        Err(err) => Err(err),
    }
}

fn insert_text(input: &Entry, text: &str) {
    let pos = input.get_position();
    let buffer = input.get_buffer();
    buffer.insert_text(pos as u16, text);
    input.grab_focus();
    input.set_position(pos + text.len() as i32);
}

#[derive(Clone)]
struct History {
    text_view: TextView,
    buffer: TextBuffer,
    history_scroll: ScrolledWindow,
}

impl History {
    fn new(text_view: TextView, history_scroll: ScrolledWindow) -> History {
        let buffer = text_view.get_buffer().unwrap();
        History {
            text_view,
            buffer,
            history_scroll,
        }
    }

    fn add(&self, text: &str) {
        if self.is_empty() {
            self.insert("\n");
        }

        self.insert(text);

        // FIXME
        let va = self.history_scroll.get_vadjustment().unwrap();
        va.set_value(va.get_upper() + 15f64);
    }

    fn insert(&self, text: &str) {
        self.buffer.insert(&mut self.buffer.get_end_iter(), text);
    }

    fn is_empty(&self) -> bool {
        let (start, end) = self.buffer.get_bounds();
        start != end
    }
}

fn handle_execute(input_buffer: &EntryBuffer, history: &History) {
    let input = input_buffer.get_text();

    if input.is_empty() {
        return;
    }

    history.add(&input);

    match parse_and_execute(&input) {
        Ok(result) => {
            let res_text = num_as_string(result);
            input_buffer.set_text(&res_text);
            history.add(&format!("= {}", &res_text));
        }
        Err(InputError { msg, pos }) => {
            input_buffer.set_text("Error");

            let marker = pos.fill('^');
            let pad = TextRange::new(pos.end, input.len()).fill(' ');

            history.add(&format!("{} {}{}", msg, marker, pad));
        }
    }
}

fn num_as_string(num: f64) -> String {
    fn decimal_digits(num: f64) -> usize {
        for n in 0..20usize {
            if (num * (10usize.pow(n as u32)) as f64) % 1f64 == 0f64 {
                return n;
            }
        }
        panic!("Could not determine decimal digits");
    }

    format!("{:.*}", decimal_digits(num), num)
}
