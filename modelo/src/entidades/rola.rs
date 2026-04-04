//representa Rola de la BD a un alto nivel
pub struct Rola {
    pub id: Option<u32>,
    pub id_performer: u32,
    pub id_album: u32,
    pub path: String,
    pub title: String,
    pub track: u32,
    pub year: u32,
    pub genre: String,
}
