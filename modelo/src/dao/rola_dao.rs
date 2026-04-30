use rusqlite::Connection;
use std::rc::Rc;
use crate::rola_vista::RolaVista;
// use crate::modelo::entidades::rola::Rola;

use crate::entidades::rola::Rola;

pub struct RolaDao {
    conexion: Rc<Connection>,
}

impl RolaDao {
    pub fn new(conexion: Rc<Connection>) -> Self {
        return Self { conexion };
    }

    //agrega a la tabla rolas, regresa el número de columnas insertadas en caso de éxito, regresa Error en caso de que falle el sql
    pub fn agregar(&self, rola: &Rola) -> rusqlite::Result<i64> {
        // println!("Insertando rola...");
        self.conexion.execute("INSERT INTO rolas (id_performer, id_album, path, title, track, year, genre) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            (&rola.id_performer, &rola.id_album, &rola.path, &rola.title, &rola.track, &rola.year, &rola.genre,),)?;
        // return Ok(());
        return Ok(self.conexion.last_insert_rowid());
    }

    //busca una rola por su path, en caso de que no exista regresa None, regresa Err si falló el sql
    pub fn buscar_por_path(&self, path: &str) -> rusqlite::Result<Option<Rola>> {        
        let mut stmt = self.conexion.prepare("SELECT id_rola, id_performer, id_album, path, title, track, year, genre FROM rolas WHERE path = ?1")?;
        let mut colummnas = stmt.query([path])?;

        if let Some(col) = colummnas.next()? {
            return Ok(Some(Rola {
                id: (Some(col.get(0)?)),
                id_performer: (col.get(1)?),
                id_album: (col.get(2)?),
                path: (col.get(3)?),
                title: (col.get(4)?),
                track: (col.get(5)?),
                year: (col.get(6)?),
                genre: (col.get(7)?)
            }));
        }else {
            return Ok(None);
        }
    }

    //busca una rola por su título, en caso de que no exista regresa None, regresa Err si falló el sql
    pub fn buscar_por_titulo(&self, titulo: &str) -> rusqlite::Result<Option<Rola>> {
        let mut stmt = self.conexion.prepare("SELECT id_rola, id_performer, id_album, path, title, track, year, genre FROM rolas WHERE title = ?1")?;
        let mut colummnas = stmt.query([titulo])?;

        if let Some(col) = colummnas.next()? {
            return Ok(Some(Rola {
                id: (Some(col.get(0)?)),
                id_performer: (col.get(1)?),
                id_album: (col.get(2)?),
                path: (col.get(3)?),
                title: (col.get(4)?),
                track: (col.get(5)?),
                year: (col.get(6)?),
                genre: (col.get(7)?)
            }));
        }else {
            return Ok(None);
        }        
    }

    //busca una rola por su track, en caso de que no exista regresa None, regresa Err si falló el sql
    pub fn buscar_por_track(&self, track: &str) -> rusqlite::Result<Option<Rola>> {
        let mut stmt = self.conexion.prepare("SELECT id_rola, id_performer, id_album, path, title, track, year, genre FROM rolas WHERE track = ?1")?;
        let mut colummnas = stmt.query([track])?;

        if let Some(col) = colummnas.next()? {
            return Ok(Some(Rola {
                id: (Some(col.get(0)?)),
                id_performer: (col.get(1)?),
                id_album: (col.get(2)?),
                path: (col.get(3)?),
                title: (col.get(4)?),
                track: (col.get(5)?),
                year: (col.get(6)?),
                genre: (col.get(7)?)
            }));
        }else {
            return Ok(None);
        }
    }

    //busca una rola por su año}, en caso de que no exista regresa None, regresa Err si falló el sql
    pub fn buscar_por_agno(&self, agno: &str) -> rusqlite::Result<Option<Rola>> {
        let mut stmt = self.conexion.prepare("SELECT id_rola, id_performer, id_album, path, title, track, year, genre FROM rolas WHERE year = ?1")?;
        let mut colummnas = stmt.query([agno])?;

        if let Some(col) = colummnas.next()? {
            return Ok(Some(Rola {
                id: (Some(col.get(0)?)),
                id_performer: (col.get(1)?),
                id_album: (col.get(2)?),
                path: (col.get(3)?),
                title: (col.get(4)?),
                track: (col.get(5)?),
                year: (col.get(6)?),
                genre: (col.get(7)?)
            }));
        }else {
            return Ok(None);
        }
    }

    //busca una rola por su género, en caso de que no exista regresa None, regresa Err si falló el sql
    pub fn buscar_por_genero(&self, genero: &str) -> rusqlite::Result<Option<Rola>> {
        let mut stmt = self.conexion.prepare("SELECT id_rola, id_performer, id_album, path, title, track, year, genre FROM rolas WHERE genre = ?1")?;
        let mut colummnas = stmt.query([genero])?;

        if let Some(col) = colummnas.next()? {
            return Ok(Some(Rola {
                id: (Some(col.get(0)?)),
                id_performer: (col.get(1)?),
                id_album: (col.get(2)?),
                path: (col.get(3)?),
                title: (col.get(4)?),
                track: (col.get(5)?),
                year: (col.get(6)?),
                genre: (col.get(7)?)
            }));
        }else {
            return Ok(None);
        }
    }

    //elimina una rola por su id
    pub fn eliminar_por_id(&self, id: i32) -> rusqlite::Result<usize> {
        let filas = self.conexion.execute(
            "DELETE FROM rolas WHERE id_rola = ?1",
            [id],
        )?;        
        return Ok(filas);
    }
    
    //elimina una rola de rolas por su dirección de archvo, regresa el número de filas borradas para saber que en efecto eliminó una canción
    pub fn eliminar_por_path(&self, path: &str) -> rusqlite::Result<usize> {
        let filas = self.conexion.execute(
            "DELETE FROM rolas WHERE path = ?1",
            [path],
        )?;
        return Ok(filas);
    }

    pub fn obtener_todas_canciones_vista(&self) -> rusqlite::Result<Vec<RolaVista>> {

        let mut stmt = self.conexion.prepare("
        SELECT r.title, p.name, a.name, r.genre
        FROM rolas r
        JOIN performers p ON r.id_performer = p.id_performer
        JOIN albums a ON r.id_album = a.id_album
    ")?;
        
        let columnas = stmt.query_map([], |col| {
            Ok(RolaVista {
                titulo: col.get(0)?,
                artista: col.get(1)?,
                album: col.get(2)?,
                genero: col.get(3)?,
            })
        })?;
        
        let mut canciones = Vec::new();
        for r in columnas {
            canciones.push(r?);
        }
        
        return Ok(canciones);
    }
}

