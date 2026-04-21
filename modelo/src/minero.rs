use walkdir::WalkDir;
use lofty::read_from_path;
use lofty::prelude::TaggedFileExt;
use lofty::tag::{Accessor, ItemKey, Tag, TagType};
use std::path::Path;
use crate::metadatos::{Album, Artista, Cancion, Grupo, Persona};

pub struct Minero {
    
}

impl Minero {

    pub fn new() -> Self {
        return Self {  };
    }
    
    //método que recibe un archivo raíz donde minar archivos mp3
    pub fn mina(&self, raiz: &str) -> Result<Vec<Cancion>, Box<dyn::std::error::Error>> {

        let mut canciones: Vec<Cancion> = Vec::new();
        
        for archivo in WalkDir::new(raiz).into_iter().filter_map(|e| e.ok()) {
            let path = archivo.path();
            
            if archivo.file_type().is_file() {
                if let Some(extension) = path.extension() {
                    if extension == "mp3" {
                        if let Ok(archivo_tagged) = read_from_path(path) {
                            if let Some(tag) = archivo_tagged.tag(TagType::Id3v2) {
                                let cancion = Self::analiza_cancion(tag, path);
                                println!("Canción:\n{:#?}", cancion);
                                canciones.push(cancion);
                                // println!("Canción: {:?}", cancion);
                            } else {
                                println!("Archivo SIN ID3v2: {}", path.display());
                            }
                        }
                    }
                }
            }
        }
        return Ok(canciones);
    }

    //Regresa los metadatos del archivo analizado
    fn analiza_cancion(etiqueta: &Tag, direccion: &Path) -> Cancion {
        let art = Self::analiza_artista(etiqueta);
        let tipo_art = Self::tipo_artista(&art);
        let cancion = Cancion {
            //para el título primero se intenta con la etiqueta, en caso de no encontrar nada usa el nombre del archivo por omisión
            titulo: match &etiqueta.title() {
                Some(tit) => tit.to_string(),
                None => match direccion.file_name() {
                    Some(dir) => dir.to_string_lossy().into_owned().replace(".mp3", ""),
                    //teoricamente esto pasaría solo en casos realmente excepcionales
                    None => String::from("Desconocido"),
                },
            },
            path: direccion.to_string_lossy().into_owned(),
            artista: art,
            album: Self::analiza_album(etiqueta, direccion),
            track: etiqueta.track().map(|x| x as i64),
            agno: etiqueta.year().map(|x| x as i64),
            genero: match &etiqueta.genre() {
                Some(genro) => Some(genro.to_string()),
                None => None,
            },
            tipo_artista: tipo_art,
        };
        return cancion;
    }

    //devuelve los metadatos del artista (persona o grupo) obtenidos del Id3v2
    fn analiza_artista(etiqueta: &Tag) -> Artista<Persona, Grupo> {
        if let Some(grupo) = etiqueta.get_string(&ItemKey::AlbumArtist) {
            return Artista::Grupo(Grupo {
                nombre: Some(grupo.to_string()),
                fecha_inicio: None,
                fecha_separacion: None,
            });
        }else if let Some(persona) = etiqueta.artist() {
            return Artista::Persona(Persona{
                nombre_artistico: Some(persona.to_string()),
                nombre_real: None,
                fecha_nacimiento: None,
                fecha_fallecimiento: None,
            });
            //en caso de no distinguir, se usa persona por omisión
        }else {
            return Artista::Persona(Persona {
                nombre_artistico: None,
                nombre_real: None,
                fecha_nacimiento: None,
                fecha_fallecimiento: None,
            });
        }
    }
    
    //devuelve los metadatos que lofty minó del album del Id3v2
    fn analiza_album(etiqueta: &Tag, direccion: &Path) -> Album {
        let album = Album {
            path: direccion.to_string_lossy().into_owned(),
            nombre: match etiqueta.album() {
                Some(alb) => Some(alb.to_string()),
                None => None,
            },
            //por omision tomamos el año de la canción como año del album ya que Id3v2 no distingue entre ambos
            agno: etiqueta.year().map(|x| x as i64),
        };
        return album;
    }

    //regresa el tipo de artista (en número dependiendo de la variante del enum Artista)
    fn tipo_artista(artista: &Artista<Persona, Grupo>) -> i64 {
        match artista {
            Artista::Grupo(_) => return 1,
            Artista::Persona(p) => {
                if p.nombre_artistico != None {
                    return 0;
                } else {
                  return 2;  
                }
            }
        }
    }    
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_id3v2() {
//         for archivo in WalkDir::new("/home/kralmasol/Music/pruebaMusica").into_iter().filter_map(|e| e.ok()) {
//             let path = archivo.path();

//             if archivo.file_type().is_file() {
//                 if let Some(extension) = path.extension() {
//                     if extension == "mp3" {
//                         // let actual = Probe::open(path).unwrap().read().unwrap();
//                         let actual = match Probe::open {
//                             Ok()
//                         }
//                         assert_eq!(actual.primary_tag_type(), TagType::Id3v2);
//                     }
//                 }
//             }
//         }
//     }
// }

