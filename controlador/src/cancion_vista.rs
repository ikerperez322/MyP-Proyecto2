//struct para guardar los datos que se van a mostrar en la vista
#[derive(Debug, Clone)]
pub struct CancionVista {
    pub titulo: String,
    pub artista: String,
    pub album: String,
    pub genero: String,
}

impl CancionVista {
    pub fn new(titulo: String, artista: String, album: String, genero: String) -> Self {
        return Self {
            titulo: titulo,
            artista: artista,
            album: album,
            genero: genero,
        }
    }
}

