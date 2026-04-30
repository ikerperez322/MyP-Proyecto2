use std::cell::RefCell;
use std::rc::Rc;

use controlador::controlador::Controlador;
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
    cancion_actual: Rc<RefCell<Option<CancionVista>>>,    
}

impl Reproductor {
    pub fn new(container: gtk::Box, play_button: gtk::Button, pause_button: gtk::Button, next_button: gtk::Button, song_label: gtk::Label, progress_bar: gtk::Scale, controlador: Rc<RefCell<Controlador>>) -> Self {
        
        let play_button = play_button;
        let pause_button = pause_button;
        let next_button = next_button;
        let cover_image = Image::from_icon_name("audio-x-generic-symbolic");
        let song_label = song_label;
        let progress_bar = progress_bar;
        let cancion_actual = Rc::new(RefCell::new(None::<CancionVista>));

        let controlador_clon = controlador.clone();
        let cancion_actual_clon = Rc::clone(&cancion_actual);
        
        play_button.connect_clicked(move |_| {
            if let Some(cancion) = cancion_actual_clon.borrow().as_ref() {
                println!("Reproduciendo: {}", cancion.titulo);
                
                controlador_clon.borrow_mut().reproduce_cancion(cancion.path.as_str());
            } else {
                println!("No hay canción seleccionada");
            }
        });
        
        return Self {
            container,
            play_button,
            pause_button,
            next_button,
            cover_image,
            song_label,
            progress_bar,
            cancion_actual,
        }
    }

    //método para que el reproductor sepa que canción se seleccionó y se va a reproducir la canción que se va a poner en el reproductor
    pub fn set_cancion(&self, cancion: &CancionVista) {
        self.song_label.set_text(&cancion.titulo);
        *self.cancion_actual.borrow_mut() = Some(cancion.clone());               
    }
    
}
