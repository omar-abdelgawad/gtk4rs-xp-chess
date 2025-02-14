use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Image};

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Button with Image")
        .default_width(300)
        .default_height(300)
        .build();

    let button = Button::new();

    // Load an image from file
    let image = Image::from_file("./resources/Chess_bdt60.png"); // Replace with your image path
    button.set_child(Some(&image));

    window.set_child(Some(&button));
    window.present();
}

fn main() {
    let app = Application::builder()
        .application_id("com.example.imagebutton")
        .build();

    app.connect_activate(build_ui);
    app.run();
}
