use rusqlite::Connection;
// use crate::modelo::entidades::album::Album;
use crate::entidades::album::Album;

pub struct AlbumDao<'a> {
    conexion: &'a Connection,
}

impl<'a> AlbumDao<'a> {
    pub fn new(conexion: &'a Connection) -> Self {
        return Self { conexion };
    }

    //agrega a la tabla album, regresa el número de columnas insertadas en caso de éxito, regresa Error en caso de que falle el sql
    pub fn agregar(&self, album: &Album) -> rusqlite::Result<()> {
        self.conexion.execute("INSERT INTO albums (path, name, year) VALUES (?1, ?2, ?3)",
            (&album.path, &album.name, &album.year,),)?;
        return Ok(());
    }

    //regresa un album por su path, en caso de que no exista regresa None, regresa Err si falló el sql
    pub fn buscar_por_path(&self, path: &str) -> rusqlite::Result<Option<Album>> {
        let mut stmt = self.conexion.prepare("SELECT id_album, path, name, year FROM albums WHERE path = ?1")?;
        let mut columnas = stmt.query([path])?;

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
        let mut columnas = stmt.query([nombre])?;

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
    pub fn buscar_por_agno(&self, agno: &str) -> rusqlite::Result<Option<Album>> {
        let mut stmt = self.conexion.prepare("SELECT id_album, path, name, year FROM albums WHERE year = ?1")?;
        let mut columnas = stmt.query([agno])?;

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
