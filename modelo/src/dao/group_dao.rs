use rusqlite::Connection;
// use crate::modelo::entidades::group::Group;
use crate::entidades::group::Group;

pub struct GroupDao<'a> {
    conexion: &'a Connection,
}

impl<'a> GroupDao<'a> {
    pub fn new(conexion: &'a Connection) -> Self {
        return Self { conexion };
    }

    //agrega a la tabla group, regresa el número de columnas insertadas en caso de éxito, regresa Error en caso de que falle el sql
    pub fn agregar(&self, group: &Group) -> rusqlite::Result<()> {
        self.conexion.execute("INSERT INTO groups (name, start_date, end_date) VALUES (?1, ?2, ?3)",
            (&group.name, &group.start_date, &group.end_date,),)?;
        return Ok(());
    }

    //regresa un grupo por su nombre, en caso de que no exista regresa None, regresa Err si falló el sql
    pub fn buscar_por_nombre(&self, nombre: &str) -> rusqlite::Result<Option<Group>> {
        let mut stmt = self.conexion.prepare("SELECT id_group, name, start_date, end_date FROM groups WHERE name = ?1")?;
        let mut columnas = stmt.query([nombre])?;

        if let Some(col) = columnas.next()? {
            return Ok(Some(Group {
                id: (Some(col.get(0)?)),
                name: (col.get(1)?),
                start_date: (col.get(2)?),
                end_date: (col.get(3)?)
            }));
        }else {
            return Ok(None);
        }
    }

    //regresa un grupo por su fecha de inicio, en caso de que no exista regresa None, regresa Err si falló el sql
    pub fn buscar_por_fecha_inicio(&self, fecha_inicio: &str) -> rusqlite::Result<Option<Group>> {
        let mut stmt = self.conexion.prepare("SELECT id_group, name, start_date, end_date FROM groups WHERE start_date = ?1")?;
        let mut columnas = stmt.query([fecha_inicio])?;

        if let Some(col) = columnas.next()? {
            return Ok(Some(Group {
                id: (Some(col.get(0)?)),
                name: (col.get(1)?),
                start_date: (col.get(2)?),
                end_date: (col.get(3)?)
            }));
        }else {
            return Ok(None);
        }
    }

    //regresa un grupo por su fecha de desaparición, en caso de que no exista regresa None, regresa Err si falló el sql
    pub fn buscar_por_fecha_final(&self, fecha_final: &str) -> rusqlite::Result<Option<Group>> {
        let mut stmt = self.conexion.prepare("SELECT id_group, name, start_date, end_date FROM groups WHERE end_date = ?1")?;
        let mut columnas = stmt.query([fecha_final])?;

        if let Some(col) = columnas.next()? {
            return Ok(Some(Group {
                id: (Some(col.get(0)?)),
                name: (col.get(1)?),
                start_date: (col.get(2)?),
                end_date: (col.get(3)?)
            }));
        }else {
            return Ok(None);
        }
    }

    //elimina un grupo por su id
    pub fn eliminar_por_id(&self, id: i32) -> rusqlite::Result<usize> {
        let filas = self.conexion.execute(
            "DELETE FROM rolas WHERE id_group = ?1",
            [id],
        )?;        
        return Ok(filas);
    }
    
}

