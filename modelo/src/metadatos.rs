//structs con los metadatos que sacó el minero y datos por omisión sirven para construir los objetos definidos en entidades/ tomando en cuenta que esos objetos ya deben manejar la información prácticamente como si fueran la base de datos, estos structs son bastante volátiles

//struct que guarda los metadatos que recogió el minero para poder agregar a la base de datos
#[derive(Debug)]
pub struct Cancion {
    pub titulo: String,
    pub path: String,
    //el primer atributo representa persona (None si es grupo), el segundo representa el grupo (None si es persona)
    pub artista: Artista<Persona, Grupo>,
    pub album: Album,
    pub track: Option<i64>,
    pub agno: Option<i64>,
    pub genero: Option<String>,
    //0 persona, 1 grupo, 2 desconocido
    pub tipo_artista: i64,
}

#[derive(Debug)]
pub struct Grupo {
    pub nombre: Option<String>,
    pub fecha_inicio: Option<String>,
    pub fecha_separacion: Option<String>,
}

#[derive(Debug)]
pub struct Persona {
    pub nombre_artistico: Option<String>,
    pub nombre_real: Option<String>,
    pub fecha_nacimiento: Option<String>,
    pub fecha_fallecimiento: Option<String>,
}

#[derive(Debug)]
pub enum Artista<T, U> {
    Persona(T),
    Grupo(U),
}

#[derive(Debug)]
pub struct Album {
    pub path: String,
    pub nombre: Option<String>,
    pub agno: Option<i64>,
}


