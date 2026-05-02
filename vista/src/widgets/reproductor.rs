use gtk::prelude::*;
use gtk::Label;
use std::cell::RefCell;
use std::rc::Rc;
use controlador::{controlador::Controlador, cancion_vista::CancionVista};

pub struct Reproductor {
    song_label: Label,
    cancion_actual: Rc<RefCell<Option<CancionVista>>>,    
}

impl Reproductor {
    pub fn new(play_button: gtk::Button, song_label: gtk::Label, controlador: Rc<RefCell<Controlador>>) -> Self {
        
        let play_button = play_button;
        let song_label = song_label;
        let cancion_actual = Rc::new(RefCell::new(None::<CancionVista>));

        let controlador_clon = controlador.clone();
        let cancion_actual_clon = Rc::clone(&cancion_actual);
        
        play_button.connect_clicked(move |_| {
            if let Some(cancion) = cancion_actual_clon.borrow().as_ref() {
                println!("Reproduciendo: {}", cancion.titulo);
                
                if let Err(e) = controlador_clon.borrow_mut().reproduce_cancion(cancion.path.as_str()) {
                    println!("Error reproduciendo canción: {}", e);
                };
            } else {
                println!("No hay canción seleccionada");
            }
        });
        
        return Self {
            song_label,
            cancion_actual,
        }
    }

    //método para que el reproductor sepa que canción se seleccionó y se va a reproducir la canción que se va a poner en el reproductor
    pub fn set_cancion(&self, cancion: &CancionVista) {
        self.song_label.set_text(&cancion.titulo);
        *self.cancion_actual.borrow_mut() = Some(cancion.clone());               
    }
    
}
