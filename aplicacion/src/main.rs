use gtk::prelude::*;
use gtk::Application;
use modelo::minero::Minero;

fn main() {
    // let minero = Minero {};
    // minero.mina("/home/kralmasol/Music/pruebaMusica");

    let aplicacion = gtk::Application::builder().application_id("com.reproductor").build();

    aplicacion.connect_activate(|app| {
        let vista = vista::Vista::new(app);

        vista.window.show();
    });

    aplicacion.run();
}
