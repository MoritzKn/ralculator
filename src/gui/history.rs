use gtk::prelude::*;
use gtk::{ScrolledWindow, TextBuffer, TextView};

#[derive(Clone)]
pub struct History {
    text_view: TextView,
    buffer: TextBuffer,
    history_scroll: ScrolledWindow,
}

impl History {
    pub fn new(text_view: TextView, history_scroll: ScrolledWindow) -> History {
        let buffer = text_view.get_buffer().unwrap();
        History {
            text_view,
            buffer,
            history_scroll,
        }
    }

    pub fn add(&self, text: &str) {
        if self.is_empty() {
            self.insert("\n");
        }

        self.insert(text);

        // FIXME: New content is added two lines under the bottom
        let va = self.history_scroll.get_vadjustment().unwrap();
        va.set_value(va.get_upper() + 15f64);
    }

    fn insert(&self, text: &str) {
        self.buffer.insert(&mut self.buffer.get_end_iter(), text);
    }

    pub fn is_empty(&self) -> bool {
        let (start, end) = self.buffer.get_bounds();
        start != end
    }
}
