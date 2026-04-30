use std::sync::{Arc, Mutex};
use gtk::ffi::GtkAlertDialog;
use gtk::{ResponseType, TextView, prelude::*};
use gtk::{Application, ApplicationWindow, Adjustment, Box, Button, Paned, Stack, ScrolledWindow, 
    FlowBox, ColumnView, Label, Image, Scale, Orientation, SelectionModel};
// use rusqlite::Result;
use std::rc::Rc;
use std::cell::RefCell;
use controlador::controlador::Controlador;
use crate::widgets::{Biblioteca, Reproductor};


pub fn construir(app: &Application, controlador: Rc<RefCell<Controlador>>) -> rusqlite::Result<()> {
    
    let window = ApplicationWindow::new(app);
    window.set_title(Some("Reproductor de Música"));
    window.set_default_size(900, 600);

    // Crear un Box principal vertical
    let main_box = Box::new(Orientation::Vertical, 0);
    window.set_child(Some(&main_box));
    
    // Contenedor para el botón minero (barra superior)
    let top_bar = Box::new(Orientation::Horizontal, 0);
    top_bar.set_margin_top(5);
    top_bar.set_margin_end(5);
    top_bar.set_halign(gtk::Align::End);
    
    let minero_button = Button::with_label("Minar");
    minero_button.add_css_class("minero-button");
    top_bar.append(&minero_button);
    main_box.append(&top_bar);
    
    let paned = Paned::new(Orientation::Horizontal);
    paned.set_vexpand(true);
    // window.set_child(Some(&paned);
    main_box.append(&paned);
        
    let (left_box, play_button, pause_button, next_button, song_label, progress_bar) = crear_panel_reproductor();
    left_box.set_width_request(220);
    left_box.set_hexpand(false);
    paned.set_position(200);
    paned.set_start_child(Some(&left_box));
    // paned.set_start_child(Some(&left_box));
        
    let (right_box, stack, flowbox, column_view, btn_grid, btn_table) = crear_panel_biblioteca();
    right_box.set_hexpand(true);
    right_box.set_vexpand(true);
    
    // paned.set_end_child(Some(&right_box));

    let text_view = TextView::new();
    text_view.add_css_class("consulta-area");
    text_view.set_margin_top(5);
    text_view.set_margin_bottom(5);

    // if let Some(buffer) = text_view.buffer() {
    //     buffer.set_text("obtener canciones");
    // }

    let scroll = ScrolledWindow::builder()
        .min_content_height(120)
        .child(&text_view)
        .build();

    let ejecutar_button = Button::with_label("Ejecutar");
    ejecutar_button.add_css_class("execute-button");
    ejecutar_button.set_halign(gtk::Align::Center);

    let consulta_box = Box::new(Orientation::Vertical, 5);
    consulta_box.set_spacing(8);
    consulta_box.append(&scroll);
    consulta_box.append(&ejecutar_button);

    let right_container = Box::new(Orientation::Vertical, 5);
    right_container.set_margin_top(10);
    right_container.set_margin_start(10);
    right_container.set_margin_end(10);
    right_container.set_margin_bottom(10);
    right_container.append(&consulta_box);
    right_container.append(&right_box);

    paned.set_end_child(Some(&right_container));

    // paned.set_position(280);

    let biblioteca = Biblioteca::new(flowbox.clone(), column_view);

    //PANTALLA INICIAL ----Carga las canciones en el grid, por eso se hace aquío y no en btn_grid
    let canciones = controlador.borrow().obtener_canciones()?;
    biblioteca.cargar_en_flowbox(&canciones);

    // clones
    let biblioteca_para_tabla = biblioteca.clone();
    let biblioteca_para_minero = biblioteca.clone();
    let controlador1 = controlador.clone();
    let stack1 = stack.clone();
    let stack2 = stack.clone();
    
    btn_grid.connect_clicked(move |_| {
        stack1.set_visible_child_name("grid_view");
    });

    btn_table.connect_clicked(move |_| {
        stack2.set_visible_child_name("table_view");
        
        match controlador1.borrow().obtener_canciones() {
            Ok(canciones) => {
                biblioteca_para_tabla.cargar_tabla(&canciones);
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    });
    
    let controlador_minero = controlador.clone();
    let ventana_clon = window.clone();
    
    minero_button.connect_clicked(move |_| {
        mostrar_dialogo_minero(
            &ventana_clon,
            controlador_minero.clone(),
            biblioteca_para_minero.clone(),
        );
    });

    let controlador_reproductor = controlador.clone();
    
    let reproductor = Reproductor::new(left_box, play_button, pause_button, next_button, song_label, progress_bar, controlador_reproductor);
    let canciones_referencia = biblioteca.canciones.clone();
    
    flowbox.connect_selected_children_changed(move |flowbox| {
        if let Some(child) = flowbox.selected_children().first() {
            let index = child.index() as usize;
            let canciones = canciones_referencia.borrow();

            if let Some(cancion) = canciones.get(index) {
                reproductor.set_cancion(cancion);
                println!("Se seleccionó {}", cancion.titulo);
            }
            
        }
    });
    
    
    window.present();
    
    return Ok(());
}

//muestra el dialogo del botón del minero para escoger la carpeta raíz para ejecutar el miner
fn mostrar_dialogo_minero(parent: &ApplicationWindow, controlador: Rc<RefCell<Controlador>>, biblioteca: Biblioteca) {
    let dialogo = gtk::FileDialog::builder()
        .title("Selecciona carpeta raíz para ejecutar el minero.")
        .accept_label("Abrir")
        .modal(true)
        .build();
    
    dialogo.select_folder(
        Some(parent), 
        None::<&gtk::gio::Cancellable>, 
        move |result| {
            match result {
                Ok(folder) => {                    
                    if let Some(path) = folder.path() {
                        println!("Carpeta seleccionada: {:?}", path);

                        controlador.borrow_mut().poblar_bd(&path);

                        match controlador
                            .borrow()
                            .obtener_canciones() {
                                Ok(c) => {
                                    biblioteca.cargar_en_flowbox(&c);
                                    biblioteca.cargar_tabla(&c);
                                }
                                Err(e) =>{
                                    println!("Error al obtener canciones: {}", e);
                                }
                            };                                                
                    }
                }
                Err(err) => {
                    // El usuario canceló o hubo un error
                    eprintln!("Error o cancelación: {:?}", err);
                }
            }
        },
    );
}

// fn muestra_progreso_minero(parent: &ApplicationWindow, path: &std::path::Path, controlador: Arc<Mutex<Controlador>>) {
//     let dialogo = gtk::AlertDialog::builder()
//         .message(&format!("Iniciando minero en: \n{}\n\n....", path.display())).build();

//     let clon_path = path.to_path_buf();
//     let controlador_clon = controlador.clone();
//     let dialogo_clon = dialogo.clone();
//     let parent_clon = parent.clone();

//      std::thread::spawn(move || {
//         let resultado = {
//             let mut ctrl = controlador_clon.lock().unwrap();
//             ctrl.poblar_bd(&clon_path)
//         };

//         gtk::glib::MainContext::default().invoke(move || {
//             match resultado {
//                 Ok(()) => {
//                     dialogo_clon.set_message("Minero completado");
//                 }
//                 Err(e) => {
//                     dialogo_clon.set_message(&format!("Error:\n{}", e));
//                 }
//             }
//             dialogo_clon.show(Some(&parent_clon));
//         });
//     });

//     dialogo.show(Some(parent));
// }


fn crear_panel_reproductor() -> (Box, Button, Button, Button, Label, Scale) {

    let left_box = Box::new(Orientation::Vertical, 10);
    // left_box.set_margin_all(10);
    left_box.add_css_class("reproductor");

    let spacer_top = Box::new(Orientation::Vertical, 0);
    spacer_top.set_vexpand(true);

    let center_box = Box::new(Orientation::Vertical, 8);
    center_box.set_valign(gtk::Align::Center);

    let cover_image = Image::from_icon_name("audio-x-generic-symbolic");
    cover_image.set_pixel_size(160);

    let song_label = Label::new(Some("No hay canción seleccionada"));
    song_label.add_css_class("song-title");

    center_box.append(&cover_image);
    center_box.append(&song_label);

    let progress_bar = Scale::new(Orientation::Horizontal, None::<&Adjustment>);
    progress_bar.add_css_class("progress-bar");

    let buttons_box = Box::new(Orientation::Horizontal, 10);
    buttons_box.set_halign(gtk::Align::Center);

    let play_button = Button::with_label("▶");
    let pause_button = Button::with_label("⏸");
    let next_button = Button::with_label("⏭");

    play_button.add_css_class("play-button");
    pause_button.add_css_class("pause-button");
    next_button.add_css_class("next-button");

    buttons_box.append(&play_button);
    buttons_box.append(&pause_button);
    buttons_box.append(&next_button);

    let spacer_bottom = Box::new(Orientation::Vertical, 0);
    spacer_bottom.set_vexpand(true);

    left_box.append(&spacer_top);
    left_box.append(&center_box);
    left_box.append(&progress_bar);
    left_box.append(&buttons_box);
    left_box.append(&spacer_bottom);
    
    return (left_box, play_button, pause_button, next_button, song_label, progress_bar);
}

fn crear_panel_biblioteca() -> (Box, Stack, FlowBox, ColumnView, Button, Button) {
    let right_box = Box::new(Orientation::Vertical, 5);
    
    // Botones para cambiar vista
    let view_buttons_box = Box::new(Orientation::Horizontal, 5);
    view_buttons_box.set_margin_top(10);
    view_buttons_box.set_margin_end(10);
    view_buttons_box.set_halign(gtk::Align::End);
    
    let btn_grid = Button::with_label("Tarjetas");
    let btn_table = Button::with_label("Tabla");
    
    btn_grid.add_css_class("view-button");
    btn_table.add_css_class("view-button");
    
    view_buttons_box.append(&btn_grid);
    view_buttons_box.append(&btn_table);
    right_box.append(&view_buttons_box);
    
    // Stack para las vistas
    let stack = Stack::new();
    stack.set_vexpand(true);
    right_box.append(&stack);
    
    // Vista Grid
    let grid_scroll = ScrolledWindow::new();
    let flowbox = FlowBox::new();
    flowbox.set_row_spacing(10);
    flowbox.set_column_spacing(10);
    // flowbox.set_max_children_per_line(4);
    flowbox.set_max_children_per_line(6);
    flowbox.set_min_children_per_line(6);
    flowbox.set_homogeneous(true);
    flowbox.set_selection_mode(gtk::SelectionMode::Single);
    flowbox.set_halign(gtk::Align::Center);
    grid_scroll.set_child(Some(&flowbox));
    
    // Vista Tabla
    let table_scroll = ScrolledWindow::new();
    let selection = gtk::SingleSelection::new(None::<gtk::gio::ListModel>);
    // let column_view = ColumnView::new(None::<SelectionModel>);
    let column_view = ColumnView::new(Some(selection));
    column_view.add_css_class("tabla-canciones");
    table_scroll.set_child(Some(&column_view));
    
    // Agregar vistas al stack
    stack.add_named(&grid_scroll, Some("grid_view"));
    stack.add_named(&table_scroll, Some("table_view"));
    
    // // Configurar botones
    // let stack_clone = stack.clone();
    // btn_grid.connect_clicked(move |_| {
    //     stack_clone.set_visible_child_name("grid_view");
    // });

    // let stack2 = stack.clone();
    // btn_table.connect_clicked(move |_| {
    //     stack2.set_visible_child_name("table_view");
    // });
    
    // Mostrar vista inicial
    stack.set_visible_child_name("grid_view");
    
    return (right_box, stack, flowbox, column_view, btn_grid, btn_table);
}
