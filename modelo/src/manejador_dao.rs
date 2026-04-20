// use crate::modelo::metadatos::{Artista, Cancion};
use rusqlite::Connection;
use crate::{dao::{album_dao::AlbumDao, performer_dao::{self, PerformerDao}}, entidades::{album::Album, performer::Performer, rola::Rola}, metadatos::{Artista, Cancion}};

//objeto que manipula los daos de dao/ a un alto nivel
pub struct ManejadorDao<'a> {
    performer_dao: PerformerDao<'a>,
    album_dao: AlbumDao<'a>,
}

impl<'a> ManejadorDao<'a> {

    pub fn new(conexion: &'a Connection) -> Self {
        return Self {
            performer_dao: PerformerDao::new(conexion),
            album_dao: AlbumDao::new(conexion),
        };
    }
    
    // pub fn agregar_cancion(&self, cancion: Cancion) -> Result<(), Box<dyn::std::error::Error>> {
        
    // }

    // fn construye_rola(cancion: &Cancion) -> Rola {
    //     return Rola {
    //         id: (),
    //         id_performer: (),
    //         id_album: (),
    //         path: (cancion.path),
    //         title: (cancion.titulo),
    //         track: match cancion.track {
    //             Some(t) => t,
    //             None => 0,
    //         },
    //         year: match cancion.agno {
    //             Some(a) => a,
    //             None => 0,
    //         },
    //         genre: match cancion.genero {
    //             Some(g) => g,
    //             None => String::from("Desconocido"),
    //         },
    //     };
    // }


    //regresa el id del artista, puede que ya exista en la tabla performers, en caso de que sea un nuevo performer lo 
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


    fn resolver_album(&self, cancion: &Cancion) -> Result<i64, Box<dyn::std::error::Error>> {
        let nombre = match &cancion.album.nombre {
            Some(n) => n,
            None => String::from("Desconocido"),
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
            path: cancion.album.path,
            name: match cancion.album.nombre {
                Some(n) => n,
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
