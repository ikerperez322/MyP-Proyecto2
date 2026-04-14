use gtk::prelude::*;
use gtk::{Builder, Button, Scale, Label};

pub struct Reproductor {
    pub play: Button,
    pub pause: Button,
    pub progress: Scale,
    pub label: Label,
}

impl Reproductor {
    pub fn new(builder: &Builder) -> Self {
        Self {
            play: builder.object("play_button").unwrap(),
            pause: builder.object("pause_button").unwrap(),
            progress: builder.object("progress_bar").unwrap(),
            label: builder.object("song_label").unwrap(),
        }
    }
}
