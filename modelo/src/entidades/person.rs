//representa Person de la BD a un alto nivel
pub struct Person {
    pub id: Option<i32>,
    pub stage_name: String,
    pub real_name: String,
    pub birth_date: String,
    pub death_date: Option<String>,
}
