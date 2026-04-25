use std::rc::Rc;
use rusqlite::Connection;
// use crate::modelo::entidades::person::Person;
use crate::entidades::person::Person;

pub struct PersonDao {
    conexion: Rc<Connection>,
}

impl PersonDao {
    pub fn new(conexion: Rc<Connection>) -> Self {
        return Self { conexion };
    }

    //agrega a la tabla person, regresa el número de columnas insertadas en caso de éxito, regresa Error en caso de que falle el sql
    pub fn agregar(&self, person: &Person) -> rusqlite::Result<()> {
        self.conexion.execute("INSERT INTO persons (stage_name, real_name, birth_date, death_date) VALUES (?1, ?2, ?3, ?4)",
            (&person.stage_name, &person.real_name, &person.birth_date, &person.death_date,),)?;
        return Ok(());
    }

    //regresa una persona por su nombre, en caso de que no exista regresa None, regresa Err si falló el sql
    pub fn buscar_por_nombre_artistico(&self, nombre_artistico: &str) -> rusqlite::Result<Option<Person>> {
        let mut stmt = self.conexion.prepare("SELECT id_person, stage_name, real_name, birth_date, death_date FROM persons WHERE stage_name = ?1")?;
        let mut columnas = stmt.query([nombre_artistico])?;

        if let Some(col) = columnas.next()? {
            return Ok(Some(Person {
                id: (Some(col.get(0)?)),
                stage_name: (col.get(1)?),
                real_name: (col.get(2)?),
                birth_date: (col.get(3)?),
                death_date: (col.get(4)?)
            }));
        } else {
            return Ok(None);
        }
    }

    //regresa una persona por su nombre, en caso de que no exista regresa None, regresa Err si falló el sql
    pub fn buscar_por_nombre_real(&self, nombre_real: &str) -> rusqlite::Result<Option<Person>> {
        let mut stmt = self.conexion.prepare("SELECT id_person, stage_name, real_name, birth_date, death_date FROM persons WHERE real_name = ?1")?;
        let mut columnas = stmt.query([nombre_real])?;

        if let Some(col) = columnas.next()? {
            return Ok(Some(Person {
                id: (Some(col.get(0)?)),
                stage_name: (col.get(1)?),
                real_name: (col.get(2)?),
                birth_date: (col.get(3)?),
                death_date: (col.get(4)?)
            }));
        } else {
            return Ok(None);
        }
    }

    //regresa una persona por su nombre, en caso de que no exista regresa None, regresa Err si falló el sql
    pub fn buscar_por_nombre_fecha_nacimiento(&self, fecha_nacimiento: &str) -> rusqlite::Result<Option<Person>> {
        let mut stmt = self.conexion.prepare("SELECT id_person, stage_name, real_name, birth_date, death_date FROM persons WHERE birth_date = ?1")?;
        let mut columnas = stmt.query([fecha_nacimiento])?;

        if let Some(col) = columnas.next()? {
            return Ok(Some(Person {
                id: (Some(col.get(0)?)),
                stage_name: (col.get(1)?),
                real_name: (col.get(2)?),
                birth_date: (col.get(3)?),
                death_date: (col.get(4)?)
            }));
        } else {
            return Ok(None);
        }
    }

    //regresa una persona por su nombre, en caso de que no exista regresa None, regresa Err si falló el sql
    pub fn buscar_por_fecha_fallecimiento(&self, fecha_fallecimiento: &str) -> rusqlite::Result<Option<Person>> {
        let mut stmt = self.conexion.prepare("SELECT id_person, stage_name, real_name, birth_date, death_date FROM persons WHERE death_date = ?1")?;
        let mut columnas = stmt.query([fecha_fallecimiento])?;

        if let Some(col) = columnas.next()? {
            return Ok(Some(Person {
                id: (Some(col.get(0)?)),
                stage_name: (col.get(1)?),
                real_name: (col.get(2)?),
                birth_date: (col.get(3)?),
                death_date: (col.get(4)?)
            }));
        } else {
            return Ok(None);
        }
    }

    //elimina una persona por su id
    pub fn eliminar_por_id(&self, id: i32) -> rusqlite::Result<usize> {
        let filas = self.conexion.execute(
            "DELETE FROM rolas WHERE id_person = ?1",
            [id],
        )?;        
        return Ok(filas);
    }
    
}
