use std::path::Path;
use std::rc::Rc;
// use rusqlite;
use rusqlite::Connection;
use modelo::minero::Minero;
use modelo::manejador_dao::ManejadorDao;
use modelo::dao::rola_dao::RolaDao;
use modelo::reproductor::reproductor::Reproductor;
// use modelo::minero::Minero;
use crate::cancion_vista::CancionVista;

pub struct Controlador {
    conexion: Rc<Connection>,
    ruta_base_datos: String,
}

impl Controlador {
    pub fn new(conexion: Rc<Connection>, ruta_base_datos: String) -> Self {
        return Self {
            conexion: conexion,
            ruta_base_datos: ruta_base_datos,
        };
    }

    //método para crear la bd
    pub fn crea_bd(&self) -> Result<(), Box<dyn::std::error::Error>> {

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

        // self.conexion.execute("PRAGMA foreign_keys = ON", [])?;
        
        // let ruta = Path::new(&self.ruta_base_datos);
        
        self.conexion.execute_batch(esquema)?;

        
        // println!("Antes de stmt");
        // let mut stmt = self.conexion.prepare(
        //     "SELECT name FROM sqlite_master WHERE type='table' AND name='rolas';"
        // )?;

        // let mut columnas = stmt.query([])?;

        // println!("Antes de columnas.next.is_some");
        // if columnas.next()?.is_some() {
        //     return Ok(());
        // }

        // println!("Después de columnas.next.is_some");
        


        return Ok(());
    }
    
    //método para invocar al minero desde la parte del usuario y poblar la base de datos
    pub fn poblar_bd(&self, raiz: &Path) -> Result<(), Box<dyn::std::error::Error>> {

        // let ruta = Path::new(&self.ruta_base_datos);
        
        // if !ruta.exists(){
        //     self.crea_bd()?;
        // }
        
        let minero = Minero::new();
        let manejador = ManejadorDao::new(self.conexion.clone());        
        
        let canciones = minero.mina(raiz)?;
        
        for cancion in canciones {
            match manejador.agrega_rola(&cancion) {
                Ok(id) => {
                    println!("Insertada canción con id {}", id);
                    // println!("Canción:\n{:#?}", cancion);
                },
                Err(e) => {
                    eprintln!("Error insertando canción: {}", e);
                    // println!("Canción:\n{:#?}", cancion);
                },
            }
        }

        return Ok(());
    }

    //método para obtener las canciones de la bd en formato  para la vista
    pub fn obtener_canciones(&self) -> rusqlite::Result<Vec<CancionVista>> {

        let rola_dao = RolaDao::new(self.conexion.clone());
        let canciones_bd = rola_dao.obtener_todas_canciones_vista()?;
        let mut canciones_vista: Vec<CancionVista> = Vec::new();
        
        for cancion in canciones_bd {
            let rola = CancionVista::new(cancion.titulo, cancion.artista, cancion.album, cancion.genero);
            canciones_vista.push(rola);
        }
        
        return Ok(canciones_vista);
    }

    //reproduce una canción desde la interfaz gráfica, manda a llamar al reproductor
    pub fn reproduce_cancion(&self, path_cancion: &str) -> Result<(), Box<dyn::std::error::Error>> {
        let reproductor = Reproductor::new();
        reproductor.reproduce_cancion(path_cancion)?;
        return Ok(());
    }
    
}

