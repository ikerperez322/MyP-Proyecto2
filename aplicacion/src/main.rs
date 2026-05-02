use std::rc::Rc;
use std::cell::RefCell;
use gtk::prelude::*;
use gtk::Application;
use rusqlite::Connection;
use std::path::Path;
use controlador::controlador::Controlador;

fn main() -> Result<(), Box<dyn::std::error::Error>> {

    let db_path = "bd/db.sqlite";
    let existe = Path::new(db_path).exists();
    let conn = Rc::new(Connection::open(db_path)?);
    conn.execute("PRAGMA foreign_keys = ON", [])?;
    
    let controlador = Rc::new(RefCell::new(Controlador::new(conn.clone())));
   
    if !existe {
        controlador.borrow().crea_bd()?;
    }

    gtk::init()?;
    
    let app = Application::builder()
        .application_id("com.reproductor.app")
        .build();
        
    app.connect_activate(move |app| {
        if let Err(e) = run_app(app, controlador.clone()) {
            eprintln!("Error al iniciar la aplicación: {}", e);
        }
    });
        
    app.run();
    
    return Ok(());
}


fn run_app(app: &Application, controlador: Rc<RefCell<Controlador>>) -> Result<(), Box<dyn std::error::Error>> {   
    vista::iniciar(app, controlador)?;    
    return Ok(());
}

