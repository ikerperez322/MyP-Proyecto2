//representa Rola de la BD a un alto nivel
pub struct Rola {
    pub id: Option<i32>,
    pub id_performer: i32,
    pub id_album: i32,
    pub path: String,
    pub title: String,
    pub track: i32,
    pub year: i32,
    pub genre: String,
}
