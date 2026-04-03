//representa Group de la BD a un alto nivel
pub struct Group {
    pub id: Option<i32>,
    pub name: String,
    pub start_date: String,
    pub end_date: Option<String>,
}
