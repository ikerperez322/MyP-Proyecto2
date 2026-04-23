use gtk::prelude::*;
use gtk::{Adjustment, Button, Image, Label, Scale};

pub struct Reproductor {
    container: gtk::Box,
    play_button: Button,
    pause_button: Button,
    next_button: Button,
    cover_image: Image,
    song_label: Label,
    progress_bar: Scale,
}

impl Reproductor {
    pub fn new(container: gtk::Box) -> Self {
        
        let play_button = Button::with_label("▶");
        let pause_button = Button::with_label("⏸");
        let next_button = Button::with_label("⏭");
        let cover_image = Image::from_icon_name("audio-x-generic-symbolic");
        let song_label = Label::new(Some("No hay canción seleccionada"));
        let progress_bar = Scale::new(gtk::Orientation::Horizontal, None::<&Adjustment>);
        
        return Self {
            container,
            play_button,
            pause_button,
            next_button,
            cover_image,
            song_label,
            progress_bar,
        }
    }
    
    // Aquí irán los métodos para controlar la reproducción
}
