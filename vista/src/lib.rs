use gtk::{Application, CssProvider, gdk::Display};
use std::cell::RefCell;
use std::rc::Rc;
use controlador::controlador::Controlador;

pub mod widgets;
pub mod ui;

pub fn iniciar(app: &Application, controlador: Rc<RefCell<Controlador>>) -> Result<(), Box<dyn std::error::Error>> {    
    cargar_css()?;
    ui::construir(app, controlador)?;    
    return Ok(());
}

fn cargar_css() -> Result<(), Box<dyn std::error::Error>> {
    let provider = CssProvider::new();
    provider.load_from_data(include_str!("styles.css"));
    
    gtk::style_context_add_provider_for_display(&Display::default().unwrap(), &provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    
    return Ok(());
}
