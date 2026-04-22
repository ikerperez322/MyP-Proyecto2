use rusqlite::Connection;
use rusqlite::params;
// use crate::modelo::entidades::performer::Performer;
use crate::entidades::performer::Performer;

//dao para tabla performers
pub struct PerformerDao<'a> {
    conexion: &'a Connection,
}

impl<'a> PerformerDao<'a> {
    pub fn new(conexion: &'a Connection) -> Self {
        return Self { conexion };
    }

    //agrega a la tabla performer, regresa el número de columnas insertadas en caso de éxito, regresa Error en caso de que falle el sql
    pub fn agregar(&self, performer: &Performer) -> rusqlite::Result<i64> {
        // println!("Insertando performer...");
        self.conexion.execute("INSERT INTO performers (id_type, name) VALUES (?1, ?2)",
            (&performer.id_type, &performer.name,),)?;
        return Ok(self.conexion.last_insert_rowid());
    }

    //regresa un performer por su nombre, en caso de que no exista regresa None, regresa Err si falló el sql
    pub fn buscar_por_nombre_tipo(&self, nombre: &str, tipo: i64) -> rusqlite::Result<Option<Performer>> {
        let mut stmt = self.conexion.prepare("SELECT id_performer, id_type, name FROM performers WHERE name = ?1 AND id_type = ?2")?;
        let mut columnas = stmt.query(params![nombre, &tipo])?;

        if let Some(col) = columnas.next()? {
            return Ok(Some(Performer{
                id: (Some(col.get(0)?)),
                id_type: (col.get(1)?),
                name: (col.get(2)?)
            }));
        } else {
            return Ok(None);
        }
    }

    //elimina un performer por su id
    pub fn eliminar_por_id(&self, id: i64) -> rusqlite::Result<usize> {
        let filas = self.conexion.execute(
            "DELETE FROM rolas WHERE id_performer = ?1",
            [id],
        )?;        
        return Ok(filas);
    }
}


