use gtk::prelude::*;
use gtk::{Builder, Stack};

pub struct Biblioteca {
    pub stack: Stack,
}

impl Biblioteca {
    pub fn new(builder: &Builder) -> Self {
        Self {
            stack: builder.object("biblioteca_stack").unwrap(),
        }
    }
}
