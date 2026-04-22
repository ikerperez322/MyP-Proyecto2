//representa Rola de la BD a un alto nivel
#[derive(Debug)]
pub struct Rola {
    pub id: Option<i64>,
    pub id_performer: i64,
    pub id_album: i64,
    pub path: String,
    pub title: String,
    pub track: i64,
    pub year: i64,
    pub genre: String,
}
