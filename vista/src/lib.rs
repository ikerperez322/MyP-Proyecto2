use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

pub mod builder;
pub mod widgets;

// use builder::cargar_ui;
use widgets::reproductor::Reproductor;
use widgets::biblioteca::Biblioteca;

use controlador::controlador::Controlador;

pub struct Vista {
    pub window: ApplicationWindow,
    pub reproductor: Reproductor,
    pub biblioteca: Biblioteca,
}

// impl Vista {
//     pub fn new(app: &Application, controlador: &Controlador) -> Self {
//         // let builder = cargar_ui(controlador);

//         let window: ApplicationWindow = builder.object("main_window").unwrap();

//         window.set_application(Some(app));

//         let reproductor = Reproductor::new(&builder);
//         let biblioteca = Biblioteca::new(&builder);

//         return Self {
//             window,
//             reproductor,
//             biblioteca,
//         };
//     }
// }
