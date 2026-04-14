use gtk::Builder;

pub fn cargar_ui() -> Builder {
    return Builder::from_string(include_str!("ui/main.ui"));
}
