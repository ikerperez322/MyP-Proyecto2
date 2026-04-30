use gtk::prelude::*;
use gtk::gio;
use gtk::glib;
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
        let biblioteca = Self {
            flowbox,
            column_view,
        };
        return biblioteca;
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

        // card.set_width_request(180);
        // card.set_height_request(220);
        card.set_size_request(180, 220);
        card.set_valign(gtk::Align::Center);
        
        card.set_margin_top(10);
        card.set_margin_bottom(10);
        card.set_margin_start(10);
        card.set_margin_end(10);
        card.add_css_class("card");
        card.set_data("cancion", cancion.clone());

        let top = Box::new(Orientation::Vertical, 5);
        let bottom = Box::new(Orientation::Vertical, 2);
                
        // Imagen
        let imagen = Image::from_icon_name("audio-x-generic-symbolic");
        imagen.set_halign(gtk::Align::Center);
        imagen.set_pixel_size(120);
        
        // Título
        let titulo = Label::new(Some(&cancion.titulo));
        titulo.add_css_class("card-title");
        titulo.set_wrap(true);
        titulo.set_max_width_chars(15);
        titulo.set_lines(2);
        titulo.set_ellipsize(gtk::pango::EllipsizeMode::End);
        // titulo.set_max_width_chars(20);
        titulo.set_halign(gtk::Align::Center);
        
        // Artista
        let artista = Label::new(Some(&cancion.artista));
        artista.add_css_class("card-artist");
        artista.set_max_width_chars(18);
        artista.set_halign(gtk::Align::Center);
        artista.set_ellipsize(gtk::pango::EllipsizeMode::End); 
        
        // Álbum
        let album = Label::new(Some(&cancion.album));
        album.add_css_class("card-album");
        album.set_max_width_chars(18);
        album.set_halign(gtk::Align::Center);
        album.set_ellipsize(gtk::pango::EllipsizeMode::End);
        
        // card.append(&imagen);
        // card.append(&titulo);
        // card.append(&artista);
        // card.append(&album);

        top.append(&imagen);
        
        bottom.append(&titulo);
        bottom.append(&artista);
        bottom.append(&album);
        
        card.append(&top);
        card.append(&bottom);        
        
        return card;
    }


    //crea la vista de tabla para las canciones
    pub fn cargar_tabla(&self, canciones: &Vec<CancionVista>) {

        let store = gio::ListStore::new::<glib::BoxedAnyObject>();

        for c in canciones {
            store.append(&glib::BoxedAnyObject::new(c.clone()));
        }

        let seleccion = gtk::SingleSelection::new(Some(store));
        self.column_view.set_model(Some(&seleccion));        
    }
    
    //configura las columnas en la vista de tabla para la gui
    fn configurar_columnas(&self) {
        //-----------TITULO--------------
        let fabrica_titulo = gtk::SignalListItemFactory::new();

        fabrica_titulo.connect_setup(|_,item| {
            // let item = item.downcast_ref::<gtk::ListItem>().unwrap();
            // let label = gtk::Label::new(None);
            // label.set_xalign(0.0);
            // label.set_margin_start(10);
            // label.set_margin_end(10);
            // label.set_margin_top(4);
            // label.set_margin_bottom(4);
            // item.set_child(Some(&label));
            let item = item.downcast_ref::<gtk::ListItem>().unwrap();

            let box_cell = gtk::Box::new(Orientation::Horizontal, 0);
            box_cell.add_css_class("cell");
            
            let label = gtk::Label::new(None);
            label.add_css_class("col-titulo");
            label.set_xalign(0.0);
            label.set_margin_start(10);
            label.set_margin_end(10);
            label.set_margin_top(4);
            label.set_margin_bottom(4);

            label.set_ellipsize(gtk::pango::EllipsizeMode::End);
            label.set_max_width_chars(30);
            label.set_wrap(false);
            
            box_cell.append(&label);
            item.set_child(Some(&box_cell));
        });

        fabrica_titulo.connect_bind(|_,item| {
            // let item = item.downcast_ref::<gtk::ListItem>().unwrap();
            // let obj = item.item().unwrap();
            // let boxed = obj.downcast::<glib::BoxedAnyObject>().unwrap();
            // let cancion = boxed.borrow::<CancionVista>();

            // let label = item.child().unwrap().downcast::<gtk::Label>().unwrap();
            // label.set_text(&cancion.titulo);
            let item = item.downcast_ref::<gtk::ListItem>().unwrap();
            
            let obj = item.item().unwrap();
            let boxed = obj.downcast::<glib::BoxedAnyObject>().unwrap();
            let cancion = boxed.borrow::<CancionVista>();
            
            let box_cell = item.child().unwrap().downcast::<gtk::Box>().unwrap();
            let label = box_cell.first_child().unwrap().downcast::<gtk::Label>().unwrap();
            
            label.set_text(&cancion.titulo);
        });
      
        let col_titulo = gtk::ColumnViewColumn::new(Some("Título"), Some(fabrica_titulo));
        col_titulo.set_resizable(true);
        col_titulo.set_expand(true);
        self.column_view.append_column(&col_titulo);
        
        //------ARTISTA-------
         let fabrica_artista = gtk::SignalListItemFactory::new();

        fabrica_artista.connect_setup(|_, item| {
            // let item = item.downcast_ref::<gtk::ListItem>().unwrap();
            // let label = gtk::Label::new(None);
            // label.set_xalign(0.0);
            // label.set_margin_start(10);
            // label.set_margin_end(10);
            // label.set_margin_top(4);
            // label.set_margin_bottom(4);
            // item.set_child(Some(&label));
            let item = item.downcast_ref::<gtk::ListItem>().unwrap();

            let box_cell = gtk::Box::new(Orientation::Horizontal, 0);
            box_cell.add_css_class("cell");
            
            let label = gtk::Label::new(None);
            label.add_css_class("col-artista");
            label.set_xalign(0.0);
            label.set_margin_start(10);
            label.set_margin_end(10);
            label.set_margin_top(4);
            label.set_margin_bottom(4);

            label.set_ellipsize(gtk::pango::EllipsizeMode::End);
            label.set_max_width_chars(30);
            label.set_wrap(false);
            
            box_cell.append(&label);
            item.set_child(Some(&box_cell));
        });

        fabrica_artista.connect_bind(|_, item| {
            // let item = item.downcast_ref::<gtk::ListItem>().unwrap();
            // let obj = item.item().unwrap();
            // let boxed = obj.downcast::<glib::BoxedAnyObject>().unwrap();
            // let cancion = boxed.borrow::<CancionVista>();

            // let label = item.child().unwrap().downcast::<gtk::Label>().unwrap();
            // label.set_text(&cancion.artista);
            let item = item.downcast_ref::<gtk::ListItem>().unwrap();
            
            let obj = item.item().unwrap();
            let boxed = obj.downcast::<glib::BoxedAnyObject>().unwrap();
            let cancion = boxed.borrow::<CancionVista>();
            
            let box_cell = item.child().unwrap().downcast::<gtk::Box>().unwrap();
            let label = box_cell.first_child().unwrap().downcast::<gtk::Label>().unwrap();
            
            label.set_text(&cancion.artista);
        });

        let col_artista = gtk::ColumnViewColumn::new(Some("Artista"), Some(fabrica_artista));
        self.column_view.append_column(&col_artista);

        //--------ALBUM---------
        let fabrica_album = gtk::SignalListItemFactory::new();

        fabrica_album.connect_setup(|_, item| {
            // let item = item.downcast_ref::<gtk::ListItem>().unwrap();
            // let label = gtk::Label::new(None);
            // label.set_xalign(0.0);
            // label.set_margin_start(10);
            // label.set_margin_end(10);
            // label.set_margin_top(4);
            // label.set_margin_bottom(4);
            // item.set_child(Some(&label));
            let item = item.downcast_ref::<gtk::ListItem>().unwrap();

            let box_cell = gtk::Box::new(Orientation::Horizontal, 0);
            box_cell.add_css_class("cell");
            
            let label = gtk::Label::new(None);
            label.add_css_class("col-album");
            label.set_xalign(0.0);
            label.set_margin_start(10);
            label.set_margin_end(10);
            label.set_margin_top(4);
            label.set_margin_bottom(4);

            label.set_ellipsize(gtk::pango::EllipsizeMode::End);
            label.set_max_width_chars(30);
            label.set_wrap(false);
            
            box_cell.append(&label);
            item.set_child(Some(&box_cell));
        });

        fabrica_album.connect_bind(|_, item| {
            // let item = item.downcast_ref::<gtk::ListItem>().unwrap();
            // let obj = item.item().unwrap();
            // let boxed = obj.downcast::<glib::BoxedAnyObject>().unwrap();
            // let cancion = boxed.borrow::<CancionVista>();

            // let label = item.child().unwrap().downcast::<gtk::Label>().unwrap();
            // label.set_text(&cancion.album);
            let item = item.downcast_ref::<gtk::ListItem>().unwrap();
            
            let obj = item.item().unwrap();
            let boxed = obj.downcast::<glib::BoxedAnyObject>().unwrap();
            let cancion = boxed.borrow::<CancionVista>();
            
            let box_cell = item.child().unwrap().downcast::<gtk::Box>().unwrap();
            let label = box_cell.first_child().unwrap().downcast::<gtk::Label>().unwrap();
            
            label.set_text(&cancion.album);
        });

        let col_album = gtk::ColumnViewColumn::new(Some("Álbum"), Some(fabrica_album));
        self.column_view.append_column(&col_album);

        //-----GÉNERO----------
        let fabrica_genero = gtk::SignalListItemFactory::new();

        fabrica_genero.connect_setup(|_,item| {
            // let item = item.downcast_ref::<gtk::ListItem>().unwrap();
            // let label = gtk::Label::new(None);
            // label.set_xalign(0.0);
            // label.set_margin_start(10);
            // label.set_margin_end(10);
            // label.set_margin_top(4);
            // label.set_margin_bottom(4);
            // item.set_child(Some(&label));
            let item = item.downcast_ref::<gtk::ListItem>().unwrap();

            let box_cell = gtk::Box::new(Orientation::Horizontal, 0);
            box_cell.add_css_class("cell");
            
            let label = gtk::Label::new(None);
            label.add_css_class("col-genero");
            label.set_xalign(0.0);
            label.set_margin_start(10);
            label.set_margin_end(10);
            label.set_margin_top(4);
            label.set_margin_bottom(4);

            label.set_ellipsize(gtk::pango::EllipsizeMode::End);
            label.set_max_width_chars(30);
            label.set_wrap(false);
            
            box_cell.append(&label);
            item.set_child(Some(&box_cell));
        });

        fabrica_genero.connect_bind(|_,item| {
            // let item = item.downcast_ref::<gtk::ListItem>().unwrap();
            // let obj = item.item().unwrap();
            // let boxed = obj.downcast::<glib::BoxedAnyObject>().unwrap();
            // let cancion = boxed.borrow::<CancionVista>();

            // let label = item.child().unwrap().downcast::<gtk::Label>().unwrap();
            // label.set_text(&cancion.genero);
            let item = item.downcast_ref::<gtk::ListItem>().unwrap();
            
            let obj = item.item().unwrap();
            let boxed = obj.downcast::<glib::BoxedAnyObject>().unwrap();
            let cancion = boxed.borrow::<CancionVista>();
            
            let box_cell = item.child().unwrap().downcast::<gtk::Box>().unwrap();
            let label = box_cell.first_child().unwrap().downcast::<gtk::Label>().unwrap();
            
            label.set_text(&cancion.genero);
        });

        let col_genero = gtk::ColumnViewColumn::new(Some("Género"), Some(fabrica_genero));
        self.column_view.append_column(&col_genero);
    }

}
