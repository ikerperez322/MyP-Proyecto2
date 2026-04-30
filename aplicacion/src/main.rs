use std::rc::Rc;
use std::cell::RefCell;
use gtk::prelude::*;
use gtk::Application;
use modelo::reproductor::reproductor;
use rusqlite::Connection;
use std::path::Path;
// use modelo::minero::Minero;
// use modelo::manejador_dao::ManejadorDao;
use modelo::reproductor::reproductor::Reproductor;
use controlador::controlador::Controlador;
// use modelo::dao::rola_dao::RolaDao;
// use vista::Vista;
// use vista::builder;

fn main() -> Result<(), Box<dyn::std::error::Error>> {

    let db_path = "/home/kralmasol/Documents/modeladoProgramacion/proyecto2/bd/db.sqlite";
    let existe = Path::new(db_path).exists();
    let conn = Rc::new(Connection::open(db_path)?);
    conn.execute("PRAGMA foreign_keys = ON", [])?;
    
    let controlador = Rc::new(RefCell::new(Controlador::new(conn.clone(), db_path.to_string())));
   
    if !existe {
        controlador.borrow().crea_bd()?;
    }

    // reproductor.reproducir_cancion("/home/kralmasol/Music/pruebaMusica/gis's favs/06 d'yer mak'er.mp3");
    // controlador.borrow().reproduce_cancion("/home/kralmasol/Music/pruebaMusica/gis's favs/06 d'yer mak'er.mp3")?;
    
    
    gtk::init()?;
    
    // Crear la aplicación
    let app = Application::builder()
        .application_id("com.reproductor.app")
        .build();
    
    // Conectar la señal activate
    app.connect_activate(move |app| {
        if let Err(e) = run_app(app, controlador.clone()) {
            eprintln!("Error al iniciar la aplicación: {}", e);
        }
    });
    
    // Ejecutar la aplicación
    app.run();
    

    return Ok(());
    
}


fn run_app(app: &Application, controlador: Rc<RefCell<Controlador>>) -> Result<(), Box<dyn std::error::Error>> {
    // Inicializar controlador (ajusta según tu implementación)
    // let controlador = Controlador::new(conexion);
    
    // Iniciar la interfaz desde la biblioteca vista
    vista::iniciar(app, controlador)?;
    
    return Ok(());
}

