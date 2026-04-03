
//structs con los metadatos que sacó el minero y datos por omisión sirven para construir los objetos definidos en entidades/ tomando en cuenta que esos objetos ya deben manejar la información prácticamente como si fueran la base de datos, estos structs son bastante volátiles


//struct que guarda los metadatos que recogió el minero para poder agregar a la base de datos
pub struct Cancion {
    pub titulo: String,
    pub path: String,
    //el primer atributo representa persona (None si es grupo), el segundo representa el grupo (None si es persona)
    pub artista: Artista<Persona, Grupo>,
    pub album: Album,
    pub track: Option<String>,
    pub agno: Option<i32>,
    pub genero: Option<String>,
    // pub agno_album: Option<i32>,
    //0 persona, 1 grupo, 2 desconocido
    pub tipo_artista: i32,
}

pub struct Grupo {
    pub nombre: Option<String>,
    pub fecha_inicio: Option<String>,
    pub fecha_separacion: Option<String>,
}

pub struct Persona {
    pub nombre_artistico: Option<String>,
    pub nombre_real: Option<String>,
    pub fecha_nacimiento: Option<String>,
    pub fecha_fallecimiento: Option<String>,
}

pub enum Artista<T, U> {
    Persona(T),
    Grupo(U),
}

pub struct Album {
    pub path: String,
    pub nombre: Option<String>,
    pub agno: Option<i32>,
}


