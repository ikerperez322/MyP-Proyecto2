use std::cell::RefCell;
use std::rc::Rc;
use gtk::prelude::*;
use gtk::{Application, CssProvider, StyleContext};
use gtk::gdk::Display;
use controlador::controlador::Controlador;

pub mod widgets;
pub mod ui;

pub fn iniciar(app: &Application, controlador: Rc<RefCell<Controlador>>) -> Result<(), Box<dyn std::error::Error>> {
    // cargar el css
    cargar_css()?;
    
    // construir la gui
    ui::construir(app, controlador)?;
    
    return Ok(());
}

fn cargar_css() -> Result<(), Box<dyn std::error::Error>> {
    let provider = CssProvider::new();
    provider.load_from_data(include_str!("styles.css"));
    
    StyleContext::add_provider_for_display(
        &Display::default().unwrap(),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
    
    return Ok(());
}
