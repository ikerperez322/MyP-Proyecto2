use rusqlite::Connection;
use rusqlite::params;
use std::rc::Rc;
use crate::entidades::album::Album;

pub struct AlbumDao {
    conexion: Rc<Connection>,
}

impl AlbumDao {
    pub fn new(conexion: Rc<Connection>) -> Self {
        return Self { conexion };
    }

    //agrega a la tabla album, regresa el número de columnas insertadas en caso de éxito, regresa Error en caso de que falle el sql
    pub fn agregar(&self, album: &Album) -> rusqlite::Result<i64> {
        self.conexion.execute("INSERT INTO albums (path, name, year) VALUES (?1, ?2, ?3)",
            (&album.path, &album.name, &album.year,),)?;
        return Ok(self.conexion.last_insert_rowid());
    }

    //regresa un album por su path, en caso de que no exista regresa None, regresa Err si falló el sql
    pub fn buscar_por_path(&self, path: &str) -> rusqlite::Result<Option<Album>> {
        let mut stmt = self.conexion.prepare("SELECT id_album, path, name, year FROM albums WHERE path = ?1")?;
        let mut columnas = stmt.query(params![path])?;

        if let Some(col) = columnas.next()? {
            return Ok(Some(Album {
                id: (Some(col.get(0)?)),
                path: (col.get(1)?),
                name: (col.get(2)?),
                year: (col.get(3)?)
            }));
        }else {
            return Ok(None);
        }
    }

    //regresa un album por su nombre, en caso de que no exista regresa None, regresa Err si falló el sql
    pub fn buscar_por_nombre(&self, nombre: &str) -> rusqlite::Result<Option<Album>> {
        let mut stmt = self.conexion.prepare("SELECT id_album, path, name, year FROM albums WHERE name = ?1")?;
        let mut columnas = stmt.query(params![nombre])?;

        if let Some(col) = columnas.next()? {
            return Ok(Some(Album {
                id: (Some(col.get(0)?)),
                path: (col.get(1)?),
                name: (col.get(2)?),
                year: (col.get(3)?)
            }));
        }else {
            return Ok(None);
        }
    }

    //regresa un album por su año, en caso de que no exista regresa None, regresa Err si falló el sql
    pub fn buscar_por_agno(&self, agno: i64) -> rusqlite::Result<Option<Album>> {
        let mut stmt = self.conexion.prepare("SELECT id_album, path, name, year FROM albums WHERE year = ?1")?;
        let mut columnas = stmt.query(params![agno])?;

        if let Some(col) = columnas.next()? {
            return Ok(Some(Album {
                id: (Some(col.get(0)?)),
                path: (col.get(1)?),
                name: (col.get(2)?),
                year: (col.get(3)?)
            }));
        }else {
            return Ok(None);
        }
    }

    //busca por nombre y año
    pub fn buscar_por_nombre_agno(&self, nombre: &str, agno: i64) -> rusqlite::Result<Option<Album>> {
        let mut stmt = self.conexion.prepare("SELECT id_album, path, name, year FROM albums WHERE name = ?1 AND year = ?2")?;
        let mut columnas = stmt.query(params![nombre, agno])?;

        if let Some(col) = columnas.next()? {
            return Ok(Some(Album {
                id: (Some(col.get(0)?)),
                path: (col.get(1)?),
                name: (col.get(2)?),
                year: (col.get(3)?)
            }));
        }else {
            return Ok(None);
        }        
    }

    //elimina un album por su id
    pub fn eliminar_por_id(&self, id: i32) -> rusqlite::Result<usize> {
        let filas = self.conexion.execute(
            "DELETE FROM rolas WHERE id_album = ?1",
            [id],
        )?;        
        return Ok(filas);
    }
}
