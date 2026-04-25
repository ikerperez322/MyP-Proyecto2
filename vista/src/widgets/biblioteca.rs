use gtk::prelude::*;
use gtk::{FlowBox, ColumnView, Box, Label, Image, Orientation, ScrolledWindow, SingleSelection};
// use gtk::gio::ListStore;
use controlador::cancion_vista::CancionVista;

#[derive(Clone)]
pub struct Biblioteca {
    flowbox: FlowBox,
    column_view: ColumnView,
}

impl Biblioteca {
    pub fn new(flowbox: FlowBox, column_view: ColumnView) -> Self {
        return Self {
            flowbox,
            column_view,
        }
    }
    
    pub fn cargar_en_flowbox(&self, canciones: &[CancionVista]) {
        // self.flowbox.remove_all();
        
        for cancion in canciones {
            let card = Self::crear_card(cancion);
            self.flowbox.insert(&card, -1);
        }
    }
     
    fn crear_card(cancion: &CancionVista) -> Box {
        let card = Box::new(Orientation::Vertical, 5);

        card.set_width_request(180);
        card.set_height_request(220);
        
        card.set_margin_top(10);
        card.set_margin_bottom(10);
        card.set_margin_start(10);
        card.set_margin_end(10);
        card.add_css_class("card");
        
        // Imagen
        let imagen = Image::from_icon_name("audio-x-generic-symbolic");
        imagen.set_pixel_size(120);
        
        // Título
        let titulo = Label::new(Some(&cancion.titulo));
        titulo.set_wrap(true);
        titulo.set_max_width_chars(15);
        titulo.set_lines(2);
        // titulo.set_max_width_chars(20);
        titulo.add_css_class("card-title");
        titulo.set_halign(gtk::Align::Center);
        
        // Artista
        let artista = Label::new(Some(&cancion.artista));
        artista.add_css_class("card-artist");
        artista.set_halign(gtk::Align::Center);
        artista.set_ellipsize(gtk::pango::EllipsizeMode::End); 
        
        // Álbum
        let album = Label::new(Some(&cancion.album));
        album.add_css_class("card-album");
        album.set_halign(gtk::Align::Center);
        album.set_ellipsize(gtk::pango::EllipsizeMode::End);
        
        card.append(&imagen);
        card.append(&titulo);
        card.append(&artista);
        card.append(&album);
        
        return card;
    }
    
    // pub fn configurar_tabla(&self, canciones: Vec<CancionVista>) {
    //     // Aquí irá la configuración de la tabla
    //     println!("Configurando tabla con {} canciones", canciones.len());
        
        
    // }

}
