extern crate gtk;

use gtk::prelude::*;
use gtk::{Window, WindowType, Label};

fn main() {
    if gtk::init().is_err(){
        panic!("No se pudo iniciar GTK");
    }

    let window = Window::new(WindowType::Toplevel);

    window.connect_delete_event(|_,_|{gtk::main_quit(); Inhibit(false) });
    window.set_title("Beta");
    window.set_default_size(300, 300);
    let label = Label::new(Some("Cualquier cosa"));
    window.add(&label);
    window.show_all();
    gtk::main();
}
