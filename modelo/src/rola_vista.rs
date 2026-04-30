
//struct para la info de las canciones que se muestran en la vista (este es para el lado del modelo)
pub struct RolaVista {
    pub titulo: String,
    pub artista: String,
    pub album: String,
    pub genero: String,
    pub path: String,
}

impl RolaVista {
    pub fn new(titulo: String, artista: String, album: String, genero: String, path: String) -> Self {
        return Self {
            titulo: titulo,
            artista: artista,
            album: album,
            genero: genero,
            path: path,
        }
    }
}
