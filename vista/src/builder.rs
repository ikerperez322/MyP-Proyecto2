use rusqlite::Connection;
use gtk::prelude::*;
// use gtk::Builder;
use gtk::{Builder, CssProvider, StyleContext};
use gtk::gdk::Display;
use controlador::controlador::Controlador;
use crate::widgets::biblioteca::Biblioteca;

// pub fn cargar_ui(controlador: &Controlador) -> Builder {
//     // let builder = Builder::from_string(include_str!("ui/main.ui"));
//     // let window: gtk::ApplicationWindow = builder.object("main_window").unwrap();

//     // let flow: gtk::FlowBox = builder.object("grid_view").expect("No se encontró grid_view");
// }

pub fn crea_ui(app: &gtk::Application, controlador: &Controlador) -> Result<(), Box<dyn::std::error::Error>> {

    // let controlador = Controlador::new(&conexion);

    let provider = CssProvider::new();
    // provider.load_from_path("src/ui/styles.css");
    provider.load_from_data(include_str!("ui/styles.css"));
    
    StyleContext::add_provider_for_display(
        &Display::default().unwrap(),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
    
    // let builder = gtk::Builder::from_file("src/ui/main.ui");
    let builder = gtk::Builder::from_string(include_str!("ui/main.ui"));

    let window: gtk::ApplicationWindow = builder.object("main_window").unwrap();

    window.set_application(Some(app));

    // let canciones = obtener_canciones_desde_bd();

    let canciones = controlador.obtener_canciones()?;
    
    let biblioteca = Biblioteca::new(&builder);

    biblioteca.cargar_en_flowbox(canciones);
    
    window.show();

    return Ok(());
}

