use crate::modelo::metadatos::{Artista, Cancion};
use crate::modelo::entidades::{performer::Performer, album::Album, person::Person, group::Group, rola::Rola};


pub struct CreaEntidades {

}

impl CreaEntidades {
    
    //crea el performer con la metadata pasada
    pub fn crea_performer(&self, cancion: &Cancion) -> Performer {
        let performer = Performer {
            id: None,
            id_type: cancion.tipo_artista,
            name: match &cancion.artista {
                // Some(art) => art.to_string(),
                // None => String::from("Desconocido"),
                Artista::Persona(p) => match &p.nombre_artistico {
                    Some(nombre_artistico) => nombre_artistico.to_string(),
                    None => String::from("Desconocido"),
                },
                Artista::Grupo(g) => match &g.nombre {
                    Some(nombre) => nombre.to_string(),
                    None => String::from("Desconocido"),
                }
            },
        };
        return performer;
    }

    //crea el album con la metadata pasada si el año del album es desconocido se pone año 0 por omisión
    pub fn crea_album(&self, cancion: &Cancion) -> Album {
        let album = Album {
            id: None,
            path: cancion.path.clone(),
            name: match &cancion.album.nombre {
                Some(alb) => alb.to_string(),
                None => String::from("Desconocido"),
            },
            year: match &cancion.album.agno {
                Some(agno) => agno.clone(),
                None => 0,
            },
        };
        return album;
    }

    //crea la persona a partir de la metadata que se pasa como argumento
    pub fn crea_persona(&self, cancion: &Cancion) -> Person {
        let persona = Person {
            id: None,
            stage_name: match &cancion.artista {
                Artista::Persona(p) => match &p.nombre_artistico {
                    Some(nombre_artistico) => nombre_artistico.to_string(),
                    None => String::from("Desconocido"),
                },
                _ => String::from("Desconocido"),
            },
            real_name: match &cancion.artista {
                Artista::Persona(p) => match &p.nombre_real {
                    Some(nombre_real) => nombre_real.to_string(),
                    None => String::from("Desconocido"),
                },
                _ => String::from("Desconocido"),
            },
            birth_date: match &cancion.artista {
                Artista::Persona(p) => match &p.fecha_nacimiento {
                    Some(fecha_nacimiento) => fecha_nacimiento.to_string(),
                    None => String::from("Desconocido"),
                },
                _ => String::from("Desconocido"),
            },
            death_date: match &cancion.artista {
                Artista::Persona(p) => match &p.fecha_fallecimiento {
                    Some(fecha_fallecimiento) => Some(fecha_fallecimiento.to_string()),
                    None => None,
                },
                _ => None,
            },
        };
        return persona;
    }

    //crea un grupo a partir de la metadata que se pasa como argumento
    pub fn crea_grupo(&self, cancion: &Cancion) -> Group {
        let grupo = Group {
            id: None,
            name: match &cancion.artista {
                Artista::Grupo(g) => match &g.nombre {
                    Some(nombre) => nombre.to_string(),
                    None => String::from("Desconocido"),
                },
                _ => String::from("Desconocido"),
            },
            start_date: match &cancion.artista {
                Artista::Grupo(g) => match &g.fecha_inicio {
                    Some(fecha_inicio) => fecha_inicio.to_string(),
                    _ => String::from("Desconocido"),
                },
                _ => String::from("Desconocido"),
            },
            end_date: match &cancion.artista {
                Artista::Grupo(g) => match &g.fecha_separacion {
                    Some(fecha_separacion) => Some(fecha_separacion.to_string()),
                    None => None,
                },
                _ => None,
            },
        };
        return grupo;
    }

    // pub fn crea_rola(&self, cancion: &Cancion) -> Rola {
    //     let rola = Rola {
    //         id: None,
            
    //     }
    // }
    
}

