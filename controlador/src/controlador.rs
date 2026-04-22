// use rusqlite;
use rusqlite::Connection;
use modelo::minero::Minero;
use modelo::manejador_dao::ManejadorDao;
use modelo::dao::rola_dao::RolaDao;
// use modelo::minero::Minero;
use crate::cancion_vista::CancionVista;

pub struct Controlador<'a> {
    // manejador: ManejadorDao<'a>,
    // minero: Minero,
    // rola_dao: RolaDao<'a>,
    conexion: &'a Connection,
}

impl <'a> Controlador<'a> {
    pub fn new(conexion: &'a Connection) -> Self {
        return Self {
            // manejador: manejador,
            // minero: minero,
            // rola_dao: RolaDao::new(conexion),
            conexion: conexion,
        };
    }

    //método para invocar al minero desde la parte del usuario y poblar la base de datos
    pub fn poblar_bd(&self, raiz: &str) -> Result<(), Box<dyn::std::error::Error>> {
        let minero = Minero::new();
        let manejador = ManejadorDao::new(&self.conexion);        
        
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

        let rola_dao = RolaDao::new(&self.conexion);
        
        let canciones_bd = rola_dao.obtener_todas_canciones_vista()?;

        let mut canciones_vista: Vec<CancionVista> = Vec::new();
        
        for cancion in canciones_bd {
            let rola = CancionVista::new(cancion.titulo, cancion.artista, cancion.album);
            canciones_vista.push(rola);
        }
        
        return Ok(canciones_vista);
    }
    
}

