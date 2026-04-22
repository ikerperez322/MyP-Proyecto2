use gtk::prelude::*;
use gtk::Application;
use rusqlite::Connection;
use std::path::Path;
// use modelo::minero::Minero;
// use modelo::manejador_dao::ManejadorDao;
use controlador::controlador::Controlador;
use modelo::dao::rola_dao::RolaDao;
// use vista::Vista;
use vista::builder;

fn main() -> Result<(), Box<dyn::std::error::Error>> {
    // let minero = Minero {};
    // minero.mina("/home/kralmasol/Music/pruebaMusica");

    // let conexion = Connection::open("/home/kralmasol/Documents/modeladoProgramacion/proyecto2/bd/db.sqlite")?;

//     conn.execute("PRAGMA foreign_keys = ON", [])?;

    let esquema = "CREATE TABLE types (id_type INTEGER PRIMARY KEY, description TEXT);
INSERT INTO types VALUES(0,'Person');
INSERT INTO types VALUES(1,'Group');
INSERT INTO types VALUES(2,'Unknown');
CREATE TABLE performers (id_performer INTEGER PRIMARY KEY, id_type INTEGER, name TEXT, FOREIGN KEY (id_type) REFERENCES types(id_type));
CREATE TABLE persons (id_person INTEGER PRIMARY KEY, stage_name TEXT, real_name TEXT, birth_date TEXT, death_date TEXT);
CREATE TABLE groups (id_group INTEGER PRIMARY KEY, name TEXT, start_date TEXT, end_date TEXT);
CREATE TABLE in_group (id_person INTEGER, id_group INTEGER, PRIMARY KEY   (id_person, id_group), FOREIGN KEY (id_person) REFERENCES persons(id_person), FOREIGN KEY (id_group) REFERENCES groups(id_group));
CREATE TABLE albums (id_album INTEGER PRIMARY KEY, path TEXT, name TEXT, year INTEGER);
CREATE TABLE rolas (id_rola INTEGER PRIMARY KEY, id_performer INTEGER, id_album INTEGER, path TEXT, title TEXT, track INTEGER, year INTEGER, genre TEXT, FOREIGN KEY (id_performer) REFERENCES performers(id_performer), FOREIGN KEY (id_album) REFERENCES albums(id_album));
";

    // conexion.execute_batch(esquema)?;


    let db_path = "/home/kralmasol/Documents/modeladoProgramacion/proyecto2/bd/db.sqlite";
    let existe = Path::new(db_path).exists();

    let conn = Connection::open(db_path)?;
    conn.execute("PRAGMA foreign_keys = ON", [])?;

    if !existe {
        conn.execute_batch(esquema)?;
    }
    
    // let servicio = ManejadorDao::new(&conn);
    // let minero = Minero::new();
    
    let controlador = Controlador::new(&conn);

    controlador.poblar_bd("/home/kralmasol/Music/pruebaMusica")?;

    let dao_rola = RolaDao::new(&conn);
    if let Ok(rola) = dao_rola.buscar_por_titulo("American pie") {
        if let Some(rolita) = rola {
            println!("Canción:\n{:#?}", rolita);
        }
    }

    if let Ok(rola) = dao_rola.buscar_por_titulo("Voices. Vangelis") {
        if let Some(rolita) = rola {
            println!("Canción:\n{:#?}", rolita);
        }
    }

    let app = Application::new(
        Some("com.ejemplo.reproductor"),
        Default::default(),
    );

    app.connect_activate(move |app| {

        // let conexion = rusqlite::Connection::open("db.sqlite")
        //     .expect("No se pudo abrir la BD");

        let controlador = Controlador::new(&conn);

        if let Err(e) = builder::crea_ui(app, &controlador) {
            eprintln!("Error al crear UI: {}", e);
        }
    });

    app.run();

    return Ok(());
    
}
