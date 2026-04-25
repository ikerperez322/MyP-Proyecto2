// use crate::modelo::metadatos::{Artista, Cancion};
use std::rc::Rc;
use rusqlite::Connection;
use crate::{dao::{album_dao::AlbumDao, performer_dao::{self, PerformerDao}, rola_dao::RolaDao}, entidades::{album::Album, performer::Performer, rola::Rola}, metadatos::{Artista, Cancion, Grupo, Persona}};

//objeto que manipula los daos de dao/ a un alto nivel
pub struct ManejadorDao {
    rola_dao: RolaDao,
    performer_dao: PerformerDao,
    album_dao: AlbumDao,
}

impl ManejadorDao {

    pub fn new(conexion: Rc<Connection>) -> Self {
        return Self {
            rola_dao: RolaDao::new(conexion.clone()),
            performer_dao: PerformerDao::new(conexion.clone()),
            album_dao: AlbumDao::new(conexion.clone()),
        };
    }

    //agrega la rola a la tabla rolas en la base de datos, regresa un error en caso de que el agregado falle en algún punto
    pub fn agrega_rola(&self, cancion: &Cancion) -> Result<i64, Box<dyn::std::error::Error>> {

        if let Some(rola) = self.rola_dao.buscar_por_path(&cancion.path)? {
            if let Some(id) = rola.id {
                return Ok(id);
            }
        }
        
        let rola = Rola {
            id: None,
            id_performer: Self::resolver_artista(&self, cancion)?,
            id_album: Self::resolver_album(&self, cancion)?,            
            path: cancion.path.clone(),
            title: cancion.titulo.clone(),
            track: match cancion.track {
                Some(t) => t,
                None => 0,
            },
            year: match cancion.agno {
                Some(a) => a,
                None => 0,
            },
            genre: match &cancion.genero {
                Some(g) => g.to_string(),
                None => String::from("Desconocido"),
            },
        };

        let id = self.rola_dao.agregar(&rola)?;

        return Ok(id);        
    }

    pub fn agrega_performer(&self, artista: &Artista<Persona, Grupo>) -> Result<i64, Box<dyn::std::error::Error>> {
        let (nombre, tipo) = match &artista {
            Artista::Persona(p) => (p.nombre_artistico.as_ref(), 0),
            Artista::Grupo(g) => (g.nombre.as_ref(), 1),
        };

        let nombre = nombre.ok_or(rusqlite::Error::InvalidQuery)?;
            
        if let Some(performer) = self.performer_dao.buscar_por_nombre_tipo(nombre, tipo)? {
            if let Some(id) = performer.id {
                return Ok(id);
            }
        }

        let performer = Performer {
            id: None,
            id_type: tipo,
            name: nombre.to_string(),
        };

        let id = self.performer_dao.agregar(&performer)?;

        return Ok(id);
    }
    
    //regresa el id del artista, puede que ya exista en la tabla performers, en caso de que sea un nuevo performer lo agrega a la tabla performers
    fn resolver_artista(&self, cancion: &Cancion) -> Result<i64, Box<dyn::std::error::Error>> {
        let (nombre, tipo) = match &cancion.artista {
            Artista::Persona(p) => (p.nombre_artistico.as_ref(), 0),
            Artista::Grupo(g) => (g.nombre.as_ref(), 1),
        };
        
        let nombre = nombre.ok_or(rusqlite::Error::InvalidQuery)?;

        if let Some(performer) = self.performer_dao.buscar_por_nombre_tipo(nombre, tipo)? {
            if let Some(id) = performer.id {
                return Ok(id);
            }
        }

        let nuevo = Performer {
            id: None,
            id_type: tipo,
            name: nombre.to_string(),
        };

        //deberia de regresar el id
        let id = self.performer_dao.agregar(&nuevo)?;
        
        return Ok(id);
    }


    //regresa el id del album, puede que ya exista en la tabla albums, en caso de que sea un nuevo performer lo agrega a la tabla albums
    fn resolver_album(&self, cancion: &Cancion) -> Result<i64, Box<dyn::std::error::Error>> {
        let nombre = match &cancion.album.nombre {
            Some(n) => n,
            None => &String::from("Desconocido"),
        };

        let agno = match cancion.album.agno {
            Some(a) => a,
            None => 0,
        };

        if let Some(album) = self.album_dao.buscar_por_nombre_agno(&nombre, agno)? {
            if let Some(id) = album.id {
                return Ok(id);
            }
        }

        let nuevo = Album {
            id: None,
            path: cancion.album.path.clone(),
            name: match &cancion.album.nombre {
                Some(n) => n.to_string(),
                None => String::from("Desconocido"),
            },
            year: match cancion.album.agno {
                Some(a) => a,
                None => 0,
            },
        };

        let id = self.album_dao.agregar(&nuevo)?;

        return Ok(id);        
    }
        
}
