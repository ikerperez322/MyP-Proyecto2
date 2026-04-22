
pub struct CancionVista {
    pub titulo: String,
    pub artista: String,
    pub album: String,
}

impl CancionVista {
    pub fn new(titulo: String, artista: String, album: String) -> Self {
        return Self {
            titulo: titulo,
            artista: artista,
            album: album,
        }
    }
}

