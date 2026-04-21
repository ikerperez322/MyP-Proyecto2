use modelo::minero::Minero;
use modelo::manejador_dao::ManejadorDao;

pub struct Controlador<'a> {
    manejador: ManejadorDao<'a>,
    minero: Minero,
}

impl <'a> Controlador<'a> {
    pub fn new(manejador: ManejadorDao<'a>, minero: Minero) -> Self {
        return Self {
            manejador: manejador,
            minero: minero,
        };
    }

    //método para invocar al minero desde la parte del usuario y poblar la base de datos
    pub fn poblar_bd(&self, raiz: &str) -> Result<(), Box<dyn::std::error::Error>> {
        let canciones = self.minero.mina(raiz)?;

        for cancion in canciones {
            match self.manejador.agrega_rola(&cancion) {
                Ok(id) => println!("Insertada canción con id {}", id),
                Err(e) => eprintln!("Error insertando canción: {}", e),
            }
        }

        return Ok(());
    }
}

