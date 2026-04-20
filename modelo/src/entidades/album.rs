//representa Album de la BD a un alto nivel
pub struct Album {
    pub id: Option<i64>,
    pub path: String,
    pub name: String,
    pub year: i64,
}
