use std::sync::{Arc, Mutex};
use gtk::ffi::GtkAlertDialog;
use gtk::{ResponseType, prelude::*};
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
    
    let minero_button = Button::with_label("Minero");
    minero_button.add_css_class("minero-button");
    top_bar.append(&minero_button);
    main_box.append(&top_bar);

    
    let paned = Paned::new(Orientation::Horizontal);
    paned.set_vexpand(true);
    // window.set_child(Some(&paned);
    main_box.append(&paned);
        
    let left_box = crear_panel_reproductor();
     left_box.set_width_request(280);
    left_box.set_hexpand(false);
    paned.set_start_child(Some(&left_box));
    // paned.set_start_child(Some(&left_box));
        
    let (right_box, stack, flowbox, column_view) = crear_panel_biblioteca();
    right_box.set_hexpand(true);
    right_box.set_vexpand(true);
    paned.set_end_child(Some(&right_box));
    // paned.set_end_child(Some(&right_box));

    paned.set_position(280);

    let biblioteca = Biblioteca::new(flowbox.clone(), column_view.clone());
    let controlador_clon = controlador.clone();
    let ventana_clon = window.clone();
    minero_button.connect_clicked(move |_| {
        // let ctrl = controlador_clon.borrow();
        mostrar_dialogo_minero(&ventana_clon, controlador_clon.clone(), biblioteca.clone());
    });
        
    let _reproductor = Reproductor::new(left_box);
    let biblioteca = Biblioteca::new(flowbox, column_view);
        
    let canciones = controlador.borrow().obtener_canciones()?;
    biblioteca.cargar_en_flowbox(&canciones);
        
        
    window.present();
    
    return Ok(());
}

//muestra el dialogo del botón del minero para escoger la carpeta raíz para ejecutar el miner
fn mostrar_dialogo_minero(parent: &ApplicationWindow, controlador: Rc<RefCell<Controlador>>, biblioteca: Biblioteca) {
    // let dialogo = gtk::FileChooserDialog::builder()
    //     .title("Selecciona carpeta raíz para ejecutar el minero.")
    //     .transient_for(parent)
    //     .action(gtk::FileChooserAction::SelectFolder)
    //     .build();

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

fn crear_panel_reproductor() -> Box {
    let left_box = Box::new(Orientation::Vertical, 5);
    left_box.set_margin_top(10);
    left_box.set_margin_bottom(10);
    left_box.set_margin_start(10);
    left_box.set_margin_end(10);
    
    // Contenedor superior (imagen + título)
    let top_box = Box::new(Orientation::Vertical, 5);
    top_box.set_vexpand(true);
    
    let cover_image = Image::from_icon_name("audio-x-generic-symbolic");
    cover_image.set_pixel_size(200);
    cover_image.set_halign(gtk::Align::Center);
    
    let song_label = Label::new(Some("No hay canción seleccionada"));
    song_label.add_css_class("song-title");
    song_label.set_halign(gtk::Align::Center);
    
    top_box.append(&cover_image);
    top_box.append(&song_label);
    left_box.append(&top_box);
    
    // Barra de progreso
    let progress_bar = Scale::new(Orientation::Horizontal, None::<&Adjustment>);
    progress_bar.set_draw_value(true);
    progress_bar.set_hexpand(true);
    left_box.append(&progress_bar);
    
    // Botones de control
    let buttons_box = Box::new(Orientation::Horizontal, 5);
    buttons_box.set_halign(gtk::Align::Center);
    buttons_box.set_margin_top(10);
    buttons_box.set_margin_bottom(10);
    
    let play_button = Button::with_label("▶");
    let pause_button = Button::with_label("⏸");
    let next_button = Button::with_label("⏭");
    
    play_button.add_css_class("play-button");
    pause_button.add_css_class("pause-button");
    next_button.add_css_class("next-button");
    
    buttons_box.append(&play_button);
    buttons_box.append(&pause_button);
    buttons_box.append(&next_button);
    left_box.append(&buttons_box);
    
    return left_box;
}

fn crear_panel_biblioteca() -> (Box, Stack, FlowBox, ColumnView) {
    let right_box = Box::new(Orientation::Vertical, 5);
    
    // Botones para cambiar vista
    let view_buttons_box = Box::new(Orientation::Horizontal, 5);
    view_buttons_box.set_margin_top(10);
    view_buttons_box.set_margin_end(10);
    view_buttons_box.set_halign(gtk::Align::End);
    
    let btn_grid = Button::with_label("Grid");
    let btn_table = Button::with_label("Table");
    
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
    flowbox.set_max_children_per_line(4);
    flowbox.set_selection_mode(gtk::SelectionMode::None);
    flowbox.set_halign(gtk::Align::Center);
    grid_scroll.set_child(Some(&flowbox));
    
    // Vista Tabla
    let table_scroll = ScrolledWindow::new();
    let column_view = ColumnView::new(None::<SelectionModel>);
    table_scroll.set_child(Some(&column_view));
    
    // Agregar vistas al stack
    stack.add_named(&grid_scroll, Some("grid_view"));
    stack.add_named(&table_scroll, Some("table_view"));
    
    // Configurar botones
    let stack_clone = stack.clone();
    btn_grid.connect_clicked(move |_| {
        stack_clone.set_visible_child_name("grid_view");
    });

    let stack2 = stack.clone();
    btn_table.connect_clicked(move |_| {
        stack2.set_visible_child_name("table_view");
    });
    
    // Mostrar vista inicial
    stack.set_visible_child_name("grid_view");
    
    return (right_box, stack, flowbox, column_view);
}

// fn configurar_botones_vista(_right_box: &Box, _stack: Stack) {

// }
