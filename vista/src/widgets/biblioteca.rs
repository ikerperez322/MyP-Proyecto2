use gtk::prelude::*;
use gtk::{Label, Orientation, Image, Box};
use gtk::{Builder, Stack};
use controlador::cancion_vista::CancionVista;

pub struct Biblioteca {
    pub stack: Stack,
    pub flow: gtk::FlowBox,
}

impl Biblioteca {
    pub fn new(builder: &Builder) -> Self {
        Self {
            stack: builder.object("biblioteca_stack").unwrap(),
            flow: builder.object("grid_view").unwrap(),
        }
    }

    pub fn cargar_en_flowbox(&self, canciones: Vec<CancionVista>) {
        for rola in canciones {
            let card = Self::crear_card(&rola);
            self.flow.insert(&card, -1);
        }
    }

    pub fn crear_card(cancion: &CancionVista) -> gtk::Box {
        let card = Box::new(Orientation::Vertical, 5);
        card.set_margin_top(10);
        card.set_margin_bottom(10);
        card.set_margin_start(10);
        card.set_margin_end(10);
        
        let imagen = Image::from_icon_name("media-optical-symbolic");

        let titulo = Label::new(Some(&cancion.titulo));
        titulo.set_wrap(true);
        titulo.set_max_width_chars(20);
        
        let album = Label::new(Some(&cancion.album));
        let artista = Label::new(Some(&cancion.artista));

        card.append(&imagen);
        card.append(&titulo);
        card.append(&album);
        card.append(&artista);
        
        card.add_css_class("card");
        
        return card;
    }      
}
