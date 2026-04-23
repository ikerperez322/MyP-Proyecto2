use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Adjustment, Box, Button, Paned, Stack, ScrolledWindow, 
    FlowBox, ColumnView, Label, Image, Scale, Orientation, SelectionModel};
// use rusqlite::Result;
use controlador::controlador::Controlador;
use crate::widgets::{Biblioteca, Reproductor};

pub fn construir(app: &Application, controlador: &Controlador) -> rusqlite::Result<()> {
    
    let window = ApplicationWindow::new(app);
    window.set_title(Some("Reproductor de Música"));
    window.set_default_size(900, 600);
    
    let paned = Paned::new(Orientation::Horizontal);
    window.set_child(Some(&paned));
        
    let left_box = crear_panel_reproductor();
    paned.set_start_child(Some(&left_box));
        
    let (right_box, stack, flowbox, column_view) = crear_panel_biblioteca();
    paned.set_end_child(Some(&right_box));
        
    let _reproductor = Reproductor::new(left_box);
    let biblioteca = Biblioteca::new(flowbox, column_view);
        
    let canciones = controlador.obtener_canciones()?;
    biblioteca.cargar_en_flowbox(&canciones);
    biblioteca.configurar_tabla(canciones);
        
    configurar_botones_vista(&right_box, stack);
        
    window.present();
    
    return Ok(());
}

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

fn configurar_botones_vista(_right_box: &Box, _stack: Stack) {

}
