use gtk::prelude::*;
use gtk::{Adjustment, Button, Image, Label, Scale};
use controlador::cancion_vista::CancionVista;

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
    pub fn new(container: gtk::Box, play_button: gtk::Button, pause_button: gtk::Button, next_button: gtk::Button, song_label: gtk::Label, progress_bar: gtk::Scale) -> Self {
        
        let play_button = play_button;
        let pause_button = pause_button;
        let next_button = next_button;
        let cover_image = Image::from_icon_name("audio-x-generic-symbolic");
        let song_label = song_label;
        let progress_bar = progress_bar;
        
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

    //método para configurar el título de la canción que se va a poner en el reproductor
    pub fn set_cancion(&self, cancion: &CancionVista) {
        self.song_label.set_text(&cancion.titulo);
    }
    
}
